//! Marcas do print — traços, linhas, setas, retângulos e marca-texto: o editor do print
//! ([`crate::print_editor`]) desenha na tela e a imagem final sai do mesmo desenho (o
//! `render_rgba` do form): o que se vê é o que vai para o arquivo. Os textos não estão aqui:
//! cada um é um `Memo` do form.
//!
//! * Seta: reta, do ponto inicial ao final, com a ponta no final:
//!   `(inicial)────────▶(final)`. A espessura (`width`) é a do corpo; a ponta cresce com ela.
//! * Linha: reta, sem ponta, com as pontas arredondadas.
//! * Lápis: o traço livre do mouse. Marca-texto: o mesmo traço, largo ([`MARKER_SCALE`]) e
//!   translúcido, num desenho só (onde ele se cruza não escurece).
//!
//! Coordenadas em pixels físicos do form do print (o retângulo virtual da tela).

use aylis::prelude::{Brush, Canvas, Color, LineCap, LineJoin, Path, PointF, RectF};

/// As cores do seletor do editor (ARGB), duas linhas de seis: vermelho, amarelo, verde,
/// azul, branco e preto; laranja, roxo, rosa, ciano, marrom e cinza.
pub const PALETTE: [u32; 12] = [
    0xFFE5_3935, 0xFFFD_D835, 0xFF43_A047, 0xFF1E_88E5, 0xFFFF_FFFF, 0xFF21_2121, //
    0xFFFB_8C00, 0xFF8E_24AA, 0xFFD8_1B60, 0xFF00_ACC1, 0xFF6D_4C41, 0xFF9E_9E9E,
];
/// A cor inicial (o vermelho).
pub const COLOR: u32 = PALETTE[0];
/// Espessura inicial da seta e os limites da roda do mouse, em px.
pub const WIDTH_DEFAULT: f32 = 4.0;
pub const WIDTH_MIN: f32 = 2.0;
pub const WIDTH_MAX: f32 = 40.0;
/// Tamanho inicial da fonte e os limites da roda do mouse, em px (2 px por dente).
pub const SIZE_DEFAULT: f32 = 20.0;
pub const SIZE_MIN: f32 = 12.0;
pub const SIZE_MAX: f32 = 120.0;
pub const SIZE_STEP: f32 = 2.0;
/// O estilo do texto: negrito, itálico e sublinhado somados (0: normal).
pub const STYLE_BOLD: i32 = 1;
pub const STYLE_ITALIC: i32 = 2;
pub const STYLE_UNDERLINE: i32 = 4;

/// Uma seta (ou uma linha), em px físicos.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Arrow {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
    pub width: f32,
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
    pub fn bounds(&self) -> RectF {
        let (l, t, w, h) = self.normalized();
        RectF::new(l, t, w, h).inflate((self.width / 2.0).ceil() + 2.0)
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

    /// Onde o traço desenha (para repintar).
    pub fn bounds(&self) -> RectF {
        if self.marker {
            points_bounds(&marker_path(&self.points, self.drawn_width()), self.drawn_width())
        } else {
            points_bounds(&self.points, self.drawn_width())
        }
    }
}

/// As 8 direções do marca-texto (a horizontal, a vertical e as de 45°), de 45 em 45 graus.
fn direction(i: usize) -> (f32, f32) {
    let a = i as f32 * std::f32::consts::FRAC_PI_4;
    (a.cos(), a.sin())
}

/// A direção (das 8) mais perto de (`dx`, `dy`).
fn snap(dx: f32, dy: f32) -> usize {
    let a = dy.atan2(dx) / std::f32::consts::FRAC_PI_4;
    (a.round() as i32).rem_euclid(8) as usize
}

/// O caminho do marca-texto: o traço do mouse em trechos retos, cada um na horizontal, na
/// vertical ou a 45° (a mão treme; o marcador numa linha de texto, não). O trecho segue a
/// direção dele até o mouse se afastar da reta mais que a largura do traço: aí começa outro
/// trecho, do fim deste, na direção nova.
pub fn marker_path(points: &[(f32, f32)], width: f32) -> Vec<(f32, f32)> {
    let Some(&first) = points.first() else { return Vec::new() };
    let reach = width.max(4.0);
    let mut out = vec![first];
    let (mut start, mut end, mut dir) = (first, first, None::<usize>);
    for &p in &points[1..] {
        let (dx, dy) = (p.0 - start.0, p.1 - start.1);
        match dir {
            // A direção sai do primeiro passo que conta (meia largura: o tremor do clique não).
            // Até lá, o traço vai do clique ao mouse como está (cresce para o lado do arrasto).
            None if (dx * dx + dy * dy).sqrt() < reach / 2.0 => {
                end = p;
                continue;
            }
            None => dir = Some(snap(dx, dy)),
            Some(d) => {
                let (ux, uy) = direction(d);
                let along = dx * ux + dy * uy;
                let off = (dx * uy - dy * ux).abs();
                // Saiu da reta (ou voltou para trás dela): o trecho termina onde estava.
                if off > reach || along < -reach / 2.0 {
                    if end != start {
                        out.push(end);
                    }
                    start = end;
                    dir = Some(snap(p.0 - start.0, p.1 - start.1));
                }
            }
        }
        let (ux, uy) = direction(dir.unwrap_or(0));
        let along = ((p.0 - start.0) * ux + (p.1 - start.1) * uy).max(0.0);
        end = (start.0 + ux * along, start.1 + uy * along);
    }
    if out.last() != Some(&end) {
        out.push(end);
    }
    out
}

