//! Seleção de ÁREA da tela ("Capturar... → Área" e a área de um atalho).
//!
//! Escurece TODOS os monitores com uma janela só, do tamanho da área de
//! trabalho virtual, e o usuário arrasta o retângulo que quer gravar. Soltar o
//! botão confirma; Esc ou o botão direito cancelam. (O print tem o editor dele, no
//! Aylis: [`crate::print_editor`].)
//!
//! A janela é layered com DOIS atributos ao mesmo tempo: alfa (o escuro
//! translúcido) e uma cor-chave, que vira transparente de verdade — é ela que
//! "fura" o escuro no retângulo escolhido, então o usuário vê a área nítida.
//!
//! O `WM_PAINT` monta o quadro num buffer do tamanho da região invalidada (não da
//! tela: em dois monitores 4K seriam ~66 MB) e copia para a tela de uma vez — o
//! Windows nunca mostra o desenho pela metade. Mover a seleção invalida só o que
//! muda (as faixas que entram ou saem, a moldura e o rótulo), não a área inteira.
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
use windows::Win32::UI::Input::KeyboardAndMouse::{ReleaseCapture, SetCapture, VK_ESCAPE};
use windows::Win32::UI::WindowsAndMessaging::*;

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

/// Uma imagem num DC de memória (o buffer do `WM_PAINT`).
struct MemImage {
    dc: HDC,
    bmp: HBITMAP,
    old: HGDIOBJ,
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

#[derive(Default)]
struct State {
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
    /// O buffer do `WM_PAINT` ([`paint`]): o quadro é montado nele e vai à tela de uma vez.
    /// Cresce com a maior região pintada e fica até a seleção fechar.
    back: Option<(MemImage, i32, i32)>,
}

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

/// Seleção aberta agora (a de área ou o editor do print)?
pub fn is_active() -> bool {
    #[cfg(test)]
    if TEST_PENDING.with(|p| p.get().is_some()) {
        return true;
    }
    with(|s| s.hwnd.is_some()) || crate::print_editor::is_active()
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

        // Translúcida, com o furo transparente.
        let Ok(hwnd) = CreateWindowExW(
            WS_EX_LAYERED | WS_EX_TOPMOST | WS_EX_TOOLWINDOW,
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
        let _ = SetLayeredWindowAttributes(hwnd, KEY, DIM_ALPHA, LWA_ALPHA | LWA_COLORKEY);
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
        });
        let _ = ShowWindow(hwnd, SW_SHOW);
        // Foco pra o Esc funcionar. Sem ele (trava de primeiro plano do
        // Windows) o botão direito ainda cancela.
        crate::ui::bring_to_front(hwnd);
        if GetForegroundWindow() != hwnd {
            log::log("AreaSelect: o Windows nao deu o primeiro plano a selecao (o Esc so vale depois de um clique).");
        }
        log!("AreaSelect: aberta ({vw}x{vh} em {vx},{vy}).");
    }
    true
}

/// Fecha e avisa quem abriu. O callback sai pela fila da main: ele começa a
/// gravação (pesado) e não pode rodar dentro do `WndProc` desta janela.
fn finish(result: Option<AreaRect>) {
    let cb = close();
    match result {
        Some(a) => log!("AreaSelect: area escolhida {}x{} em ({}, {}).", a.w, a.h, a.x, a.y),
        None => log::log("AreaSelect: cancelada."),
    }
    if let Some(cb) = cb {
        mainthread::queue(move || cb(result));
    }
}

/// Fecha a janela e solta o que ela segura (a fonte e o buffer); devolve quem esperava o
/// resultado.
fn close() -> Option<OnSelected> {
    let (hwnd, cb, font, back) = with(|s| (s.hwnd.take(), s.on_selected.take(), std::mem::take(&mut s.font), s.back.take()));
    drop(back);
    if let Some(h) = hwnd {
        unsafe {
            let _ = ReleaseCapture();
            let _ = DestroyWindow(h);
        }
    }
    if !font.is_invalid() {
        unsafe {
            let _ = DeleteObject(HGDIOBJ(font.0));
        }
    }
    cb
}

fn norm(a: POINT, b: POINT) -> RECT {
    RECT { left: a.x.min(b.x), top: a.y.min(b.y), right: a.x.max(b.x), bottom: a.y.max(b.y) }
}

