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
use windows::Win32::UI::Input::KeyboardAndMouse::{GetKeyState, ReleaseCapture, SetCapture, SetFocus, VK_CONTROL, VK_ESCAPE, VK_RETURN, VK_SHIFT};
use windows::Win32::UI::WindowsAndMessaging::*;

use crate::annotate::{fill_round, Arrow, Frame, Icon, Mark, Stroke, Text, PALETTE, SIZE_MAX, SIZE_MIN, SIZE_STEP, WIDTH_MAX, WIDTH_MIN};
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
    /// O buffer do `WM_PAINT` ([`paint`]): o quadro é montado nele e vai à tela de uma vez.
    /// Cresce com a maior região pintada e fica até a seleção fechar.
    back: Option<(MemImage, i32, i32)>,
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
    /// O traço livre em andamento (lápis ou marca-texto).
    stroke: Option<Stroke>,
    /// Mexendo na área já escolhida: o que (mover ou uma alça), onde o mouse apertou e a
    /// área nesse instante.
    adjust: Option<(Grab, POINT, RECT)>,
    /// A ferramenta e a cor da próxima marca. Sem ferramenta, arrastar dentro move a área.
    tool: Option<Tool>,
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
    /// A barra montada ([`layout_bar`]): o desenho e o clique usam a mesma.
    bar: Option<Bar>,
    /// O seletor de cores aberto, e o arrasto na faixa de matizes dele.
    picker: bool,
    hue_drag: bool,
    /// O botão sob o mouse (realçado, com a dica) e a dica montada (onde e o texto).
    hover: Option<ShotButton>,
    tip: Option<(RECT, Vec<u16>)>,
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

/// A ferramenta do editor: o lápis e o marca-texto seguem o mouse; a linha, a seta e o
/// retângulo vão do ponto inicial ao final; clicar com o texto começa um texto.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Tool {
    Pen,
    Line,
    #[default]
    Arrow,
    /// Arrastar demarca uma região com um retângulo (só a borda).
    Rect,
    Marker,
    Text,
}

/// As ferramentas, na ordem da barra.
const TOOLS: [Tool; 6] = [Tool::Pen, Tool::Line, Tool::Arrow, Tool::Rect, Tool::Marker, Tool::Text];

impl Tool {
    /// Segue o mouse (traço livre).
    fn freehand(self) -> bool {
        matches!(self, Tool::Pen | Tool::Marker)
    }

    fn icon(self) -> Icon {
        match self {
            Tool::Pen => Icon::Pen,
            Tool::Line => Icon::Line,
            Tool::Arrow => Icon::Arrow,
            Tool::Rect => Icon::Rect,
            Tool::Marker => Icon::Marker,
            Tool::Text => Icon::Text,
        }
    }
}

/// O arrasto `a` como marca da ferramenta `tool`.
fn drag_mark(a: Arrow, tool: Option<Tool>) -> Mark {
    match tool {
        Some(Tool::Rect) => Mark::Rect(Frame::from_drag(&a)),
        Some(Tool::Line) => Mark::Line(a),
        _ => Mark::Arrow(a),
    }
}

/// Mexendo na área escolhida: movê-la inteira ou puxar uma alça (os lados que andam).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Grab {
    Move,
    Edges { left: bool, top: bool, right: bool, bottom: bool },
}

/// A área `r` depois de arrastar `dx`, `dy` pegando em `g`: movida, presa dentro do
/// cliente; ou com os lados puxados, sem passar do lado oposto (fica com [`MIN_SIZE`]).
fn adjusted(r: RECT, g: Grab, dx: i32, dy: i32, client: RECT) -> RECT {
    match g {
        Grab::Move => {
            let (w, h) = (r.right - r.left, r.bottom - r.top);
            let l = (r.left + dx).clamp(client.left, (client.right - w).max(client.left));
            let t = (r.top + dy).clamp(client.top, (client.bottom - h).max(client.top));
            RECT { left: l, top: t, right: l + w, bottom: t + h }
        }
        Grab::Edges { left, top, right, bottom } => {
            let mut n = r;
            if left {
                n.left = (r.left + dx).min(r.right - MIN_SIZE).max(client.left);
            }
            if right {
                n.right = (r.right + dx).max(r.left + MIN_SIZE).min(client.right);
            }
            if top {
                n.top = (r.top + dy).min(r.bottom - MIN_SIZE).max(client.top);
            }
            if bottom {
                n.bottom = (r.bottom + dy).max(r.top + MIN_SIZE).min(client.bottom);
            }
            n
        }
    }
}

/// As 8 alças da área (cantos e meios dos lados): o centro e os lados que cada uma puxa.
fn handles(r: RECT) -> impl Iterator<Item = (POINT, Grab)> {
    let xs = [(r.left, true, false), ((r.left + r.right) / 2, false, false), (r.right, false, true)];
    let ys = [(r.top, true, false), ((r.top + r.bottom) / 2, false, false), (r.bottom, false, true)];
    ys.into_iter().flat_map(move |(y, top, bottom)| {
        xs.into_iter().filter_map(move |(x, left, right)| {
            (left || right || top || bottom).then_some((POINT { x, y }, Grab::Edges { left, top, right, bottom }))
        })
    })
}

/// O lado de uma alça desenhada.
fn handle_px(dpi: i32) -> i32 {
    sx(dpi, 8)
}

/// A alça sob `p` (com folga em volta, para pegar fácil).
fn handle_at(r: RECT, p: POINT, dpi: i32) -> Option<Grab> {
    let reach = sx(dpi, 7);
    handles(r).find(|(c, _)| (p.x - c.x).abs() <= reach && (p.y - c.y).abs() <= reach).map(|(_, g)| g)
}

