//! O que o log grava quando o NoOBS cai, para descobrir depois o que aconteceu numa
//! máquina onde não dá para reproduzir.
//!
//! * **panic** (erro do Rust, em qualquer thread): a mensagem, o local, a thread e a
//!   pilha como `módulo+deslocamento`, que o `.pdb` guardado da versão traduz em função
//!   e linha.
//! * **crash nativo** (acesso inválido, instrução ilegal... num driver, numa DLL ou no
//!   código): o código da exceção, onde caiu, a pilha e um minidump
//!   (`NoOBS_crash_*.dmp`, ao lado do log), que o WinDbg/Visual Studio abrem com as
//!   threads e a memória da hora. Ficam os 3 mais recentes.
//! * **estouro de pilha**: uma linha (não há pilha para mais nada).
//! * **execução anterior que não fechou** (crash que nem o filtro pega, o processo
//!   encerrado à força, o Windows desligando): a próxima abertura diz no log.

use std::os::windows::io::AsRawHandle;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};

use windows::core::PCWSTR;
use windows::Win32::Foundation::{EXCEPTION_ACCESS_VIOLATION, EXCEPTION_STACK_OVERFLOW, HANDLE, HMODULE};
use windows::Win32::System::Diagnostics::Debug::{
    AddVectoredExceptionHandler, MiniDumpWithIndirectlyReferencedMemory, MiniDumpWithThreadInfo, MiniDumpWithUnloadedModules, MiniDumpWriteDump,
    RtlCaptureStackBackTrace, SetUnhandledExceptionFilter, CONTEXT, EXCEPTION_POINTERS, MINIDUMP_EXCEPTION_INFORMATION, MINIDUMP_TYPE,
};
use windows::Win32::System::LibraryLoader::{GetModuleFileNameW, GetModuleHandleExW, GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS, GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT};
use windows::Win32::System::Threading::{GetCurrentProcess, GetCurrentProcessId, GetCurrentThreadId};

use crate::log;

/// Quantos minidumps ficam na pasta do log.
const KEEP_DUMPS: usize = 3;
/// Quadros da pilha no log.
const MAX_FRAMES: usize = 48;
/// `EXCEPTION_CONTINUE_SEARCH`: o Windows segue como antes (o relatório de erro e o fim do
/// processo); o filtro só registra.
const CONTINUE_SEARCH: i32 = 0;

/// Já estamos registrando uma queda: uma segunda (no próprio filtro) não entra.
static IN_CRASH: AtomicBool = AtomicBool::new(false);

/// Liga tudo. Uma vez, logo depois de abrir o log.
pub fn install() {
    check_previous_run();
    std::panic::set_hook(Box::new(on_panic));
    unsafe {
        SetUnhandledExceptionFilter(Some(on_unhandled));
        // Primeiro da fila: só olha o estouro de pilha (os demais seguem para quem trata).
        AddVectoredExceptionHandler(1, Some(on_vectored));
    }
}

/// O processo está fechando normalmente: a próxima abertura não acusa queda.
pub fn mark_clean_exit() {
    let _ = std::fs::remove_file(running_flag());
}

// ---------------------------------------------------------------------------
// Execução anterior

fn running_flag() -> PathBuf {
    log::log_dir().join("running.flag")
}

/// A marca de "rodando" ficou da execução anterior: ela não fechou normalmente.
fn check_previous_run() {
    let flag = running_flag();
    if let Ok(prev) = std::fs::read_to_string(&flag) {
        log::log(format!(
            "AVISO: a execucao anterior ({}) nao fechou normalmente. Se nao ha PANIC/CRASH logo antes deste startup, ela foi encerrada a forca, o Windows desligou ou caiu por um caminho que nao passa pelo filtro (abort); o log dela parou na ultima linha acima.",
            prev.trim()
        ));
    }
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
    let _ = std::fs::write(&flag, format!("pid {}, iniciada em {now}, versao {}", std::process::id(), crate::version::APP_VERSION));
}

// ---------------------------------------------------------------------------
// panic