fn inflate(r: RECT, d: i32) -> RECT {
    RECT { left: r.left - d, top: r.top - d, right: r.right + d, bottom: r.bottom + d }
}

/// Faixa do rótulo "1280 × 720", acima da seleção (ou abaixo, colada no topo).
fn label_rect(sel: RECT, dpi: i32) -> RECT {
    let ext = text_extent(label_font(dpi), &label_text(sel));
    let (w, h, gap) = (ext.cx + 2 * sx(dpi, 7), ext.cy + 2 * sx(dpi, 2), sx(dpi, 6));
    let top = if sel.top - h - gap >= 0 { sel.top - h - gap } else { sel.bottom + gap };
    RECT { left: sel.left, top, right: sel.left + w, bottom: top + h }
}

/// O texto do rótulo do tamanho: "1280 × 720".
fn label_text(sel: RECT) -> Vec<u16> {
    wide(&format!("{} × {}", sel.right - sel.left, sel.bottom - sel.top))
}

thread_local! {
    /// A fonte do rótulo do tamanho, por DPI (criada uma vez; vive até o fim do app).
    static LABEL_FONT: std::cell::Cell<(i32, HFONT)> = const { std::cell::Cell::new((0, HFONT(std::ptr::null_mut()))) };
}

/// A fonte do rótulo do tamanho: pequena, a Segoe UI Variable do Windows 11 (a Segoe UI
/// onde ela não existe).
fn label_font(dpi: i32) -> HFONT {
    LABEL_FONT.with(|c| {
        let (d, f) = c.get();
        if d == dpi && !f.is_invalid() {
            return f;
        }
        let make = |face: PCWSTR| unsafe {
            CreateFontW(-sx(dpi, 12), 0, 0, 0, FW_SEMIBOLD.0 as i32, 0, 0, 0, DEFAULT_CHARSET, OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS, CLEARTYPE_QUALITY, DEFAULT_PITCH.0 as u32, face)
        };
        let mut font = make(w!("Segoe UI Variable Text"));
        // Sem ela, o Windows troca por outra qualquer: confere o nome que saiu.
        let got = unsafe {
            let dc = CreateCompatibleDC(None);
            let old = SelectObject(dc, HGDIOBJ(font.0));
            let mut name = [0u16; 64];
            let n = GetTextFaceW(dc, Some(&mut name));
            SelectObject(dc, old);
            let _ = DeleteDC(dc);
            String::from_utf16_lossy(&name[..(n.max(1) - 1) as usize])
        };
        if !got.starts_with("Segoe UI Variable") {
            unsafe {
                let _ = DeleteObject(HGDIOBJ(font.0));
            }
            font = make(w!("Segoe UI"));
        }
        c.set((dpi, font));
        font
    })
}

/// A espessura da borda branca em volta da seleção.
fn border_px(_dpi: i32) -> i32 {
    // Fina como a das ferramentas de captura: 1 px em qualquer escala.
    1
}

/// A borda tracejada da área `r`, `w` px por fora dela: um fio escuro com traços brancos
/// de `dash` px (contraste em qualquer fundo). Os traços começam nos cantos da área (andam
/// com ela) e só os que cruzam `clip` são desenhados (o arrasto repinta pedaços).
unsafe fn dashed_border(dc: HDC, r: RECT, w: i32, dash: i32, clip: RECT) {
    let o = inflate(r, w);
    let sides = [
        RECT { left: o.left, top: o.top, right: o.right, bottom: r.top },
        RECT { left: o.left, top: r.bottom, right: o.right, bottom: o.bottom },
        RECT { left: o.left, top: r.top, right: r.left, bottom: r.bottom },
        RECT { left: r.right, top: r.top, right: o.right, bottom: r.bottom },
    ];
    let dark = CreateSolidBrush(COLORREF(0x0020_2020));
    let light = CreateSolidBrush(COLORREF(0x00FF_FFFF));
    let mut part = RECT::default();
    for (i, side) in sides.iter().enumerate() {
        if !IntersectRect(&mut part, side, &clip).as_bool() {
            continue;
        }
        FillRect(dc, side, dark);
        let horizontal = i < 2;
        let (start, end) = if horizontal { (side.left, side.right) } else { (side.top, side.bottom) };
        // Só os traços dentro do pedaço a pintar (o lado de uma área grande tem centenas).
        let (from, to) = if horizontal { (part.left, part.right) } else { (part.top, part.bottom) };
        let mut a = start + ((from - start) / (2 * dash)) * 2 * dash;
        while a < end.min(to) {
            let b = (a + dash).min(end);
            let seg = if horizontal { RECT { left: a, right: b, ..*side } } else { RECT { top: a, bottom: b, ..*side } };
            FillRect(dc, &seg, light);
            a += 2 * dash;
        }
    }
    let _ = DeleteObject(HGDIOBJ(dark.0));
    let _ = DeleteObject(HGDIOBJ(light.0));
}

