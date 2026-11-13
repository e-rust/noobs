//! Marcas do print — traços, linhas, setas, retângulos, marca-texto e textos: o editor da
//! seleção desenha na tela e a imagem final sai com elas, pelo MESMO desenho ([`draw`]) — o
//! que se vê é o que vai para o arquivo.
//!
//! * Seta: reta, do ponto inicial ao final, com a ponta no final:
//!   `(inicial)────────▶(final)`. A espessura (`width`) é a do corpo; a ponta cresce com ela.
//! * Linha: reta, sem ponta, com as pontas arredondadas.
//! * Lápis: o traço livre do mouse. Marca-texto: o mesmo traço, largo ([`MARKER_SCALE`]) e
//!   translúcido, num desenho só (onde ele se cruza não escurece).
//! * Texto: Segoe UI em negrito, do tamanho `size` (px), com um contorno de contraste
//!   (escuro em cor clara, claro em cor escura) para ler sobre qualquer fundo.
//!   Várias linhas com `\n`.
//!
//! GDI+ com antialiasing (o GDI serrilha a linha inclinada e a letra grande).

use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::RECT;
use windows::Win32::Graphics::Gdi::{CreateCompatibleDC, DeleteDC, HDC};
use windows::Win32::Graphics::GdiPlus::{
    CombineModeReplace, FillModeAlternate, FillModeWinding, GdipAddPathArc, GdipAddPathString, GdipCloneStringFormat, GdipClosePathFigure,
    GdipCreateBitmapFromScan0, GdipCreateFont, GdipDrawLines, GdipFillEllipse,
    GdipCreateFontFamilyFromName, GdipCreateFromHDC, GdipCreatePath, GdipCreatePen1, GdipCreateSolidFill, GdipDeleteBrush, GdipDeleteFont,
    GdipDeleteFontFamily, GdipDeleteGraphics, GdipDeletePath, GdipDeletePen, GdipDeleteStringFormat, GdipDisposeImage, GdipDrawLine, GdipDrawPath, GdipDrawRectangle, LineJoinMiter,
    GdipFillPath, GdipFillPolygon, GdipFillRectangle, GdipGetEmHeight, GdipGetImageGraphicsContext, GdipGetLineSpacing, GdipGetStringFormatFlags,
    GdipMeasureString, GdipSetClipRectI, GdipSetPenEndCap, GdipSetPenLineJoin, GdipSetPenStartCap, GdipSetSmoothingMode, GdipSetStringFormatFlags,
    GdipSetTextRenderingHint, GdipStringFormatGetGenericTypographic, GpBitmap, GpBrush, GpFont, GpFontFamily, GpGraphics, GpImage, GpPath, GpPen,
    GpSolidFill, GpStringFormat, LineCapFlat, LineCapRound, LineJoinRound, PointF, RectF, SmoothingModeAntiAlias, TextRenderingHintAntiAlias,
    UnitPixel,
};

/// As cores do seletor do editor (ARGB), duas linhas de seis: vermelho, amarelo, verde,
/// azul, branco e preto; laranja, roxo, rosa, ciano, marrom e cinza.
pub const PALETTE: [u32; 12] = [
    0xFFE5_3935, 0xFFFD_D835, 0xFF43_A047, 0xFF1E_88E5, 0xFFFF_FFFF, 0xFF21_2121, //
    0xFFFB_8C00, 0xFF8E_24AA, 0xFFD8_1B60, 0xFF00_ACC1, 0xFF6D_4C41, 0xFF9E_9E9E,
];
/// A cor inicial (o vermelho).
pub const COLOR: u32 = PALETTE[0];
/// Espessura inicial da seta e os limites da roda do mouse, em px.
pub const WIDTH_DEFAULT: f32 = 6.0;
pub const WIDTH_MIN: f32 = 2.0;
pub const WIDTH_MAX: f32 = 40.0;
/// Tamanho inicial da fonte e os limites da roda do mouse, em px (2 px por dente).
pub const SIZE_DEFAULT: f32 = 28.0;
pub const SIZE_MIN: f32 = 12.0;
pub const SIZE_MAX: f32 = 120.0;
pub const SIZE_STEP: f32 = 2.0;
/// `PixelFormat32bppARGB` do GDI+ (BGRA na memória, sem pré-multiplicar).
const PIXEL_FORMAT_32BPP_ARGB: i32 = 0x0026_200A;
/// `FontStyleBold`.
const BOLD: i32 = 1;
/// `StringFormatFlagsMeasureTrailingSpaces`: o espaço no fim conta na largura (o cursor anda).
const MEASURE_TRAILING_SPACES: i32 = 0x0800;

/// Uma seta, em px físicos (cliente na tela do editor; tela na entrega).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Arrow {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
    pub width: f32,
    pub color: u32,
}

/// Um texto: (`x`, `y`) é o canto superior esquerdo da primeira linha.
#[derive(Debug, Clone, PartialEq)]
pub struct Text {
    pub x: f32,
    pub y: f32,
    pub text: String,
    pub size: f32,
    pub color: u32,
}

/// Um retângulo (só o contorno) para demarcar uma região: os dois cantos opostos, em
/// qualquer ordem (de onde o arrasto começou até onde terminou).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Frame {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
    pub width: f32,
    pub color: u32,
}

impl Frame {
    /// Do arrasto (os dois pontos de uma seta) para o retângulo.
    pub fn from_drag(a: &Arrow) -> Frame {
        Frame { x1: a.x1, y1: a.y1, x2: a.x2, y2: a.y2, width: a.width, color: a.color }
    }

    /// Estreito demais para ser retângulo (um clique, ou um risco reto).
    pub fn is_tiny(&self) -> bool {
        (self.x2 - self.x1).abs() < 4.0 || (self.y2 - self.y1).abs() < 4.0
    }

    /// (esquerda, topo, largura, altura), normalizado.
    pub fn normalized(&self) -> (f32, f32, f32, f32) {
        (self.x1.min(self.x2), self.y1.min(self.y2), (self.x2 - self.x1).abs(), (self.y2 - self.y1).abs())
    }

