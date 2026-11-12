//! Seleção de ÁREA da tela ("Capturar... → Área").
//!
//! Escurece TODOS os monitores com uma janela só, do tamanho da área de
//! trabalho virtual, e o usuário arrasta o retângulo que quer gravar. Soltar o
//! botão confirma; Esc ou o botão direito cancelam.
//!
//! A janela é layered com DOIS atributos ao mesmo tempo: alfa (o escuro
//! translúcido) e uma cor-chave, que vira transparente de verdade — é ela que
//! "fura" o escuro no retângulo escolhido, então o usuário vê a área nítida.
//!
//! Sem buffer do tamanho da tela: em dois monitores 4K seriam ~66 MB por
//! quadro. O `WM_PAINT` pinta direto, só a região invalidada (o retângulo
//! antigo e o novo), e nunca pinta o escuro por cima do furo — por isso não
//! pisca.
//!
//! Coordenadas: pixels FÍSICOS (o processo é per-monitor DPI aware v2), na
//! área de trabalho virtual (origem no monitor principal, pode ser negativa).
//!
//! Main thread.

use std::cell::RefCell;

use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{COLORREF, HINSTANCE, HWND, LPARAM, LRESULT, POINT, RECT, SIZE, WPARAM};
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::HiDpi::GetDpiForSystem;
use windows::Win32::UI::Input::KeyboardAndMouse::{GetKeyState, ReleaseCapture, SetCapture, SetFocus, VK_CONTROL, VK_ESCAPE, VK_RETURN, VK_SHIFT};
use windows::Win32::UI::WindowsAndMessaging::*;

use crate::annotate::{Arrow, Frame, Mark, Text, PALETTE, SIZE_MAX, SIZE_MIN, SIZE_STEP, WIDTH_MAX, WIDTH_MIN};
use crate::engine::AreaRect;
use crate::{lang, log, mainthread};

const CLASS_NAME: PCWSTR = w!("TNoOBSAreaSelect");
/// Cor-chave: vira transparente (o "furo" do escuro). Magenta puro não aparece
/// em nenhum desenho nosso.
const KEY: COLORREF = COLORREF(0x00FF_00FF);
/// Opacidade do escuro (0..255).
const DIM_ALPHA: u8 = 150;
/// Menor área aceita, em px físicos: arrastar menos que isso é clique perdido.
pub const MIN_SIZE: i32 = 32;

pub type OnSelected = fn(Option<AreaRect>);

/// Uma imagem da tela num DC de memória (para o `BitBlt` do `WM_PAINT`).
struct MemImage {
    dc: HDC,
    bmp: HBITMAP,
    old: HGDIOBJ,
}

impl MemImage {
    /// A partir de RGBA (linhas de cima pra baixo); `scale` escurece (1.0 = como está).
    fn from_rgba(rgba: &[u8], w: i32, h: i32, scale: f32) -> Option<MemImage> {
        unsafe {
            let dc = CreateCompatibleDC(None);
            if dc.is_invalid() {
                return None;
            }
            let bmi = BITMAPINFO {
                bmiHeader: BITMAPINFOHEADER {
                    biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                    biWidth: w,
                    biHeight: -h,
                    biPlanes: 1,
                    biBitCount: 32,
                    biCompression: BI_RGB.0,
                    ..Default::default()
                },
                ..Default::default()
            };
            let mut bits: *mut std::ffi::c_void = std::ptr::null_mut();
            let Ok(bmp) = CreateDIBSection(Some(dc), &bmi, DIB_RGB_COLORS, &mut bits, None, 0) else {
                let _ = DeleteDC(dc);
                return None;
            };
            if bits.is_null() {
                let _ = DeleteObject(HGDIOBJ(bmp.0));
                let _ = DeleteDC(dc);
                return None;
            }
            let dst = std::slice::from_raw_parts_mut(bits as *mut u8, w as usize * h as usize * 4);
            let k = (scale * 256.0) as u32;
            for (d, s) in dst.chunks_exact_mut(4).zip(rgba.chunks_exact(4)) {
                // RGBA → BGRA, escurecido.
                d[0] = ((s[2] as u32 * k) >> 8) as u8;
                d[1] = ((s[1] as u32 * k) >> 8) as u8;
                d[2] = ((s[0] as u32 * k) >> 8) as u8;
                d[3] = 255;
            }
            let old = SelectObject(dc, HGDIOBJ(bmp.0));
            Some(MemImage { dc, bmp, old })
        }
    }
}

impl Drop for MemImage {
    fn drop(&mut self) {
        unsafe {
            SelectObject(self.dc, self.old);
            let _ = DeleteObject(HGDIOBJ(self.bmp.0));
            let _ = DeleteDC(self.dc);
        }
    }
}

/// A tela congelada do print: a seleção desenha esta imagem (escura fora, nítida dentro)
/// em vez de mostrar a tela ao vivo pelo "furo" — a área fica parada.
struct Frozen {
    bright: MemImage,
    dim: MemImage,
}

#[derive(Default)]
struct State {
    /// Modo congelado (o print); `None` = a tela ao vivo (a gravação de área).
    frozen: Option<Frozen>,
    registered: bool,
    hwnd: Option<HWND>,
    /// Canto superior esquerdo da área de trabalho virtual (cliente → tela).
    origin: (i32, i32),
    /// Onde o arrasto começou (cliente).
    anchor: Option<POINT>,
    /// Seleção atual (cliente), normalizada.
    sel: Option<RECT>,
    /// Onde vai a instrução antes do 1º clique (cliente): o monitor do cursor.
    hint_rect: RECT,
    hint_visible: bool,
    on_selected: Option<OnSelected>,
    /// Chave do texto de instrução (gravar agora ou só salvar a área).
    hint_key: &'static str,
    font: HFONT,
    dpi: i32,
    // ---- editor do print ([`start_shot_editor`]) ----
    /// Com ele, soltar a seleção não termina: a área fica, com a barra e as setas.
    on_shot: Option<OnShot>,
    /// A área está escolhida (a barra à vista).
    editing: bool,
    /// As marcas feitas (setas, retângulos e textos, coordenadas do cliente) e o arrasto
    /// em andamento: do ponto inicial ao atual — vira seta ou retângulo pela ferramenta
    /// ([`drag_mark`]).
    marks: Vec<Mark>,
    drawing: Option<Arrow>,
    /// A ferramenta e a cor da próxima marca.
    tool: Tool,
    color: u32,
    /// A espessura da próxima seta e o tamanho do próximo texto (a roda do mouse muda).
    width: f32,
    font_size: f32,
    /// O texto sendo digitado (índice em `marks`): o teclado vai para ele.
    typing: Option<usize>,
    /// A 1ª metade de um caractere fora do BMP (o `WM_CHAR` chega em UTF-16).
    high_surrogate: Option<u16>,
    /// A seleção de antes de um clique fora dela (sem arrasto, ela volta).
    prev_sel: Option<RECT>,
    /// Os botões da barra, onde foram desenhados (para o clique), e a barra inteira.
    buttons: Vec<(ShotButton, RECT)>,
    toolbar: Option<RECT>,
    small_font: HFONT,
}

/// O que fazer com o print.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShotAction {
    /// Salvar (o PNG na pasta de gravações e a área de transferência).
    Save,
    /// Só a área de transferência, sem arquivo.
    Copy,
}

/// A ferramenta do editor: arrastar faz uma seta ou um retângulo; clicar começa um texto.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Tool {
    #[default]
    Arrow,
    /// Arrastar demarca uma região com um retângulo (só a borda).
    Rect,
    Text,
}