fn invalidate(hwnd: HWND, r: &RECT) {
    unsafe {
        let _ = InvalidateRect(Some(hwnd), Some(r), false);
    }
}

/// Junta `b` em `a` (`mode`) e apaga `b`.
unsafe fn combine(a: HRGN, b: HRGN, mode: RGN_COMBINE_MODE) {
    let _ = CombineRgn(Some(a), Some(a), Some(b), mode);
    let _ = DeleteObject(HGDIOBJ(b.0));
}

fn rect_rgn(r: RECT) -> HRGN {
    unsafe { CreateRectRgn(r.left, r.top, r.right, r.bottom) }
}

/// O que muda na tela quando a seleção vai de `old` para `new`: só o que trocou de
/// dentro para fora (ou o contrário), as duas molduras e os dois rótulos. O miolo comum
/// fica como está — arrastar o canto de uma área grande repinta só as faixas da borda,
/// não a área inteira a cada movimento do mouse. Quem chama apaga a região.
fn selection_change(old: Option<RECT>, new: Option<RECT>, dpi: i32) -> HRGN {
    // A moldura vai da borda (por fora) até um pouco por dentro.
    let half = sx(dpi, 4) + 1;
    let b = border_px(dpi).max(half) + 1;
    let outer = |r: Option<RECT>| r.map(|r| rect_rgn(inflate(r, b))).unwrap_or_else(|| rect_rgn(RECT::default()));
    unsafe {
        let rgn = outer(old);
        combine(rgn, outer(new), RGN_XOR);
        for r in old.iter().chain(new.iter()) {
            let frame = rect_rgn(inflate(*r, b));
            // Seleção fina demais (o primeiro clique): ela toda é moldura.
            if r.right - r.left > 2 * half + 2 && r.bottom - r.top > 2 * half + 2 {
                combine(frame, rect_rgn(inflate(*r, -half)), RGN_DIFF);
            }
            combine(rgn, frame, RGN_OR);
            combine(rgn, rect_rgn(label_rect(*r, dpi)), RGN_OR);
        }
        rgn
    }
}

fn set_selection(hwnd: HWND, new_sel: Option<RECT>) {
    let (old, dpi) = with(|s| (std::mem::replace(&mut s.sel, new_sel), s.dpi));
    if old == new_sel {
        return;
    }
    unsafe {
        let rgn = selection_change(old, new_sel, dpi);
        let _ = InvalidateRgn(hwnd, Some(rgn), false);
        let _ = DeleteObject(HGDIOBJ(rgn.0));
    }
}

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

/// O tamanho de `text` na fonte `font` (num DC de memória: serve fora do `WM_PAINT`).
fn text_extent(font: HFONT, text: &[u16]) -> SIZE {
    unsafe {
        let dc = CreateCompatibleDC(None);
        let old = (!font.is_invalid()).then(|| SelectObject(dc, HGDIOBJ(font.0)));
        let mut size = SIZE::default();
        let _ = GetTextExtentPoint32W(dc, text, &mut size);
        if let Some(o) = old {
            SelectObject(dc, o);
        }
        let _ = DeleteDC(dc);
        size
    }
}

unsafe fn fill(dc: HDC, r: &RECT, bgr: u32) {
    let brush = CreateSolidBrush(COLORREF(bgr));
    FillRect(dc, r, brush);
    let _ = DeleteObject(HGDIOBJ(brush.0));
}

