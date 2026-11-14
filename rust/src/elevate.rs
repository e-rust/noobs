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
//! O UAC pergunta a cada início com a opção ligada. Até a 0.0.9 uma tarefa agendada com
//! "privilégios mais altos" abria o NoOBS elevado sem perguntar, mas o Defender marcou o
//! conjunto (exe sem assinatura + tarefa elevada criada por ele mesmo + chave Run) como
//! `Behavior:Win32/Persistence.A!ml` e pôs o NoOBS em quarentena. A tarefa saiu: quem ainda
//! a tem perde ela no próximo início elevado ([`remove_old_task`]).

use std::os::windows::process::CommandExt;

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
        let err = unsafe { GetLastError().0 };
        log!("Elevate: nao foi possivel elevar (erro {err}) — segue sem administrador.");
        on_uac_refused(err);
        false
    }
}

/// O UAC recusado (1223, `ERROR_CANCELLED`): o usuário disse não, ou não tem a senha de um
/// administrador (computador de empresa). A opção desliga para não perguntar a cada início;
/// ele religa em Configurações → Geral. No início com o Windows não: lá o UAC pode ter
/// expirado sem ninguém na frente.
fn on_uac_refused(err: u32) {
    if err == 1223 && !crate::win::cmdline_lower().contains("/autostart") {
        set_wanted(false);
        log!("Elevate: o UAC foi recusado: \"Abrir como administrador\" desligado.");
    }
}

/// A escolha do instalador (a caixa "Abrir como administrador", `publish.windows.run_as_admin`
/// no projeto): `HKCU\Software\NoOBS\RunAsAdmin` = 1 ou 0. É a mesma da opção em
/// Configurações → Geral: as duas ficam iguais (ao atualizar, o instalador mostra a caixa
/// como ela está).
const CHOICE_KEY: PCWSTR = w!("Software\\NoOBS");
const CHOICE_VALUE: PCWSTR = w!("RunAsAdmin");

/// O que está no registro (`None`: nada gravado ainda).
fn read_choice() -> Option<bool> {
    use windows::Win32::System::Registry::{RegGetValueW, HKEY_CURRENT_USER, RRF_RT_REG_DWORD};
    if cfg!(test) {
        return None;
    }
    let mut value = 0u32;
    let mut size = 4u32;
    let ok = unsafe { RegGetValueW(HKEY_CURRENT_USER, CHOICE_KEY, CHOICE_VALUE, RRF_RT_REG_DWORD, None, Some(&mut value as *mut u32 as *mut _), Some(&mut size)) }.is_ok();
    ok.then_some(value != 0)
}

fn write_choice(on: bool) {
    use windows::Win32::System::Registry::{RegSetKeyValueW, HKEY_CURRENT_USER, REG_DWORD};
    if cfg!(test) {
        return;
    }
    let value = u32::from(on);
    let _ = unsafe { RegSetKeyValueW(HKEY_CURRENT_USER, CHOICE_KEY, CHOICE_VALUE, REG_DWORD.0, Some(&value as *const u32 as *const _), 4) };
}

/// No início, antes de [`relaunch_elevated_if_wanted`]: a escolha do instalador vale para a
/// opção. Sem nada no registro (instalado antes da caixa, ou o de desenvolvimento), a opção
/// fica como está e passa para o registro.
pub fn sync_installer_choice() {
    let current = config::get_config_bool("runAsAdmin", false);
    match read_choice() {
        Some(on) if on != current => {
            config::set_config_bool("runAsAdmin", on);
            log!("Elevate: abrir como administrador = {on} (escolha do instalador).");
        }
        Some(_) => {}
        None => write_choice(current),
    }
}

/// Liga ou desliga "Abrir como administrador" (o config e o registro do instalador).
pub fn set_wanted(on: bool) {
    config::set_config_bool("runAsAdmin", on);
    write_choice(on);
}

/// `CREATE_NO_WINDOW`: o `schtasks` sem piscar um console.
const NO_WINDOW: u32 = 0x0800_0000;