/// O arrasto `a` como marca da ferramenta `tool`.
fn drag_mark(a: Arrow, tool: Tool) -> Mark {
    match tool {
        Tool::Rect => Mark::Rect(Frame::from_drag(&a)),
        _ => Mark::Arrow(a),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ShotButton {
    Tool(Tool),
    /// Índice em [`PALETTE`].
    Color(usize),
    Copy,
    Save,
    Cancel,
}

/// O print escolhido no editor: a área (tela) e as marcas (tela), e o que fazer.
#[derive(Debug, Clone)]
pub struct ShotDone {
    pub action: ShotAction,
    pub area: AreaRect,
    pub marks: Vec<Mark>,
}

/// `None` = cancelado.
pub type OnShot = fn(Option<ShotDone>);

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

fn with<R>(f: impl FnOnce(&mut State) -> R) -> R {
    STATE.with(|s| f(&mut s.borrow_mut()))
}

fn hinstance() -> HINSTANCE {
    unsafe { GetModuleHandleW(None).map(|h| HINSTANCE(h.0)).unwrap_or_default() }
}

fn sx(dpi: i32, v: i32) -> i32 {
    (v * dpi + 48) / 96
}

/// Seleção aberta agora?
pub fn is_active() -> bool {
    #[cfg(test)]
    if TEST_PENDING.with(|p| p.get().is_some()) || TEST_SHOT.with(|p| p.get().is_some()) {
        return true;
    }
    with(|s| s.hwnd.is_some())
}

// Teste: a seleção de verdade escurece todos os monitores e espera o mouse.
// Aqui ela só fica "aberta" até o teste dizer o que o usuário escolheu.
#[cfg(test)]
thread_local! {
    static TEST_PENDING: std::cell::Cell<Option<OnSelected>> = const { std::cell::Cell::new(None) };
}

/// Teste: termina a seleção aberta como se o usuário tivesse arrastado
/// (`Some`) ou apertado Esc (`None`). `false` se não havia seleção aberta.
#[cfg(test)]
pub fn test_finish(result: Option<AreaRect>) -> bool {
    match TEST_PENDING.with(|p| p.take()) {
        Some(cb) => {
            cb(result);
            true
        }
        None => false,
    }
}

/// Abre a seleção. `false` se já há uma aberta ou a janela não pôde nascer.
pub fn start(on_selected: OnSelected) -> bool {
    start_with_hint(on_selected, "Arraste para escolher a área que será gravada  •  Esc cancela")
}

/// Como [`start_with_hint`], sobre uma imagem congelada da área de trabalho virtual
/// inteira (RGBA `w`×`h`, o print tirado no instante do atalho): o que aparece na
/// seleção é essa imagem parada, não a tela ao vivo. Imagem de outro tamanho (um
/// monitor entrou ou saiu no meio): cai na seleção ao vivo.
pub fn start_frozen(on_selected: OnSelected, hint_key: &'static str, rgba: &[u8], w: i32, h: i32) -> bool {
    if is_active() {
        return false;
    }
    #[cfg(not(test))]
    {
        let (vw, vh) = unsafe { (GetSystemMetrics(SM_CXVIRTUALSCREEN), GetSystemMetrics(SM_CYVIRTUALSCREEN)) };
        let frozen = if (w, h) == (vw, vh) && rgba.len() == w as usize * h as usize * 4 {
            MemImage::from_rgba(rgba, w, h, 1.0)
                .zip(MemImage::from_rgba(rgba, w, h, 1.0 - DIM_ALPHA as f32 / 255.0))
                .map(|(bright, dim)| Frozen { bright, dim })
        } else {
            log!("AreaSelect: imagem congelada {w}x{h} nao bate com a tela {vw}x{vh} — selecao ao vivo.");
            None
        };
        with(|s| s.frozen = frozen);
    }
    #[cfg(test)]
    let _ = (rgba, w, h);
    let ok = start_with_hint(on_selected, hint_key);
    if !ok {
        with(|s| s.frozen = None);
    }
    ok
}

/// O editor do print: a seleção sobre a tela congelada ([`start_frozen`]), mas soltar o
/// mouse não termina — a área fica, com a barra (Seta/Retângulo/Texto, as cores, Copiar,
/// Salvar, Cancelar). Com a Seta, arrastar DENTRO da área desenha uma seta (do ponto
/// inicial ao final, a ponta no final); com o Retângulo, a borda de um retângulo de canto
/// a canto (a espessura é a mesma da seta); com o Texto, clicar começa um texto ali (ou edita o que
/// estiver sob o clique). Arrastar FORA escolhe outra área. Roda do mouse: a espessura
/// da seta ou o tamanho da fonte; Ctrl+Z desfaz a última marca; Ctrl+C copia; Enter
/// salva (digitando: termina o texto; Shift+Enter, nova linha); Esc cancela.
/// A ferramenta, a cor e os tamanhos ficam para o próximo print.
pub fn start_shot_editor(on_shot: OnShot, rgba: &[u8], w: i32, h: i32) -> bool {
    if is_active() {
        return false;
    }
    #[cfg(test)]
    {
        crate::testkit::record_effect("area_select:print".to_string());
        let _ = (rgba, w, h);
        TEST_SHOT.with(|p| p.set(Some(on_shot)));
        return true;
    }
    #[allow(unreachable_code)]
    {
        with(|s| {
            s.on_shot = Some(on_shot);
            s.editing = false;
            reset_editor(s);
            s.prev_sel = None;
            s.buttons.clear();
            s.toolbar = None;
        });
        let ok = start_frozen(|_| {}, "Arraste para escolher a área do print  •  Esc cancela", rgba, w, h);
        if !ok {
            with(|s| s.on_shot = None);
        }
        ok
    }
}

/// Começo de um print: sem marcas; a ferramenta, a cor e os tamanhos do último print
/// (na primeira vez, os padrões).
fn reset_editor(s: &mut State) {
    s.editing = false;
    s.marks.clear();
    s.drawing = None;
    s.typing = None;
    s.high_surrogate = None;
    if s.width == 0.0 {
        s.width = crate::annotate::WIDTH_DEFAULT;
        s.font_size = crate::annotate::SIZE_DEFAULT;
        s.color = crate::annotate::COLOR;
        s.tool = Tool::Arrow;
    }
}

#[cfg(test)]
thread_local! {
    static TEST_SHOT: std::cell::Cell<Option<OnShot>> = const { std::cell::Cell::new(None) };
}

/// Como [`start`], com outra instrução na tela (ex.: definir a área salva).
pub fn start_with_hint(on_selected: OnSelected, hint_key: &'static str) -> bool {
    if is_active() {
        return false;
    }
    #[cfg(test)]
    {
        crate::testkit::record_effect(format!("area_select:{hint_key}"));
        TEST_PENDING.with(|p| p.set(Some(on_selected)));
        return true;
    }
    #[allow(unreachable_code)]
    ensure_class();
    unsafe {
        let vx = GetSystemMetrics(SM_XVIRTUALSCREEN);
        let vy = GetSystemMetrics(SM_YVIRTUALSCREEN);
        let vw = GetSystemMetrics(SM_CXVIRTUALSCREEN);
        let vh = GetSystemMetrics(SM_CYVIRTUALSCREEN);
        let dpi = GetDpiForSystem().max(96) as i32;

        // Instrução no monitor onde o cursor está (é pra onde o usuário olha).
        let mut cur = POINT::default();
        let _ = GetCursorPos(&mut cur);
        let mon = MonitorFromPoint(cur, MONITOR_DEFAULTTOPRIMARY);
        let mut mi = MONITORINFO { cbSize: std::mem::size_of::<MONITORINFO>() as u32, ..Default::default() };
        let _ = GetMonitorInfoW(mon, &mut mi);
        let m = mi.rcMonitor;
        let hint = RECT { left: m.left - vx, top: m.top - vy, right: m.right - vx, bottom: m.bottom - vy };

        let font = CreateFontW(-sx(dpi, 18), 0, 0, 0, FW_SEMIBOLD.0 as i32, 0, 0, 0, DEFAULT_CHARSET, OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS, CLEARTYPE_QUALITY, DEFAULT_PITCH.0 as u32, w!("Segoe UI"));

        // Congelada: opaca, desenha a imagem. Ao vivo: translúcida, com o furo transparente.
        let frozen = with(|s| s.frozen.is_some());
        let layered = if frozen { WINDOW_EX_STYLE(0) } else { WS_EX_LAYERED };
        let Ok(hwnd) = CreateWindowExW(
            layered | WS_EX_TOPMOST | WS_EX_TOOLWINDOW,
            CLASS_NAME,
            w!("NoOBS - area"),
            WS_POPUP,
            vx,
            vy,
            vw,
            vh,
            None,
            None,
            Some(hinstance()),
            None,
        ) else {
            log::log("AreaSelect: CreateWindowEx falhou.");
            let _ = DeleteObject(HGDIOBJ(font.0));
            return false;
        };
        if !frozen {
            let _ = SetLayeredWindowAttributes(hwnd, KEY, DIM_ALPHA, LWA_ALPHA | LWA_COLORKEY);
        }
        with(|s| {
            s.hwnd = Some(hwnd);
            s.origin = (vx, vy);
            s.anchor = None;
            s.sel = None;
            s.hint_rect = hint;
            s.hint_visible = true;
            s.on_selected = Some(on_selected);
            s.hint_key = hint_key;
            s.font = font;
            s.dpi = dpi;
            if s.on_shot.is_some() {
                s.small_font = CreateFontW(-sx(dpi, 14), 0, 0, 0, FW_SEMIBOLD.0 as i32, 0, 0, 0, DEFAULT_CHARSET, OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS, CLEARTYPE_QUALITY, DEFAULT_PITCH.0 as u32, w!("Segoe UI"));
            }
        });
        let _ = ShowWindow(hwnd, SW_SHOW);
        // Foco pra o Esc funcionar. Sem ele (trava de primeiro plano do
        // Windows) o botão direito ainda cancela.
        let _ = SetForegroundWindow(hwnd);
        let _ = SetFocus(Some(hwnd));
        log!("AreaSelect: aberta ({vw}x{vh} em {vx},{vy}){}.", if frozen { ", sobre a tela congelada" } else { "" });
    }
    true
}

/// Fecha e avisa quem abriu. O callback sai pela fila da main: ele começa a
/// gravação (pesado) e não pode rodar dentro do `WndProc` desta janela.
fn finish(result: Option<AreaRect>) {
    let (cb, on_shot) = close();
    match result {
        Some(a) => log!("AreaSelect: area escolhida {}x{} em ({}, {}).", a.w, a.h, a.x, a.y),
        None => log::log("AreaSelect: cancelada."),
    }
    if let Some(cb) = cb {
        mainthread::queue(move || cb(result));
    }
    // O editor do print fechado sem Copiar/Salvar (Esc, botão direito): cancelado.
    if let Some(cb) = on_shot {
        mainthread::queue(move || cb(None));
    }
}

/// O editor do print termina com `action`: a área e as marcas vão em coordenadas de tela
/// (o texto sendo digitado entra como está).
fn finish_shot(action: ShotAction) {
    end_typing();
    let (sel, origin, marks) = with(|s| (s.sel, s.origin, std::mem::take(&mut s.marks)));
    let Some(r) = sel else { return };
    let area = AreaRect { x: r.left + origin.0, y: r.top + origin.1, w: r.right - r.left, h: r.bottom - r.top };
    let marks: Vec<Mark> = marks.iter().map(|m| m.offset(origin.0 as f32, origin.1 as f32)).collect();
    let (_, on_shot) = close();
    let texts = marks.iter().filter(|m| matches!(m, Mark::Text(_))).count();
    log!("AreaSelect: print {:?} {}x{} em ({}, {}), {} seta(s), {} texto(s).", action, area.w, area.h, area.x, area.y, marks.len() - texts, texts);
    if let Some(cb) = on_shot {
        mainthread::queue(move || cb(Some(ShotDone { action, area, marks })));
    }
}

/// Termina o texto sendo digitado; vazio, ele sai.
fn end_typing() {
    with(|s| {
        s.high_surrogate = None;
        if let Some(i) = s.typing.take() {
            if matches!(s.marks.get(i), Some(Mark::Text(t)) if t.text.trim().is_empty()) {
                s.marks.remove(i);
            }
        }
    });
}

/// Fecha a janela e solta o que ela segura (fontes, as imagens congeladas — dezenas de
/// MB); devolve quem esperava o resultado.
fn close() -> (Option<OnSelected>, Option<OnShot>) {
    let (hwnd, cb, on_shot, fonts, frozen) = with(|s| {
        s.editing = false;
        s.drawing = None;
        s.typing = None;
        s.marks.clear();
        s.buttons.clear();
        s.toolbar = None;
        (s.hwnd.take(), s.on_selected.take(), s.on_shot.take(), [std::mem::take(&mut s.font), std::mem::take(&mut s.small_font)], s.frozen.take())
    });
    drop(frozen);
    if let Some(h) = hwnd {
        unsafe {
            let _ = ReleaseCapture();
            let _ = DestroyWindow(h);
        }
    }
    for font in fonts {
        if !font.is_invalid() {
            unsafe {
                let _ = DeleteObject(HGDIOBJ(font.0));
            }
        }
    }
    (cb, on_shot)
}

fn norm(a: POINT, b: POINT) -> RECT {
    RECT { left: a.x.min(b.x), top: a.y.min(b.y), right: a.x.max(b.x), bottom: a.y.max(b.y) }
}

fn inflate(r: RECT, d: i32) -> RECT {
    RECT { left: r.left - d, top: r.top - d, right: r.right + d, bottom: r.bottom + d }
}

/// Faixa do rótulo "1280 × 720", acima da seleção (ou abaixo, colada no topo).
fn label_rect(sel: RECT, dpi: i32) -> RECT {
    let (w, h, gap) = (sx(dpi, 120), sx(dpi, 28), sx(dpi, 6));
    let top = if sel.top - h - gap >= 0 { sel.top - h - gap } else { sel.bottom + gap };
    RECT { left: sel.left, top, right: sel.left + w, bottom: top + h }
}

/// Tudo o que a seleção desenha (furo + borda + rótulo), pra invalidar.
fn painted_bounds(sel: RECT, dpi: i32) -> [RECT; 2] {
    [inflate(sel, sx(dpi, 3)), label_rect(sel, dpi)]
}

fn invalidate(hwnd: HWND, r: &RECT) {
    unsafe {
        let _ = InvalidateRect(Some(hwnd), Some(r), false);
    }
}

fn set_selection(hwnd: HWND, new_sel: Option<RECT>) {
    let (old, dpi) = with(|s| (std::mem::replace(&mut s.sel, new_sel), s.dpi));
    for r in old.iter().chain(new_sel.iter()) {
        for b in painted_bounds(*r, dpi) {
            invalidate(hwnd, &b);
        }
    }
}

/// A barra do editor do print, colada embaixo da área (em cima, ou dentro, sem espaço):
/// Seta e Texto (a ferramenta), as cores, o tamanho (da roda do mouse) e Cancelar, Copiar
/// e Salvar. Guarda onde cada botão ficou.
fn paint_toolbar(hwnd: HWND, dc: HDC, sel: RECT) {
    let (dpi, font, tool, color) = with(|s| (s.dpi, s.small_font, s.tool, s.color));
    // O tamanho mostrado é o do que a roda muda agora: o texto digitado, ou a ferramenta.
    let size = with(|s| match (s.typing.and_then(|i| s.marks.get(i)), tool) {
        (Some(Mark::Text(t)), _) => t.size,
        (_, Tool::Text) => s.font_size,
        (_, Tool::Arrow | Tool::Rect) => s.width,
    });
    unsafe {
        let mut client = RECT::default();
        let _ = GetClientRect(hwnd, &mut client);
        let old_font = SelectObject(dc, HGDIOBJ(font.0));
        let measure = |text: &[u16]| {
            let mut size = SIZE::default();
            let _ = GetTextExtentPoint32W(dc, text, &mut size);
            size.cx
        };
        let px = format!("{size:.0}");
        let label_key = if tool == Tool::Text { "Fonte {{px}} px (roda do mouse)" } else { "Espessura {{px}} px (roda do mouse)" };
        let label: Vec<u16> = lang::tr_with(label_key, &[("px", &px)]).encode_utf16().collect();
        let tools: [(ShotButton, Vec<u16>); 3] = [
            (ShotButton::Tool(Tool::Arrow), lang::tr("Seta").encode_utf16().collect()),
            (ShotButton::Tool(Tool::Rect), lang::tr("Retângulo").encode_utf16().collect()),
            (ShotButton::Tool(Tool::Text), lang::tr("Texto").encode_utf16().collect()),
        ];
        let buttons: [(ShotButton, Vec<u16>); 3] = [
            (ShotButton::Cancel, lang::tr("Cancelar (Esc)").encode_utf16().collect()),
            (ShotButton::Copy, lang::tr("Copiar (Ctrl+C)").encode_utf16().collect()),
            (ShotButton::Save, lang::tr("Salvar (Enter)").encode_utf16().collect()),
        ];
        let (pad, gap, h, margin) = (sx(dpi, 14), sx(dpi, 8), sx(dpi, 34), sx(dpi, 10));
        let swatch = sx(dpi, 24);
        let label_w = measure(&label);
        let tool_widths: Vec<i32> = tools.iter().map(|(_, t)| measure(t) + 2 * pad).collect();
        let widths: Vec<i32> = buttons.iter().map(|(_, t)| measure(t) + 2 * pad).collect();
        let tools_w = tool_widths.iter().sum::<i32>() + gap * (tool_widths.len() as i32 - 1);
        let swatches_w = (swatch + gap / 2) * PALETTE.len() as i32 - gap / 2;
        let bar_w = gap + tools_w + 2 * gap + swatches_w + 2 * gap + label_w + gap + widths.iter().sum::<i32>() + gap * widths.len() as i32;
        let bar_h = h + 2 * gap;
        let x = (sel.right - bar_w).clamp(0, (client.right - bar_w).max(0));
        let mut y = sel.bottom + margin;
        if y + bar_h > client.bottom {
            y = sel.top - margin - bar_h;
        }
        if y < 0 {
            y = sel.bottom - bar_h - margin;
        }
        let bar = RECT { left: x, top: y, right: x + bar_w, bottom: y + bar_h };
        let bg = CreateSolidBrush(COLORREF(0x0020_2020));
        FillRect(dc, &bar, bg);
        let _ = DeleteObject(HGDIOBJ(bg.0));
        SetBkMode(dc, TRANSPARENT);
        let fill = |r: &RECT, bgr: u32| {
            let brush = CreateSolidBrush(COLORREF(bgr));
            FillRect(dc, r, brush);
            let _ = DeleteObject(HGDIOBJ(brush.0));
        };
        let mut placed = Vec::new();
        let mut bx = x + gap;
        // Um botão com texto: `bg` (BGR) de fundo.
        let mut text_button = |kind: ShotButton, text: Vec<u16>, w: i32, bg: u32, bx: &mut i32| {
            let br = RECT { left: *bx, top: y + gap, right: *bx + w, bottom: y + gap + h };
            fill(&br, bg);
            SetTextColor(dc, COLORREF(0x00FF_FFFF));
            let (mut text, mut tr) = (text, br);
            DrawTextW(dc, &mut text, &mut tr, DT_CENTER | DT_VCENTER | DT_SINGLELINE);
            placed.push((kind, br));
            *bx += w + gap;
        };
        // A ferramenta escolhida em azul.
        for ((kind, text), w) in tools.into_iter().zip(tool_widths) {
            let on = kind == ShotButton::Tool(tool);
            text_button(kind, text, w, if on { 0x00D9_6F1E } else { 0x0045_4545 }, &mut bx);
        }
        bx += gap;
        // As cores: a escolhida com moldura branca; as outras, cinza.
        let sy = y + gap + (h - swatch) / 2;
        let mut swatches = Vec::new();
        for (i, &argb) in PALETTE.iter().enumerate() {
            let r = RECT { left: bx, top: sy, right: bx + swatch, bottom: sy + swatch };
            let on = argb == color;
            fill(&inflate(r, if on { sx(dpi, 3) } else { 1 }), if on { 0x00FF_FFFF } else { 0x0070_7070 });
            let bgr = ((argb & 0xFF) << 16) | (argb & 0xFF00) | ((argb >> 16) & 0xFF);
            fill(&r, bgr);
            swatches.push((ShotButton::Color(i), r));
            bx += swatch + gap / 2;
        }
        bx += 2 * gap - gap / 2;
        SetTextColor(dc, COLORREF(0x00DD_DDDD));
        let mut lr = RECT { left: bx, top: y + gap, right: bx + label_w, bottom: y + gap + h };
        let mut label = label;
        DrawTextW(dc, &mut label, &mut lr, DT_LEFT | DT_VCENTER | DT_SINGLELINE);
        bx = lr.right + gap;
        for ((kind, text), w) in buttons.into_iter().zip(widths) {
            // Salvar em verde (a ação principal); os outros, cinza.
            text_button(kind, text, w, if kind == ShotButton::Save { 0x0032_7D2E } else { 0x0045_4545 }, &mut bx);
        }
        placed.extend(swatches);
        SelectObject(dc, old_font);
        with(|s| {
            s.buttons = placed;
            s.toolbar = Some(bar);
        });
    }
}

fn paint(hwnd: HWND) {
    let (sel, dpi, font, hint_rect, hint_visible) = with(|s| (s.sel, s.dpi, s.font, s.hint_rect, s.hint_visible));
    // Congelada: (escura, nítida) — os DCs das imagens, desenhados no lugar do preto e do furo.
    let frozen = with(|s| s.frozen.as_ref().map(|f| (f.dim.dc, f.bright.dc)));
    unsafe {
        let mut ps = PAINTSTRUCT::default();
        let dc = BeginPaint(hwnd, &mut ps);
        let black = CreateSolidBrush(COLORREF(0));
        let key = CreateSolidBrush(KEY);
        let white = CreateSolidBrush(COLORREF(0x00FF_FFFF));
        let border = sx(dpi, 2).max(1);

        // 1. Escuro, SEM passar por cima do furo + borda (senão pisca).
        let saved = SaveDC(dc);
        if let Some(r) = sel {
            let o = inflate(r, border);
            ExcludeClipRect(dc, o.left, o.top, o.right, o.bottom);
        }
        let rc = ps.rcPaint;
        match frozen {
            Some((dim, _)) => {
                let _ = BitBlt(dc, rc.left, rc.top, rc.right - rc.left, rc.bottom - rc.top, Some(dim), rc.left, rc.top, SRCCOPY);
            }
            None => {
                FillRect(dc, &rc, black);
            }
        }
        let _ = RestoreDC(dc, saved);

        if let Some(r) = sel {
            // 2. Furo (cor-chave = transparente; congelada: a imagem nítida) e borda
            // branca por FORA dele.
            match frozen {
                Some((_, bright)) => {
                    let _ = BitBlt(dc, r.left, r.top, r.right - r.left, r.bottom - r.top, Some(bright), r.left, r.top, SRCCOPY);
                }
                None => {
                    FillRect(dc, &r, key);
                }
            }
            // As marcas do editor do print, só dentro da área (o que vai para o print),
            // com o cursor no texto sendo digitado.
            let (marks, caret) = with(|s| (s.marks.iter().cloned().chain(s.drawing.map(|a| drag_mark(a, s.tool))).collect::<Vec<_>>(), s.typing));
            crate::annotate::draw_on_hdc(dc, &marks, r, caret);
            let o = inflate(r, border);
            for side in [
                RECT { left: o.left, top: o.top, right: o.right, bottom: r.top },
                RECT { left: o.left, top: r.bottom, right: o.right, bottom: o.bottom },
                RECT { left: o.left, top: r.top, right: r.left, bottom: r.bottom },
                RECT { left: r.right, top: r.top, right: o.right, bottom: r.bottom },
            ] {
                FillRect(dc, &side, white);
            }
            // 3. Tamanho, em px físicos — é o tamanho do vídeo (arredondado
            // pro múltiplo de 16 na gravação).
            let lr = label_rect(r, dpi);
            FillRect(dc, &lr, black);
            let old_font = SelectObject(dc, HGDIOBJ(font.0));
            SetBkMode(dc, TRANSPARENT);
            SetTextColor(dc, COLORREF(0x00FF_FFFF));
            let mut text: Vec<u16> = format!("{} × {}", r.right - r.left, r.bottom - r.top).encode_utf16().collect();
            let mut lr2 = lr;
            DrawTextW(dc, &mut text, &mut lr2, DT_CENTER | DT_VCENTER | DT_SINGLELINE);
            SelectObject(dc, old_font);
            // 4. A barra do editor do print (a área já escolhida).
            if with(|s| s.editing) {
                paint_toolbar(hwnd, dc, r);
            }
        } else if hint_visible {
            // Instrução no meio do monitor do cursor, até o 1º clique.
            let old_font = SelectObject(dc, HGDIOBJ(font.0));
            SetBkMode(dc, TRANSPARENT);
            SetTextColor(dc, COLORREF(0x00FF_FFFF));
            let mut text: Vec<u16> = lang::tr(with(|s| s.hint_key)).encode_utf16().collect();
            let mut hr = hint_rect;
            DrawTextW(dc, &mut text, &mut hr, DT_CENTER | DT_VCENTER | DT_SINGLELINE);
            SelectObject(dc, old_font);
        }

        let _ = DeleteObject(HGDIOBJ(black.0));
        let _ = DeleteObject(HGDIOBJ(key.0));
        let _ = DeleteObject(HGDIOBJ(white.0));
        let _ = EndPaint(hwnd, &ps);
    }
}

fn point_from(lparam: LPARAM) -> POINT {
    // GET_X/Y_LPARAM: com sinal (monitor à esquerda do principal).
    POINT { x: (lparam.0 & 0xFFFF) as i16 as i32, y: ((lparam.0 >> 16) & 0xFFFF) as i16 as i32 }
}

fn in_rect(r: &RECT, p: POINT) -> bool {
    p.x >= r.left && p.x < r.right && p.y >= r.top && p.y < r.bottom
}

/// O botão da barra do editor sob `p`.
fn button_at(p: POINT) -> Option<ShotButton> {
    with(|s| if s.editing { s.buttons.iter().find(|(_, r)| in_rect(r, p)).map(|(b, _)| *b) } else { None })
}

fn invalidate_all(hwnd: HWND) {
    unsafe {
        let _ = InvalidateRect(Some(hwnd), None, false);
    }
}

fn run_button(hwnd: HWND, b: ShotButton) {
    match b {
        ShotButton::Cancel => finish(None),
        ShotButton::Copy => finish_shot(ShotAction::Copy),
        ShotButton::Save => finish_shot(ShotAction::Save),
        ShotButton::Tool(t) => {
            end_typing();
            with(|s| s.tool = t);
            invalidate_all(hwnd);
        }
        ShotButton::Color(i) => {
            // A cor das próximas marcas — e a do texto sendo digitado, que continua.
            let c = PALETTE[i];
            with(|s| {
                s.color = c;
                if let Some(Mark::Text(t)) = s.typing.and_then(|i| s.marks.get_mut(i)) {
                    t.color = c;
                }
            });
            invalidate_all(hwnd);
        }
    }
}

/// A roda do mouse: no texto sendo digitado (ou com a ferramenta Texto), o tamanho da
/// fonte, 2 px por dente; com a Seta, a espessura, 1 px por dente (a seta sendo arrastada
/// acompanha).
fn wheel_steps(hwnd: HWND, steps: i32) {
    let text_size = with(|s| {
        if let Some(Mark::Text(t)) = s.typing.and_then(|i| s.marks.get_mut(i)) {
            t.size = (t.size + steps as f32 * SIZE_STEP).clamp(SIZE_MIN, SIZE_MAX);
            s.font_size = t.size;
            true
        } else if s.tool == Tool::Text && s.drawing.is_none() {
            s.font_size = (s.font_size + steps as f32 * SIZE_STEP).clamp(SIZE_MIN, SIZE_MAX);
            true
        } else {
            false
        }
    });
    if text_size {
        invalidate_all(hwnd);
        return;
    }
    let (old, new, toolbar) = with(|s| {
        s.width = (s.width + steps as f32).clamp(WIDTH_MIN, WIDTH_MAX);
        let old = s.drawing;
        if let Some(d) = s.drawing.as_mut() {
            d.width = s.width;
        }
        (old, s.drawing, s.toolbar)
    });
    // A barra muda de largura com o número (e se alinha à direita da área): a faixa inteira.
    let toolbar = toolbar.map(|t| RECT { left: 0, right: i32::MAX / 2, ..t });
    for r in [old.map(|a| a.bounds()), new.map(|a| a.bounds()), toolbar].into_iter().flatten() {
        invalidate(hwnd, &r);
    }
}

/// Clique com a ferramenta Texto dentro da área: edita o texto sob o clique (o mais de
/// cima) ou começa um novo ali, com a linha centrada no clique.
fn start_text(hwnd: HWND, p: POINT) {
    end_typing();
    let hit = with(|s| {
        s.marks.iter().rposition(|m| matches!(m, Mark::Text(_)) && in_rect(&m.bounds(), p))
    });
    with(|s| match hit {
        Some(i) => {
            if let Some(Mark::Text(t)) = s.marks.get(i) {
                s.color = t.color;
                s.font_size = t.size;
            }
            s.typing = Some(i);
        }
        None => {
            let size = s.font_size;
            let y = p.y as f32 - crate::annotate::line_height(size) / 2.0;
            s.marks.push(Mark::Text(Text { x: p.x as f32, y, text: String::new(), size, color: s.color }));
            s.typing = Some(s.marks.len() - 1);
        }
    });
    invalidate_all(hwnd);
}

/// Um caractere digitado (`WM_CHAR`) no texto em edição. `false`: não há texto em edição.
fn type_char(hwnd: HWND, unit: u16) -> bool {
    let typed = with(|s| {
        let Some(i) = s.typing else { return false };
        let pending = s.high_surrogate.take();
        let Some(Mark::Text(t)) = s.marks.get_mut(i) else { return false };
        match unit {
            0x08 => {
                t.text.pop();
            }
            0xD800..=0xDBFF => s.high_surrogate = Some(unit),
            0xDC00..=0xDFFF => {
                if let Some(c) = pending.and_then(|hi| char::decode_utf16([hi, unit]).next()).and_then(Result::ok) {
                    t.text.push(c);
                }
            }
            // Controles (Ctrl+letra, Enter, Tab, Esc) não viram texto: o Enter é tratado no WM_KEYDOWN.
            u if u < 0x20 || u == 0x7F => {}
            u => {
                if let Some(c) = char::from_u32(u as u32) {
                    t.text.push(c);
                }
            }
        }
        true
    });
    if typed {
        invalidate_all(hwnd);
    }
    typed
}

extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let editor = with(|s| s.on_shot.is_some());
    match msg {
        WM_PAINT => {
            paint(hwnd);
            LRESULT(0)
        }
        WM_ERASEBKGND => LRESULT(1),
        WM_SETCURSOR => {
            // No editor, a mãozinha sobre os botões da barra e o I do texto dentro da
            // área (com a ferramenta Texto); no resto, a cruz.
            let mut p = POINT::default();
            let known = editor && unsafe { GetCursorPos(&mut p).is_ok() && ScreenToClient(hwnd, &mut p).as_bool() };
            let over_button = known && button_at(p).is_some();
            let over_text = known && with(|s| s.editing && s.tool == Tool::Text && s.sel.is_some_and(|r| in_rect(&r, p)));
            let cursor = if over_button {
                IDC_HAND
            } else if over_text {
                IDC_IBEAM
            } else {
                IDC_CROSS
            };
            unsafe {
                let _ = SetCursor(LoadCursorW(None, cursor).ok());
            }
            LRESULT(1)
        }
        WM_LBUTTONDOWN => {
            // Arrasto já começou: um 2º "aperto" (visto no teste: chega um
            // logo depois do 1º, quando a janela é ativada pelo clique) não
            // pode reiniciar a seleção — o ponto de partida é o primeiro.
            if with(|s| s.anchor.is_some() || s.drawing.is_some()) {
                return LRESULT(0);
            }
            let p = point_from(lparam);
            if editor {
                if let Some(b) = button_at(p) {
                    run_button(hwnd, b);
                    return LRESULT(0);
                }
                // No fundo da barra (entre os botões): nada.
                if with(|s| s.editing && s.toolbar.is_some_and(|t| in_rect(&t, p))) {
                    return LRESULT(0);
                }
                let (editing, sel, width, color, tool) = with(|s| (s.editing, s.sel, s.width, s.color, s.tool));
                if editing && sel.is_some_and(|r| in_rect(&r, p)) {
                    if tool == Tool::Text {
                        start_text(hwnd, p);
                        return LRESULT(0);
                    }
                    // Dentro da área: uma seta (ou um retângulo), do ponto inicial ao final.
                    end_typing();
                    let (x, y) = (p.x as f32, p.y as f32);
                    with(|s| s.drawing = Some(Arrow { x1: x, y1: y, x2: x, y2: y, width, color }));
                    unsafe {
                        SetCapture(hwnd);
                    }
                    return LRESULT(0);
                }
                // Fora: outra área (a de antes volta se for só um clique). A barra sai.
                end_typing();
                with(|s| {
                    s.prev_sel = s.sel;
                    s.editing = false;
                    s.buttons.clear();
                    s.toolbar = None;
                });
                invalidate_all(hwnd);
            }
            let origin = with(|s| s.origin);
            log!("AreaSelect: arrasto comeca em ({}, {}).", p.x + origin.0, p.y + origin.1);
            let hint = with(|s| {
                s.anchor = Some(p);
                std::mem::replace(&mut s.hint_visible, false).then_some(s.hint_rect)
            });
            if let Some(h) = hint {
                invalidate(hwnd, &h);
            }
            unsafe {
                SetCapture(hwnd);
            }
            set_selection(hwnd, Some(RECT { left: p.x, top: p.y, right: p.x, bottom: p.y }));
            LRESULT(0)
        }
        WM_MOUSEMOVE => {
            let p = point_from(lparam);
            if let Some(mut a) = with(|s| s.drawing) {
                let old = a.bounds();
                (a.x2, a.y2) = (p.x as f32, p.y as f32);
                with(|s| s.drawing = Some(a));
                invalidate(hwnd, &old);
                invalidate(hwnd, &a.bounds());
                return LRESULT(0);
            }
            if let Some(a) = with(|s| s.anchor) {
                set_selection(hwnd, Some(norm(a, p)));
            }
            LRESULT(0)
        }
        WM_LBUTTONUP => {
            let p = point_from(lparam);
            if let Some(mut a) = with(|s| s.drawing.take()) {
                unsafe {
                    let _ = ReleaseCapture();
                }
                (a.x2, a.y2) = (p.x as f32, p.y as f32);
                let mark = drag_mark(a, with(|s| s.tool));
                let tiny = match &mark {
                    Mark::Rect(f) => f.is_tiny(),
                    _ => a.is_tiny(),
                };
                if !tiny {
                    with(|s| s.marks.push(mark));
                }
                invalidate(hwnd, &a.bounds());
                return LRESULT(0);
            }
            let (anchor, origin) = with(|s| (s.anchor.take(), s.origin));
            if let Some(a) = anchor {
                unsafe {
                    let _ = ReleaseCapture();
                }
                let r = norm(a, p);
                let (w, h) = (r.right - r.left, r.bottom - r.top);
                if w >= MIN_SIZE && h >= MIN_SIZE {
                    if editor {
                        // A área do print fica, com a barra; as setas vêm depois.
                        with(|s| {
                            s.editing = true;
                            s.prev_sel = None;
                        });
                        set_selection(hwnd, Some(r));
                        invalidate_all(hwnd);
                    } else {
                        finish(Some(AreaRect { x: r.left + origin.0, y: r.top + origin.1, w, h }));
                    }
                } else if editor {
                    // Clique sem arrasto fora da área: a de antes volta.
                    let prev = with(|s| s.prev_sel.take());
                    set_selection(hwnd, prev);
                    with(|s| s.editing = prev.is_some());
                    invalidate_all(hwnd);
                } else {
                    // Clique sem arrasto: volta a esperar.
                    set_selection(hwnd, None);
                }
            }
            LRESULT(0)
        }
        WM_MOUSEWHEEL if editor => {
            // A roda: a espessura da seta ou o tamanho da fonte ([`wheel_steps`]).
            // Touchpad manda frações de dente: conta 1.
            let delta = ((wparam.0 >> 16) & 0xFFFF) as u16 as i16 as i32;
            let steps = if delta.abs() >= 120 { delta / 120 } else { delta.signum() };
            wheel_steps(hwnd, steps);
            LRESULT(0)
        }
        WM_RBUTTONUP => {
            finish(None);
            LRESULT(0)
        }
        WM_CHAR if editor => {
            type_char(hwnd, wparam.0 as u16);
            LRESULT(0)
        }
        WM_KEYDOWN => {
            let vk = wparam.0 as u16;
            let ctrl = unsafe { GetKeyState(VK_CONTROL.0 as i32) } < 0;
            let shift = unsafe { GetKeyState(VK_SHIFT.0 as i32) } < 0;
            let (editing, typing) = with(|s| (s.editing, s.typing.is_some()));
            if editor && typing && vk == VK_ESCAPE.0 {
                // Digitando, o Esc só termina o texto (o print continua).
                end_typing();
                invalidate_all(hwnd);
            } else if editor && typing && vk == VK_RETURN.0 {
                // Shift+Enter: nova linha; Enter: termina o texto (não salva ainda).
                if shift {
                    with(|s| {
                        if let Some(Mark::Text(t)) = s.typing.and_then(|i| s.marks.get_mut(i)) {
                            t.text.push('\n');
                        }
                    });
                } else {
                    end_typing();
                }
                invalidate_all(hwnd);
            } else if vk == VK_ESCAPE.0 {
                finish(None);
            } else if editor && editing && vk == VK_RETURN.0 {
                finish_shot(ShotAction::Save);
            } else if editor && editing && ctrl && vk == u16::from(b'C') {
                finish_shot(ShotAction::Copy);
            } else if editor && ctrl && vk == u16::from(b'Z') {
                // Desfaz a última marca (digitando: o texto em edição termina antes).
                end_typing();
                with(|s| s.marks.pop());
                invalidate_all(hwnd);
            } else if editor && typing {
                // As outras teclas viram WM_CHAR (TranslateMessage); nada de atalho aqui.
                return LRESULT(0);
            } else {
                return unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) };
            }
            LRESULT(0)
        }
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}