impl MemImage {
    /// Uma imagem em branco do formato da tela (o buffer do `WM_PAINT`).
    fn blank(w: i32, h: i32) -> Option<MemImage> {
        unsafe {
            let screen = GetDC(None);
            let dc = CreateCompatibleDC(Some(screen));
            let bmp = CreateCompatibleBitmap(screen, w, h);
            ReleaseDC(None, screen);
            if dc.is_invalid() || bmp.is_invalid() {
                if !bmp.is_invalid() {
                    let _ = DeleteObject(HGDIOBJ(bmp.0));
                }
                if !dc.is_invalid() {
                    let _ = DeleteDC(dc);
                }
                return None;
            }
            let old = SelectObject(dc, HGDIOBJ(bmp.0));
            Some(MemImage { dc, bmp, old })
        }
    }
}

/// O DC do buffer do `WM_PAINT`, de pelo menos `w`×`h` (cresce, não encolhe).
fn back_buffer(w: i32, h: i32) -> Option<HDC> {
    with(|s| {
        let fits = s.back.as_ref().is_some_and(|(_, bw, bh)| *bw >= w && *bh >= h);
        if !fits {
            let (bw, bh) = s.back.as_ref().map_or((w, h), |(_, bw, bh)| ((*bw).max(w), (*bh).max(h)));
            s.back = None;
            s.back = MemImage::blank(bw, bh).map(|img| (img, bw, bh));
        }
        s.back.as_ref().map(|(img, _, _)| img.dc)
    })
}

/// O `WM_PAINT` em buffer duplo: monta só a região a pintar (o recorte do buffer é a
/// região, então as cópias grandes tocam só nela) e copia para a tela de uma vez. Sem
/// isso, o Windows mostrava o meio do desenho (o escuro já por cima, a imagem nítida
/// ainda não) — a borda "atrasada" ao arrastar.
fn paint(hwnd: HWND) {
    LAST_PAINT.with(|c| c.set(Some(std::time::Instant::now())));
    paint_inner(hwnd);
}

/// Os retângulos de uma região (vazio se não der para ler).
fn region_rects(rgn: HRGN) -> Vec<RECT> {
    unsafe {
        let size = GetRegionData(rgn, 0, None);
        if size == 0 {
            return Vec::new();
        }
        // u64: o RGNDATA fica alinhado.
        let mut buf = vec![0u64; (size as usize).div_ceil(8)];
        let data = buf.as_mut_ptr() as *mut RGNDATA;
        if GetRegionData(rgn, size, Some(data)) == 0 {
            return Vec::new();
        }
        let n = (*data).rdh.nCount as usize;
        let first = (*data).Buffer.as_ptr() as *const RECT;
        std::slice::from_raw_parts(first, n).to_vec()
    }
}

fn paint_inner(hwnd: HWND) {
    unsafe {
        // A região antes do BeginPaint, que a esvazia.
        let rgn = CreateRectRgn(0, 0, 0, 0);
        let kind = GetUpdateRgn(hwnd, rgn, false);
        let mut ps = PAINTSTRUCT::default();
        let screen = BeginPaint(hwnd, &mut ps);
        let rc = ps.rcPaint;
        let (w, h) = (rc.right - rc.left, rc.bottom - rc.top);
        if w > 0 && h > 0 {
            match back_buffer(w, h) {
                Some(buf) => {
                    if kind == NULLREGION || kind == RGN_ERROR {
                        let _ = SetRectRgn(rgn, rc.left, rc.top, rc.right, rc.bottom);
                    }
                    let _ = OffsetRgn(rgn, -rc.left, -rc.top);
                    let _ = SelectClipRgn(buf, Some(rgn));
                    compose(buf, (rc.left, rc.top), rc);
                    let _ = SelectClipRgn(buf, None);
                    // Para a tela, só os retângulos da região (o arrasto muda faixas nas
                    // bordas; o retângulo em volta delas pode ser a tela quase inteira).
                    // Retalhada demais, vai o retângulo todo.
                    let parts = region_rects(rgn);
                    if parts.is_empty() || parts.len() > 64 {
                        let _ = BitBlt(screen, rc.left, rc.top, w, h, Some(buf), 0, 0, SRCCOPY);
                    } else {
                        for p in parts {
                            let _ = BitBlt(screen, rc.left + p.left, rc.top + p.top, p.right - p.left, p.bottom - p.top, Some(buf), p.left, p.top, SRCCOPY);
                        }
                    }
                }
                // Sem memória para o buffer: direto na tela, como antes.
                None => compose(screen, (0, 0), rc),
            }
        }
        let _ = DeleteObject(HGDIOBJ(rgn.0));
        let _ = EndPaint(hwnd, &ps);
    }
}

