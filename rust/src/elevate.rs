//! "Abrir como administrador" (`runAsAdmin`).
//!
//! Jogo rodando como administrador esconde o teclado de quem não é (UIPI): o
//! atalho de gravar não chega por nenhum método. Elevado, o
//! `RegisterHotKey` de sempre chega.
//!
//! O dispatcher do `main` chama [`relaunch_elevated_if_wanted`] ANTES de tudo
//! (mutex, janela, motor). UAC recusado = segue normal: a opção é um pedido,
//! nunca um bloqueio.
//!
//! O mesmo UIPI ao contrário: janela ELEVADA não recebe mensagens acima de
//! `WM_USER` de quem não é (clique na bandeja, `TaskbarCreated`, aviso de uma
//! 2ª instância). [`allow_from_lower_il`] libera essas.
//!
//! Sem o UAC a cada início: na primeira vez elevado, o NoOBS cria uma tarefa agendada
//! com "privilégios mais altos" ([`ensure_task`]). Daí em diante, aberto sem
//! elevação (atalho, início com o Windows), ele dispara a tarefa e sai: a tarefa o abre
//! já elevado, sem perguntar. A tarefa não leva argumentos: os do lançador (o
//! `/autostart`) vão por um arquivo que a cópia elevada lê ([`take_handoff`]). Uma
//! tarefa por exe (o nome leva um código do caminho): o NoOBS instalado e o de
//! desenvolvimento não abrem um ao outro.

use std::os::windows::process::CommandExt;
use std::path::PathBuf;
use std::sync::OnceLock;

use windows::core::{w, HSTRING, PCWSTR};
use windows::Win32::Foundation::{CloseHandle, GetLastError, HANDLE, HWND};
use windows::Win32::Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY};
use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};
use windows::Win32::UI::Shell::{ShellExecuteExW, SEE_MASK_NOASYNC, SHELLEXECUTEINFOW};
use windows::Win32::UI::WindowsAndMessaging::{ChangeWindowMessageFilterEx, MSGFLT_ALLOW, SW_SHOWNORMAL};

use crate::{config, log};

/// O processo atual tem token de administrador?
pub fn is_process_elevated() -> bool {
    unsafe {
        let mut tok = HANDLE::default();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut tok).is_err() {
            return false;
        }
        let mut elev = TOKEN_ELEVATION::default();
        let mut len = 0u32;
        let ok = GetTokenInformation(
            tok,
            TokenElevation,
            Some(&mut elev as *mut _ as *mut _),
            std::mem::size_of::<TOKEN_ELEVATION>() as u32,
            &mut len,
        )
        .is_ok();
        let _ = CloseHandle(tok);
        ok && elev.TokenIsElevated != 0
    }
}