    /// Onde o retângulo desenha: a linha fica centrada na borda (meia espessura para fora).
    pub fn bounds(&self) -> RECT {
        let (l, t, w, h) = self.normalized();
        let pad = (self.width / 2.0).ceil() as i32 + 2;
        RECT { left: l as i32 - pad, top: t as i32 - pad, right: (l + w) as i32 + pad + 1, bottom: (t + h) as i32 + pad + 1 }
    }
}

/// Quantas vezes a espessura escolhida o marca-texto tem.
pub const MARKER_SCALE: f32 = 3.0;
/// A opacidade do marca-texto (0..255).
const MARKER_ALPHA: u32 = 0x80;

/// Um traço livre (o lápis ou o marca-texto): os pontos por onde o mouse passou.
#[derive(Debug, Clone, PartialEq)]
pub struct Stroke {
    pub points: Vec<(f32, f32)>,
    pub width: f32,
    pub color: u32,
    /// Marca-texto: largo e translúcido.
    pub marker: bool,
}

impl Stroke {
    /// A espessura desenhada (o marca-texto é mais largo que a escolhida).
    pub fn drawn_width(&self) -> f32 {
        if self.marker {
            self.width * MARKER_SCALE
        } else {
            self.width
        }
    }

    /// A cor desenhada (o marca-texto, translúcido).
    pub fn drawn_color(&self) -> u32 {
        if self.marker {
            (self.color & 0x00FF_FFFF) | (MARKER_ALPHA << 24)
        } else {
            self.color
        }
    }

    /// Onde o traço desenha (para invalidar).
    pub fn bounds(&self) -> RECT {
        let pad = (self.drawn_width() / 2.0).ceil() as i32 + 2;
        let (mut l, mut t, mut r, mut b) = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
        for &(x, y) in &self.points {
            (l, t, r, b) = (l.min(x), t.min(y), r.max(x), b.max(y));
        }
        if self.points.is_empty() {
            return RECT::default();
        }
        RECT { left: l as i32 - pad, top: t as i32 - pad, right: r as i32 + pad + 1, bottom: b as i32 + pad + 1 }
    }
}

/// Uma marca do print, desenhada na ordem em que foi feita.
#[derive(Debug, Clone, PartialEq)]
pub enum Mark {
    Arrow(Arrow),
    /// Uma linha reta, sem ponta (os dois pontos de uma [`Arrow`]).
    Line(Arrow),
    Rect(Frame),
    Stroke(Stroke),
    Text(Text),
}

impl Mark {
    /// A mesma marca deslocada (cliente → tela, tela → recorte).
    pub fn offset(&self, dx: f32, dy: f32) -> Mark {
        let moved = |a: &Arrow| Arrow { x1: a.x1 + dx, y1: a.y1 + dy, x2: a.x2 + dx, y2: a.y2 + dy, ..*a };
        match self {
            Mark::Arrow(a) => Mark::Arrow(moved(a)),
            Mark::Line(a) => Mark::Line(moved(a)),
            Mark::Rect(f) => Mark::Rect(Frame { x1: f.x1 + dx, y1: f.y1 + dy, x2: f.x2 + dx, y2: f.y2 + dy, ..*f }),
            Mark::Stroke(s) => Mark::Stroke(Stroke { points: s.points.iter().map(|&(x, y)| (x + dx, y + dy)).collect(), ..s.clone() }),
            Mark::Text(t) => Mark::Text(Text { x: t.x + dx, y: t.y + dy, ..t.clone() }),
        }
    }

    /// Onde a marca desenha (para invalidar e, no texto, para clicar nele).
    pub fn bounds(&self) -> RECT {
        match self {
            Mark::Arrow(a) => a.bounds(),
            Mark::Line(a) => {
                let pad = (a.width / 2.0).ceil() as i32 + 2;
                RECT {
                    left: a.x1.min(a.x2) as i32 - pad,
                    top: a.y1.min(a.y2) as i32 - pad,
                    right: a.x1.max(a.x2) as i32 + pad + 1,
                    bottom: a.y1.max(a.y2) as i32 + pad + 1,
                }
            }
            Mark::Rect(f) => f.bounds(),
            Mark::Stroke(s) => s.bounds(),
            Mark::Text(t) => t.bounds(),
        }
    }
}

impl Arrow {
    pub fn len(&self) -> f32 {
        ((self.x2 - self.x1).powi(2) + (self.y2 - self.y1).powi(2)).sqrt()
    }

    /// Curta demais para ser seta (um clique sem arrasto).
    pub fn is_tiny(&self) -> bool {
        self.len() < 4.0
    }

    /// Onde a seta desenha (para invalidar): o retângulo dela com a ponta e a espessura.
    pub fn bounds(&self) -> RECT {
        let pad = (head_size(self.width).1 + self.width) as i32 + 2;
        RECT {
            left: self.x1.min(self.x2) as i32 - pad,
            top: self.y1.min(self.y2) as i32 - pad,
            right: self.x1.max(self.x2) as i32 + pad,
            bottom: self.y1.max(self.y2) as i32 + pad,
        }
    }
}

impl Text {
    /// A espessura do contorno.
    fn outline(&self) -> f32 {
        (self.size / 7.0).max(2.0)
    }

    /// O retângulo do texto (com o contorno e o lugar do cursor no fim). Vazio: o do cursor.
    pub fn bounds(&self) -> RECT {
        let (lh, widths) = measure(self);
        let w = widths.iter().copied().fold(0.0f32, f32::max);
        let pad = self.outline() + 3.0;
        RECT {
            left: (self.x - pad) as i32,
            top: (self.y - pad) as i32,
            right: (self.x + w + pad) as i32 + 1,
            bottom: (self.y + lh * widths.len() as f32 + pad) as i32 + 1,
        }
    }
}

/// Altura de uma linha de texto do tamanho `size` (a da Segoe UI; sem GDI+, uma estimativa).
pub fn line_height(size: f32) -> f32 {
    with_family(|fam| unsafe {
        let (mut ls, mut em) = (0u16, 0u16);
        let ok = GdipGetLineSpacing(fam, BOLD, &mut ls).0 == 0 && GdipGetEmHeight(fam, BOLD, &mut em).0 == 0 && em > 0;
        ok.then(|| size * ls as f32 / em as f32)
    })
    .flatten()
    .unwrap_or(size * 1.33)
}