/// Desenha o quadro na área `rc` (cliente) de `dc`, cujo (0, 0) é o ponto `org` do
/// cliente: o escuro, o furo da área, a borda, o rótulo do tamanho (ou a instrução, antes
/// do 1º clique).
unsafe fn compose(dc: HDC, org: (i32, i32), rc: RECT) {
    let _ = SetViewportOrgEx(dc, -org.0, -org.1, None);
    let (sel, dpi, font, hint_rect, hint_visible) = with(|s| (s.sel, s.dpi, s.font, s.hint_rect, s.hint_visible));
    let border = border_px(dpi);

    // 1. Escuro, fora do furo + borda.
    let saved = SaveDC(dc);
    if let Some(r) = sel {
        let o = inflate(r, border);
        ExcludeClipRect(dc, o.left, o.top, o.right, o.bottom);
    }
    fill(dc, &rc, 0);
    let _ = RestoreDC(dc, saved);

    if let Some(r) = sel {
        // 2. Furo (cor-chave = transparente), só onde a área a pintar o cruza, e a borda
        // por FORA dele.
        let mut part = RECT::default();
        if IntersectRect(&mut part, &r, &rc).as_bool() {
            fill(dc, &part, KEY.0);
        }
        dashed_border(dc, r, border, sx(dpi, 4).max(2), rc);
        // 3. Tamanho, em px físicos — é o tamanho do vídeo (arredondado
        // pro múltiplo de 16 na gravação).
        let lr = label_rect(r, dpi);
        fill(dc, &lr, 0x0017_1514);
        let old_font = SelectObject(dc, HGDIOBJ(label_font(dpi).0));
        SetBkMode(dc, TRANSPARENT);
        SetTextColor(dc, COLORREF(0x00E6_E6E6));
        let mut text = label_text(r);
        let mut lr2 = lr;
        DrawTextW(dc, &mut text, &mut lr2, DT_CENTER | DT_VCENTER | DT_SINGLELINE);
        SelectObject(dc, old_font);
    } else if hint_visible {
        // Instrução no meio do monitor do cursor, até o 1º clique.
        let old_font = SelectObject(dc, HGDIOBJ(font.0));
        SetBkMode(dc, TRANSPARENT);
        SetTextColor(dc, COLORREF(0x00FF_FFFF));
        let mut text = wide(&lang::tr(with(|s| s.hint_key)));
        let mut hr = hint_rect;
        DrawTextW(dc, &mut text, &mut hr, DT_CENTER | DT_VCENTER | DT_SINGLELINE);
        SelectObject(dc, old_font);
    }
    let _ = SetViewportOrgEx(dc, 0, 0, None);
}

fn point_from(lparam: LPARAM) -> POINT {
    // GET_X/Y_LPARAM: com sinal (monitor à esquerda do principal).
    POINT { x: (lparam.0 & 0xFFFF) as i16 as i32, y: ((lparam.0 >> 16) & 0xFFFF) as i16 as i32 }
}

thread_local! {
    /// A última pintura e o intervalo entre quadros do monitor.
    static LAST_PAINT: std::cell::Cell<Option<std::time::Instant>> = const { std::cell::Cell::new(None) };
    static FRAME: std::cell::Cell<Option<std::time::Duration>> = const { std::cell::Cell::new(None) };
}

/// Um quadro do monitor (a maior taxa entre as telas; 60 Hz se não der para ler).
fn frame_interval() -> std::time::Duration {
    FRAME.with(|c| {
        if let Some(d) = c.get() {
            return d;
        }
        let d = {
            let mut hz = 0u32;
            unsafe {
                let mut i = 0;
                loop {
                    let mut dd = DISPLAY_DEVICEW { cb: std::mem::size_of::<DISPLAY_DEVICEW>() as u32, ..Default::default() };
                    if !EnumDisplayDevicesW(PCWSTR::null(), i, &mut dd, 0).as_bool() {
                        break;
                    }
                    i += 1;
                    let mut dm = DEVMODEW { dmSize: std::mem::size_of::<DEVMODEW>() as u16, ..Default::default() };
                    if EnumDisplaySettingsW(PCWSTR(dd.DeviceName.as_ptr()), ENUM_CURRENT_SETTINGS, &mut dm).as_bool() {
                        hz = hz.max(dm.dmDisplayFrequency);
                    }
                }
            }
            let hz = if (24..=1000).contains(&hz) { hz } else { 60 };
            log!("AreaSelect: monitor a {hz} Hz (no maximo um quadro por atualizacao da tela).");
            std::time::Duration::from_secs_f64(1.0 / hz as f64)
        };
        c.set(Some(d));
        d
    })
}