/// O cursor de quem puxa a alça `g`.
fn grab_cursor(g: Grab) -> PCWSTR {
    match g {
        Grab::Move => IDC_SIZEALL,
        Grab::Edges { left, top, right, bottom } => match (left || right, top || bottom) {
            (true, true) if (left && top) || (right && bottom) => IDC_SIZENWSE,
            (true, true) => IDC_SIZENESW,
            (true, false) => IDC_SIZEWE,
            _ => IDC_SIZENS,
        },
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ShotButton {
    Tool(Tool),
    /// Desfaz a última marca (como o Ctrl+Z).
    Undo,
    /// A cor atual: abre (ou fecha) o seletor de cores.
    ColorMenu,
    /// No seletor: uma cor de [`PALETTE`] (o índice)...
    Color(usize),
    /// ...ou a faixa de matizes (a cor sai do ponto clicado ou arrastado).
    Hue,
    /// − e + do tamanho (a espessura ou a fonte), e o número entre eles (só a dica).
    Smaller,
    Size,
    Bigger,
    Copy,
    Save,
    Cancel,
}

/// A barra do editor montada: onde ela fica, os botões (com os do seletor de cores, se
/// aberto), os separadores entre os grupos (x) e o quadro do seletor.
#[derive(Debug, Clone, Default)]
struct Bar {
    rect: RECT,
    buttons: Vec<(ShotButton, RECT)>,
    seps: Vec<i32>,
    picker: Option<RECT>,
}

impl Bar {
    fn button(&self, b: ShotButton) -> Option<RECT> {
        self.buttons.iter().find(|(k, _)| *k == b).map(|(_, r)| *r)
    }

    /// O que ela ocupa na tela (a barra e o seletor).
    fn areas(&self) -> impl Iterator<Item = RECT> {
        [Some(self.rect), self.picker].into_iter().flatten()
    }
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
/// mouse não termina — a área fica, com a barra (Seta/Retângulo/Texto, a cor, o tamanho,
/// Cancelar, Copiar e Salvar). Com a Seta, arrastar DENTRO da área desenha uma seta (do
/// ponto inicial ao final, a ponta no final); com o Retângulo, a borda de um retângulo de
/// canto a canto (a espessura é a mesma da seta); com o Texto, clicar começa um texto ali
/// (ou edita o que estiver sob o clique). Arrastar FORA escolhe outra área. A cor abre um
/// seletor (as cores prontas e a faixa de matizes). Roda do mouse (ou − e +): a espessura
/// ou o tamanho da fonte; Ctrl+Z desfaz a última marca; Ctrl+C copia; Enter salva
/// (digitando: termina o texto; Shift+Enter, nova linha); Esc cancela.
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
            reset_editor(s);
            s.prev_sel = None;
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
    s.stroke = None;
    s.adjust = None;
    s.typing = None;
    s.high_surrogate = None;
    s.bar = None;
    s.picker = false;
    s.hue_drag = false;
    s.hover = None;
    s.tip = None;
    if s.width == 0.0 {
        s.width = crate::annotate::WIDTH_DEFAULT;
        s.font_size = crate::annotate::SIZE_DEFAULT;
        s.color = crate::annotate::COLOR;
        s.tool = Some(Tool::Arrow);
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
                s.small_font = CreateFontW(-sx(dpi, 13), 0, 0, 0, FW_SEMIBOLD.0 as i32, 0, 0, 0, DEFAULT_CHARSET, OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS, CLEARTYPE_QUALITY, DEFAULT_PITCH.0 as u32, w!("Segoe UI"));
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
    log!("AreaSelect: print {:?} {}x{} em ({}, {}), {} desenho(s), {} texto(s).", action, area.w, area.h, area.x, area.y, marks.len() - texts, texts);
    if let Some(cb) = on_shot {
        mainthread::queue(move || cb(Some(ShotDone { action, area, marks })));
    }
}

/// Onde a marca desenha, com folga (o contorno, o cursor do texto).
fn mark_area(m: &Mark) -> RECT {
    inflate(m.bounds(), 4)
}

/// Onde o texto sendo digitado desenha agora.
fn typing_area(s: &State) -> Option<RECT> {
    s.typing.and_then(|i| s.marks.get(i)).map(mark_area)
}

/// Repinta `r` (cliente) na janela aberta.
fn dirty(r: RECT) {
    if let Some(h) = with(|s| s.hwnd) {
        invalidate(h, &r);
    }
}

/// Mexe no estado com `f` e repinta o texto sendo digitado: onde ele estava e onde ficou.
fn change_typing<R>(f: impl FnOnce(&mut State) -> R) -> R {
    let (before, out, after) = with(|s| {
        let before = typing_area(s);
        let out = f(s);
        (before, out, typing_area(s))
    });
    before.into_iter().chain(after).for_each(dirty);
    out
}

/// Termina o texto sendo digitado; vazio, ele sai.
fn end_typing() {
    change_typing(|s| {
        s.high_surrogate = None;
        if let Some(i) = s.typing.take() {
            if matches!(s.marks.get(i), Some(Mark::Text(t)) if t.text.trim().is_empty()) {
                s.marks.remove(i);
            }
        }
    });
}

/// Fecha a janela e solta o que ela segura (fontes, as imagens congeladas e o buffer —
/// dezenas de MB); devolve quem esperava o resultado.
fn close() -> (Option<OnSelected>, Option<OnShot>) {
    let (hwnd, cb, on_shot, fonts, frozen, back) = with(|s| {
        s.editing = false;
        s.drawing = None;
        s.stroke = None;
        s.adjust = None;
        s.typing = None;
        s.marks.clear();
        s.bar = None;
        s.picker = false;
        s.hue_drag = false;
        s.hover = None;
        s.tip = None;
        (s.hwnd.take(), s.on_selected.take(), s.on_shot.take(), [std::mem::take(&mut s.font), std::mem::take(&mut s.small_font)], s.frozen.take(), s.back.take())
    });
    drop((frozen, back));
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

fn offset(r: RECT, dx: i32, dy: i32) -> RECT {
    RECT { left: r.left + dx, top: r.top + dy, right: r.right + dx, bottom: r.bottom + dy }
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
    // A moldura vai da borda (por fora) até o meio das alças (por dentro e por fora).
    let half = handle_px(dpi) / 2 + 1;
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

fn client_rect(hwnd: HWND) -> RECT {
    let mut r = RECT::default();
    unsafe {
        let _ = GetClientRect(hwnd, &mut r);
    }
    r
}

/// O tamanho que a roda (e o − e o +) muda agora: o da fonte (digitando ou com o Texto)?
fn sizing_text(s: &State) -> bool {
    s.typing.is_some() || s.tool == Some(Tool::Text)
}

/// O número entre o − e o +.
fn size_label(s: &State) -> String {
    let size = match s.typing.and_then(|i| s.marks.get(i)) {
        Some(Mark::Text(t)) => t.size,
        _ if s.tool == Some(Tool::Text) => s.font_size,
        _ => s.width,
    };
    format!("{size:.0} px")
}

/// Monta a barra do editor (com a área escolhida), colada embaixo da área (em cima, ou
/// dentro, sem espaço), alinhada à direita dela, em grupos:
/// [Lápis Linha Seta Retângulo Marca-texto Texto] | [cor ▾  − 6 px +] | [Desfazer] |
/// [Copiar Salvar Fechar]. Só ícones (o nome e o atalho vão na dica). Com o seletor de
/// cores aberto, ele vai embaixo da barra (ou em cima).
fn layout_bar(s: &State, client: RECT) -> Option<Bar> {
    let sel = s.sel.filter(|_| s.editing)?;
    let d = s.dpi;
    let (h, gap, sep, margin) = (sx(d, 32), sx(d, 4), sx(d, 13), sx(d, 10));
    enum It {
        B(ShotButton, i32),
        Sep,
    }
    let mut items: Vec<It> = TOOLS.iter().map(|t| It::B(ShotButton::Tool(*t), h)).collect();
    items.extend([
        It::Sep,
        It::B(ShotButton::ColorMenu, h + sx(d, 12)),
        It::B(ShotButton::Smaller, sx(d, 24)),
        // Largura fixa (a do maior número): a barra não muda de tamanho com a roda.
        It::B(ShotButton::Size, text_extent(s.small_font, &wide("000 px")).cx + sx(d, 4)),
        It::B(ShotButton::Bigger, sx(d, 24)),
        It::Sep,
        It::B(ShotButton::Undo, h),
        It::Sep,
        It::B(ShotButton::Copy, h),
        It::B(ShotButton::Save, h),
        It::B(ShotButton::Cancel, h),
    ]);
    let (mut x, mut rel, mut seps) = (gap, Vec::new(), Vec::new());
    for it in items {
        match it {
            It::B(b, w) => {
                rel.push((b, x, w));
                x += w + gap;
            }
            It::Sep => {
                seps.push(x + (sep - gap) / 2);
                x += sep;
            }
        }
    }
    let (bar_w, bar_h) = (x, h + 2 * gap);
    let bx = (sel.right - bar_w).clamp(0, (client.right - bar_w).max(0));
    let mut by = sel.bottom + margin;
    if by + bar_h > client.bottom {
        by = sel.top - margin - bar_h;
    }
    if by < 0 {
        by = sel.bottom - bar_h - margin;
    }
    let rect = RECT { left: bx, top: by, right: bx + bar_w, bottom: by + bar_h };
    let mut buttons: Vec<(ShotButton, RECT)> = rel.into_iter().map(|(b, x, w)| (b, RECT { left: bx + x, top: by + gap, right: bx + x + w, bottom: by + gap + h })).collect();
    let seps = seps.into_iter().map(|x| bx + x).collect();
    let mut picker = None;
    if s.picker {
        // Duas linhas de seis cores e a faixa de matizes embaixo.
        let (sw, g, p, hue_h, cols) = (sx(d, 22), sx(d, 6), sx(d, 8), sx(d, 18), 6);
        let pw = 2 * p + cols * sw + (cols - 1) * g;
        let rows = PALETTE.len().div_ceil(cols as usize) as i32;
        let ph = 2 * p + rows * (sw + g) + hue_h;
        let anchor = buttons.iter().find(|(b, _)| *b == ShotButton::ColorMenu).map(|(_, r)| *r).unwrap_or(rect);
        let px = anchor.left.clamp(0, (client.right - pw).max(0));
        let py = if rect.bottom + g + ph <= client.bottom { rect.bottom + g } else { rect.top - g - ph };
        for i in 0..PALETTE.len() {
            let (row, col) = (i as i32 / cols, i as i32 % cols);
            let (l, t) = (px + p + col * (sw + g), py + p + row * (sw + g));
            buttons.push((ShotButton::Color(i), RECT { left: l, top: t, right: l + sw, bottom: t + sw }));
        }
        let ht = py + p + rows * (sw + g);
        buttons.push((ShotButton::Hue, RECT { left: px + p, top: ht, right: px + pw - p, bottom: ht + hue_h }));
        picker = Some(RECT { left: px, top: py, right: px + pw, bottom: py + ph });
    }
    Some(Bar { rect, buttons, seps, picker })
}

/// O nome (e o atalho) do botão, na dica.
fn tip_key(s: &State, b: ShotButton) -> Option<&'static str> {
    Some(match b {
        // A ativa: clicar de novo a solta (e arrastar dentro passa a mover a área).
        ShotButton::Tool(t) if s.tool == Some(t) => "Clique de novo para mover a área",
        ShotButton::Tool(Tool::Pen) => "Lápis",
        ShotButton::Tool(Tool::Line) => "Linha",
        ShotButton::Tool(Tool::Arrow) => "Seta",
        ShotButton::Tool(Tool::Rect) => "Retângulo",
        ShotButton::Tool(Tool::Marker) => "Marca-texto",
        ShotButton::Tool(Tool::Text) => "Texto",
        ShotButton::Undo => "Desfazer (Ctrl+Z)",
        ShotButton::ColorMenu => "Cor",
        ShotButton::Smaller | ShotButton::Size | ShotButton::Bigger => {
            if sizing_text(s) {
                "Tamanho da fonte (roda do mouse)"
            } else {
                "Espessura (roda do mouse)"
            }
        }
        ShotButton::Cancel => "Cancelar (Esc)",
        ShotButton::Copy => "Copiar (Ctrl+C)",
        ShotButton::Save => "Salvar (Enter)",
        ShotButton::Color(_) | ShotButton::Hue => return None,
    })
}

/// A dica do botão sob o mouse: centrada nele, em cima da barra (ou embaixo, sem espaço).
/// Com o seletor aberto, nenhuma (ele fica no caminho).
fn layout_tip(s: &State, client: RECT) -> Option<(RECT, Vec<u16>)> {
    let b = s.hover.filter(|_| !s.picker)?;
    let bar = s.bar.as_ref()?;
    let br = bar.button(b)?;
    let text = wide(&lang::tr(tip_key(s, b)?));
    let ext = text_extent(s.small_font, &text);
    let (px, py, gap) = (sx(s.dpi, 8), sx(s.dpi, 4), sx(s.dpi, 6));
    let (w, h) = (ext.cx + 2 * px, ext.cy + 2 * py);
    let x = ((br.left + br.right) / 2 - w / 2).clamp(0, (client.right - w).max(0));
    let y = if bar.rect.top - gap - h >= 0 { bar.rect.top - gap - h } else { bar.rect.bottom + gap };
    Some((RECT { left: x, top: y, right: x + w, bottom: y + h }, text))
}

/// Remonta a barra, o seletor e a dica pelo estado atual e repinta onde estavam e onde
/// ficaram — chamada depois de qualquer mudança que apareça neles.
fn refresh_bar(hwnd: HWND) {
    let client = client_rect(hwnd);
    let (old, new) = with(|s| {
        let areas = |s: &State| s.bar.iter().flat_map(|b| b.areas()).chain(s.tip.as_ref().map(|t| t.0)).collect::<Vec<_>>();
        let old = areas(s);
        s.bar = layout_bar(s, client);
        if s.bar.is_none() {
            s.picker = false;
            s.hover = None;
        }
        s.tip = layout_tip(s, client);
        (old, areas(s))
    });
    for r in old.iter().chain(&new) {
        invalidate(hwnd, r);
    }
}

/// O botão sob o mouse mudou: o realce e a dica acompanham.
fn set_hover(hwnd: HWND, b: Option<ShotButton>) {
    if with(|s| std::mem::replace(&mut s.hover, b)) != b {
        refresh_bar(hwnd);
    }
}

/// ARGB → o `COLORREF` do GDI (0x00BBGGRR).
fn bgr(argb: u32) -> u32 {
    ((argb & 0xFF) << 16) | (argb & 0xFF00) | ((argb >> 16) & 0xFF)
}

unsafe fn fill(dc: HDC, r: &RECT, bgr: u32) {
    let brush = CreateSolidBrush(COLORREF(bgr));
    FillRect(dc, r, brush);
    let _ = DeleteObject(HGDIOBJ(brush.0));
}

unsafe fn frame(dc: HDC, r: &RECT, bgr: u32) {
    let brush = CreateSolidBrush(COLORREF(bgr));
    FrameRect(dc, r, brush);
    let _ = DeleteObject(HGDIOBJ(brush.0));
}

/// A cor da faixa de matizes no x `x` (cliente).
fn hue_at(r: RECT, x: i32) -> u32 {
    let t = (x - r.left) as f32 / (r.right - r.left - 1).max(1) as f32;
    crate::annotate::hsv(t.clamp(0.0, 1.0) * 360.0, 0.85, 0.95)
}

/// As cores da barra (ARGB): o fundo, a borda, o realce sob o mouse, o separador e o
/// verde do NoOBS (a ferramenta ativa e o Salvar).
const BAR_BG: u32 = 0xFF1C_1D1F;
const BAR_BORDER: u32 = 0xFF3A_3B3E;
const HOVER_BG: u32 = 0xFF34_3539;
const SEP: u32 = 0x0045_4649;
const GREEN: u32 = 0xFF10_B981;
const GREEN_HOVER: u32 = 0xFF34_D399;
const GREEN_SOFT: u32 = 0xFF0F_3B2F;
const WHITE: u32 = 0xFFFF_FFFF;
const ICON: u32 = 0xFFE8_E8EA;

/// A barra (e o seletor e a dica) em `dc`, na posição montada por [`layout_bar`].
/// `org`: o canto do cliente que o (0, 0) de `dc` representa (para o GDI+, que não segue
/// a origem do GDI). Primeiro os fundos arredondados (GDI+), depois o texto e as cores
/// (GDI), por último os ícones (GDI+, com antialiasing).
unsafe fn paint_bar(dc: HDC, org: (i32, i32)) {
    let Some(bar) = with(|s| s.bar.clone()) else { return };
    let (dpi, font, tool, color, hover, picker, tip, size) = with(|s| (s.dpi, s.small_font, s.tool, s.color, s.hover, s.picker, s.tip.clone(), size_label(s)));
    let (radius, inner) = (sx(dpi, 10) as f32, sx(dpi, 7) as f32);
    let at = |r: RECT| offset(r, -org.0, -org.1);
    let mut icons: Vec<(Icon, RECT, u32)> = Vec::new();
    gdiplus(dc, org, || {
        fill_round(dc, at(bar.rect), radius, BAR_BG, Some(BAR_BORDER));
        if let Some(pr) = bar.picker {
            fill_round(dc, at(pr), radius, BAR_BG, Some(BAR_BORDER));
        }
        for (b, r) in &bar.buttons {
            let hov = hover == Some(*b);
            let bg = match *b {
                ShotButton::Tool(t) if tool == Some(t) => Some(GREEN_SOFT),
                ShotButton::Save => Some(if hov { GREEN_HOVER } else { GREEN }),
                ShotButton::ColorMenu if picker => Some(HOVER_BG),
                ShotButton::Color(_) | ShotButton::Hue | ShotButton::Size => None,
                _ if hov => Some(HOVER_BG),
                _ => None,
            };
            if let Some(bg) = bg {
                fill_round(dc, at(*r), inner, bg, None);
            }
            let icon = match *b {
                ShotButton::Tool(t) => Some((t.icon(), if tool == Some(t) { GREEN_HOVER } else { ICON })),
                ShotButton::Undo => Some((Icon::Undo, ICON)),
                ShotButton::Copy => Some((Icon::Copy, ICON)),
                ShotButton::Save => Some((Icon::Save, WHITE)),
                ShotButton::Cancel => Some((Icon::Close, ICON)),
                ShotButton::Smaller => Some((Icon::Minus, ICON)),
                ShotButton::Bigger => Some((Icon::Plus, ICON)),
                _ => None,
            };
            if let Some((icon, c)) = icon {
                // Os ícones um pouco menores que o botão (mais respiro).
                icons.push((icon, inflate(*r, -sx(dpi, 3)), c));
            }
        }
        if let Some((tr, _)) = &tip {
            fill_round(dc, at(*tr), inner, 0xF012_1214, Some(BAR_BORDER));
        }
    });
    let inset = sx(dpi, 9);
    for x in &bar.seps {
        fill(dc, &RECT { left: *x, top: bar.rect.top + inset, right: *x + sx(dpi, 1).max(1), bottom: bar.rect.bottom - inset }, SEP);
    }
    let old_font = (!font.is_invalid()).then(|| SelectObject(dc, HGDIOBJ(font.0)));
    SetBkMode(dc, TRANSPARENT);
    for (b, r) in &bar.buttons {
        match *b {
            ShotButton::ColorMenu => {
                // A cor atual numa amostra, e o ▾ do seletor.
                let m = sx(dpi, 8);
                let side = (r.bottom - r.top) - 2 * m;
                let sw = RECT { left: r.left + m, top: r.top + m, right: r.left + m + side, bottom: r.top + m + side };
                fill(dc, &inflate(sw, 1), 0x00A0_A0A0);
                fill(dc, &sw, bgr(color));
                icons.push((Icon::Caret, RECT { left: sw.right, ..*r }, 0xFFDD_DDDD));
            }
            ShotButton::Size => {
                SetTextColor(dc, COLORREF(0x00DD_DDDD));
                let (mut t, mut tr) = (wide(&size), *r);
                DrawTextW(dc, &mut t, &mut tr, DT_CENTER | DT_VCENTER | DT_SINGLELINE);
            }
            ShotButton::Color(i) => {
                // A escolhida com moldura branca; as outras, um fio cinza.
                let on = PALETTE[i] == color;
                let hov = hover == Some(*b);
                fill(dc, &inflate(*r, if on || hov { sx(dpi, 2) } else { 1 }), if on { 0x00FF_FFFF } else if hov { 0x00A0_A0A0 } else { 0x0060_6060 });
                fill(dc, r, bgr(PALETTE[i]));
            }
            ShotButton::Hue => {
                for x in r.left..r.right {
                    fill(dc, &RECT { left: x, right: x + 1, ..*r }, bgr(hue_at(*r, x)));
                }
                frame(dc, &inflate(*r, 1), 0x0060_6060);
            }
            _ => {}
        }
    }
    if let Some((tr, t)) = tip {
        SetTextColor(dc, COLORREF(0x00FF_FFFF));
        let (mut t, mut r) = (t, tr);
        DrawTextW(dc, &mut t, &mut r, DT_CENTER | DT_VCENTER | DT_SINGLELINE);
    }
    if let Some(o) = old_font {
        SelectObject(dc, o);
    }
    gdiplus(dc, org, || {
        for (icon, r, c) in icons {
            crate::annotate::draw_icon(dc, icon, at(r), c);
        }
    });
}

/// Roda `f` (desenho do GDI+) com a origem do `dc` no (0, 0) do buffer: quem desenha
/// desconta `org` à mão. Depois, a origem do cliente volta.
unsafe fn gdiplus(dc: HDC, org: (i32, i32), f: impl FnOnce()) {
    let _ = SetViewportOrgEx(dc, 0, 0, None);
    f();
    let _ = SetViewportOrgEx(dc, -org.0, -org.1, None);
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
/// cliente: o escuro, a área nítida com as marcas, a borda, o rótulo do tamanho e a
/// barra do editor (ou a instrução, antes do 1º clique).
unsafe fn compose(dc: HDC, org: (i32, i32), rc: RECT) {
    let _ = SetViewportOrgEx(dc, -org.0, -org.1, None);
    let (sel, dpi, font, hint_rect, hint_visible) = with(|s| (s.sel, s.dpi, s.font, s.hint_rect, s.hint_visible));
    // Congelada: (escura, nítida) — os DCs das imagens, desenhados no lugar do preto e do furo.
    let frozen = with(|s| s.frozen.as_ref().map(|f| (f.dim.dc, f.bright.dc)));
    let border = border_px(dpi);

    // 1. Escuro, fora do furo + borda.
    let saved = SaveDC(dc);
    if let Some(r) = sel {
        let o = inflate(r, border);
        ExcludeClipRect(dc, o.left, o.top, o.right, o.bottom);
    }
    match frozen {
        Some((dim, _)) => {
            let _ = BitBlt(dc, rc.left, rc.top, rc.right - rc.left, rc.bottom - rc.top, Some(dim), rc.left, rc.top, SRCCOPY);
        }
        None => fill(dc, &rc, 0),
    }
    let _ = RestoreDC(dc, saved);

    if let Some(r) = sel {
        // 2. Furo (cor-chave = transparente; congelada: a imagem nítida), só onde a área
        // a pintar o cruza, e a borda branca por FORA dele.
        let mut part = RECT::default();
        if IntersectRect(&mut part, &r, &rc).as_bool() {
            match frozen {
                Some((_, bright)) => {
                    let _ = BitBlt(dc, part.left, part.top, part.right - part.left, part.bottom - part.top, Some(bright), part.left, part.top, SRCCOPY);
                }
                None => fill(dc, &part, KEY.0),
            }
            // As marcas do editor do print, só dentro da área (o que vai para o print),
            // com o cursor no texto sendo digitado.
            let (marks, caret) = with(|s| {
                let live = s.drawing.map(|a| drag_mark(a, s.tool)).into_iter().chain(s.stroke.clone().map(Mark::Stroke));
                (s.marks.iter().cloned().chain(live).collect::<Vec<_>>(), s.typing)
            });
            if !marks.is_empty() {
                gdiplus(dc, org, || crate::annotate::draw_on_hdc(dc, &marks, r, caret, org));
            }
        }
        dashed_border(dc, r, border, sx(dpi, 4).max(2), rc);
        // As alças do editor (a área já escolhida): puxam os lados.
        if with(|s| s.editing && s.on_shot.is_some()) {
            let hp = handle_px(dpi);
            for (c, _) in handles(r) {
                let h = RECT { left: c.x - hp / 2, top: c.y - hp / 2, right: c.x - hp / 2 + hp, bottom: c.y - hp / 2 + hp };
                fill(dc, &h, 0x0020_2020);
                fill(dc, &inflate(h, -1), 0x00FF_FFFF);
            }
        }
        // 3. Tamanho, em px físicos — é o tamanho do vídeo (arredondado
        // pro múltiplo de 16 na gravação).
        let lr = label_rect(r, dpi);
        gdiplus(dc, org, || fill_round(dc, offset(lr, -org.0, -org.1), sx(dpi, 5) as f32, 0xD914_1517, Some(0xFF2E_2F33)));
        let old_font = SelectObject(dc, HGDIOBJ(label_font(dpi).0));
        SetBkMode(dc, TRANSPARENT);
        SetTextColor(dc, COLORREF(0x00E6_E6E6));
        let mut text = label_text(r);
        let mut lr2 = lr;
        DrawTextW(dc, &mut text, &mut lr2, DT_CENTER | DT_VCENTER | DT_SINGLELINE);
        SelectObject(dc, old_font);
        // 4. A barra do editor do print (a área já escolhida).
        paint_bar(dc, org);
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

fn in_rect(r: &RECT, p: POINT) -> bool {
    p.x >= r.left && p.x < r.right && p.y >= r.top && p.y < r.bottom
}

/// O botão da barra (ou do seletor aberto) sob `p`.
fn button_at(p: POINT) -> Option<ShotButton> {
    with(|s| s.bar.as_ref().and_then(|b| b.buttons.iter().find(|(_, r)| in_rect(r, p)).map(|(b, _)| *b)))
}

/// `p` está na barra ou no seletor (num botão ou no fundo)?
fn on_bar(p: POINT) -> bool {
    with(|s| s.bar.as_ref().is_some_and(|b| b.areas().any(|r| in_rect(&r, p))))
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

/// A cor das próximas marcas — e a do texto sendo digitado, que continua.
fn set_color(hwnd: HWND, c: u32) {
    change_typing(|s| {
        s.color = c;
        if let Some(Mark::Text(t)) = s.typing.and_then(|i| s.marks.get_mut(i)) {
            t.color = c;
        }
    });
    refresh_bar(hwnd);
}

fn run_button(hwnd: HWND, b: ShotButton, p: POINT) {
    match b {
        ShotButton::Cancel => finish(None),
        ShotButton::Copy => finish_shot(ShotAction::Copy),
        ShotButton::Save => finish_shot(ShotAction::Save),
        ShotButton::Tool(t) => {
            // A ativa de novo: solta (sem ferramenta, arrastar dentro move a área).
            end_typing();
            with(|s| s.tool = if s.tool == Some(t) { None } else { Some(t) });
            refresh_bar(hwnd);
        }
        ShotButton::Undo => undo(hwnd),
        ShotButton::ColorMenu => {
            with(|s| s.picker = !s.picker);
            refresh_bar(hwnd);
        }
        ShotButton::Color(i) => {
            with(|s| s.picker = false);
            set_color(hwnd, PALETTE[i]);
        }
        ShotButton::Hue => {
            // Arrastar na faixa escolhe a matiz; soltar fecha o seletor.
            let Some(r) = with(|s| s.bar.as_ref().and_then(|b| b.button(ShotButton::Hue))) else { return };
            with(|s| s.hue_drag = true);
            set_color(hwnd, hue_at(r, p.x));
            unsafe {
                SetCapture(hwnd);
            }
        }
        ShotButton::Smaller => wheel_steps(hwnd, -1),
        ShotButton::Bigger => wheel_steps(hwnd, 1),
        ShotButton::Size => {}
    }
}

/// Desfaz a última marca (o botão e o Ctrl+Z; digitando, o texto em edição termina antes).
fn undo(hwnd: HWND) {
    end_typing();
    if let Some(m) = with(|s| s.marks.pop()) {
        invalidate(hwnd, &mark_area(&m));
    }
    refresh_bar(hwnd);
}

/// A roda do mouse (e o − e o +): no texto sendo digitado (ou com a ferramenta Texto), o
/// tamanho da fonte, 2 px por dente; com a Seta e o Retângulo, a espessura, 1 px por dente
/// (a marca sendo arrastada acompanha).
fn wheel_steps(hwnd: HWND, steps: i32) {
    let text_size = change_typing(|s| {
        if let Some(Mark::Text(t)) = s.typing.and_then(|i| s.marks.get_mut(i)) {
            t.size = (t.size + steps as f32 * SIZE_STEP).clamp(SIZE_MIN, SIZE_MAX);
            s.font_size = t.size;
            true
        } else if s.tool == Some(Tool::Text) && s.drawing.is_none() && s.stroke.is_none() {
            s.font_size = (s.font_size + steps as f32 * SIZE_STEP).clamp(SIZE_MIN, SIZE_MAX);
            true
        } else {
            false
        }
    });
    if !text_size {
        // A marca sendo feita acompanha (a linha, a seta, o retângulo ou o traço livre).
        let live = |s: &State| {
            let a = s.drawing.map(|a| inflate(drag_mark(a, s.tool).bounds(), 4));
            let k = s.stroke.as_ref().map(|k| k.bounds());
            [a, k]
        };
        let (old, new) = with(|s| {
            s.width = (s.width + steps as f32).clamp(WIDTH_MIN, WIDTH_MAX);
            let old = live(s);
            let w = s.width;
            if let Some(d) = s.drawing.as_mut() {
                d.width = w;
            }
            if let Some(k) = s.stroke.as_mut() {
                k.width = w;
            }
            (old, live(s))
        });
        for r in old.into_iter().chain(new).flatten() {
            invalidate(hwnd, &r);
        }
    }
    refresh_bar(hwnd);
}

/// Clique com a ferramenta Texto dentro da área: edita o texto sob o clique (o mais de
/// cima) ou começa um novo ali, com a linha centrada no clique.
fn start_text(hwnd: HWND, p: POINT) {
    end_typing();
    let hit = with(|s| s.marks.iter().rposition(|m| matches!(m, Mark::Text(_)) && in_rect(&m.bounds(), p)));
    change_typing(|s| match hit {
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
    refresh_bar(hwnd);
}

/// Um caractere digitado (`WM_CHAR`) no texto em edição. `false`: não há texto em edição.
fn type_char(unit: u16) -> bool {
    change_typing(|s| {
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
    })
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
            // No editor, a mãozinha sobre os botões, a seta no fundo da barra e o I do
            // texto dentro da área (com a ferramenta Texto); no resto, a cruz.
            let mut p = POINT::default();
            let known = editor && unsafe { GetCursorPos(&mut p).is_ok() && ScreenToClient(hwnd, &mut p).as_bool() };
            // A área escolhida: a alça sob o mouse, ou dentro dela.
            let (grab, inside, tool) = with(|s| {
                let sel = s.sel.filter(|_| s.editing && known);
                let grab = s.adjust.map(|(g, _, _)| g).or_else(|| sel.and_then(|r| handle_at(r, p, s.dpi)));
                (grab, sel.is_some_and(|r| in_rect(&r, p)), s.tool)
            });
            let cursor = if known && button_at(p).is_some_and(|b| b != ShotButton::Size) {
                IDC_HAND
            } else if known && on_bar(p) {
                IDC_ARROW
            } else if let Some(g) = grab {
                grab_cursor(g)
            } else if inside && tool.is_none() {
                IDC_SIZEALL
            } else if inside && tool == Some(Tool::Text) {
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
            if with(|s| s.anchor.is_some() || s.drawing.is_some() || s.stroke.is_some() || s.adjust.is_some() || s.hue_drag) {
                return LRESULT(0);
            }
            let p = point_from(lparam);
            if editor {
                // Seletor de cores aberto e clique fora dele: fecha. Um botão da barra
                // funciona direto (menos o da cor, que era o que fechava); fora da barra,
                // o clique só fechou o seletor.
                let picker = with(|s| s.bar.as_ref().and_then(|b| b.picker));
                if picker.is_some_and(|r| !in_rect(&r, p)) {
                    with(|s| s.picker = false);
                    refresh_bar(hwnd);
                    if let Some(b) = button_at(p).filter(|b| *b != ShotButton::ColorMenu) {
                        run_button(hwnd, b, p);
                    }
                    return LRESULT(0);
                }
                if let Some(b) = button_at(p) {
                    run_button(hwnd, b, p);
                    return LRESULT(0);
                }
                // No fundo da barra ou do seletor: nada.
                if on_bar(p) {
                    return LRESULT(0);
                }
                let (editing, sel, width, color, tool, dpi) = with(|s| (s.editing, s.sel, s.width, s.color, s.tool, s.dpi));
                // Uma alça (com qualquer ferramenta) puxa os lados; dentro, sem ferramenta,
                // arrastar move a área inteira.
                let grab = sel.filter(|_| editing).and_then(|r| {
                    handle_at(r, p, dpi).or_else(|| (tool.is_none() && in_rect(&r, p)).then_some(Grab::Move)).map(|g| (g, r))
                });
                if let Some((g, r)) = grab {
                    end_typing();
                    with(|s| s.adjust = Some((g, p, r)));
                    unsafe {
                        SetCapture(hwnd);
                    }
                    return LRESULT(0);
                }
                if editing && sel.is_some_and(|r| in_rect(&r, p)) {
                    let (x, y) = (p.x as f32, p.y as f32);
                    match tool {
                        Some(Tool::Text) => {
                            start_text(hwnd, p);
                            return LRESULT(0);
                        }
                        // O lápis e o marca-texto: o traço segue o mouse até soltar.
                        Some(t) if t.freehand() => {
                            end_typing();
                            with(|s| s.stroke = Some(Stroke { points: vec![(x, y)], width, color, marker: t == Tool::Marker }));
                        }
                        // A linha, a seta e o retângulo: do ponto inicial ao final.
                        _ => {
                            end_typing();
                            with(|s| s.drawing = Some(Arrow { x1: x, y1: y, x2: x, y2: y, width, color }));
                        }
                    }
                    unsafe {
                        SetCapture(hwnd);
                    }
                    refresh_bar(hwnd);
                    return LRESULT(0);
                }
                // Fora: outra área (a de antes volta se for só um clique). A barra sai.
                end_typing();
                with(|s| {
                    s.prev_sel = s.sel;
                    s.editing = false;
                });
                refresh_bar(hwnd);
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
            if with(|s| s.hue_drag) {
                if let Some(r) = with(|s| s.bar.as_ref().and_then(|b| b.button(ShotButton::Hue))) {
                    set_color(hwnd, hue_at(r, p.x));
                    paint_now(hwnd);
                }
                return LRESULT(0);
            }
            // Movendo a área ou puxando uma alça: a barra vai junto.
            if let Some((g, from, r0)) = with(|s| s.adjust) {
                let r = adjusted(r0, g, p.x - from.x, p.y - from.y, client_rect(hwnd));
                set_selection(hwnd, Some(r));
                refresh_bar(hwnd);
                paint_now(hwnd);
                return LRESULT(0);
            }
            // O traço livre: um ponto a cada passo do mouse; repinta só o pedaço novo.
            if let Some(seg) = with(|s| {
                let k = s.stroke.as_mut()?;
                let (x, y) = (p.x as f32, p.y as f32);
                let &(lx, ly) = k.points.last()?;
                if (x - lx).abs() < 1.0 && (y - ly).abs() < 1.0 {
                    return None;
                }
                k.points.push((x, y));
                let piece = Stroke { points: vec![(lx, ly), (x, y)], width: k.width, color: k.color, marker: k.marker };
                Some(piece.bounds())
            }) {
                invalidate(hwnd, &seg);
                paint_now(hwnd);
                return LRESULT(0);
            }
            if with(|s| s.stroke.is_some()) {
                return LRESULT(0);
            }
            if let Some(mut a) = with(|s| s.drawing) {
                let tool = with(|s| s.tool);
                let old = inflate(drag_mark(a, tool).bounds(), 4);
                (a.x2, a.y2) = (p.x as f32, p.y as f32);
                with(|s| s.drawing = Some(a));
                invalidate(hwnd, &old);
                invalidate(hwnd, &inflate(drag_mark(a, tool).bounds(), 4));
                paint_now(hwnd);
                return LRESULT(0);
            }
            if let Some(a) = with(|s| s.anchor) {
                set_selection(hwnd, Some(norm(a, p)));
                paint_now(hwnd);
            } else if editor {
                set_hover(hwnd, button_at(p));
            }
            LRESULT(0)
        }
        WM_LBUTTONUP => {
            let p = point_from(lparam);
            if with(|s| std::mem::replace(&mut s.hue_drag, false)) {
                unsafe {
                    let _ = ReleaseCapture();
                }
                with(|s| s.picker = false);
                refresh_bar(hwnd);
                return LRESULT(0);
            }
            if with(|s| s.adjust.take()).is_some() {
                unsafe {
                    let _ = ReleaseCapture();
                }
                refresh_bar(hwnd);
                return LRESULT(0);
            }
            // O traço livre entra como está (um clique sem arrasto vira um pingo).
            if let Some(k) = with(|s| s.stroke.take()) {
                unsafe {
                    let _ = ReleaseCapture();
                }
                let area = k.bounds();
                with(|s| s.marks.push(Mark::Stroke(k)));
                invalidate(hwnd, &area);
                return LRESULT(0);
            }
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
                let area = inflate(mark.bounds(), 4);
                if !tiny {
                    with(|s| s.marks.push(mark));
                }
                invalidate(hwnd, &area);
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
                        // A área do print fica, com a barra; as marcas vêm depois.
                        with(|s| {
                            s.editing = true;
                            s.prev_sel = None;
                        });
                        set_selection(hwnd, Some(r));
                        refresh_bar(hwnd);
                    } else {
                        finish(Some(AreaRect { x: r.left + origin.0, y: r.top + origin.1, w, h }));
                    }
                } else if editor {
                    // Clique sem arrasto fora da área: a de antes volta.
                    let prev = with(|s| s.prev_sel.take());
                    set_selection(hwnd, prev);
                    with(|s| s.editing = prev.is_some());
                    refresh_bar(hwnd);
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
            type_char(wparam.0 as u16);
            LRESULT(0)
        }
        WM_KEYDOWN => {
            let vk = wparam.0 as u16;
            let ctrl = unsafe { GetKeyState(VK_CONTROL.0 as i32) } < 0;
            let shift = unsafe { GetKeyState(VK_SHIFT.0 as i32) } < 0;
            let (editing, typing, picker) = with(|s| (s.editing, s.typing.is_some(), s.picker));
            if editor && picker && vk == VK_ESCAPE.0 {
                // Com o seletor de cores aberto, o Esc só fecha ele.
                with(|s| s.picker = false);
                refresh_bar(hwnd);
            } else if editor && typing && vk == VK_ESCAPE.0 {
                // Digitando, o Esc só termina o texto (o print continua).
                end_typing();
                refresh_bar(hwnd);
            } else if editor && typing && vk == VK_RETURN.0 {
                // Shift+Enter: nova linha; Enter: termina o texto (não salva ainda).
                if shift {
                    change_typing(|s| {
                        if let Some(Mark::Text(t)) = s.typing.and_then(|i| s.marks.get_mut(i)) {
                            t.text.push('\n');
                        }
                    });
                } else {
                    end_typing();
                    refresh_bar(hwnd);
                }
            } else if vk == VK_ESCAPE.0 {
                finish(None);
            } else if editor && editing && vk == VK_RETURN.0 {
                finish_shot(ShotAction::Save);
            } else if editor && editing && ctrl && vk == u16::from(b'C') {
                finish_shot(ShotAction::Copy);
            } else if editor && ctrl && vk == u16::from(b'Z') {
                undo(hwnd);
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

    /// Clica no botão `b` da barra (as cores e a faixa de matizes: abre o seletor antes).
    fn click(hwnd: HWND, b: ShotButton) {
        show_bar(hwnd);
        if matches!(b, ShotButton::Color(_) | ShotButton::Hue) && !with(|s| s.picker) {
            click(hwnd, ShotButton::ColorMenu);
        }
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
        assert_eq!(with(|s| s.tool), Some(Tool::Text));
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
        // Arrastado de baixo-direita para cima-esquerda, com a roda +2 no meio (longe das alças).
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
        assert_eq!(with(|s| (s.tool, s.color, s.font_size, s.marks.len())), (Some(Tool::Text), PALETTE[1], crate::annotate::SIZE_DEFAULT - 4.0, 0));
    }

    /// Mover: a área inteira anda e não sai do cliente. Alça: os lados puxados andam, sem
    /// passar do oposto (fica com o mínimo) nem sair do cliente.
    #[test]
    fn mover_e_puxar_a_area() {
        let client = RECT { left: 0, top: 0, right: 1000, bottom: 800 };
        let r = RECT { left: 100, top: 100, right: 400, bottom: 300 };
        assert_eq!(adjusted(r, Grab::Move, 50, -20, client), RECT { left: 150, top: 80, right: 450, bottom: 280 });
        assert_eq!(adjusted(r, Grab::Move, 900, -500, client), RECT { left: 700, top: 0, right: 1000, bottom: 200 });
        let corner = Grab::Edges { left: false, top: false, right: true, bottom: true };
        assert_eq!(adjusted(r, corner, 30, 40, client), RECT { right: 430, bottom: 340, ..r });
        let left = Grab::Edges { left: true, top: false, right: false, bottom: false };
        assert_eq!(adjusted(r, left, 1000, 0, client), RECT { left: 400 - MIN_SIZE, ..r }, "não passa do lado oposto");
        assert_eq!(adjusted(r, left, -500, 0, client), RECT { left: 0, ..r }, "nem sai do cliente");
        // As 8 alças: os cantos puxam dois lados; os meios, um.
        assert_eq!(handles(r).count(), 8);
        assert_eq!(handle_at(r, POINT { x: 402, y: 298 }, 96), Some(corner));
        assert_eq!(handle_at(r, POINT { x: 250, y: 100 }, 96), Some(Grab::Edges { left: false, top: true, right: false, bottom: false }));
        assert_eq!(handle_at(r, POINT { x: 250, y: 200 }, 96), None, "o meio da área não é alça");
    }

    /// Sem ferramenta (clicar de novo na ativa), arrastar dentro move a área e as marcas
    /// ficam onde estavam na tela; a alça do canto puxa os dois lados, com qualquer
    /// ferramenta.
    #[test]
    fn editor_move_e_redimensiona_a_area() {
        let _t = crate::testkit::begin();
        take_done();
        let hwnd = open_editor();
        drag(hwnd, (100, 100), (500, 400));
        drag(hwnd, (150, 150), (300, 200)); // uma seta
        click(hwnd, ShotButton::Tool(Tool::Arrow)); // a ativa: solta
        assert_eq!(with(|s| s.tool), None);
        drag(hwnd, (300, 300), (350, 250));
        assert_eq!(with(|s| s.sel), Some(RECT { left: 150, top: 50, right: 550, bottom: 350 }));
        assert_eq!(with(|s| s.marks.clone()), vec![arrow(150.0, 150.0, 300.0, 200.0, crate::annotate::WIDTH_DEFAULT)], "a seta fica no lugar");
        assert!(with(|s| s.editing), "a área continua escolhida");

        // A alça do canto de baixo à direita, com a seta escolhida de novo.
        click(hwnd, ShotButton::Tool(Tool::Arrow));
        drag(hwnd, (550, 350), (600, 420));
        assert_eq!(with(|s| s.sel), Some(RECT { left: 150, top: 50, right: 600, bottom: 420 }));
        assert_eq!(with(|s| s.marks.len()), 1, "puxar a alça não desenha");

        click(hwnd, ShotButton::Save);
        let d = take_done().pop().flatten().expect("salvo");
        assert_eq!(d.area, AreaRect { x: 150 - 1920, y: 50, w: 450, h: 370 });
    }

    /// O lápis guarda o caminho do mouse; o marca-texto também, largo e translúcido; a
    /// linha vai de ponta a ponta sem ponta de seta; o Desfazer tira a última marca.
    #[test]
    fn editor_lapis_marca_texto_linha_e_desfazer() {
        let _t = crate::testkit::begin();
        take_done();
        let hwnd = open_editor();
        drag(hwnd, (100, 100), (700, 500));
        click(hwnd, ShotButton::Tool(Tool::Pen));
        send(hwnd, WM_LBUTTONDOWN, 1, lp(200, 200));
        for (x, y) in [(210, 205), (220, 215), (240, 230)] {
            send(hwnd, WM_MOUSEMOVE, 1, lp(x, y));
        }
        send(hwnd, WM_LBUTTONUP, 0, lp(240, 230));
        let pen = |m: &Mark| matches!(m, Mark::Stroke(k) if !k.marker && k.points == vec![(200.0, 200.0), (210.0, 205.0), (220.0, 215.0), (240.0, 230.0)]);
        assert!(with(|s| s.marks.last().is_some_and(pen)), "{:?}", with(|s| s.marks.clone()));

        click(hwnd, ShotButton::Tool(Tool::Marker));
        click(hwnd, ShotButton::Color(1)); // amarelo
        drag(hwnd, (300, 300), (400, 300));
        let marker = with(|s| s.marks.last().cloned());
        let Some(Mark::Stroke(k)) = marker else { panic!("marca-texto: {marker:?}") };
        assert!(k.marker && k.color == PALETTE[1]);
        assert_eq!(k.drawn_width(), k.width * crate::annotate::MARKER_SCALE);
        assert_eq!(k.drawn_color() >> 24, 0x80, "translúcido");

        click(hwnd, ShotButton::Tool(Tool::Line));
        drag(hwnd, (150, 450), (650, 450));
        assert!(matches!(with(|s| s.marks.last().cloned()), Some(Mark::Line(a)) if (a.x1, a.x2) == (150.0, 650.0)));

        click(hwnd, ShotButton::Undo);
        assert_eq!(with(|s| s.marks.len()), 2, "a linha saiu");
        click(hwnd, ShotButton::Save);
        let d = take_done().pop().flatten().expect("salvo");
        assert!(matches!(&d.marks[0], Mark::Stroke(k) if k.points[0] == (200.0 - 1920.0, 200.0)), "as marcas vão em coordenadas de tela");
    }

    /// A barra montada (o que a janela de verdade faz ao escolher a área).
    fn show_bar(hwnd: HWND) {
        assert!(with(|s| s.sel).is_some(), "área");
        refresh_bar(hwnd);
    }

    fn center(b: ShotButton) -> (i32, i32) {
        let r = with(|s| s.bar.as_ref().and_then(|bar| bar.button(b))).expect("botão");
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

        show_bar(hwnd);
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
        show_bar(hwnd);
        let bar = with(|s| s.bar.as_ref().map(|b| b.rect)).expect("barra");
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

    /// A cor: a barra mostra só a atual; clicar nela abre o seletor (as cores prontas e a
    /// faixa de matizes). Escolher fecha; Esc e clicar fora também fecham, sem cancelar o
    /// print nem desenhar; a faixa dá a cor do ponto (o arrasto acompanha).
    #[test]
    fn editor_seletor_de_cores() {
        let _t = crate::testkit::begin();
        take_done();
        let hwnd = open_editor();
        drag(hwnd, (100, 100), (700, 500));
        show_bar(hwnd);
        let colors = |s: &State| s.bar.as_ref().map_or(0, |b| b.buttons.iter().filter(|(k, _)| matches!(k, ShotButton::Color(_))).count());
        assert_eq!(with(|s| colors(s)), 0, "fechado: só a cor atual");
        click(hwnd, ShotButton::ColorMenu);
        assert_eq!(with(|s| (s.picker, colors(s))), (true, PALETTE.len()));
        click(hwnd, ShotButton::Color(7));
        assert_eq!(with(|s| (s.color, s.picker)), (PALETTE[7], false));

        // Esc com o seletor aberto: só fecha ele.
        click(hwnd, ShotButton::ColorMenu);
        key(hwnd, VK_ESCAPE);
        assert!(with(|s| !s.picker && s.on_shot.is_some()), "o print continua");

        // Arrastar na área com o seletor aberto: só fecha (nada desenhado, a área fica).
        click(hwnd, ShotButton::ColorMenu);
        drag(hwnd, (200, 200), (400, 300));
        assert!(with(|s| !s.picker && s.marks.is_empty()));
        assert_eq!(with(|s| s.sel), Some(RECT { left: 100, top: 100, right: 700, bottom: 500 }));

        // A faixa de matizes: o começo é vermelho; arrastado até o meio, ciano.
        click(hwnd, ShotButton::ColorMenu);
        let hue = with(|s| s.bar.as_ref().and_then(|b| b.button(ShotButton::Hue))).expect("faixa");
        let (y, mid) = ((hue.top + hue.bottom) / 2, (hue.left + hue.right - 1) / 2);
        send(hwnd, WM_LBUTTONDOWN, 1, lp(hue.left, y));
        assert_eq!(with(|s| s.color), crate::annotate::hsv(0.0, 0.85, 0.95));
        send(hwnd, WM_MOUSEMOVE, 1, lp(mid, y));
        send(hwnd, WM_LBUTTONUP, 0, lp(mid, y));
        let c = with(|s| s.color);
        assert!((c >> 16) & 0xFF < 0x40 && (c & 0xFF) > 0xC0 && (c >> 8) & 0xFF > 0xC0, "ciano: {c:08X}");
        assert!(!with(|s| s.picker), "soltar fecha o seletor");
        key(hwnd, VK_ESCAPE);
        take_done();
    }

    /// O − e o + mudam o tamanho como a roda (a espessura; com o Texto, a fonte), e a barra
    /// não muda de largura com o número.
    #[test]
    fn editor_menos_e_mais() {
        let _t = crate::testkit::begin();
        take_done();
        let hwnd = open_editor();
        drag(hwnd, (100, 100), (700, 500));
        show_bar(hwnd);
        let bar = || with(|s| s.bar.as_ref().map(|b| b.rect)).expect("barra");
        let before = bar();
        for _ in 0..3 {
            click(hwnd, ShotButton::Bigger);
        }
        click(hwnd, ShotButton::Smaller);
        assert_eq!(with(|s| s.width), crate::annotate::WIDTH_DEFAULT + 2.0);
        click(hwnd, ShotButton::Tool(Tool::Text));
        click(hwnd, ShotButton::Bigger);
        assert_eq!(with(|s| s.font_size), crate::annotate::SIZE_DEFAULT + crate::annotate::SIZE_STEP);
        assert_eq!(bar(), before, "a barra fica do mesmo tamanho");
        key(hwnd, VK_ESCAPE);
        take_done();
    }

    /// O quadro montado no buffer: fora da área, a imagem escurecida; dentro, a nítida;
    /// em volta, a borda branca — e o buffer começando no meio do cliente (`org`) não
    /// desloca nada.
    #[test]
    fn quadro_no_buffer_com_origem() {
        let _t = crate::testkit::begin();
        let rgba = vec![200u8; 400 * 300 * 4];
        let frozen = Frozen { bright: MemImage::from_rgba(&rgba, 400, 300, 1.0).expect("nítida"), dim: MemImage::from_rgba(&rgba, 400, 300, 0.5).expect("escura") };
        with(|s| {
            s.frozen = Some(frozen);
            s.sel = Some(RECT { left: 100, top: 100, right: 300, bottom: 250 });
            s.dpi = 96;
            s.editing = false;
            s.bar = None;
            s.marks.clear();
            s.drawing = None;
            s.hint_visible = false;
        });
        let buf = MemImage::blank(250, 200).expect("buffer");
        unsafe {
            compose(buf.dc, (50, 50), RECT { left: 50, top: 50, right: 300, bottom: 250 });
        }
        let px = |x: i32, y: i32| unsafe { GetPixel(buf.dc, x - 50, y - 50) }.0 & 0xFF;
        assert_eq!(px(150, 150), 200, "dentro: nítida");
        assert!((px(60, 60) as i32 - 100).abs() <= 1, "fora: escurecida ({})", px(60, 60));
        // A borda: tracejada (traços brancos e o fio escuro alternando) e fina (1 px: o
        // pixel de fora dela já é a imagem escurecida).
        let side: Vec<u32> = (100..140).map(|y| px(99, y)).collect();
        assert!(side.contains(&255) && side.contains(&0x20), "tracejada: {side:?}");
        assert!((px(98, 120) as i32 - 100).abs() <= 1, "fina ({})", px(98, 120));
        with(|s| {
            s.frozen = None;
            s.sel = None;
        });
    }

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