/// Comprimento e meia-largura da ponta para a espessura `w`: longa e estreita (cerca de
/// 17° de cada lado), a ponta fina das ferramentas de captura.
fn head_size(w: f32) -> (f32, f32) {
    ((w * 5.2).max(20.0), (w * 1.6).max(6.0))
}

/// A forma: o corpo (do início até a base da ponta) e o triângulo da ponta.
/// Seta mais curta que a ponta: só a ponta, encolhida no comprimento.
pub fn shape(a: &Arrow) -> ((f32, f32), (f32, f32), [(f32, f32); 3]) {
    let len = a.len().max(0.001);
    let (dx, dy) = ((a.x2 - a.x1) / len, (a.y2 - a.y1) / len);
    let (mut head_len, half) = head_size(a.width);
    head_len = head_len.min(len);
    let base = (a.x2 - dx * head_len, a.y2 - dy * head_len);
    let (px, py) = (-dy * half, dx * half);
    let head = [(a.x2, a.y2), (base.0 + px, base.1 + py), (base.0 - px, base.1 - py)];
    // O corpo entra um pouco na ponta: sem fresta entre os dois com antialiasing.
    let shaft_end = (base.0 + dx * head_len * 0.2, base.1 + dy * head_len * 0.2);
    ((a.x1, a.y1), shaft_end, head)
}

/// O contorno do texto: escuro numa cor clara, claro numa escura.
fn outline_color(color: u32) -> u32 {
    let (r, g, b) = ((color >> 16) & 0xFF, (color >> 8) & 0xFF, color & 0xFF);
    let luma = (299 * r + 587 * g + 114 * b) / 1000;
    if luma > 140 {
        0xD000_0000
    } else {
        0xE0FF_FFFF
    }
}

/// A família Segoe UI, só durante `f`. `None` sem GDI+ ou sem a fonte.
fn with_family<R>(f: impl FnOnce(*mut GpFontFamily) -> R) -> Option<R> {
    if !crate::rec_indicator::ensure_gdiplus() {
        return None;
    }
    unsafe {
        let mut fam: *mut GpFontFamily = std::ptr::null_mut();
        if GdipCreateFontFamilyFromName(w!("Segoe UI"), std::ptr::null_mut(), &mut fam).0 != 0 || fam.is_null() {
            return None;
        }
        let r = f(fam);
        GdipDeleteFontFamily(fam);
        Some(r)
    }
}

/// O formato tipográfico (sem as margens do formato padrão), contando o espaço no fim.
unsafe fn typographic() -> *mut GpStringFormat {
    let mut generic: *mut GpStringFormat = std::ptr::null_mut();
    let mut fmt: *mut GpStringFormat = std::ptr::null_mut();
    if GdipStringFormatGetGenericTypographic(&mut generic).0 == 0 && GdipCloneStringFormat(generic, &mut fmt).0 == 0 && !fmt.is_null() {
        let mut flags = 0;
        GdipGetStringFormatFlags(fmt, &mut flags);
        GdipSetStringFormatFlags(fmt, flags | MEASURE_TRAILING_SPACES);
    }
    fmt
}

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

/// A altura da linha e a largura de cada linha de `t` (ao menos uma, mesmo vazio).
fn measure(t: &Text) -> (f32, Vec<f32>) {
    let lh = line_height(t.size);
    let lines: Vec<&str> = t.text.split('\n').collect();
    let widths = with_family(|fam| unsafe {
        let dc = CreateCompatibleDC(None);
        let mut gfx: *mut GpGraphics = std::ptr::null_mut();
        let mut font: *mut GpFont = std::ptr::null_mut();
        let ok = GdipCreateFromHDC(dc, &mut gfx).0 == 0 && !gfx.is_null() && GdipCreateFont(fam, t.size, BOLD, UnitPixel, &mut font).0 == 0;
        let widths = if ok { measure_lines(gfx, font, &lines) } else { None };
        if !font.is_null() {
            GdipDeleteFont(font);
        }
        if !gfx.is_null() {
            GdipDeleteGraphics(gfx);
        }
        let _ = DeleteDC(dc);
        widths
    })
    .flatten()
    // Sem GDI+: uma estimativa (meia letra por caractere).
    .unwrap_or_else(|| lines.iter().map(|l| l.chars().count() as f32 * t.size * 0.55).collect());
    (lh, widths)
}

unsafe fn measure_lines(gfx: *mut GpGraphics, font: *mut GpFont, lines: &[&str]) -> Option<Vec<f32>> {
    let fmt = typographic();
    let mut out = Vec::with_capacity(lines.len());
    for line in lines {
        let w = wide(line);
        if w.is_empty() {
            out.push(0.0);
            continue;
        }
        let layout = RectF { X: 0.0, Y: 0.0, Width: 0.0, Height: 0.0 };
        let mut bx = RectF::default();
        GdipMeasureString(gfx, PCWSTR(w.as_ptr()), w.len() as i32, font, &layout, fmt, &mut bx, std::ptr::null_mut(), std::ptr::null_mut());
        out.push(bx.Width);
    }
    if !fmt.is_null() {
        GdipDeleteStringFormat(fmt);
    }
    Some(out)
}

unsafe fn solid(color: u32) -> *mut GpSolidFill {
    let mut brush: *mut GpSolidFill = std::ptr::null_mut();
    GdipCreateSolidFill(color, &mut brush);
    brush
}

unsafe fn draw_arrow(gfx: *mut GpGraphics, a: &Arrow, dx: f32, dy: f32) {
    let (s0, s1, head) = shape(a);
    if a.len() > head_size(a.width).0 {
        let mut pen: *mut GpPen = std::ptr::null_mut();
        GdipCreatePen1(a.color, a.width, UnitPixel, &mut pen);
        if !pen.is_null() {
            GdipSetPenStartCap(pen, LineCapRound);
            GdipSetPenEndCap(pen, LineCapFlat);
            GdipDrawLine(gfx, pen, s0.0 + dx, s0.1 + dy, s1.0 + dx, s1.1 + dy);
            GdipDeletePen(pen);
        }
    }
    let brush = solid(a.color);
    if !brush.is_null() {
        let pts = head.map(|(x, y)| PointF { X: x + dx, Y: y + dy });
        GdipFillPolygon(gfx, brush as *mut GpBrush, pts.as_ptr(), 3, FillModeAlternate);
        GdipDeleteBrush(brush as *mut GpBrush);
    }
}

