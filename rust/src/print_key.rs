//! A tecla Print Screen nos atalhos.
//!
//! O `RegisterHotKey` aceita a Print Screen, mas não a tira do Windows: com a opção
//! "Usar a tecla Print Screen para abrir a captura de tela" (ligada por padrão no
//! Windows 11), a Ferramenta de Captura abriria junto com o print do NoOBS. Por isso os
//! atalhos com ela vão por um hook de teclado de baixo nível (`WH_KEYBOARD_LL`), que
//! roda ANTES dos atalhos do sistema: a combinação configurada dispara o atalho e é
//! engolida (descida e subida), e o Windows não a vê. Outras combinações com a Print
//! Screen (Alt+Print Screen, se só a tecla sozinha está configurada) passam normalmente.
//!
//! Com uma janela de administrador à frente, o Windows não passa a tecla ao hook (o
//! NoOBS sem privilégio não vê as teclas de um processo elevado: UIPI). O
//! `RegisterHotKey` também não a recebe nesse caso (testado em 13/11/2026). Só o NoOBS
//! aberto como administrador (Configurações → Geral) pega o atalho por cima deles.
//!
//! O hook fica numa thread só dele, com o próprio laço de mensagens: o Windows remove
//! em silêncio um hook que demora a responder, e a thread principal às vezes está
//! ocupada. O hook só compara e põe o disparo na fila da main ([`mainthread::queue`]).

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Mutex;

use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::Input::{RegisterRawInputDevices, RAWINPUTDEVICE, RIDEV_REMOVE};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::Threading::GetCurrentThreadId;
use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_CONTROL, VK_LWIN, VK_MENU, VK_RWIN, VK_SHIFT};
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, GetMessageW, PostThreadMessageW, SetWindowsHookExW, UnhookWindowsHookEx, HC_ACTION, KBDLLHOOKSTRUCT, MSG, WH_KEYBOARD_LL, WM_KEYDOWN,
    WM_KEYUP, WM_QUIT, WM_SYSKEYDOWN, WM_SYSKEYUP,
};

use crate::hotkey::{MOD_ALT_, MOD_CTRL, MOD_SHIFT_, MOD_WIN_, VK_SNAPSHOT};
use crate::{log, mainthread};

/// (modificadores, índice do item) de cada atalho com a Print Screen.
static BINDINGS: Mutex<Vec<(u32, usize)>> = Mutex::new(Vec::new());
static ON_PRESS: Mutex<Option<fn(usize)>> = Mutex::new(None);
/// A thread do hook (0 = sem hook).
static THREAD: AtomicU32 = AtomicU32::new(0);
/// A descida foi engolida: a subida também é.
static HELD: AtomicBool = AtomicBool::new(false);

/// Troca os atalhos com a Print Screen. Vazio: tira o hook (a tecla volta a ser do
/// Windows). Main thread.
pub fn set(bindings: Vec<(u32, usize)>, on_press: fn(usize)) {
    let empty = bindings.is_empty();
    *BINDINGS.lock().unwrap_or_else(|p| p.into_inner()) = bindings;
    *ON_PRESS.lock().unwrap_or_else(|p| p.into_inner()) = Some(on_press);
    if empty {
        stop();
    } else if THREAD.load(Ordering::SeqCst) == 0 {
        start();
    }
}

/// Tira o hook (desligar o app, ou nenhum atalho com a Print Screen).
pub fn stop() {
    BINDINGS.lock().unwrap_or_else(|p| p.into_inner()).clear();
    let tid = THREAD.swap(0, Ordering::SeqCst);
    if tid != 0 {
        unsafe {
            let _ = PostThreadMessageW(tid, WM_QUIT, WPARAM(0), LPARAM(0));
        }
        log!("PrintKey: hook removido (a Print Screen volta a ser do Windows).");
    }
}

fn start() {
    let (tx, rx) = std::sync::mpsc::channel::<bool>();
    let spawned = std::thread::Builder::new().name("noobs-printkey".into()).spawn(move || unsafe {
        let hmod = GetModuleHandleW(None).ok();
        let hook = match SetWindowsHookExW(WH_KEYBOARD_LL, Some(hook_proc), hmod.map(Into::into), 0) {
            Ok(h) => h,
            Err(e) => {
                log!("PrintKey: o hook de teclado nao subiu ({e}).");
                let _ = tx.send(false);
                return;
            }
        };
        THREAD.store(GetCurrentThreadId(), Ordering::SeqCst);
        let _ = tx.send(true);
        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).as_bool() {}
        let _ = UnhookWindowsHookEx(hook);
    });
    match spawned {
        Ok(_) => {
            if rx.recv().unwrap_or(false) {
                log!("PrintKey: hook ligado (a Print Screen dos atalhos e do NoOBS, nao do Windows).");
            }
        }
        Err(e) => log!("PrintKey: a thread do hook nao subiu ({e})."),
    }
}