/// Argumentos originais (sem o exe), re-entre-aspados quando têm espaço.
fn original_args() -> String {
    std::env::args()
        .skip(1)
        .map(|a| if a.contains(' ') { format!("\"{a}\"") } else { a })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Com `runAsAdmin` ligado e sem elevação, reabre elevado e devolve `true`
/// (quem chamou sai na hora). `false` = seguir normal.
pub fn relaunch_elevated_if_wanted() -> bool {
    if !config::get_config_bool("runAsAdmin", false) {
        return false;
    }
    if is_process_elevated() {
        log::log("Elevate: rodando como administrador.");
        return false;
    }
    let args_s = original_args();
    // A tarefa (criada na primeira vez elevado): abre elevado sem o UAC.
    if run_task(&args_s) {
        log!("Elevate: aberto como administrador pela tarefa \"{}\" (args=\"{args_s}\"); este processo sai.", task_name());
        return true;
    }
    let exe = HSTRING::from(std::env::current_exe().map(|p| p.to_string_lossy().into_owned()).unwrap_or_default());
    let args = HSTRING::from(args_s.as_str());
    let mut info = SHELLEXECUTEINFOW {
        cbSize: std::mem::size_of::<SHELLEXECUTEINFOW>() as u32,
        fMask: SEE_MASK_NOASYNC,
        lpVerb: w!("runas"),
        lpFile: PCWSTR(exe.as_ptr()),
        lpParameters: PCWSTR(args.as_ptr()),
        nShow: SW_SHOWNORMAL.0,
        ..Default::default()
    };
    if unsafe { ShellExecuteExW(&mut info) }.is_ok() {
        log!("Elevate: reaberto como administrador (args=\"{args_s}\"); este processo sai.");
        true
    } else {
        // 1223 = ERROR_CANCELLED (o usuário disse não no UAC).
        log!("Elevate: nao foi possivel elevar (erro {}) — segue sem administrador.", unsafe { GetLastError().0 });
        false
    }
}

// ---------------------------------------------------------------------------
// A tarefa agendada (abrir elevado sem o UAC)

/// `CREATE_NO_WINDOW`: o `schtasks` sem piscar um console.
const NO_WINDOW: u32 = 0x0800_0000;
/// Por quanto tempo os argumentos deixados pelo lançador valem (o elevado sobe em
/// menos de um segundo; um arquivo velho é de uma tentativa que não deu certo).
const HANDOFF_SECS: u64 = 30;

fn current_exe() -> String {
    std::env::current_exe().map(|p| p.to_string_lossy().into_owned()).unwrap_or_default()
}

/// O nome da tarefa deste exe: "NoOBS elevado" e um código do caminho (FNV-1a).
pub fn task_name() -> String {
    let mut h: u32 = 0x811C_9DC5;
    for b in current_exe().to_lowercase().bytes() {
        h = (h ^ u32::from(b)).wrapping_mul(0x0100_0193);
    }
    format!("NoOBS elevado {h:08x}")
}

fn schtasks(args: &[&str]) -> bool {
    // Teste: sem mexer no Agendador de Tarefas de verdade.
    if cfg!(test) {
        let _ = args;
        return false;
    }
    std::process::Command::new("schtasks")
        .args(args)
        .creation_flags(NO_WINDOW)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .is_ok_and(|s| s.success())
}

fn task_exists() -> bool {
    schtasks(&["/query", "/tn", &task_name()])
}

fn handoff_path() -> PathBuf {
    crate::config::config_dir().join("elevate-args.txt")
}

/// Sem elevação: deixa os argumentos para a cópia elevada e dispara a tarefa. `false`:
/// não há tarefa (ainda) ou ela não rodou — segue pelo UAC.
fn run_task(args: &str) -> bool {
    if !task_exists() {
        return false;
    }
    let _ = std::fs::write(handoff_path(), args);
    // A janela da cópia elevada pode vir para a frente (a tarefa não é "o usuário").
    unsafe {
        let _ = windows::Win32::UI::WindowsAndMessaging::AllowSetForegroundWindow(windows::Win32::UI::WindowsAndMessaging::ASFW_ANY);
    }
    if schtasks(&["/run", "/tn", &task_name()]) {
        return true;
    }
    let _ = std::fs::remove_file(handoff_path());
    log!("Elevate: a tarefa \"{}\" nao rodou — segue pelo UAC.", task_name());
    false
}

static HANDOFF: OnceLock<String> = OnceLock::new();

/// No começo, antes de ler a linha de comando: a cópia elevada aberta pela tarefa (sem
/// argumentos) pega os que o lançador deixou, e apaga o arquivo.
pub fn take_handoff() {
    let mut got = String::new();
    if std::env::args().len() <= 1 && is_process_elevated() {
        let path = handoff_path();
        let fresh = std::fs::metadata(&path).and_then(|m| m.modified()).ok().and_then(|t| t.elapsed().ok()).is_some_and(|age| age.as_secs() <= HANDOFF_SECS);
        if fresh {
            got = std::fs::read_to_string(&path).unwrap_or_default().trim().to_owned();
        }
        let _ = std::fs::remove_file(&path);
    }
    let _ = HANDOFF.set(got);
}

/// Os argumentos que vieram do lançador (vazio: nenhum).
pub fn handoff() -> &'static str {
    HANDOFF.get().map_or("", String::as_str)
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

/// A definição da tarefa: abrir o exe com privilégios mais altos, só sob pedido (sem
/// gatilho), sem limite de tempo, mesmo na bateria, com prioridade NORMAL (o padrão das
/// tarefas é abaixo do normal, ruim para gravar) e em paralelo (a 2ª instância avisa a
/// 1ª, como sempre).
fn task_xml(exe: &str, user: &str) -> String {
    let dir = std::path::Path::new(exe).parent().map(|p| p.to_string_lossy().into_owned()).unwrap_or_default();
    format!(
        r#"<?xml version="1.0" encoding="UTF-16"?>
<Task version="1.2" xmlns="http://schemas.microsoft.com/windows/2004/02/mit/task">
  <RegistrationInfo>
    <Description>Abre o NoOBS como administrador sem a pergunta do UAC (Configurações → Geral → Abrir como administrador).</Description>
  </RegistrationInfo>
  <Principals>
    <Principal id="Author">
      <UserId>{user}</UserId>
      <LogonType>InteractiveToken</LogonType>
      <RunLevel>HighestAvailable</RunLevel>
    </Principal>
  </Principals>
  <Settings>
    <MultipleInstancesPolicy>Parallel</MultipleInstancesPolicy>
    <DisallowStartIfOnBatteries>false</DisallowStartIfOnBatteries>
    <StopIfGoingOnBatteries>false</StopIfGoingOnBatteries>
    <AllowStartOnDemand>true</AllowStartOnDemand>
    <Enabled>true</Enabled>
    <ExecutionTimeLimit>PT0S</ExecutionTimeLimit>
    <Priority>4</Priority>
  </Settings>
  <Actions Context="Author">
    <Exec>
      <Command>"{exe}"</Command>
      <WorkingDirectory>{dir}</WorkingDirectory>
    </Exec>
  </Actions>
</Task>
"#,
        user = xml_escape(user),
        exe = xml_escape(exe),
        dir = xml_escape(&dir),
    )
}

/// Elevado com a opção ligada: cria a tarefa deste exe se ainda não existe (numa thread:
/// o `schtasks` leva uns décimos de segundo). Só elevado dá para criar uma tarefa com
/// privilégios mais altos.
pub fn ensure_task() {
    if !config::get_config_bool("runAsAdmin", false) || !is_process_elevated() {
        return;
    }
    let _ = std::thread::Builder::new().name("noobs-elevate-task".into()).spawn(|| {
        if task_exists() {
            return;
        }
        let user = format!("{}\\{}", std::env::var("USERDOMAIN").unwrap_or_default(), std::env::var("USERNAME").unwrap_or_default());
        let xml = task_xml(&current_exe(), &user);
        // O `schtasks /xml` quer UTF-16 com BOM.
        let path = std::env::temp_dir().join(format!("noobs-task-{}.xml", std::process::id()));
        let bytes: Vec<u8> = [0xFFu8, 0xFE].into_iter().chain(xml.encode_utf16().flat_map(u16::to_le_bytes)).collect();
        let ok = std::fs::write(&path, bytes).is_ok() && schtasks(&["/create", "/tn", &task_name(), "/xml", &path.to_string_lossy(), "/f"]);
        let _ = std::fs::remove_file(&path);
        if ok {
            log!("Elevate: tarefa \"{}\" criada — os proximos inicios sobem como administrador sem o UAC.", task_name());
        } else {
            log!("Elevate: nao deu para criar a tarefa \"{}\" — o UAC continua perguntando.", task_name());
        }
    });
}

/// A opção desligada: tira a tarefa (precisa de elevação; sem ela, a tarefa fica, mas
/// nada a usa com a opção desligada).
pub fn remove_task() {
    let _ = std::thread::Builder::new().name("noobs-elevate-task".into()).spawn(|| {
        if task_exists() {
            let ok = schtasks(&["/delete", "/tn", &task_name(), "/f"]);
            log!("Elevate: tarefa \"{}\" {}.", task_name(), if ok { "removida" } else { "nao removida (sem administrador)" });
        }
    });
}

/// Deixa a janela receber `msgs` de processos sem elevação (no-op sem elevação).
pub fn allow_from_lower_il(hwnd: HWND, msgs: &[u32]) {
    if hwnd.0.is_null() || !is_process_elevated() {
        return;
    }
    for &m in msgs.iter().filter(|m| **m != 0) {
        unsafe {
            let _ = ChangeWindowMessageFilterEx(hwnd, m, MSGFLT_ALLOW, None);
        }
    }
}