/// A linha reta: as duas pontas arredondadas.
unsafe fn draw_line(gfx: *mut GpGraphics, a: &Arrow, dx: f32, dy: f32) {
    let mut pen: *mut GpPen = std::ptr::null_mut();
    GdipCreatePen1(a.color, a.width, UnitPixel, &mut pen);
    if !pen.is_null() {
        GdipSetPenStartCap(pen, LineCapRound);
        GdipSetPenEndCap(pen, LineCapRound);
        GdipDrawLine(gfx, pen, a.x1 + dx, a.y1 + dy, a.x2 + dx, a.y2 + dy);
        GdipDeletePen(pen);
    }
}

/// O traço livre, num desenho só (o marca-texto translúcido não escurece onde se cruza);
/// um ponto sozinho (um clique) vira um pingo redondo.
unsafe fn draw_stroke(gfx: *mut GpGraphics, s: &Stroke, dx: f32, dy: f32) {
    let (w, color) = (s.drawn_width(), s.drawn_color());
    if s.points.len() == 1 {
        let (x, y) = s.points[0];
        let brush = solid(color);
        if !brush.is_null() {
            GdipFillEllipse(gfx, brush as *mut GpBrush, x + dx - w / 2.0, y + dy - w / 2.0, w, w);
            GdipDeleteBrush(brush as *mut GpBrush);
        }
        return;
    }
    let mut pen: *mut GpPen = std::ptr::null_mut();
    GdipCreatePen1(color, w, UnitPixel, &mut pen);
    if !pen.is_null() {
        GdipSetPenStartCap(pen, LineCapRound);
        GdipSetPenEndCap(pen, LineCapRound);
        GdipSetPenLineJoin(pen, LineJoinRound);
        let pts: Vec<PointF> = s.points.iter().map(|&(x, y)| PointF { X: x + dx, Y: y + dy }).collect();
        GdipDrawLines(gfx, pen, pts.as_ptr(), pts.len() as i32);
        GdipDeletePen(pen);
    }
}

/// O retângulo: só a borda, na cor e na espessura dele, com os cantos vivos.
unsafe fn draw_rect(gfx: *mut GpGraphics, f: &Frame, dx: f32, dy: f32) {
    let (l, t, w, h) = f.normalized();
    let mut pen: *mut GpPen = std::ptr::null_mut();
    GdipCreatePen1(f.color, f.width, UnitPixel, &mut pen);
    if !pen.is_null() {
        GdipSetPenLineJoin(pen, LineJoinMiter);
        GdipDrawRectangle(gfx, pen, l + dx, t + dy, w, h);
        GdipDeletePen(pen);
    }
}

/// O texto como contorno de letras: primeiro o contorno grosso (cor de contraste),
/// depois o preenchimento na cor do texto.
unsafe fn draw_text(gfx: *mut GpGraphics, fam: *mut GpFontFamily, t: &Text, dx: f32, dy: f32) {
    if t.text.is_empty() {
        return;
    }
    let w = wide(&t.text);
    let mut path: *mut GpPath = std::ptr::null_mut();
    if GdipCreatePath(FillModeWinding, &mut path).0 != 0 || path.is_null() {
        return;
    }
    let fmt = typographic();
    let layout = RectF { X: t.x + dx, Y: t.y + dy, Width: 0.0, Height: 0.0 };
    GdipAddPathString(path, PCWSTR(w.as_ptr()), w.len() as i32, fam, BOLD, t.size, &layout, fmt);
    let mut pen: *mut GpPen = std::ptr::null_mut();
    GdipCreatePen1(outline_color(t.color), t.outline(), UnitPixel, &mut pen);
    if !pen.is_null() {
        GdipSetPenLineJoin(pen, LineJoinRound);
        GdipDrawPath(gfx, pen, path);
        GdipDeletePen(pen);
    }
    let brush = solid(t.color);
    if !brush.is_null() {
        GdipFillPath(gfx, brush as *mut GpBrush, path);
        GdipDeleteBrush(brush as *mut GpBrush);
    }
    if !fmt.is_null() {
        GdipDeleteStringFormat(fmt);
    }
    GdipDeletePath(path);
}

/// Desenha `marks` deslocadas por (`dx`, `dy`) em `gfx`.
///
/// # Safety
/// `gfx` válido.
unsafe fn draw(gfx: *mut GpGraphics, marks: &[Mark], dx: f32, dy: f32) {
    GdipSetSmoothingMode(gfx, SmoothingModeAntiAlias);
    GdipSetTextRenderingHint(gfx, TextRenderingHintAntiAlias);
    let has_text = marks.iter().any(|m| matches!(m, Mark::Text(_)));
    let draw_all = |fam: *mut GpFontFamily| {
        for m in marks {
            match m {
                Mark::Arrow(a) => draw_arrow(gfx, a, dx, dy),
                Mark::Line(a) => draw_line(gfx, a, dx, dy),
                Mark::Rect(f) => draw_rect(gfx, f, dx, dy),
                Mark::Stroke(s) => draw_stroke(gfx, s, dx, dy),
                Mark::Text(t) if !fam.is_null() => draw_text(gfx, fam, t, dx, dy),
                Mark::Text(_) => {}
            }
        }
    };
    if !has_text || with_family(draw_all).is_none() {
        // Sem texto (ou sem a fonte): as setas mesmo assim.
        if has_text {
            log::log_once_missing_font();
        }
        draw_all(std::ptr::null_mut());
    }
}

mod log {
    use std::sync::atomic::{AtomicBool, Ordering};
    static SAID: AtomicBool = AtomicBool::new(false);
    pub fn log_once_missing_font() {
        if !SAID.swap(true, Ordering::Relaxed) {
            crate::log!("Print: a fonte Segoe UI nao abriu no GDI+; os textos ficam de fora.");
        }
    }
}