/// O nome da tarefa que a 0.0.9 criava para este exe: "NoOBS elevado" e um código do
/// caminho (FNV-1a).
fn old_task_name() -> String {
    let exe = std::env::current_exe().map(|p| p.to_string_lossy().into_owned()).unwrap_or_default();
    let mut h: u32 = 0x811C_9DC5;
    for b in exe.to_lowercase().bytes() {
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

/// Elevado: apaga a tarefa da 0.0.9, se ela existe (numa thread: o `schtasks` leva uns
/// décimos de segundo). Só elevado dá para apagar uma tarefa com privilégios mais altos.
pub fn remove_old_task() {
    if !is_process_elevated() {
        return;
    }
    let _ = std::thread::Builder::new().name("noobs-elevate-task".into()).spawn(|| {
        let name = old_task_name();
        if schtasks(&["/query", "/tn", &name]) {
            let ok = schtasks(&["/delete", "/tn", &name, "/f"]);
            log!("Elevate: a tarefa antiga \"{name}\" {}.", if ok { "foi apagada" } else { "nao foi apagada" });
        }
    });
}

/// Elevado, deixa o próprio usuário, sem elevação, ENCERRAR este processo. O instalador
/// roda sem administrador e fecha o NoOBS aberto com `taskkill /F`; sem isto, o Windows
/// nega (o processo elevado só aceita isso de administrador, e o rótulo de integridade
/// alta barra quem é de integridade média), o NoOBS fica aberto e o instalador não
/// consegue gravar o `NoOBS.exe`. Libera só encerrar e consultar: nada de ler a memória
/// nem injetar código. No-op sem elevação.
pub fn let_user_terminate() {
    use windows::Win32::Foundation::{LocalFree, HLOCAL};
    use windows::Win32::Security::Authorization::{
        ConvertStringSecurityDescriptorToSecurityDescriptorW, GetSecurityInfo, SetEntriesInAclW, SetSecurityInfo, EXPLICIT_ACCESS_W, GRANT_ACCESS,
        SDDL_REVISION_1, SE_KERNEL_OBJECT, TRUSTEE_IS_SID, TRUSTEE_IS_USER, TRUSTEE_W,
    };
    use windows::Win32::Security::{
        GetSecurityDescriptorSacl, TokenUser, ACL, DACL_SECURITY_INFORMATION, LABEL_SECURITY_INFORMATION, NO_INHERITANCE, PSECURITY_DESCRIPTOR, TOKEN_USER,
    };
    use windows::Win32::System::Threading::{PROCESS_QUERY_LIMITED_INFORMATION, PROCESS_SYNCHRONIZE, PROCESS_TERMINATE};

    if !is_process_elevated() {
        return;
    }
    unsafe {
        let me = GetCurrentProcess();
        // O usuário deste token (o mesmo do instalador: o token elevado e o comum têm o
        // mesmo SID de usuário).
        let mut tok = HANDLE::default();
        if OpenProcessToken(me, TOKEN_QUERY, &mut tok).is_err() {
            return;
        }
        let mut buf = vec![0u64; 64];
        let mut len = 0u32;
        let ok = GetTokenInformation(tok, TokenUser, Some(buf.as_mut_ptr() as *mut _), (buf.len() * 8) as u32, &mut len).is_ok();
        let _ = CloseHandle(tok);
        if !ok {
            log!("Elevate: o usuario do token nao saiu; o instalador nao vai conseguir fechar o NoOBS elevado.");
            return;
        }
        let sid = (*(buf.as_ptr() as *const TOKEN_USER)).User.Sid;

        // A DACL de agora + o usuário podendo encerrar.
        let mut old_dacl: *mut ACL = std::ptr::null_mut();
        let mut sd = PSECURITY_DESCRIPTOR::default();
        let err = GetSecurityInfo(me, SE_KERNEL_OBJECT, DACL_SECURITY_INFORMATION, None, None, Some(&mut old_dacl), None, Some(&mut sd));
        if err.0 != 0 {
            log!("Elevate: GetSecurityInfo falhou ({}).", err.0);
            return;
        }
        let ea = EXPLICIT_ACCESS_W {
            grfAccessPermissions: PROCESS_TERMINATE.0 | PROCESS_QUERY_LIMITED_INFORMATION.0 | PROCESS_SYNCHRONIZE.0,
            grfAccessMode: GRANT_ACCESS,
            grfInheritance: NO_INHERITANCE,
            Trustee: TRUSTEE_W { TrusteeForm: TRUSTEE_IS_SID, TrusteeType: TRUSTEE_IS_USER, ptstrName: windows::core::PWSTR(sid.0 as *mut u16), ..Default::default() },
        };
        let mut new_dacl: *mut ACL = std::ptr::null_mut();
        let err = SetEntriesInAclW(Some(&[ea]), Some(old_dacl), &mut new_dacl);
        let _ = LocalFree(Some(HLOCAL(sd.0)));
        if err.0 != 0 {
            log!("Elevate: SetEntriesInAcl falhou ({}).", err.0);
            return;
        }

        // Rótulo de integridade média, barrando só a escrita (o padrão do processo
        // elevado é alta): quem é de integridade média passa a poder encerrar.
        let mut label_sd = PSECURITY_DESCRIPTOR::default();
        let mut sacl: *mut ACL = std::ptr::null_mut();
        let have_label = ConvertStringSecurityDescriptorToSecurityDescriptorW(w!("S:(ML;;NW;;;ME)"), SDDL_REVISION_1, &mut label_sd, None).is_ok() && {
            let (mut present, mut defaulted) = (windows::core::BOOL(0), windows::core::BOOL(0));
            GetSecurityDescriptorSacl(label_sd, &mut present, &mut sacl, &mut defaulted).is_ok() && present.as_bool()
        };
        let what = if have_label { DACL_SECURITY_INFORMATION | LABEL_SECURITY_INFORMATION } else { DACL_SECURITY_INFORMATION };
        let err = SetSecurityInfo(me, SE_KERNEL_OBJECT, what, None, None, Some(new_dacl), have_label.then_some(sacl as *const ACL));
        let _ = LocalFree(Some(HLOCAL(new_dacl as *mut _)));
        if !label_sd.0.is_null() {
            let _ = LocalFree(Some(HLOCAL(label_sd.0)));
        }
        if err.0 == 0 {
            log!("Elevate: o instalador (sem administrador) pode fechar este processo.");
        } else {
            log!("Elevate: SetSecurityInfo falhou ({}); o instalador nao vai conseguir fechar o NoOBS elevado.", err.0);
        }
    }
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