/// Os modificadores apertados agora, no formato do `RegisterHotKey`.
fn current_mods() -> u32 {
    let down = |vk: u16| unsafe { GetAsyncKeyState(i32::from(vk)) } < 0;
    let mut m = 0;
    if down(VK_CONTROL.0) {
        m |= MOD_CTRL;
    }
    if down(VK_SHIFT.0) {
        m |= MOD_SHIFT_;
    }
    if down(VK_MENU.0) {
        m |= MOD_ALT_;
    }
    if down(VK_LWIN.0) || down(VK_RWIN.0) {
        m |= MOD_WIN_;
    }
    m
}

/// O atalho de `mods` + Print Screen, se houver um.
pub fn binding_for(bindings: &[(u32, usize)], mods: u32) -> Option<usize> {
    bindings.iter().find(|(m, _)| *m == mods).map(|(_, i)| *i)
}

/// Tira o registro de raw input do teclado que o winit (por baixo do Aylis) faz ao criar
/// o laço de eventos. Com ele, o Windows não chama o hook de baixo nível DESTE processo
/// enquanto a janela dele está em primeiro plano: com o NoOBS à frente, a Print Screen ia
/// direto para a Ferramenta de Captura e o print do NoOBS não saía. O Aylis não usa os
/// eventos de teclado raw (o teclado chega pelas mensagens da janela); o do mouse fica.
/// Main thread, depois de a janela existir.
pub fn release_raw_keyboard() {
    let device = RAWINPUTDEVICE { usUsagePage: 0x01, usUsage: 0x06, dwFlags: RIDEV_REMOVE, hwndTarget: HWND::default() };
    let ok = unsafe { RegisterRawInputDevices(&[device], std::mem::size_of::<RAWINPUTDEVICE>() as u32) };
    if let Err(e) = ok {
        log!("PrintKey: o raw input do teclado nao saiu ({e}); com o NoOBS em primeiro plano a Print Screen fica com o Windows.");
    }
}

unsafe extern "system" fn hook_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code == HC_ACTION as i32 {
        let k = &*(lparam.0 as *const KBDLLHOOKSTRUCT);
        if k.vkCode == VK_SNAPSHOT {
            let msg = wparam.0 as u32;
            if msg == WM_KEYDOWN || msg == WM_SYSKEYDOWN {
                let mods = current_mods();
                let hit = binding_for(&BINDINGS.lock().unwrap_or_else(|p| p.into_inner()), mods);
                // Uma linha por aperto (não por repetição): quando o recorte do Windows abre
                // mesmo assim, o log diz se o hook nem viu a tecla ou se ela não casou.
                let first = !HELD.load(Ordering::SeqCst);
                if first {
                    log!("PrintKey: Print Screen (mods=0x{mods:X}, flags=0x{:X}) -> {}", k.flags.0, if hit.is_some() { "atalho do NoOBS" } else { "sem atalho, fica com o Windows" });
                }
                if let Some(index) = hit {
                    // Segurar a tecla repete a descida: dispara só a primeira.
                    if !HELD.swap(true, Ordering::SeqCst) {
                        if let Some(f) = *ON_PRESS.lock().unwrap_or_else(|p| p.into_inner()) {
                            mainthread::queue(move || f(index));
                        }
                    }
                    return LRESULT(1);
                }
            } else if (msg == WM_KEYUP || msg == WM_SYSKEYUP) && HELD.swap(false, Ordering::SeqCst) {
                return LRESULT(1);
            }
        }
    }
    CallNextHookEx(None, code, wparam, lparam)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn so_a_combinacao_exata_dispara() {
        let b = vec![(0, 3), (MOD_CTRL, 5)];
        assert_eq!(binding_for(&b, 0), Some(3));
        assert_eq!(binding_for(&b, MOD_CTRL), Some(5));
        assert_eq!(binding_for(&b, MOD_ALT_), None, "Alt+Print Screen continua do Windows");
        assert_eq!(binding_for(&b, MOD_CTRL | MOD_SHIFT_), None);
    }
}
