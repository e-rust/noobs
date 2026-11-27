//! O registro das janelas: uma thread olha a cada 100 ms todas as janelas do NoOBS (e as de
//! outros processos com "NoOBS" no título, como o fantasma que o Windows põe no lugar de uma
//! janela que parou de responder) e anota no log cada mudança: aparecer, sumir, minimizar,
//! maximizar, encobrir, com a classe, o lugar, a janela em primeiro plano e se o NoOBS está
//! hibernando.
//!
//! Para achar a janela branca que ficava aberta depois de um print com o NoOBS na bandeja
//! (relatado em 26/11/2026): ela não era a janela principal, que continuava escondida. Os
//! pedidos do próprio NoOBS para mostrar a principal ficam no log como "UI: ...".

use std::collections::HashMap;
use std::sync::Once;
use std::time::Duration;

use windows::core::BOOL;
use windows::Win32::Foundation::{HWND, LPARAM, RECT};
use windows::Win32::Graphics::Dwm::{DwmGetWindowAttribute, DWMWA_CLOAKED};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetClassNameW, GetForegroundWindow, GetWindow, GetWindowRect, GetWindowTextW, GetWindowThreadProcessId, IsIconic, IsWindowVisible, IsZoomed, GW_OWNER,
};

static START: Once = Once::new();

/// O que se vê de uma janela.
#[derive(Clone, PartialEq, Eq)]
struct Estado {
    visivel: bool,
    minimizada: bool,
    maximizada: bool,
    encoberta: bool,
    rect: (i32, i32, i32, i32),
    titulo: String,
}

/// Começa a olhar as janelas (uma vez só). `principal` é o HWND da janela principal, para o log
/// dizer qual é ela.
pub fn start(principal: isize) {
    START.call_once(|| {
        let spawned = std::thread::Builder::new().name("noobs-janelas".into()).spawn(move || vigiar(principal));
        if let Err(e) = spawned {
            crate::log::log(format!("Janela: o registro das janelas nao subiu ({e})."));
        }
    });
}

impl Estado {
    /// O que conta como mudança: aparecer, sumir, minimizar, maximizar, encobrir e o título.
    fn chave(&self) -> (bool, bool, bool, bool, &str) {
        (self.visivel, self.minimizada, self.maximizada, self.encoberta, &self.titulo)
    }
}

fn vigiar(principal: isize) {
    let mut antes: HashMap<isize, Estado> = HashMap::new();
    let mut primeira = true;
    loop {
        if crate::bridge::is_shutting_down() {
            return;
        }
        let agora = janelas();
        // Só as visíveis e as que mudam de visível para escondida contam: o NoOBS tem janelas
        // escondidas de serviço (a da bandeja, a do IME) que não interessam enquanto não aparecem.
        for (h, e) in &agora {
            // Só o estado conta (o lugar muda a cada passo de um arrasto).
            let mudou = antes.get(h).map(Estado::chave) != Some(e.chave());
            let relevante = e.visivel || antes.get(h).is_some_and(|a| a.visivel) || *h == principal;
            if mudou && relevante && !(primeira && !e.visivel && *h != principal) {
                crate::log::log(format!("Janela: {} (frente: {}; hibernando: {}).", descreve(*h, e, principal), frente(), crate::bridge::is_light()));
            }
        }
        for (h, e) in &antes {
            if e.visivel && !agora.contains_key(h) {
                crate::log::log(format!("Janela: fechou {}.", nome(*h, e, principal)));
            }
        }
        antes = agora;
        primeira = false;
        std::thread::sleep(Duration::from_millis(100));
    }
}

/// As janelas de primeiro nível deste processo e as de outros com "NoOBS" no título.
fn janelas() -> HashMap<isize, Estado> {
    let mut out: HashMap<isize, Estado> = HashMap::new();
    // SAFETY: o callback só recebe o ponteiro para `out`, vivo durante o EnumWindows.
    unsafe {
        let _ = EnumWindows(Some(coleta), LPARAM(&mut out as *mut _ as isize));
    }
    out
}

unsafe extern "system" fn coleta(h: HWND, l: LPARAM) -> BOOL {
    // SAFETY: `l` é o `&mut HashMap` do `janelas`, vivo durante a enumeração.
    let out = unsafe { &mut *(l.0 as *mut HashMap<isize, Estado>) };
    let mut pid = 0u32;
    // SAFETY: consultas de leitura de uma janela qualquer (pode ter fechado: devolvem falso).
    unsafe {
        GetWindowThreadProcessId(h, Some(&mut pid));
        let titulo = texto(h);
        if pid != std::process::id() && !titulo.contains("NoOBS") {
            return true.into();
        }
        let mut cloaked = 0u32;
        let _ = DwmGetWindowAttribute(h, DWMWA_CLOAKED, &mut cloaked as *mut u32 as *mut core::ffi::c_void, 4);
        let mut r = RECT::default();
        let _ = GetWindowRect(h, &mut r);
        let titulo = if pid == std::process::id() { titulo } else { format!("{titulo} [outro processo {pid}]") };
        out.insert(
            h.0 as isize,
            Estado {
                visivel: IsWindowVisible(h).as_bool(),
                minimizada: IsIconic(h).as_bool(),
                maximizada: IsZoomed(h).as_bool(),
                encoberta: cloaked != 0,
                rect: (r.left, r.top, r.right - r.left, r.bottom - r.top),
                titulo,
            },
        );
    }
    true.into()
}

fn texto(h: HWND) -> String {
    let mut buf = [0u16; 120];
    // SAFETY: leitura do título para um buffer local.
    let n = unsafe { GetWindowTextW(h, &mut buf) }.max(0) as usize;
    String::from_utf16_lossy(&buf[..n])
}

fn classe(h: HWND) -> String {
    let mut buf = [0u16; 80];
    // SAFETY: leitura da classe para um buffer local.
    let n = unsafe { GetClassNameW(h, &mut buf) }.max(0) as usize;
    String::from_utf16_lossy(&buf[..n])
}

fn nome(h: isize, e: &Estado, principal: isize) -> String {
    let qual = if h == principal { "a principal".to_string() } else { format!("'{}'", e.titulo) };
    let hwnd = HWND(h as *mut _);
    // SAFETY: leitura da dona (pode ser nula).
    let dona = unsafe { GetWindow(hwnd, GW_OWNER) }.map(|o| o.0 as isize).unwrap_or(0);
    format!("{qual} ({} {h}{})", classe(hwnd), if dona != 0 { format!(", dona {dona}") } else { String::new() })
}

fn descreve(h: isize, e: &Estado, principal: isize) -> String {
    let quando = if !e.visivel {
        "escondida"
    } else if e.minimizada {
        "minimizada"
    } else if e.maximizada {
        "maximizada"
    } else {
        "à vista"
    };
    let (x, y, w, hh) = e.rect;
    format!("{} {quando}{} em ({x}, {y}) {w}x{hh}", nome(h, e, principal), if e.encoberta { ", encoberta" } else { "" })
}

/// A janela em primeiro plano: o título e se é do NoOBS.
fn frente() -> String {
    // SAFETY: leitura da janela em primeiro plano (pode ser nula ou de outro processo).
    unsafe {
        let fg = GetForegroundWindow();
        if fg.0.is_null() {
            return "nenhuma".into();
        }
        let mut pid = 0u32;
        GetWindowThreadProcessId(fg, Some(&mut pid));
        let dono = if pid == std::process::id() { "do NoOBS" } else { "de outro programa" };
        format!("'{}' {dono}", texto(fg))
    }
}