/// Na janela da seleção: `marks` (coordenadas do cliente) só dentro de `clip`; com
/// `caret`, o cursor de texto no fim do texto `marks[caret]` (o que está sendo digitado).
/// `org`: onde o (0, 0) do `hdc` cai no cliente — o buffer do `WM_PAINT` começa no canto
/// da região pintada, então tudo anda (-org). O recorte do `hdc` continua valendo.
pub fn draw_on_hdc(hdc: HDC, marks: &[Mark], clip: RECT, caret: Option<usize>, org: (i32, i32)) {
    if marks.is_empty() || !crate::rec_indicator::ensure_gdiplus() {
        return;
    }
    let (dx, dy) = (-org.0 as f32, -org.1 as f32);
    unsafe {
        let mut gfx: *mut GpGraphics = std::ptr::null_mut();
        if GdipCreateFromHDC(hdc, &mut gfx).0 != 0 || gfx.is_null() {
            return;
        }
        GdipSetClipRectI(gfx, clip.left - org.0, clip.top - org.1, clip.right - clip.left, clip.bottom - clip.top, CombineModeReplace);
        draw(gfx, marks, dx, dy);
        if let Some(Mark::Text(t)) = caret.and_then(|i| marks.get(i)) {
            let (x, y, h) = caret_at(t);
            let brush = solid(t.color);
            if !brush.is_null() {
                GdipFillRectangle(gfx, brush as *mut GpBrush, x + dx, y + dy, (t.size / 14.0).max(2.0), h);
                GdipDeleteBrush(brush as *mut GpBrush);
            }
        }
        GdipDeleteGraphics(gfx);
    }
}

/// Os ícones da barra do editor, desenhados (não por fonte de ícones: a Segoe Fluent
/// Icons só existe no Windows 11).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Icon {
    Pen,
    Line,
    Arrow,
    Rect,
    Marker,
    Text,
    Undo,
    Copy,
    Save,
    Close,
    Minus,
    Plus,
    /// O triângulo para baixo (o seletor de cores abre).
    Caret,
}

/// Um retângulo de cantos arredondados em `hdc` (antialiasing), com borda de 1 px opcional.
/// Cores ARGB.
pub fn fill_round(hdc: HDC, r: RECT, radius: f32, fill: u32, border: Option<u32>) {
    if !crate::rec_indicator::ensure_gdiplus() {
        return;
    }
    unsafe {
        let mut gfx: *mut GpGraphics = std::ptr::null_mut();
        if GdipCreateFromHDC(hdc, &mut gfx).0 != 0 || gfx.is_null() {
            return;
        }
        GdipSetSmoothingMode(gfx, SmoothingModeAntiAlias);
        let mut path: *mut GpPath = std::ptr::null_mut();
        if GdipCreatePath(FillModeWinding, &mut path).0 == 0 && !path.is_null() {
            let (x, y) = (r.left as f32, r.top as f32);
            let (w, h) = ((r.right - r.left) as f32 - 1.0, (r.bottom - r.top) as f32 - 1.0);
            let d = (radius * 2.0).min(w).min(h).max(0.1);
            GdipAddPathArc(path, x, y, d, d, 180.0, 90.0);
            GdipAddPathArc(path, x + w - d, y, d, d, 270.0, 90.0);
            GdipAddPathArc(path, x + w - d, y + h - d, d, d, 0.0, 90.0);
            GdipAddPathArc(path, x, y + h - d, d, d, 90.0, 90.0);
            GdipClosePathFigure(path);
            let brush = solid(fill);
            if !brush.is_null() {
                GdipFillPath(gfx, brush as *mut GpBrush, path);
                GdipDeleteBrush(brush as *mut GpBrush);
            }
            if let Some(b) = border {
                let mut pen: *mut GpPen = std::ptr::null_mut();
                GdipCreatePen1(b, 1.0, UnitPixel, &mut pen);
                if !pen.is_null() {
                    GdipDrawPath(gfx, pen, path);
                    GdipDeletePen(pen);
                }
            }
            GdipDeletePath(path);
        }
        GdipDeleteGraphics(gfx);
    }
}