fn on_panic(info: &std::panic::PanicHookInfo<'_>) {
    let msg = info
        .payload()
        .downcast_ref::<&str>()
        .map(|s| (*s).to_owned())
        .or_else(|| info.payload().downcast_ref::<String>().cloned())
        .unwrap_or_else(|| "(sem mensagem)".into());
    let loc = info.location().map(|l| format!("{}:{}", l.file(), l.line())).unwrap_or_default();
    log::log(format!("PANIC em {loc} (thread {}): {msg}", thread_label()));
    // Os endereços como módulo+deslocamento: o `.pdb` guardado da versão traduz em função e
    // linha. (O `Backtrace` do Rust não serve aqui: sem o `.pdb` ao lado do exe, que o
    // instalador não leva, ele só dá `<unknown>` e nomes errados de outras DLLs.)
    let mut frames = [std::ptr::null_mut(); MAX_FRAMES];
    let n = unsafe { RtlCaptureStackBackTrace(1, &mut frames, None) } as usize;
    log_frames("PANIC pilha", frames[..n].iter().map(|f| *f as usize));
}

// ---------------------------------------------------------------------------
// Crash nativo

unsafe extern "system" fn on_vectored(ep: *mut EXCEPTION_POINTERS) -> i32 {
    let code = unsafe { ep.as_ref().and_then(|e| e.ExceptionRecord.as_ref()).map(|r| r.ExceptionCode) };
    if code == Some(EXCEPTION_STACK_OVERFLOW) && !IN_CRASH.swap(true, Ordering::SeqCst) {
        // Pouca pilha sobrando: só uma linha, sem dump.
        log::log("CRASH: estouro de pilha (0xC00000FD) — o processo vai fechar.");
        IN_CRASH.store(false, Ordering::SeqCst);
    }
    CONTINUE_SEARCH
}

unsafe extern "system" fn on_unhandled(ep: *const EXCEPTION_POINTERS) -> i32 {
    if IN_CRASH.swap(true, Ordering::SeqCst) {
        return CONTINUE_SEARCH;
    }
    let Some(e) = (unsafe { ep.as_ref() }) else { return CONTINUE_SEARCH };
    let Some(rec) = (unsafe { e.ExceptionRecord.as_ref() }) else { return CONTINUE_SEARCH };
    let code = rec.ExceptionCode.0 as u32;
    let at = rec.ExceptionAddress as usize;
    let detail = if rec.ExceptionCode == EXCEPTION_ACCESS_VIOLATION && rec.NumberParameters >= 2 {
        let op = match rec.ExceptionInformation[0] {
            0 => "leitura",
            1 => "escrita",
            8 => "execucao",
            _ => "acesso",
        };
        format!(" ({op} invalida em 0x{:X})", rec.ExceptionInformation[1])
    } else {
        String::new()
    };
    // Primeiro a linha (o mais barato), depois o dump, por último a pilha: se algo cair
    // no meio, o que veio antes já está no disco.
    log::log(format!("CRASH: excecao 0x{code:08X}{detail} em {} (thread {})", module_offset(at), thread_label()));
    match write_dump(ep) {
        Ok(path) => log::log(format!("CRASH: minidump em {}", path.display())),
        Err(err) => log::log(format!("CRASH: o minidump nao saiu ({err})")),
    }
    if let Some(ctx) = unsafe { e.ContextRecord.as_ref() } {
        log_frames("CRASH pilha", walk_stack(ctx).into_iter());
    }
    log::log("CRASH: o processo vai fechar.");
    CONTINUE_SEARCH
}