/// Pinta já o que foi invalidado (o arrasto acompanha o mouse sem esperar a fila vazia),
/// mas no máximo um quadro por atualização do monitor: um mouse de 1000 Hz mandava mais
/// quadros do que a tela mostra, e o excesso só pesava (o ponteiro chegava a engasgar). O
/// que fica para depois sai no próximo movimento ou no `WM_PAINT`, quando o mouse para.
fn paint_now(hwnd: HWND) {
    #[cfg(not(test))]
    {
        let early = LAST_PAINT.with(|c| c.get()).is_some_and(|t| t.elapsed() < frame_interval());
        if early {
            return;
        }
    }
    unsafe {
        let _ = UpdateWindow(hwnd);
    }
}

extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_PAINT => {
            paint(hwnd);
            LRESULT(0)
        }
        WM_ERASEBKGND => LRESULT(1),
        WM_LBUTTONDOWN => {
            // Arrasto já começou: um 2º "aperto" (visto no teste: chega um
            // logo depois do 1º, quando a janela é ativada pelo clique) não
            // pode reiniciar a seleção — o ponto de partida é o primeiro.
            if with(|s| s.anchor.is_some()) {
                return LRESULT(0);
            }
            let p = point_from(lparam);
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
            if let Some(a) = with(|s| s.anchor) {
                set_selection(hwnd, Some(norm(a, p)));
                paint_now(hwnd);
            }
            LRESULT(0)
        }
        WM_LBUTTONUP => {
            let p = point_from(lparam);
            let (anchor, origin) = with(|s| (s.anchor.take(), s.origin));
            if let Some(a) = anchor {
                unsafe {
                    let _ = ReleaseCapture();
                }
                let r = norm(a, p);
                let (w, h) = (r.right - r.left, r.bottom - r.top);
                if w >= MIN_SIZE && h >= MIN_SIZE {
                    finish(Some(AreaRect { x: r.left + origin.0, y: r.top + origin.1, w, h }));
                } else {
                    // Clique sem arrasto: volta a esperar.
                    set_selection(hwnd, None);
                }
            }
            LRESULT(0)
        }
        WM_RBUTTONUP => {
            finish(None);
            LRESULT(0)
        }
        WM_KEYDOWN if wparam.0 as u16 == VK_ESCAPE.0 => {
            finish(None);
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

    /// Arrastar o canto: a região a repintar é a faixa que entrou, as molduras e os
    /// rótulos — não o miolo da área, que não muda.
    #[test]
    fn mover_a_selecao_repinta_so_o_que_muda() {
        let old = RECT { left: 100, top: 100, right: 500, bottom: 400 };
        let new = RECT { right: 520, ..old };
        let rgn = selection_change(Some(old), Some(new), 96);
        let has = |x: i32, y: i32| unsafe { PtInRegion(rgn, x, y).as_bool() };
        assert!(!has(300, 250), "o miolo fica");
        assert!(has(510, 250), "a faixa nova");
        assert!(has(500, 250) && has(521, 250), "as molduras");
        assert!(has(150, 80), "o rótulo");
        assert!(!has(50, 250), "longe, nada");
        unsafe {
            let _ = DeleteObject(HGDIOBJ(rgn.0));
        }
        // O primeiro clique (a área é um ponto): o ponto e a moldura em volta.
        let p = RECT { left: 10, top: 10, right: 10, bottom: 10 };
        let rgn = selection_change(None, Some(p), 96);
        assert!(unsafe { PtInRegion(rgn, 10, 10).as_bool() && PtInRegion(rgn, 8, 8).as_bool() });
        unsafe {
            let _ = DeleteObject(HGDIOBJ(rgn.0));
        }
    }
}