/// O ícone `icon` centrado em `r` (coordenadas do `hdc`), na cor `color` (ARGB), com
/// antialiasing. Desenhado para 28 px e escalado pelo lado menor de `r`.
pub fn draw_icon(hdc: HDC, icon: Icon, r: RECT, color: u32) {
    if !crate::rec_indicator::ensure_gdiplus() {
        return;
    }
    unsafe {
        let mut gfx: *mut GpGraphics = std::ptr::null_mut();
        if GdipCreateFromHDC(hdc, &mut gfx).0 != 0 || gfx.is_null() {
            return;
        }
        GdipSetSmoothingMode(gfx, SmoothingModeAntiAlias);
        let (w, h) = ((r.right - r.left) as f32, (r.bottom - r.top) as f32);
        let (cx, cy) = (r.left as f32 + w / 2.0, r.top as f32 + h / 2.0);
        // O ▾ vai numa faixa estreita: a escala pela altura.
        let u = if icon == Icon::Caret { h / 28.0 } else { w.min(h) / 28.0 };
        let stroke = (2.0 * u).max(1.5);
        let line = |x1: f32, y1: f32, x2: f32, y2: f32| {
            let mut pen: *mut GpPen = std::ptr::null_mut();
            GdipCreatePen1(color, stroke, UnitPixel, &mut pen);
            if !pen.is_null() {
                GdipSetPenStartCap(pen, LineCapRound);
                GdipSetPenEndCap(pen, LineCapRound);
                GdipDrawLine(gfx, pen, x1, y1, x2, y2);
                GdipDeletePen(pen);
            }
        };
        let poly = |pts: &[(f32, f32)]| {
            let brush = solid(color);
            if !brush.is_null() {
                let pts: Vec<PointF> = pts.iter().map(|&(x, y)| PointF { X: x, Y: y }).collect();
                GdipFillPolygon(gfx, brush as *mut GpBrush, pts.as_ptr(), pts.len() as i32, FillModeAlternate);
                GdipDeleteBrush(brush as *mut GpBrush);
            }
        };
        let rect = |x: f32, y: f32, rw: f32, rh: f32| poly(&[(x, y), (x + rw, y), (x + rw, y + rh), (x, y + rh)]);
        // Uma linha quebrada (contornos e curvas), com as junções e as pontas redondas.
        let polyline = |pts: &[(f32, f32)], c: u32| {
            let mut pen: *mut GpPen = std::ptr::null_mut();
            GdipCreatePen1(c, stroke, UnitPixel, &mut pen);
            if !pen.is_null() {
                GdipSetPenStartCap(pen, LineCapRound);
                GdipSetPenEndCap(pen, LineCapRound);
                GdipSetPenLineJoin(pen, LineJoinRound);
                let pts: Vec<PointF> = pts.iter().map(|&(x, y)| PointF { X: x, Y: y }).collect();
                GdipDrawLines(gfx, pen, pts.as_ptr(), pts.len() as i32);
                GdipDeletePen(pen);
            }
        };
        // O lápis e o marca-texto: inclinados, a ponta embaixo à esquerda. `a` sobe para a
        // direita ao longo do corpo; `n` é a largura.
        let d = std::f32::consts::FRAC_1_SQRT_2;
        let (a, n) = ((d, -d), (d, d));
        let at = |p: (f32, f32), k: f32, m: f32| (p.0 + a.0 * k + n.0 * m, p.1 + a.1 * k + n.1 * m);
        match icon {
            Icon::Pen => {
                let tip = (cx - 7.0 * u, cy + 7.0 * u);
                let half = 2.4 * u;
                poly(&[tip, at(tip, 4.5 * u, half), at(tip, 4.5 * u, -half)]);
                poly(&[at(tip, 5.7 * u, half), at(tip, 18.5 * u, half), at(tip, 18.5 * u, -half), at(tip, 5.7 * u, -half)]);
            }
            Icon::Line => line(cx - 7.0 * u, cy + 7.0 * u, cx + 7.0 * u, cy - 7.0 * u),
            Icon::Marker => {
                // O traço translúcido embaixo e o marcador de ponta chanfrada em cima.
                rect(cx - 8.0 * u, cy + 5.5 * u, 16.0 * u, 3.0 * u);
                let tip = (cx - 4.5 * u, cy + 2.5 * u);
                let half = 3.0 * u;
                poly(&[at(tip, 0.0, 1.3 * u), at(tip, 0.0, -1.3 * u), at(tip, 3.0 * u, -half), at(tip, 3.0 * u, half)]);
                poly(&[at(tip, 4.2 * u, half), at(tip, 13.0 * u, half), at(tip, 13.0 * u, -half), at(tip, 4.2 * u, -half)]);
            }
            Icon::Undo => {
                // A curva por cima, da direita para a esquerda, e a ponta descendo à esquerda.
                let (ox, oy, r) = (cx + 1.0 * u, cy + 1.5 * u, 6.0 * u);
                let pts: Vec<(f32, f32)> = (0..=24)
                    .map(|i| {
                        let t = (-35.0 + 230.0 * i as f32 / 24.0).to_radians();
                        (ox + r * t.cos(), oy - r * t.sin())
                    })
                    .collect();
                polyline(&pts, color);
                let end = 195.0f32.to_radians();
                let e = (ox + r * end.cos(), oy - r * end.sin());
                let dir = (-end.sin(), -end.cos());
                let perp = (-dir.1, dir.0);
                let (hl, hw) = (4.5 * u, 3.6 * u);
                poly(&[
                    (e.0 + dir.0 * hl, e.1 + dir.1 * hl),
                    (e.0 + perp.0 * hw, e.1 + perp.1 * hw),
                    (e.0 - perp.0 * hw, e.1 - perp.1 * hw),
                ]);
            }
            Icon::Copy => {
                // Duas folhas: a de trás aparece em cima e à direita da da frente.
                polyline(&[(cx - 3.0 * u, cy - 4.0 * u), (cx - 3.0 * u, cy - 7.5 * u), (cx + 7.0 * u, cy - 7.5 * u), (cx + 7.0 * u, cy + 3.5 * u), (cx + 3.5 * u, cy + 3.5 * u)], color);
                polyline(&[(cx - 7.0 * u, cy - 4.0 * u), (cx + 3.0 * u, cy - 4.0 * u), (cx + 3.0 * u, cy + 7.5 * u), (cx - 7.0 * u, cy + 7.5 * u), (cx - 7.0 * u, cy - 4.0 * u)], color);
            }
            Icon::Save => {
                // A seta para baixo, entrando na bandeja.
                line(cx, cy - 7.5 * u, cx, cy + 0.5 * u);
                poly(&[(cx - 4.5 * u, cy - 1.0 * u), (cx + 4.5 * u, cy - 1.0 * u), (cx, cy + 4.0 * u)]);
                polyline(&[(cx - 7.5 * u, cy + 2.5 * u), (cx - 7.5 * u, cy + 7.5 * u), (cx + 7.5 * u, cy + 7.5 * u), (cx + 7.5 * u, cy + 2.5 * u)], color);
            }
            Icon::Arrow => {
                // Do canto de baixo à esquerda para o de cima à direita, com a ponta cheia.
                let (x1, y1, x2, y2) = (cx - 7.0 * u, cy + 7.0 * u, cx + 7.0 * u, cy - 7.0 * u);
                let d = std::f32::consts::FRAC_1_SQRT_2;
                let (hl, hw) = (7.0 * u, 4.0 * u);
                let base = (x2 - d * hl, y2 + d * hl);
                line(x1, y1, base.0 + d * hl * 0.3, base.1 - d * hl * 0.3);
                poly(&[(x2, y2), (base.0 + d * hw, base.1 + d * hw), (base.0 - d * hw, base.1 - d * hw)]);
            }
            Icon::Rect => {
                let mut pen: *mut GpPen = std::ptr::null_mut();
                GdipCreatePen1(color, stroke, UnitPixel, &mut pen);
                if !pen.is_null() {
                    GdipSetPenLineJoin(pen, LineJoinMiter);
                    GdipDrawRectangle(gfx, pen, cx - 7.5 * u, cy - 5.5 * u, 15.0 * u, 11.0 * u);
                    GdipDeletePen(pen);
                }
            }
            Icon::Text => {
                let t = 2.6 * u;
                rect(cx - 6.5 * u, cy - 7.5 * u, 13.0 * u, t);
                rect(cx - t / 2.0, cy - 7.5 * u, t, 15.0 * u);
            }
            Icon::Close => {
                line(cx - 5.0 * u, cy - 5.0 * u, cx + 5.0 * u, cy + 5.0 * u);
                line(cx - 5.0 * u, cy + 5.0 * u, cx + 5.0 * u, cy - 5.0 * u);
            }
            Icon::Minus => rect(cx - 5.0 * u, cy - stroke / 2.0, 10.0 * u, stroke),
            Icon::Plus => {
                rect(cx - 5.0 * u, cy - stroke / 2.0, 10.0 * u, stroke);
                rect(cx - stroke / 2.0, cy - 5.0 * u, stroke, 10.0 * u);
            }
            Icon::Caret => poly(&[(cx - 4.0 * u, cy - 2.0 * u), (cx + 4.0 * u, cy - 2.0 * u), (cx, cy + 2.5 * u)]),
        }
        GdipDeleteGraphics(gfx);
    }
}