/// O minidump da hora (threads, módulos e a memória que as pilhas apontam).
fn write_dump(ep: *const EXCEPTION_POINTERS) -> Result<PathBuf, String> {
    let dir = log::log_dir();
    let path = dir.join(format!("NoOBS_crash_{}.dmp", chrono::Local::now().format("%Y-%m-%d_%H-%M-%S")));
    let file = std::fs::File::create(&path).map_err(|e| e.to_string())?;
    let info = MINIDUMP_EXCEPTION_INFORMATION { ThreadId: unsafe { GetCurrentThreadId() }, ExceptionPointers: ep as *mut _, ClientPointers: false.into() };
    let kind = MINIDUMP_TYPE(MiniDumpWithThreadInfo.0 | MiniDumpWithUnloadedModules.0 | MiniDumpWithIndirectlyReferencedMemory.0);
    let ok = unsafe { MiniDumpWriteDump(GetCurrentProcess(), GetCurrentProcessId(), HANDLE(file.as_raw_handle()), kind, Some(&info), None, None) };
    drop(file);
    prune_dumps(&dir);
    ok.map(|_| path).map_err(|e| e.to_string())
}

fn prune_dumps(dir: &std::path::Path) {
    let Ok(rd) = std::fs::read_dir(dir) else { return };
    let mut names: Vec<String> = rd
        .filter_map(|e| e.ok())
        .filter_map(|e| e.file_name().into_string().ok())
        .filter(|n| n.starts_with("NoOBS_crash_") && n.ends_with(".dmp"))
        .collect();
    names.sort();
    if names.len() > KEEP_DUMPS {
        for old in &names[..names.len() - KEEP_DUMPS] {
            let _ = std::fs::remove_file(dir.join(old));
        }
    }
}

/// A pilha a partir do contexto da exceção (as tabelas de unwind do x64).
#[cfg(target_arch = "x86_64")]
fn walk_stack(ctx: &CONTEXT) -> Vec<usize> {
    use windows::Win32::System::Diagnostics::Debug::{RtlLookupFunctionEntry, RtlVirtualUnwind, UNW_FLAG_NHANDLER};
    let mut c = *ctx;
    let mut out = Vec::new();
    while out.len() < MAX_FRAMES && c.Rip != 0 {
        out.push(c.Rip as usize);
        let mut base = 0u64;
        let f = unsafe { RtlLookupFunctionEntry(c.Rip, &mut base, None) };
        if f.is_null() {
            // Função folha: o retorno está no topo da pilha.
            if c.Rsp == 0 {
                break;
            }
            c.Rip = unsafe { *(c.Rsp as *const u64) };
            c.Rsp += 8;
        } else {
            let mut handler = std::ptr::null_mut();
            let mut frame = 0u64;
            unsafe { RtlVirtualUnwind(UNW_FLAG_NHANDLER, base, c.Rip, f, &mut c, &mut handler, &mut frame, None) };
        }
    }
    out
}

#[cfg(not(target_arch = "x86_64"))]
fn walk_stack(_ctx: &CONTEXT) -> Vec<usize> {
    Vec::new()
}

// ---------------------------------------------------------------------------
// Comum

fn log_frames(title: &str, frames: impl Iterator<Item = usize>) {
    let list: Vec<String> = frames.enumerate().map(|(i, a)| format!("  #{i:02} {}", module_offset(a))).collect();
    log::log(format!("{title} ({} quadros):", list.len()));
    for line in list {
        log::log(line);
    }
}

/// `NoOBS.exe+0x1A2B3C` (o módulo do endereço e o deslocamento dentro dele).
fn module_offset(addr: usize) -> String {
    let mut module = HMODULE::default();
    let flags = GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT;
    if unsafe { GetModuleHandleExW(flags, PCWSTR(addr as *const u16), &mut module) }.is_err() || module.is_invalid() {
        return format!("0x{addr:X} (fora de modulo)");
    }
    let mut buf = [0u16; 260];
    let n = unsafe { GetModuleFileNameW(Some(module), &mut buf) } as usize;
    let path = String::from_utf16_lossy(&buf[..n]);
    let name = path.rsplit(['\\', '/']).next().unwrap_or("?");
    format!("{name}+0x{:X}", addr - module.0 as usize)
}

fn thread_label() -> String {
    let t = std::thread::current();
    format!("{} / id {}", t.name().unwrap_or("?"), unsafe { GetCurrentThreadId() })
}