fn ensure_class() {
    if with(|s| std::mem::replace(&mut s.registered, true)) {
        return;
    }
    let wc = WNDCLASSW {
        lpfnWndProc: Some(wnd_proc),
        hInstance: hinstance(),
        hCursor: unsafe { LoadCursorW(None, IDC_CROSS).unwrap_or_default() },
        lpszClassName: CLASS_NAME,
        ..Default::default()
    };
    unsafe {
        RegisterClassW(&wc);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A imagem congelada no DC: a nítida é o print como está (RGBA → a cor certa no
    /// GDI); a escura é a mesma imagem escurecida como o véu da seleção ao vivo.
    #[test]
    fn imagem_congelada_nitida_e_escurecida() {
        // 2x1: vermelho puro e um cinza.
        let rgba = [255u8, 0, 0, 255, 200, 100, 50, 255];
        let bright = MemImage::from_rgba(&rgba, 2, 1, 1.0).expect("nítida");
        let k = 1.0 - DIM_ALPHA as f32 / 255.0;
        let dim = MemImage::from_rgba(&rgba, 2, 1, k).expect("escura");
        let px = |img: &MemImage, x: i32| {
            let c = unsafe { GetPixel(img.dc, x, 0) }.0;
            ((c & 0xFF) as u8, ((c >> 8) & 0xFF) as u8, ((c >> 16) & 0xFF) as u8)
        };
        assert_eq!(px(&bright, 0), (255, 0, 0), "vermelho continua vermelho");
        assert_eq!(px(&bright, 1), (200, 100, 50));
        let (r, g, b) = px(&dim, 1);
        let esperado = |v: f32| (v * k) as i32;
        assert!((r as i32 - esperado(200.0)).abs() <= 1 && (g as i32 - esperado(100.0)).abs() <= 1 && (b as i32 - esperado(50.0)).abs() <= 1, "escurecida: {:?}", (r, g, b));
    }

    // ---- editor do print: a janela de verdade, recebendo o mouse e o teclado ----

    static DONE: std::sync::Mutex<Vec<Option<ShotDone>>> = std::sync::Mutex::new(Vec::new());

    fn on_done(d: Option<ShotDone>) {
        DONE.lock().unwrap().push(d);
    }

    fn lp(x: i32, y: i32) -> LPARAM {
        LPARAM(((y as u16 as isize) << 16) | x as u16 as isize)
    }

    fn send(hwnd: HWND, msg: u32, wparam: usize, lparam: LPARAM) {
        wnd_proc(hwnd, msg, WPARAM(wparam), lparam);
    }

    fn drag(hwnd: HWND, from: (i32, i32), to: (i32, i32)) {
        send(hwnd, WM_LBUTTONDOWN, 1, lp(from.0, from.1));
        send(hwnd, WM_MOUSEMOVE, 1, lp((from.0 + to.0) / 2, (from.1 + to.1) / 2));
        send(hwnd, WM_MOUSEMOVE, 1, lp(to.0, to.1));
        send(hwnd, WM_LBUTTONUP, 0, lp(to.0, to.1));
    }

    fn wheel(hwnd: HWND, notches: i32) {
        send(hwnd, WM_MOUSEWHEEL, ((notches * 120) as i16 as u16 as usize) << 16, lp(0, 0));
    }

    /// Uma janela da classe da seleção (escondida) com o editor ligado; origem da tela
    /// virtual em (-1920, 0), como com um monitor à esquerda.
    fn open_editor() -> HWND {
        ensure_class();
        let hwnd = unsafe { CreateWindowExW(WINDOW_EX_STYLE(0), CLASS_NAME, w!(""), WS_POPUP, 0, 0, 1200, 900, None, None, Some(hinstance()), None) }.expect("janela");
        with(|s| {
            s.hwnd = Some(hwnd);
            s.origin = (-1920, 0);
            s.dpi = 96;
            s.on_shot = Some(on_done);
            s.width = 0.0; // os padrões, como no primeiro print
            reset_editor(s);
            s.sel = None;
        });
        hwnd
    }

    fn arrow(x1: f32, y1: f32, x2: f32, y2: f32, width: f32) -> Mark {
        Mark::Arrow(Arrow { x1, y1, x2, y2, width, color: crate::annotate::COLOR })
    }

    fn chars(hwnd: HWND, text: &str) {
        for u in text.encode_utf16() {
            send(hwnd, WM_CHAR, u as usize, LPARAM(0));
        }
    }

    fn key(hwnd: HWND, vk: windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY) {
        send(hwnd, WM_KEYDOWN, vk.0 as usize, LPARAM(0));
    }

    fn click(hwnd: HWND, b: ShotButton) {
        paint_bar(hwnd);
        let (x, y) = center(b);
        send(hwnd, WM_LBUTTONDOWN, 1, lp(x, y));
        send(hwnd, WM_LBUTTONUP, 0, lp(x, y));
    }

    fn texts() -> Vec<Text> {
        with(|s| s.marks.iter().filter_map(|m| if let Mark::Text(t) = m { Some(t.clone()) } else { None }).collect())
    }

    /// Texto: a ferramenta Texto, um clique na área e o teclado. A cor e a roda mudam o
    /// texto sendo digitado; Enter termina o texto sem salvar; clicar nele de novo edita;
    /// Backspace apaga; emoji (dois UTF-16) entra inteiro. Salvar leva o texto (em tela).
    #[test]
    fn editor_texto_com_cor_e_tamanho() {
        let _t = crate::testkit::begin();
        take_done();
        let hwnd = open_editor();
        drag(hwnd, (100, 100), (700, 500));
        click(hwnd, ShotButton::Tool(Tool::Text));
        assert_eq!(with(|s| s.tool), Tool::Text);
        send(hwnd, WM_LBUTTONDOWN, 1, lp(200, 200));
        send(hwnd, WM_LBUTTONUP, 0, lp(200, 200));
        assert_eq!(with(|s| s.typing), Some(0), "digitando no texto novo");
        chars(hwnd, "Olá!x");
        send(hwnd, WM_CHAR, 0x08, LPARAM(0)); // Backspace
        chars(hwnd, " 👍");
        chars(hwnd, "\u{3}"); // Ctrl+C não vira texto (o WM_CHAR de controle)
        click(hwnd, ShotButton::Color(3)); // azul, no texto sendo digitado
        wheel(hwnd, 4); // +8 px
        let t = texts().pop().expect("texto");
        assert_eq!(t.text, "Olá! 👍");
        assert_eq!((t.color, t.size), (PALETTE[3], crate::annotate::SIZE_DEFAULT + 8.0));
        assert_eq!(with(|s| s.typing), Some(0), "a cor e a roda não terminam o texto");

        key(hwnd, VK_RETURN);
        assert_eq!(with(|s| s.typing), None, "Enter termina o texto");
        assert!(take_done().is_empty(), "Enter digitando não salva");

        // Clicar no texto edita ele (não cria outro).
        let b = Mark::Text(t.clone()).bounds();
        let (cx, cy) = ((b.left + b.right) / 2, (b.top + b.bottom) / 2);
        send(hwnd, WM_LBUTTONDOWN, 1, lp(cx, cy));
        send(hwnd, WM_LBUTTONUP, 0, lp(cx, cy));
        assert_eq!(with(|s| s.typing), Some(0));
        chars(hwnd, "?");
        key(hwnd, VK_ESCAPE); // digitando, o Esc só termina o texto
        assert!(with(|s| s.on_shot.is_some()), "o print continua aberto");
        assert_eq!(texts().len(), 1);
        assert_eq!(texts()[0].text, "Olá! 👍?");

        // Um texto começado e deixado vazio some; a seta sai na cor escolhida (azul).
        send(hwnd, WM_LBUTTONDOWN, 1, lp(600, 450));
        send(hwnd, WM_LBUTTONUP, 0, lp(600, 450));
        click(hwnd, ShotButton::Tool(Tool::Arrow));
        assert_eq!(texts().len(), 1, "o texto vazio saiu");
        drag(hwnd, (150, 400), (400, 420));
        assert!(matches!(with(|s| s.marks.last().cloned()), Some(Mark::Arrow(a)) if a.color == PALETTE[3]));

        click(hwnd, ShotButton::Save);
        let d = take_done().pop().flatten().expect("salvo");
        let saved: Vec<&Text> = d.marks.iter().filter_map(|m| if let Mark::Text(t) = m { Some(t) } else { None }).collect();
        assert_eq!(saved.len(), 1);
        assert_eq!((saved[0].x, saved[0].text.as_str()), (200.0 - 1920.0, "Olá! 👍?"));
        assert_eq!(d.marks.len(), 2, "o texto e a seta");
    }

    /// Retângulo: arrastar dentro da área demarca de canto a canto, na cor e na
    /// espessura escolhidas (a roda muda a espessura, como na seta); um risco sem altura
    /// não vira retângulo. Ctrl+Z não é testável aqui (Ctrl vem do teclado de verdade).
    #[test]
    fn editor_retangulo_demarca_area() {
        let _t = crate::testkit::begin();
        take_done();
        let hwnd = open_editor();
        drag(hwnd, (100, 100), (700, 500));
        click(hwnd, ShotButton::Tool(Tool::Rect));
        click(hwnd, ShotButton::Color(2));
        // Arrastado de baixo-direita para cima-esquerda, com a roda +2 no meio.
        send(hwnd, WM_LBUTTONDOWN, 1, lp(400, 350));
        send(hwnd, WM_MOUSEMOVE, 1, lp(300, 250));
        wheel(hwnd, 2);
        assert!(matches!(with(|s| s.drawing.map(|a| drag_mark(a, s.tool))), Some(Mark::Rect(f)) if f.width == 8.0));
        send(hwnd, WM_LBUTTONUP, 0, lp(200, 150));
        drag(hwnd, (150, 300), (450, 301)); // sem altura: nada
        let marks = with(|s| s.marks.clone());
        assert_eq!(marks, vec![Mark::Rect(Frame { x1: 400.0, y1: 350.0, x2: 200.0, y2: 150.0, width: 8.0, color: PALETTE[2] })]);
        assert_eq!(with(|s| s.sel), Some(RECT { left: 100, top: 100, right: 700, bottom: 500 }), "demarcar não mexe na área");

        click(hwnd, ShotButton::Save);
        let d = take_done().pop().flatten().expect("salvo");
        assert_eq!(d.marks, vec![Mark::Rect(Frame { x1: 400.0 - 1920.0, y1: 350.0, x2: 200.0 - 1920.0, y2: 150.0, width: 8.0, color: PALETTE[2] })]);
    }

    /// A ferramenta, a cor e os tamanhos ficam para o próximo print; as marcas, não.
    #[test]
    fn editor_lembra_ferramenta_e_cor() {
        let _t = crate::testkit::begin();
        take_done();
        let hwnd = open_editor();
        drag(hwnd, (100, 100), (500, 400));
        click(hwnd, ShotButton::Tool(Tool::Text));
        click(hwnd, ShotButton::Color(1));
        wheel(hwnd, -2);
        key(hwnd, VK_ESCAPE);
        take_done();
        with(reset_editor);
        assert_eq!(with(|s| (s.tool, s.color, s.font_size, s.marks.len())), (Tool::Text, PALETTE[1], crate::annotate::SIZE_DEFAULT - 4.0, 0));
    }

    /// A barra é desenhada no `WM_PAINT` (que a janela escondida não recebe): aqui, num DC de memória.
    fn paint_bar(hwnd: HWND) {
        let sel = with(|s| s.sel).expect("área");
        let img = MemImage::from_rgba(&vec![0u8; 1200 * 900 * 4], 1200, 900, 1.0).expect("dc");
        paint_toolbar(hwnd, img.dc, sel);
    }

    fn center(b: ShotButton) -> (i32, i32) {
        let r = with(|s| s.buttons.iter().find(|(k, _)| *k == b).map(|(_, r)| *r)).expect("botão");
        ((r.left + r.right) / 2, (r.top + r.bottom) / 2)
    }

    fn take_done() -> Vec<Option<ShotDone>> {
        crate::mainthread::drain();
        std::mem::take(&mut *DONE.lock().unwrap())
    }

    /// Soltar a seleção não termina: a área fica com a barra. Arrastar dentro dela faz
    /// uma seta (do ponto inicial ao final), com a espessura da roda; Salvar devolve a
    /// área e as setas em coordenadas de tela.
    #[test]
    fn editor_seleciona_desenha_seta_e_salva() {
        let _t = crate::testkit::begin();
        take_done();
        let hwnd = open_editor();
        drag(hwnd, (100, 100), (500, 400));
        assert!(with(|s| s.editing), "a área ficou, esperando");
        assert_eq!(with(|s| s.sel), Some(RECT { left: 100, top: 100, right: 500, bottom: 400 }));
        assert!(take_done().is_empty(), "nada entregue ainda");

        // Seta: roda +3 durante o arrasto (a seta em andamento acompanha).
        send(hwnd, WM_LBUTTONDOWN, 1, lp(150, 150));
        send(hwnd, WM_MOUSEMOVE, 1, lp(300, 220));
        wheel(hwnd, 3);
        assert_eq!(with(|s| s.drawing.map(|a| a.width)), Some(crate::annotate::WIDTH_DEFAULT + 3.0));
        send(hwnd, WM_LBUTTONUP, 0, lp(320, 230));
        // Um clique sem arrasto não vira seta.
        drag(hwnd, (200, 300), (201, 301));
        // A roda não passa dos limites.
        wheel(hwnd, -100);
        assert_eq!(with(|s| s.width), WIDTH_MIN);
        wheel(hwnd, 100);
        assert_eq!(with(|s| s.width), WIDTH_MAX);

        assert_eq!(with(|s| s.marks.clone()), vec![arrow(150.0, 150.0, 320.0, 230.0, 9.0)]);
        assert_eq!(with(|s| s.sel), Some(RECT { left: 100, top: 100, right: 500, bottom: 400 }), "desenhar não mexe na área");

        paint_bar(hwnd);
        let (x, y) = center(ShotButton::Save);
        send(hwnd, WM_LBUTTONDOWN, 1, lp(x, y));
        let done = take_done();
        assert_eq!(done.len(), 1);
        let d = done[0].clone().expect("salvo");
        assert_eq!(d.action, ShotAction::Save);
        assert_eq!(d.area, AreaRect { x: 100 - 1920, y: 100, w: 400, h: 300 });
        assert_eq!(d.marks, vec![arrow(150.0 - 1920.0, 150.0, 320.0 - 1920.0, 230.0, 9.0)]);
        assert!(!is_active() && with(|s| s.hwnd.is_none() && s.on_shot.is_none()), "fechou");
    }

    /// Seleção ruim: arrastar fora da área escolhe outra (as setas ficam); só um clique
    /// fora devolve a de antes. Copiar pelo botão, Enter salva e Esc cancela.
    #[test]
    fn editor_refaz_a_area_copia_e_cancela() {
        let _t = crate::testkit::begin();
        take_done();
        let hwnd = open_editor();
        drag(hwnd, (100, 100), (300, 300));
        drag(hwnd, (150, 150), (250, 200)); // uma seta
        drag(hwnd, (600, 500), (601, 500)); // clique fora: a área de antes volta
        assert!(with(|s| s.editing));
        assert_eq!(with(|s| s.sel), Some(RECT { left: 100, top: 100, right: 300, bottom: 300 }));
        drag(hwnd, (50, 60), (700, 600)); // outra área, maior
        assert_eq!(with(|s| s.sel), Some(RECT { left: 50, top: 60, right: 700, bottom: 600 }));
        assert_eq!(with(|s| s.marks.len()), 1, "a seta continua");

        // O fundo da barra não começa outra seleção.
        paint_bar(hwnd);
        let bar = with(|s| s.toolbar).expect("barra");
        send(hwnd, WM_LBUTTONDOWN, 1, lp(bar.left + 2, bar.top + 2));
        send(hwnd, WM_LBUTTONUP, 0, lp(bar.left + 2, bar.top + 2));
        assert_eq!(with(|s| s.sel), Some(RECT { left: 50, top: 60, right: 700, bottom: 600 }));

        let (x, y) = center(ShotButton::Copy);
        send(hwnd, WM_LBUTTONDOWN, 1, lp(x, y));
        let d = take_done().pop().flatten().expect("copiado");
        assert_eq!(d.action, ShotAction::Copy);
        assert_eq!(d.area, AreaRect { x: 50 - 1920, y: 60, w: 650, h: 540 });

        // Enter salva; antes de escolher a área, Enter não faz nada.
        let hwnd = open_editor();
        send(hwnd, WM_KEYDOWN, VK_RETURN.0 as usize, LPARAM(0));
        assert!(take_done().is_empty());
        drag(hwnd, (10, 10), (210, 110));
        send(hwnd, WM_KEYDOWN, VK_RETURN.0 as usize, LPARAM(0));
        assert_eq!(take_done().pop().flatten().map(|d| d.action), Some(ShotAction::Save));

        // Esc: cancelado (None), e nada mais.
        let hwnd = open_editor();
        drag(hwnd, (10, 10), (210, 110));
        send(hwnd, WM_KEYDOWN, VK_ESCAPE.0 as usize, LPARAM(0));
        let done = take_done();
        assert!(done.len() == 1 && done[0].is_none(), "cancelado: {done:?}");
    }
}