/// A cor (ARGB, opaca) da matiz `h` (graus) com saturação `s` e brilho `v` (0..1).
pub fn hsv(h: f32, s: f32, v: f32) -> u32 {
    let h = h.rem_euclid(360.0) / 60.0;
    let c = v * s;
    let x = c * (1.0 - (h % 2.0 - 1.0).abs());
    let (r, g, b) = match h as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let m = v - c;
    let to = |f: f32| ((f + m) * 255.0).round().clamp(0.0, 255.0) as u32;
    0xFF00_0000 | (to(r) << 16) | (to(g) << 8) | to(b)
}

/// Onde fica o cursor no fim de `t`: (x, topo, altura).
pub fn caret_at(t: &Text) -> (f32, f32, f32) {
    let (lh, widths) = measure(t);
    let last = widths.len().saturating_sub(1);
    (t.x + widths.get(last).copied().unwrap_or(0.0) + 1.0, t.y + lh * last as f32, lh)
}

/// Na imagem final: `rgba` (`w`×`h`, linhas de cima pra baixo) recebe `marks`
/// deslocadas por (`dx`, `dy`) — a marca em coordenadas de tela vira coordenada do recorte.
/// `false` se o GDI+ não está disponível (a imagem fica como estava).
pub fn draw_on_rgba(rgba: &mut [u8], w: u32, h: u32, marks: &[Mark], dx: f32, dy: f32) -> bool {
    if marks.is_empty() {
        return true;
    }
    if rgba.len() != w as usize * h as usize * 4 || !crate::rec_indicator::ensure_gdiplus() {
        return false;
    }
    // GDI+ quer BGRA: troca R e B na ida e na volta.
    let swap = |buf: &mut [u8]| buf.chunks_exact_mut(4).for_each(|p| p.swap(0, 2));
    swap(rgba);
    let ok = unsafe {
        let mut bmp: *mut GpBitmap = std::ptr::null_mut();
        let created = GdipCreateBitmapFromScan0(w as i32, h as i32, (w * 4) as i32, PIXEL_FORMAT_32BPP_ARGB, Some(rgba.as_mut_ptr() as *const u8), &mut bmp);
        if created.0 != 0 || bmp.is_null() {
            false
        } else {
            let mut gfx: *mut GpGraphics = std::ptr::null_mut();
            let ok = GdipGetImageGraphicsContext(bmp as *mut GpImage, &mut gfx).0 == 0 && !gfx.is_null();
            if ok {
                draw(gfx, marks, dx, dy);
                GdipDeleteGraphics(gfx);
            }
            GdipDisposeImage(bmp as *mut GpImage);
            ok
        }
    };
    swap(rgba);
    ok
}

#[cfg(test)]
mod tests {
    use super::*;

    fn arrow(x1: f32, y1: f32, x2: f32, y2: f32, width: f32) -> Arrow {
        Arrow { x1, y1, x2, y2, width, color: COLOR }
    }

    #[test]
    fn a_ponta_fica_no_final_e_aponta_para_ele() {
        let a = arrow(0.0, 0.0, 100.0, 0.0, 6.0);
        let (s0, s1, head) = shape(&a);
        assert_eq!(s0, (0.0, 0.0), "o corpo sai do ponto inicial");
        assert_eq!(head[0], (100.0, 0.0), "a ponta no ponto final");
        // Base da ponta antes do final, simétrica em volta da linha.
        assert!(head[1].0 < 100.0 && (head[1].1 + head[2].1).abs() < 1e-4);
        assert!(s1.0 < 100.0 && s1.0 > head[1].0, "o corpo entra na ponta sem passar dela");
    }

    #[test]
    fn ponta_cresce_com_a_espessura() {
        let fina = shape(&arrow(0.0, 0.0, 200.0, 0.0, 4.0)).2;
        let grossa = shape(&arrow(0.0, 0.0, 200.0, 0.0, 20.0)).2;
        assert!(grossa[1].1 > fina[1].1, "meia-largura maior");
        assert!(grossa[1].0 < fina[1].0, "base mais longe do final");
    }

    #[test]
    fn seta_curta_e_so_ponta() {
        let (_, _, head) = shape(&arrow(0.0, 0.0, 5.0, 0.0, 6.0));
        assert!(head[1].0 >= -0.001, "a ponta não passa do ponto inicial");
        assert!(arrow(0.0, 0.0, 2.0, 2.0, 6.0).is_tiny());
    }

    fn px(rgba: &[u8], w: u32, x: u32, y: u32) -> (u8, u8, u8) {
        let i = ((y * w + x) * 4) as usize;
        (rgba[i], rgba[i + 1], rgba[i + 2])
    }