/// O retângulo de `points` com a meia espessura `width` em volta (e a folga do antialiasing).
pub fn points_bounds(points: &[(f32, f32)], width: f32) -> RectF {
    let Some(&(x0, y0)) = points.first() else { return RectF::default() };
    let (mut l, mut t, mut r, mut b) = (x0, y0, x0, y0);
    for &(x, y) in points {
        (l, t, r, b) = (l.min(x), t.min(y), r.max(x), b.max(y));
    }
    // Meia diagonal: a ponta quadrada do marca-texto inclinada passa da meia espessura.
    RectF::new(l, t, r - l, b - t).inflate((width * std::f32::consts::FRAC_1_SQRT_2).ceil() + 2.0)
}

/// Uma marca do print, desenhada na ordem em que foi feita.
#[derive(Debug, Clone, PartialEq)]
pub enum Mark {
    Arrow(Arrow),
    /// Uma linha reta, sem ponta (os dois pontos de uma [`Arrow`]).
    Line(Arrow),
    Rect(Frame),
    Stroke(Stroke),
}

impl Mark {
    /// Onde a marca desenha (para repintar).
    pub fn bounds(&self) -> RectF {
        match self {
            Mark::Arrow(a) => a.bounds(),
            Mark::Line(a) => points_bounds(&[(a.x1, a.y1), (a.x2, a.y2)], a.width),
            Mark::Rect(f) => f.bounds(),
            Mark::Stroke(s) => s.bounds(),
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

    /// Onde a seta desenha: o retângulo dela com a ponta e a espessura.
    pub fn bounds(&self) -> RectF {
        let pad = head_size(self.width).1 + self.width + 2.0;
        RectF::from_points(PointF::new(self.x1, self.y1), PointF::new(self.x2, self.y2)).inflate(pad)
    }
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

/// ARGB → a cor do Aylis.
pub fn color(argb: u32) -> Color {
    Color::rgba((argb >> 16) as u8, (argb >> 8) as u8, argb as u8, (argb >> 24) as u8)
}

/// A caneta de `width` px na cor ARGB `c`, com as pontas e as junções redondas.
fn pen(c: u32, width: f32) -> aylis::prelude::Stroke {
    let mut p = aylis::prelude::Stroke::new(color(c), width);
    p.cap = LineCap::Round;
    p.join = LineJoin::Round;
    p
}

fn pt((x, y): (f32, f32)) -> PointF {
    PointF::new(x, y)
}

/// Desenha `m` em `canvas` (coordenadas do form).
pub fn draw(canvas: &mut dyn Canvas, m: &Mark) {
    match m {
        Mark::Arrow(a) => {
            let (s0, s1, head) = shape(a);
            if a.len() > head_size(a.width).0 {
                canvas.draw_line(pt(s0), pt(s1), &pen(a.color, a.width));
            }
            let mut path = Path::new();
            path.move_to(head[0].0, head[0].1).line_to(head[1].0, head[1].1).line_to(head[2].0, head[2].1).close();
            canvas.fill_path(&path, &Brush::Solid(color(a.color)));
        }
        Mark::Line(a) => canvas.draw_line(PointF::new(a.x1, a.y1), PointF::new(a.x2, a.y2), &pen(a.color, a.width)),
        Mark::Rect(f) => {
            let (l, t, w, h) = f.normalized();
            let mut p = aylis::prelude::Stroke::new(color(f.color), f.width);
            p.join = LineJoin::Miter;
            canvas.stroke_rect(RectF::new(l, t, w, h), 0.0, &p);
        }
        Mark::Stroke(s) => {
            let (w, c) = (s.drawn_width(), s.drawn_color());
            let dot = RectF::new(0.0, 0.0, w, w);
            // O marca-texto anda em trechos retos (horizontal, vertical, 45°).
            let points = if s.marker { marker_path(&s.points, w) } else { s.points.clone() };
            match points.as_slice() {
                [] => {}
                // Um ponto sozinho (um clique): no lápis, um pingo redondo; no marca-texto, nada
                // (ele pinta do clique para o lado do arrasto, e um clique não tem lado).
                [_] if s.marker => {}
                [(x, y)] => canvas.fill_ellipse(dot.offset(x - w / 2.0, y - w / 2.0), &Brush::Solid(color(c))),
                [first, rest @ ..] => {
                    // Num desenho só: o marca-texto translúcido não escurece onde se cruza.
                    let mut path = Path::new();
                    path.move_to(first.0, first.1);
                    for p in rest {
                        path.line_to(p.0, p.1);
                    }
                    let mut p = pen(c, w);
                    if s.marker {
                        // O marca-texto: as pontas retas, cortadas no ponto (começa onde o
                        // clique foi e termina no mouse; a quadrada passava meia largura dos
                        // dois lados), e a troca de direção arredondada por dentro (sem quina
                        // viva no meio da pintura).
                        p.cap = LineCap::Butt;
                        p.join = LineJoin::Round;
                    }
                    canvas.stroke_path(&path, &p);
                }
            }
        }
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

    /// O retângulo normaliza o arrasto (de baixo para cima, da direita para a esquerda dá no
    /// mesmo), cabe no retângulo de repintar e um risco reto não é retângulo.
    #[test]
    fn retangulo_normalizado_e_limites() {
        let f = Frame { x1: 1160.0, y1: 1090.0, x2: 1020.0, y2: 1020.0, width: 6.0, color: PALETTE[2] };
        assert_eq!(f.normalized(), (1020.0, 1020.0, 140.0, 70.0));
        let b = f.bounds();
        assert!(b.x <= 1017.0 && b.y <= 1017.0 && b.right() >= 1163.0 && b.bottom() >= 1093.0, "{b:?}");
        assert!(!Frame { y2: 1095.0, ..f }.is_tiny() && Frame { y2: 1088.0, ..f }.is_tiny(), "altura 5: retângulo; altura 2: risco");
    }

    /// O marca-texto é largo e translúcido; o lápis, na espessura e na cor escolhidas.
    #[test]
    fn marca_texto_largo_e_translucido() {
        let pen = Stroke { points: vec![(0.0, 0.0), (50.0, 0.0)], width: 4.0, color: PALETTE[1], marker: false };
        let marker = Stroke { marker: true, ..pen.clone() };
        assert_eq!((pen.drawn_width(), pen.drawn_color()), (4.0, PALETTE[1]));
        assert_eq!((marker.drawn_width(), marker.drawn_color() >> 24), (12.0, 0x80));
        assert!(marker.bounds().height > pen.bounds().height);
    }

    /// O marca-texto anda reto: a mão que treme numa linha quase horizontal dá uma linha
    /// horizontal; virando para baixo, um trecho vertical a partir do fim dela; a 45°, a
    /// diagonal. O tremor do começo (menos de meia largura) não decide a direção.
    #[test]
    fn marca_texto_em_trechos_retos() {
        let w = 12.0;
        let horizontal = marker_path(&[(0.0, 0.0), (2.0, 3.0), (40.0, 4.0), (80.0, -3.0), (120.0, 5.0)], w);
        assert_eq!(horizontal.len(), 2);
        assert_eq!(horizontal[0], (0.0, 0.0));
        assert!((horizontal[1].0 - 120.0).abs() < 0.01 && horizontal[1].1.abs() < 0.01, "{horizontal:?}");
        let turn = marker_path(&[(0.0, 0.0), (60.0, 2.0), (120.0, 1.0), (122.0, 40.0), (121.0, 90.0)], w);
        assert_eq!(turn.len(), 3, "{turn:?}");
        assert!((turn[1].0 - 120.0).abs() < 0.01 && turn[1].1.abs() < 0.01);
        assert!((turn[2].0 - 120.0).abs() < 0.01 && (turn[2].1 - 90.0).abs() < 0.01, "desce na vertical: {turn:?}");
        let diagonal = marker_path(&[(0.0, 0.0), (30.0, 28.0), (60.0, 62.0)], w);
        let (x, y) = diagonal[1];
        assert!((x - y).abs() < 0.01 && x > 55.0, "a 45°: {diagonal:?}");
        // Um traço na marca: o retângulo de repintar cobre a reta com a meia largura em volta.
        let s = Stroke { points: vec![(0.0, 0.0), (60.0, 2.0), (120.0, 1.0)], width: 4.0, color: COLOR, marker: true };
        let b = s.bounds();
        assert!(b.x <= -6.0 && b.right() >= 126.0, "{b:?}");
    }

    #[test]
    fn cores_argb_e_matiz() {
        assert_eq!(color(0x80FF_0000), Color::rgba(255, 0, 0, 128));
        assert_eq!(hsv(0.0, 1.0, 1.0), 0xFFFF_0000);
        assert_eq!(hsv(120.0, 1.0, 1.0), 0xFF00_FF00);
    }
}