    /// Na imagem: o meio do corpo e a ponta ficam da cor da seta; longe dela, nada muda.
    #[test]
    fn desenha_na_imagem_com_deslocamento() {
        let (w, h) = (200u32, 100u32);
        let mut rgba = vec![255u8; (w * h * 4) as usize];
        // Seta em coordenadas de "tela" (1000, 1000) → (1150, 1000); o recorte começa em (980, 950).
        let a = Mark::Arrow(arrow(1000.0, 1000.0, 1150.0, 1000.0, 8.0));
        assert!(draw_on_rgba(&mut rgba, w, h, &[a], -980.0, -950.0));
        let (r, g, b) = px(&rgba, w, 90, 50); // meio do corpo: (1070, 1000) − (980, 950)
        assert!(r > 200 && g < 100 && b < 100, "corpo vermelho: {:?}", (r, g, b));
        let (r, g, b) = px(&rgba, w, 165, 50); // dentro da ponta, perto do final (1145, 1000)
        assert!(r > 200 && g < 100 && b < 100, "ponta vermelha: {:?}", (r, g, b));
        assert_eq!(px(&rgba, w, 10, 10), (255, 255, 255), "longe da seta: intocado");
        assert_eq!(px(&rgba, w, 90, 90), (255, 255, 255));
    }

    /// A seta sai na cor escolhida (azul).
    #[test]
    fn seta_na_cor_escolhida() {
        let (w, h) = (100u32, 40u32);
        let mut rgba = vec![255u8; (w * h * 4) as usize];
        let a = Mark::Arrow(Arrow { color: PALETTE[3], ..arrow(5.0, 20.0, 95.0, 20.0, 8.0) });
        assert!(draw_on_rgba(&mut rgba, w, h, &[a], 0.0, 0.0));
        let (r, g, b) = px(&rgba, w, 40, 20);
        assert!(b > 180 && r < 80, "azul: {:?}", (r, g, b));
    }

    /// O retângulo: a borda na cor e na espessura dele (arrastado de baixo para cima,
    /// da direita para a esquerda, dá no mesmo); o miolo e o lado de fora ficam intocados.
    #[test]
    fn retangulo_so_a_borda() {
        let (w, h) = (200u32, 120u32);
        let mut rgba = vec![255u8; (w * h * 4) as usize];
        let f = Frame { x1: 1160.0, y1: 1090.0, x2: 1020.0, y2: 1020.0, width: 6.0, color: PALETTE[2] };
        assert_eq!(f.normalized(), (1020.0, 1020.0, 140.0, 70.0));
        assert!(draw_on_rgba(&mut rgba, w, h, &[Mark::Rect(f)], -1000.0, -1000.0));
        let green = |p: (u8, u8, u8)| p.1 > 120 && p.0 < 120 && p.2 < 120;
        assert!(green(px(&rgba, w, 20, 50)), "borda esquerda: {:?}", px(&rgba, w, 20, 50));
        assert!(green(px(&rgba, w, 90, 20)), "borda de cima");
        assert!(green(px(&rgba, w, 160, 60)), "borda direita");
        assert!(green(px(&rgba, w, 90, 90)), "borda de baixo");
        assert_eq!(px(&rgba, w, 90, 55), (255, 255, 255), "o miolo fica como estava");
        assert_eq!(px(&rgba, w, 5, 5), (255, 255, 255), "fora, também");
        assert_eq!(px(&rgba, w, 190, 110), (255, 255, 255));
        // A borda cabe no retângulo de invalidar.
        let b = f.bounds();
        assert!(b.left <= 1017 && b.top <= 1017 && b.right >= 1163 && b.bottom >= 1093, "{b:?}");
        assert!(!Frame { y2: 1095.0, ..f }.is_tiny() && Frame { y2: 1088.0, ..f }.is_tiny(), "altura 5: retângulo; altura 2: risco");
    }

    /// O texto desenha na cor dele, dentro do retângulo que `bounds` diz; fora, nada.
    #[test]
    fn texto_na_imagem_na_cor_e_no_lugar() {
        let (w, h) = (300u32, 120u32);
        let mut rgba = vec![0u8; (w * h * 4) as usize]; // fundo preto: contorno claro
        let t = Text { x: 520.0, y: 220.0, text: "HHHH".into(), size: 48.0, color: PALETTE[1] };
        let b = t.bounds();
        assert!(draw_on_rgba(&mut rgba, w, h, &[Mark::Text(t)], -500.0, -200.0));
        // Algum pixel amarelo (R e G altos, B baixo) dentro do retângulo; nenhum pintado fora.
        let (mut yellow, mut outside) = (0, 0);
        for y in 0..h {
            for x in 0..w {
                let (r, g, bl) = px(&rgba, w, x, y);
                let (sx, sy) = (x as i32 + 500, y as i32 + 200);
                let inside = sx >= b.left && sx < b.right && sy >= b.top && sy < b.bottom;
                if r > 200 && g > 180 && bl < 120 {
                    yellow += 1;
                }
                if !inside && (r, g, bl) != (0, 0, 0) {
                    outside += 1;
                }
            }
        }
        assert!(yellow > 200, "texto amarelo desenhado: {yellow} px");
        assert_eq!(outside, 0, "nada fora do retângulo do texto");
    }

    /// O retângulo cresce com o tamanho da fonte, com mais letras e com mais linhas; o
    /// cursor anda com o espaço no fim e desce com a nova linha.
    #[test]
    fn medidas_do_texto() {
        let t = |s: &str, size: f32| Text { x: 0.0, y: 0.0, text: s.into(), size, color: COLOR };
        let width = |r: RECT| r.right - r.left;
        let height = |r: RECT| r.bottom - r.top;
        assert!(width(t("Oi", 48.0).bounds()) > width(t("Oi", 20.0).bounds()));
        assert!(width(t("Olá mundo", 28.0).bounds()) > width(t("Olá", 28.0).bounds()));
        assert!(height(t("a\nb", 28.0).bounds()) > height(t("a", 28.0).bounds()) + 20);
        let (x1, _, _) = caret_at(&t("Oi", 28.0));
        let (x2, _, _) = caret_at(&t("Oi ", 28.0));
        assert!(x2 > x1 + 3.0, "o espaço no fim move o cursor: {x1} → {x2}");
        let (x3, y3, _) = caret_at(&t("Oi\n", 28.0));
        assert!(x3 < 3.0 && y3 > 20.0, "nova linha: o cursor volta ao começo, embaixo");
        assert!(width(t("", 28.0).bounds()) > 0, "vazio ainda tem o lugar do cursor");
    }
}
