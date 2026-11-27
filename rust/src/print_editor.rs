//! O editor do print (Print Screen → escolher a área → anotar → copiar ou salvar), no form
//! `frmPrint` do Aylis: sem borda, sempre na frente, fora da barra de tarefas, cobrindo a
//! área de trabalho virtual inteira (todos os monitores) com 1 unidade = 1 pixel físico.
//!
//! * `imgShot`: a tela congelada no instante do atalho (o print), nítida;
//! * `pnlShotStage`: por cima dela, o escurecido fora da área, a borda tracejada, as alças, o
//!   rótulo do tamanho, as marcas (setas, linhas, retângulos, lápis e marca-texto) e a dica do
//!   botão sob o mouse; recebe o mouse (escolher a área, mexer nela, desenhar);
//! * `lyShotTexts`: os textos, cada um um `Memo` só texto (`plain`) que cresce com o que se
//!   digita (`auto_size`), recortados na área escolhida (a camada anda com ela). O Memo traz
//!   o cursor, a seleção, os atalhos (setas, Home, End, Ctrl+A/C/X/V/Z...) e o menu de texto;
//! * `pnlShotBar`: a barra (Lápis, Linha, Seta, Retângulo, Marca-texto, Texto | cor, − tamanho
//!   + | Desfazer | Copiar, Salvar, Fechar) e o seletor de cores, desenhados aqui.
//!
//! A imagem final é o pedaço do form na área escolhida (`Ui::render_rgba`), sem o escurecido,
//! a borda, as alças, a barra e o cursor dos textos: o mesmo desenho da tela.
//!
//! Teclado (o form vê antes, `key_preview`): Esc cancela (com o seletor aberto, fecha ele;
//! digitando, termina o texto); Enter salva (digitando, termina o texto; Shift+Enter, nova
//! linha); Ctrl+C copia; Ctrl+Z desfaz a última marca; Ctrl+B, Ctrl+I e Ctrl+S trocam o
//! negrito, o itálico e o sublinhado do texto. A roda do mouse (ou − e +): a espessura ou o
//! tamanho da fonte. A ferramenta, a cor, os tamanhos e o estilo ficam no config.

use std::cell::RefCell;

use aylis::prelude::*;

use crate::annotate as an;
use crate::engine::AreaRect;
use crate::generated::FrmPrint;
use crate::{config, lang, log, mainthread};

/// Menor área aceita, em px físicos: arrastar menos que isso é clique perdido.
/// Não é o mínimo da seleção da gravação (32 px, do encoder): o print de um ícone ou de
/// uma linha de texto é pequeno de propósito.
const MIN_SIZE: f32 = 4.0;
/// O escurecido fora da área (0..255): o mesmo véu da seleção de área da gravação.
const DIM_ALPHA: u8 = 150;

/// O que fazer com o print.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShotAction {
    /// Salvar (o PNG na pasta de gravações e a área de transferência).
    Save,
    /// Só a área de transferência, sem arquivo.
    Copy,
}

/// O print pronto: a área (coordenadas da tela) e a imagem dela com as marcas (RGBA).
#[derive(Debug, Clone)]
pub struct ShotDone {
    pub action: ShotAction,
    pub area: AreaRect,
    pub rgba: Vec<u8>,
}

/// `None` = cancelado.
pub type OnShot = fn(Option<ShotDone>);

/// A ferramenta: o lápis e o marca-texto seguem o mouse; a linha, a seta e o retângulo vão
/// do ponto inicial ao final; clicar com o texto começa um texto.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Tool {
    Pen,
    Line,
    #[default]
    Arrow,
    Rect,
    Marker,
    Text,
}

/// As ferramentas, na ordem da barra.
const TOOLS: [Tool; 6] = [Tool::Pen, Tool::Line, Tool::Arrow, Tool::Rect, Tool::Marker, Tool::Text];

impl Tool {
    fn freehand(self) -> bool {
        matches!(self, Tool::Pen | Tool::Marker)
    }

    fn icon(self) -> &'static str {
        match self {
            Tool::Pen => "shot-pen",
            Tool::Line => "shot-line",
            Tool::Arrow => "shot-arrow",
            Tool::Rect => "shot-rect",
            Tool::Marker => "shot-marker",
            Tool::Text => "shot-text",
        }
    }

    fn key(self) -> &'static str {
        match self {
            Tool::Pen => "pen",
            Tool::Line => "line",
            Tool::Arrow => "arrow",
            Tool::Rect => "rect",
            Tool::Marker => "marker",
            Tool::Text => "text",
        }
    }
}

/// Os ícones da barra (traço, grade 24).
const ICONS: [(&str, &str); 13] = [
    ("shot-pen", "M21.17 6.81a2.82 2.82 0 0 0-3.99-3.99L3.84 16.17a2 2 0 0 0-.5.83l-1.32 4.35a.5.5 0 0 0 .62.62l4.35-1.32a2 2 0 0 0 .83-.5z"),
    ("shot-line", "M5 19L19 5"),
    ("shot-arrow", "M5 19L19 5 M9 5h10v10"),
    ("shot-rect", "M5 4h14a1 1 0 0 1 1 1v14a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1V5a1 1 0 0 1 1-1z"),
    ("shot-marker", "M9 11l-6 6v3h9l3-3 M22 12l-4.6 4.6a2 2 0 0 1-2.8 0l-5.2-5.2a2 2 0 0 1 0-2.8L14 4"),
    ("shot-text", "M4 7V4h16v3 M9 20h6 M12 4v16"),
    ("shot-undo", "M9 14L4 9l5-5 M4 9h10.5a5.5 5.5 0 0 1 0 11H11"),
    ("shot-copy", "M10 8h10a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H10a2 2 0 0 1-2-2V10a2 2 0 0 1 2-2z M4 16a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2"),
    ("shot-save", "M5 3h11l5 5v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2z M17 21v-8H7v8 M7 3v5h8"),
    ("shot-close", "M18 6L6 18 M6 6l12 12"),
    ("shot-minus", "M5 12h14"),
    ("shot-plus", "M5 12h14 M12 5v14"),
    ("shot-caret", "M6 9l6 6 6-6"),
];

/// Mexendo na área escolhida: movê-la inteira ou puxar uma alça (os lados que andam).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Grab {
    Move,
    Edges { left: bool, top: bool, right: bool, bottom: bool },
}

/// A área `r` depois de arrastar `dx`, `dy` pegando em `g`: movida, presa dentro do form
/// (`w`×`h`); ou com os lados puxados, sem passar do lado oposto (fica com [`MIN_SIZE`]).
fn adjusted(r: RectF, g: Grab, dx: f32, dy: f32, w: f32, h: f32) -> RectF {
    match g {
        Grab::Move => {
            let x = (r.x + dx).clamp(0.0, (w - r.width).max(0.0));
            let y = (r.y + dy).clamp(0.0, (h - r.height).max(0.0));
            RectF::new(x, y, r.width, r.height)
        }
        Grab::Edges { left, top, right, bottom } => {
            let (mut l, mut t, mut rr, mut b) = (r.x, r.y, r.right(), r.bottom());
            if left {
                l = (r.x + dx).min(r.right() - MIN_SIZE).max(0.0);
            }
            if right {
                rr = (r.right() + dx).max(r.x + MIN_SIZE).min(w);
            }
            if top {
                t = (r.y + dy).min(r.bottom() - MIN_SIZE).max(0.0);
            }
            if bottom {
                b = (r.bottom() + dy).max(r.y + MIN_SIZE).min(h);
            }
            RectF::new(l, t, rr - l, b - t)
        }
    }
}

/// As 8 alças da área (cantos e meios dos lados): o centro e os lados que cada uma puxa.
fn handles(r: RectF) -> impl Iterator<Item = (PointF, Grab)> {
    let xs = [(r.x, true, false), ((r.x + r.right()) / 2.0, false, false), (r.right(), false, true)];
    let ys = [(r.y, true, false), ((r.y + r.bottom()) / 2.0, false, false), (r.bottom(), false, true)];
    ys.into_iter().flat_map(move |(y, top, bottom)| {
        xs.into_iter().filter_map(move |(x, left, right)| {
            (left || right || top || bottom).then_some((PointF::new(x, y), Grab::Edges { left, top, right, bottom }))
        })
    })
}

/// O cursor de quem puxa a alça `g`.
fn grab_cursor(g: Grab) -> &'static str {
    match g {
        Grab::Move => "move",
        Grab::Edges { left, top, right, bottom } => match (left || right, top || bottom) {
            (true, true) if (left && top) || (right && bottom) => "resize_nw_se",
            (true, true) => "resize_ne_sw",
            (true, false) => "resize_horizontal",
            _ => "resize_vertical",
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
    /// No seletor: uma cor de [`an::PALETTE`] (o índice)...
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

/// A barra montada: onde ela fica, os botões (com os do seletor de cores, se aberto), os
/// separadores entre os grupos (x) e o quadro do seletor. Coordenadas do form.
#[derive(Debug, Clone, Default)]
struct Bar {
    rect: RectF,
    buttons: Vec<(ShotButton, RectF)>,
    seps: Vec<f32>,
    picker: Option<RectF>,
}

impl Bar {
    fn button(&self, b: ShotButton) -> Option<RectF> {
        self.buttons.iter().find(|(k, _)| *k == b).map(|(_, r)| *r)
    }

    /// O que ela ocupa (a barra e o seletor): o lugar do controle da barra.
    fn area(&self) -> RectF {
        self.picker.map_or(self.rect, |p| self.rect.union(p))
    }
}

/// Um texto do print: o Memo e o que ele usa (a cor, o tamanho e o estilo).
#[derive(Debug, Clone, Copy, PartialEq)]
struct TextMark {
    memo: ComponentId,
    color: u32,
    size: f32,
    style: i32,
}

/// Uma marca na ordem em que foi feita: um desenho ou um texto.
#[derive(Debug, Clone, PartialEq)]
enum Item {
    Mark(an::Mark),
    Text(TextMark),
}

#[derive(Default)]
struct State {
    /// O editor aberto (do atalho até fechar; o form nasce na fila da interface).
    open: bool,
    on_shot: Option<OnShot>,
    form: Option<ComponentId>,
    stage: Option<ComponentId>,
    texts: Option<ComponentId>,
    bar_id: Option<ComponentId>,
    /// O canto da área de trabalho virtual (form → tela) e o tamanho do form.
    origin: (i32, i32),
    size: (f32, f32),
    /// Pixels físicos por pixel lógico do sistema (os tamanhos da barra, das alças e do rótulo).
    k: f32,
    /// Onde o arrasto da área começou.
    anchor: Option<PointF>,
    /// A área (form) e a de antes de um clique fora dela (sem arrasto, ela volta).
    sel: Option<RectF>,
    prev_sel: Option<RectF>,
    /// A área está escolhida (a barra à vista).
    editing: bool,
    items: Vec<Item>,
    /// A seta, a linha ou o retângulo sendo arrastado (do ponto inicial ao atual).
    drawing: Option<an::Arrow>,
    /// O traço livre em andamento (lápis ou marca-texto).
    stroke: Option<an::Stroke>,
    /// Mexendo na área: o que, onde o mouse apertou e a área nesse instante.
    adjust: Option<(Grab, PointF, RectF)>,
    tool: Option<Tool>,
    color: u32,
    width: f32,
    font_size: f32,
    text_style: i32,
    prefs: bool,
    /// O texto sendo digitado (o Memo com o foco).
    typing: Option<ComponentId>,
    /// Onde a camada dos textos está (a área quando ela foi posta).
    texts_at: (f32, f32),
    bar: Option<Bar>,
    picker: bool,
    hue_drag: bool,
    hover: Option<ShotButton>,
    cursor: &'static str,
    /// Montando a imagem final: só o print e as marcas.
    exporting: bool,
    /// O Esc desceu no editor: o print cancela quando ele sobe. Fechando na descida, a subida
    /// ia para o app de trás, que recebia um Esc.
    esc_down: bool,
}

thread_local! {
    static ST: RefCell<State> = RefCell::new(State::default());
}

fn with<R>(f: impl FnOnce(&mut State) -> R) -> R {
    ST.with(|s| f(&mut s.borrow_mut()))
}

#[cfg(test)]
thread_local! {
    static TEST_SHOT: std::cell::Cell<Option<OnShot>> = const { std::cell::Cell::new(None) };
}

/// O editor aberto agora?
pub fn is_active() -> bool {
    #[cfg(test)]
    if TEST_SHOT.with(|p| p.get().is_some()) {
        return true;
    }
    with(|s| s.open)
}

/// Teste: termina o editor aberto pelo atalho como se o usuário tivesse cancelado.
#[cfg(test)]
pub fn test_cancel() -> bool {
    match TEST_SHOT.with(|p| p.take()) {
        Some(cb) => {
            cb(None);
            true
        }
        None => false,
    }
}

/// Abre o editor sobre `rgba` (`w`×`h`, a área de trabalho virtual inteira, tirada no
/// instante do atalho). `false`: já há um aberto, ou a imagem não é do tamanho da tela (um
/// monitor entrou ou saiu no meio). O form nasce na fila da interface. Com `preset` (a
/// área do último print, em coordenadas de tela), o editor já abre com ela escolhida.
pub fn start(on_shot: OnShot, rgba: Vec<u8>, w: i32, h: i32, preset: Option<AreaRect>) -> bool {
    if is_active() || crate::area_select::is_active() {
        return false;
    }
    #[cfg(test)]
    {
        crate::testkit::record_effect("area_select:print".to_string());
        let _ = (rgba, w, h, preset);
        TEST_SHOT.with(|p| p.set(Some(on_shot)));
        return true;
    }
    #[allow(unreachable_code)]
    {
        let screen = crate::screenshot::virtual_screen();
        if (w, h) != (screen.w, screen.h) || rgba.len() != w as usize * h as usize * 4 {
            log!("Print: a imagem {w}x{h} nao bate com a tela {}x{}.", screen.w, screen.h);
            return false;
        }
        with(|s| {
            s.open = true;
            s.on_shot = Some(on_shot);
        });
        crate::app_view::with_ui(move |ui| {
            if let Err(e) = open(ui, rgba, w, h, (screen.x, screen.y), system_scale(), preset) {
                log!("Print: o editor nao abriu ({e}).");
                cancel(ui);
            }
        });
        true
    }
}

/// Pixels físicos por pixel lógico do sistema.
fn system_scale() -> f32 {
    // SAFETY: sem argumentos nem efeito colateral.
    let dpi = unsafe { windows::Win32::UI::HiDpi::GetDpiForSystem() };
    dpi.max(96) as f32 / 96.0
}

/// Monta o form do print, liga os eventos e mostra.
fn open(ui: &mut Ui, rgba: Vec<u8>, w: i32, h: i32, origin: (i32, i32), k: f32, preset: Option<AreaRect>) -> Result<ComponentId, String> {
    for (name, d) in ICONS {
        let _ = ui.register_icon_svg(name, d, "", 24.0);
    }
    let form = FrmPrint::create(ui).map_err(|e| e.to_string())?;
    let f = FrmPrint::components(ui).ok_or("os componentes do frmPrint")?;
    let (fw, fh) = (w as f32, h as f32);
    if let Some(cb) = ui.control_mut(form) {
        cb.set_size(fw, fh);
    }
    if let Some(v) = ui.get_mut::<ImageView>(f.imgShot.id) {
        v.set_picture(Some(aylis::prelude::Image::rgba(w as u32, h as u32, rgba)));
    }
    with(|s| {
        s.open = true;
        s.form = Some(form);
        s.stage = Some(f.pnlShotStage.id);
        s.texts = Some(f.lyShotTexts.id);
        s.bar_id = Some(f.pnlShotBar.id);
        s.origin = origin;
        s.size = (fw, fh);
        s.k = k;
        reset(s);
    });
    wire(ui, &f);
    let _ = ui.set_prop(f.pnlShotBar.id, "visible", false);
    // Sem a animação do Windows ao abrir: o `wsShot` do form (`animation` desligado) vale ao
    // criar a janela, antes de ela aparecer; o editor aparece na hora sobre a tela congelada.
    // Sem dona: com o `show`, o Aylis faz a janela do print ser possuída pela do form aberto
    // antes (a principal, escondida na bandeja), e o Windows mostra, minimiza e ativa as duas
    // juntas; depois do print a principal aparecia branca (hibernada, sem pintar).
    ui.show_owned(form, None).map_err(|e| e.to_string())?;
    ui.set_focus(f.pnlShotStage.id);
    // A área do último print: já escolhida, com a barra e as alças (dá para ajustar, desenhar
    // ou salvar direto). Só se cabe inteira no form.
    let full = RectF::new(0.0, 0.0, w as f32, h as f32);
    let preset = preset.map(|a| RectF::new((a.x - origin.0) as f32, (a.y - origin.1) as f32, a.w as f32, a.h as f32)).filter(|r| full.contains_rect(*r) && r.width >= MIN_SIZE && r.height >= MIN_SIZE);
    if let Some(r) = preset {
        with(|s| s.editing = true);
        set_selection(ui, Some(r));
        refresh_bar(ui);
        damage_stage(ui);
        log!("Print: abrindo na ultima area {}x{} em ({}, {}).", r.width, r.height, r.x, r.y);
    }
    log!("Print: editor aberto ({w}x{h} em {}, {}).", origin.0, origin.1);
    Ok(form)
}

/// Começo de um print: sem área nem marcas; a ferramenta, a cor e os tamanhos do último
/// (na primeira vez, os do config).
fn reset(s: &mut State) {
    s.anchor = None;
    s.sel = None;
    s.prev_sel = None;
    s.editing = false;
    s.items.clear();
    s.drawing = None;
    s.stroke = None;
    s.adjust = None;
    s.typing = None;
    s.texts_at = (0.0, 0.0);
    s.bar = None;
    s.picker = false;
    s.hue_drag = false;
    s.hover = None;
    s.cursor = "crosshair";
    s.exporting = false;
    s.esc_down = false;
    if !s.prefs {
        load_prefs(s);
    }
}

/// As escolhas do editor (a ferramenta, a cor, a espessura, o tamanho e o estilo do texto)
/// ficam no config: valem para o próximo print, o próximo início e depois de reinstalar.
fn load_prefs(s: &mut State) {
    let num = |key: &str, default: f32, min: f32, max: f32| config::get_config_str(key, "").trim().parse::<f32>().map_or(default, |v| v.clamp(min, max));
    s.width = num("shotWidth", an::WIDTH_DEFAULT, an::WIDTH_MIN, an::WIDTH_MAX);
    s.font_size = num("shotFontSize", an::SIZE_DEFAULT, an::SIZE_MIN, an::SIZE_MAX);
    s.text_style = config::get_config_int("shotTextStyle", 0) & 7;
    s.color = u32::from_str_radix(config::get_config_str("shotColor", "").trim(), 16).unwrap_or(an::COLOR);
    s.tool = match config::get_config_str("shotTool", "").as_str() {
        "none" => None,
        key => Some(TOOLS.into_iter().find(|t| t.key() == key).unwrap_or(Tool::Arrow)),
    };
    s.prefs = true;
}

/// Grava as escolhas do editor (ao fechar o print). Só o que mudou vai para o disco.
fn save_prefs(s: &State) {
    if !s.prefs {
        return;
    }
    let tool = s.tool.map_or("none", Tool::key);
    for (key, value) in [("shotWidth", s.width.to_string()), ("shotFontSize", s.font_size.to_string()), ("shotColor", format!("{:08X}", s.color)), ("shotTool", tool.to_string())] {
        if config::get_config_str(key, "") != value {
            config::set_config_str(key, &value);
        }
    }
    if config::get_config_int("shotTextStyle", 0) != s.text_style {
        config::set_config_int("shotTextStyle", s.text_style);
    }
}

// ---------------------------------------------------------------------------
// Fechar
// ---------------------------------------------------------------------------

/// Fecha o form (e solta o print, dezenas de MB) e devolve quem esperava o resultado.
fn close(ui: &mut Ui) -> Option<OnShot> {
    let (form, cb) = with(|s| {
        save_prefs(s);
        let out = (s.form.take(), s.on_shot.take());
        s.open = false;
        s.stage = None;
        s.texts = None;
        s.bar_id = None;
        s.typing = None;
        s.items.clear();
        s.bar = None;
        out
    });
    if let Some(form) = form {
        ui.close(form);
        // Sem trazer a janela principal para a frente: ao fechar um form, o Aylis pede para
        // ativar a dona dele ou o último form aberto (a principal, mesmo atrás de outras
        // janelas). Sem o pedido, o Windows devolve o foco à janela que estava na frente antes
        // do print.
        let _ = ui.take_activation();
    }
    cb
}

/// Cancelado (Esc, o botão direito, Fechar).
fn cancel(ui: &mut Ui) {
    log::log("Print: cancelado.");
    if let Some(cb) = close(ui) {
        mainthread::queue(move || cb(None));
    }
}

/// O print termina com `action`: a imagem é o pedaço do form na área (o print e as marcas,
/// sem a moldura, a barra e o cursor).
fn finish(ui: &mut Ui, action: ShotAction) {
    end_typing(ui);
    let (sel, origin, form, bar, n) = with(|s| (s.sel, s.origin, s.form, s.bar_id, s.items.len()));
    let (Some(sel), Some(form)) = (sel, form) else { return };
    with(|s| {
        s.exporting = true;
        s.hover = None;
    });
    if let Some(bar) = bar {
        let _ = ui.set_prop(bar, "visible", false);
    }
    ui.layout();
    let image = ui.render_rgba(form, sel, 1.0);
    with(|s| s.exporting = false);
    let area = AreaRect { x: sel.x.round() as i32 + origin.0, y: sel.y.round() as i32 + origin.1, w: sel.width.round() as i32, h: sel.height.round() as i32 };
    let cb = close(ui);
    let Some((w, h, rgba)) = image.filter(|(w, h, _)| *w as i32 == area.w && *h as i32 == area.h) else {
        log!("Print: a imagem da area {}x{} nao saiu do desenho.", area.w, area.h);
        crate::app_view::show_toast(&lang::tr("Erro"), &lang::tr("Não foi possível capturar a tela."), true, 0);
        if let Some(cb) = cb {
            mainthread::queue(move || cb(None));
        }
        return;
    };
    log!("Print: {action:?} {w}x{h} em ({}, {}), {n} marca(s).", area.x, area.y);
    if let Some(cb) = cb {
        mainthread::queue(move || cb(Some(ShotDone { action, area, rgba })));
    }
}

// ---------------------------------------------------------------------------
// Medidas
// ---------------------------------------------------------------------------

fn sx(k: f32, v: f32) -> f32 {
    (v * k).round()
}

fn handle_px(k: f32) -> f32 {
    sx(k, 8.0)
}

/// A alça sob `p` (com folga em volta, para pegar fácil).
fn handle_at(r: RectF, p: PointF, k: f32) -> Option<Grab> {
    let reach = sx(k, 7.0);
    handles(r).find(|(c, _)| (p.x - c.x).abs() <= reach && (p.y - c.y).abs() <= reach).map(|(_, g)| g)
}

/// Altura de uma linha de texto do tamanho `size` (a da Segoe UI).
fn line_height(size: f32) -> f32 {
    size * 1.33
}

/// O rótulo "1280 × 720": o texto e a faixa dele, acima da área (ou abaixo, sem espaço).
fn label(sel: RectF, k: f32) -> (String, RectF) {
    let text = format!("{} × {}", sel.width.round(), sel.height.round());
    let (w, h, gap) = (text.chars().count() as f32 * sx(k, 7.0) + 2.0 * sx(k, 7.0), sx(k, 20.0), sx(k, 6.0));
    let y = if sel.y - h - gap >= 0.0 { sel.y - h - gap } else { sel.bottom() + gap };
    (text, RectF::new(sel.x, y, w, h))
}

/// O tamanho que a roda (e o − e o +) muda agora: o da fonte (digitando ou com o Texto)?
fn sizing_text(s: &State) -> bool {
    s.typing.is_some() || s.tool == Some(Tool::Text)
}

fn text_of(s: &State, memo: ComponentId) -> Option<&TextMark> {
    s.items.iter().find_map(|i| match i {
        Item::Text(t) if t.memo == memo => Some(t),
        _ => None,
    })
}

fn text_mut(s: &mut State, memo: ComponentId) -> Option<&mut TextMark> {
    s.items.iter_mut().find_map(|i| match i {
        Item::Text(t) if t.memo == memo => Some(t),
        _ => None,
    })
}

/// O número entre o − e o +.
fn size_label(s: &State) -> String {
    let size = match s.typing.and_then(|m| text_of(s, m)) {
        Some(t) => t.size,
        None if s.tool == Some(Tool::Text) => s.font_size,
        None => s.width,
    };
    format!("{size:.0} px")
}

/// Monta a barra (com a área escolhida), colada embaixo da área (em cima, ou dentro, sem
/// espaço), alinhada à direita dela: [ferramentas] | [cor ▾  − 4 px +] | [Desfazer] |
/// [Copiar Salvar Fechar]. Com o seletor de cores aberto, ele vai embaixo da barra (ou em cima).
fn layout_bar(s: &State) -> Option<Bar> {
    let sel = s.sel.filter(|_| s.editing)?;
    let k = s.k;
    let (cw, ch) = s.size;
    let (h, gap, sep, margin) = (sx(k, 32.0), sx(k, 4.0), sx(k, 13.0), sx(k, 10.0));
    enum It {
        B(ShotButton, f32),
        Sep,
    }
    let mut items: Vec<It> = TOOLS.iter().map(|t| It::B(ShotButton::Tool(*t), h)).collect();
    items.extend([
        It::Sep,
        It::B(ShotButton::ColorMenu, h + sx(k, 12.0)),
        It::B(ShotButton::Smaller, sx(k, 24.0)),
        // Largura fixa (a do maior número): a barra não muda de tamanho com a roda.
        It::B(ShotButton::Size, sx(k, 46.0)),
        It::B(ShotButton::Bigger, sx(k, 24.0)),
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
                seps.push(x + (sep - gap) / 2.0);
                x += sep;
            }
        }
    }
    let (bar_w, bar_h) = (x, h + 2.0 * gap);
    let bx = (sel.right() - bar_w).clamp(0.0, (cw - bar_w).max(0.0));
    let mut by = sel.bottom() + margin;
    if by + bar_h > ch {
        by = sel.y - margin - bar_h;
    }
    if by < 0.0 {
        by = sel.bottom() - bar_h - margin;
    }
    let rect = RectF::new(bx, by, bar_w, bar_h);
    let mut buttons: Vec<(ShotButton, RectF)> = rel.into_iter().map(|(b, x, w)| (b, RectF::new(bx + x, by + gap, w, h))).collect();
    let seps = seps.into_iter().map(|x| bx + x).collect();
    let mut picker = None;
    if s.picker {
        // Duas linhas de seis cores e a faixa de matizes embaixo.
        let (sw, g, p, hue_h, cols) = (sx(k, 22.0), sx(k, 6.0), sx(k, 8.0), sx(k, 18.0), 6usize);
        let pw = 2.0 * p + cols as f32 * sw + (cols as f32 - 1.0) * g;
        let rows = an::PALETTE.len().div_ceil(cols) as f32;
        let ph = 2.0 * p + rows * (sw + g) + hue_h;
        let anchor = buttons.iter().find(|(b, _)| *b == ShotButton::ColorMenu).map_or(rect, |(_, r)| *r);
        let px = anchor.x.clamp(0.0, (cw - pw).max(0.0));
        let py = if rect.bottom() + g + ph <= ch { rect.bottom() + g } else { rect.y - g - ph };
        for i in 0..an::PALETTE.len() {
            let (row, col) = ((i / cols) as f32, (i % cols) as f32);
            buttons.push((ShotButton::Color(i), RectF::new(px + p + col * (sw + g), py + p + row * (sw + g), sw, sw)));
        }
        buttons.push((ShotButton::Hue, RectF::new(px + p, py + p + rows * (sw + g), pw - 2.0 * p, hue_h)));
        picker = Some(RectF::new(px, py, pw, ph));
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

/// A dica do botão sob o mouse: o texto e a faixa, centrada nele, em cima da barra (ou
/// embaixo, sem espaço). Com o seletor aberto, nenhuma (ele fica no caminho).
fn layout_tip(s: &State) -> Option<(String, RectF)> {
    let b = s.hover.filter(|_| !s.picker)?;
    let bar = s.bar.as_ref()?;
    let br = bar.button(b)?;
    let text = lang::tr(tip_key(s, b)?);
    let k = s.k;
    let (w, h, gap) = (text.chars().count() as f32 * sx(k, 7.0) + 2.0 * sx(k, 8.0), sx(k, 24.0), sx(k, 6.0));
    let x = (br.center().x - w / 2.0).clamp(0.0, (s.size.0 - w).max(0.0));
    let y = if bar.rect.y - gap - h >= 0.0 { bar.rect.y - gap - h } else { bar.rect.bottom() + gap };
    Some((text, RectF::new(x, y, w, h)))
}

/// A cor da faixa de matizes no x `x`.
fn hue_at(r: RectF, x: f32) -> u32 {
    let t = (x - r.x) / (r.width - 1.0).max(1.0);
    an::hsv(t.clamp(0.0, 1.0) * 360.0, 0.85, 0.95)
}

// ---------------------------------------------------------------------------
// Repintar
// ---------------------------------------------------------------------------

fn damage_stage(ui: &mut Ui) {
    if let Some(stage) = with(|s| s.stage) {
        ui.damage(stage);
    }
}

fn damage_stage_part(ui: &mut Ui, r: RectF) {
    if let Some(stage) = with(|s| s.stage) {
        ui.damage_part(stage, r);
    }
}

/// O que muda quando a área vai de `old` para `new`: as faixas das bordas das duas (o
/// contorno, as alças e o escurecido que entra ou sai) e os rótulos; o miolo comum fica.
fn selection_edges(old: Option<RectF>, new: Option<RectF>, k: f32) -> Vec<RectF> {
    let (a, b) = match (old, new) {
        (Some(a), Some(b)) => (a, b),
        (Some(r), None) | (None, Some(r)) => return vec![r.inflate(handle_px(k)), label(r, k).1.inflate(2.0)],
        (None, None) => return Vec::new(),
    };
    let step = (b.x - a.x).abs().max((b.y - a.y).abs()).max((b.right() - a.right()).abs()).max((b.bottom() - a.bottom()).abs());
    let band = handle_px(k) + 2.0 + step;
    let mut out = Vec::new();
    for r in [a, b] {
        let o = r.inflate(band);
        out.push(RectF::new(o.x, o.y, o.width, 2.0 * band));
        out.push(RectF::new(o.x, r.bottom() - band, o.width, 2.0 * band));
        out.push(RectF::new(o.x, o.y, 2.0 * band, o.height));
        out.push(RectF::new(r.right() - band, o.y, 2.0 * band, o.height));
        out.push(label(r, k).1.inflate(2.0));
    }
    out
}

/// Troca a área: repinta só o que mudou e leva a camada dos textos junto.
fn set_selection(ui: &mut Ui, new: Option<RectF>) {
    let (old, k) = with(|s| (std::mem::replace(&mut s.sel, new), s.k));
    if old == new {
        return;
    }
    for r in selection_edges(old, new, k) {
        damage_stage_part(ui, r);
    }
    place_texts(ui);
}

/// A camada dos textos fica sobre a área (os textos recortados nela); os Memos andam ao
/// contrário para ficar parados na tela.
fn place_texts(ui: &mut Ui) {
    let (layer, sel, at, memos) = with(|s| {
        let memos: Vec<ComponentId> = s.items.iter().filter_map(|i| if let Item::Text(t) = i { Some(t.memo) } else { None }).collect();
        (s.texts, s.sel, s.texts_at, memos)
    });
    let (Some(layer), Some(sel)) = (layer, sel) else { return };
    let (dx, dy) = (sel.x - at.0, sel.y - at.1);
    if let Some(cb) = ui.control_mut(layer) {
        cb.set_rect(sel.x, sel.y, sel.width, sel.height);
    }
    if dx != 0.0 || dy != 0.0 {
        for m in memos {
            if let Some(cb) = ui.control_mut(m) {
                let (l, t) = (cb.left(), cb.top());
                cb.set_position(l - dx, t - dy);
            }
        }
    }
    with(|s| s.texts_at = (sel.x, sel.y));
    ui.invalidate();
}

/// Remonta a barra e a dica pelo estado e repinta.
fn refresh_bar(ui: &mut Ui) {
    let (bar_id, old_tip, area) = with(|s| {
        let old_tip = layout_tip(s).map(|t| t.1);
        s.bar = layout_bar(s);
        if s.bar.is_none() {
            s.picker = false;
            s.hover = None;
        }
        (s.bar_id, old_tip, s.bar.as_ref().map(Bar::area))
    });
    let Some(bar_id) = bar_id else { return };
    match area {
        Some(r) => {
            if let Some(cb) = ui.control_mut(bar_id) {
                cb.set_rect(r.x, r.y, r.width, r.height);
            }
            let _ = ui.set_prop(bar_id, "visible", true);
            ui.invalidate();
        }
        None => {
            let _ = ui.set_prop(bar_id, "visible", false);
        }
    }
    ui.damage(bar_id);
    let new_tip = with(|s| layout_tip(s).map(|t| t.1));
    for r in old_tip.into_iter().chain(new_tip) {
        damage_stage_part(ui, r.inflate(2.0));
    }
}

fn set_hover(ui: &mut Ui, b: Option<ShotButton>) {
    if with(|s| std::mem::replace(&mut s.hover, b)) != b {
        // A dica de antes some: o retângulo dela é o do botão antigo.
        damage_stage(ui);
        refresh_bar(ui);
    }
}

fn set_cursor(ui: &mut Ui, cursor: &'static str) {
    let (stage, changed) = with(|s| (s.stage, std::mem::replace(&mut s.cursor, cursor) != cursor));
    if let (Some(stage), true) = (stage, changed) {
        let _ = ui.set_prop(stage, "cursor", cursor);
    }
}

// ---------------------------------------------------------------------------
// Desenho
// ---------------------------------------------------------------------------

const BAR_BG: u32 = 0xFF1C_1D1F;
const BAR_BORDER: u32 = 0xFF3A_3B3E;
const HOVER_BG: u32 = 0xFF34_3539;
const SEP: u32 = 0xFF45_4649;
const GREEN: u32 = 0xFF10_B981;
const GREEN_HOVER: u32 = 0xFF34_D399;
const GREEN_SOFT: u32 = 0xFF0F_3B2F;
const WHITE: u32 = 0xFFFF_FFFF;
const ICON: u32 = 0xFFE8_E8EA;

fn solid(c: u32) -> Brush {
    Brush::Solid(an::color(c))
}

fn text_style(size: f32, color: u32, align: TextAlign) -> TextStyle {
    TextStyle {
        font: FontSpec { family: "Segoe UI".into(), size, weight: FontWeight::SemiBold, ..Default::default() },
        color: an::color(color),
        h_align: align,
        v_align: TextAlign::Center,
        ..Default::default()
    }
}

/// Um retângulo arredondado com borda de 1 px.
fn round_box(canvas: &mut dyn Canvas, r: RectF, radius: f32, fill: u32, border: Option<u32>) {
    canvas.fill_rect(r, radius, &solid(fill));
    if let Some(b) = border {
        canvas.stroke_rect(r.inflate(-0.5), radius, &aylis::prelude::Stroke::new(an::color(b), 1.0));
    }
}

/// O palco: o escurecido fora da área, as marcas dentro dela, a borda, as alças, o rótulo
/// e a dica.
fn paint_stage(canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
    ST.with(|st| {
        let Ok(s) = st.try_borrow() else { return };
        let full = RectF::new(0.0, 0.0, ctx.rect.width, ctx.rect.height);
        let dim = Brush::Solid(Color::rgba(0, 0, 0, DIM_ALPHA));
        let Some(r) = s.sel else {
            canvas.fill_rect(full, 0.0, &dim);
            return;
        };
        if !s.exporting {
            canvas.fill_rect(RectF::new(0.0, 0.0, full.width, r.y), 0.0, &dim);
            canvas.fill_rect(RectF::new(0.0, r.bottom(), full.width, full.height - r.bottom()), 0.0, &dim);
            canvas.fill_rect(RectF::new(0.0, r.y, r.x, r.height), 0.0, &dim);
            canvas.fill_rect(RectF::new(r.right(), r.y, full.width - r.right(), r.height), 0.0, &dim);
        }
        // As marcas só dentro da área (o que vai para o print).
        canvas.save();
        canvas.clip_rect(r);
        for item in &s.items {
            if let Item::Mark(m) = item {
                an::draw(canvas, m);
            }
        }
        if let Some(a) = s.drawing {
            an::draw(canvas, &drag_mark(a, s.tool));
        }
        if let Some(k) = &s.stroke {
            an::draw(canvas, &an::Mark::Stroke(k.clone()));
        }
        canvas.restore();
        if s.exporting {
            return;
        }
        let k = s.k;
        // A borda tracejada por fora da área: um fio escuro com traços brancos.
        let edge = r.inflate(0.5);
        canvas.stroke_rect(edge, 0.0, &aylis::prelude::Stroke::new(Color::rgb(0x20, 0x20, 0x20), 1.0));
        let dash = sx(k, 4.0).max(2.0);
        canvas.stroke_rect(edge, 0.0, &aylis::prelude::Stroke::new(Color::WHITE, 1.0).dashed(vec![dash, dash]));
        // As alças (a área já escolhida): puxam os lados.
        if s.editing {
            let hp = handle_px(k);
            for (c, _) in handles(r) {
                let h = RectF::new(c.x - hp / 2.0, c.y - hp / 2.0, hp, hp);
                canvas.fill_rect(h, 0.0, &Brush::Solid(Color::rgb(0x20, 0x20, 0x20)));
                canvas.fill_rect(h.inflate(-1.0), 0.0, &Brush::Solid(Color::WHITE));
            }
        }
        // O tamanho, em px físicos (o do print).
        let (text, lr) = label(r, k);
        let style = text_style(sx(k, 12.0), 0xFFE6_E6E6, TextAlign::Center);
        let tw = canvas.measure_text(&text, &style).width;
        let lr = RectF::new(lr.x, lr.y, tw + 2.0 * sx(k, 7.0), lr.height);
        round_box(canvas, lr, sx(k, 5.0), 0xD914_1517, Some(0xFF2E_2F33));
        canvas.draw_text(&text, lr, &style);
        // A dica do botão sob o mouse.
        if let Some((text, tr)) = layout_tip(&s) {
            let style = text_style(sx(k, 13.0), WHITE, TextAlign::Center);
            let tw = canvas.measure_text(&text, &style).width;
            let tr = RectF::new(tr.center().x - (tw / 2.0 + sx(k, 8.0)), tr.y, tw + 2.0 * sx(k, 8.0), tr.height);
            let tr = RectF::new(tr.x.clamp(0.0, (full.width - tr.width).max(0.0)), tr.y, tr.width, tr.height);
            round_box(canvas, tr, sx(k, 7.0), 0xF012_1214, Some(BAR_BORDER));
            canvas.draw_text(&text, tr, &style);
        }
    });
}

/// A barra e o seletor de cores (o controle da barra cobre os dois).
fn paint_bar(canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
    ST.with(|st| {
        let Ok(s) = st.try_borrow() else { return };
        let Some(bar) = &s.bar else { return };
        let area = bar.area();
        let k = s.k;
        canvas.save();
        canvas.translate(-area.x, -area.y);
        let (radius, inner) = (sx(k, 10.0), sx(k, 7.0));
        round_box(canvas, bar.rect, radius, BAR_BG, Some(BAR_BORDER));
        if let Some(pr) = bar.picker {
            round_box(canvas, pr, radius, BAR_BG, Some(BAR_BORDER));
        }
        let inset = sx(k, 9.0);
        for x in &bar.seps {
            canvas.fill_rect(RectF::new(*x, bar.rect.y + inset, sx(k, 1.0).max(1.0), bar.rect.height - 2.0 * inset), 0.0, &solid(SEP));
        }
        let size = size_label(&s);
        for (b, r) in &bar.buttons {
            let hov = s.hover == Some(*b);
            let bg = match *b {
                ShotButton::Tool(t) if s.tool == Some(t) => Some(GREEN_SOFT),
                ShotButton::Save => Some(if hov { GREEN_HOVER } else { GREEN }),
                ShotButton::ColorMenu if s.picker => Some(HOVER_BG),
                ShotButton::Color(_) | ShotButton::Hue | ShotButton::Size => None,
                _ if hov => Some(HOVER_BG),
                _ => None,
            };
            if let Some(bg) = bg {
                canvas.fill_rect(*r, inner, &solid(bg));
            }
            let icon = match *b {
                ShotButton::Tool(t) => Some((t.icon(), if s.tool == Some(t) { GREEN_HOVER } else { ICON })),
                ShotButton::Undo => Some(("shot-undo", ICON)),
                ShotButton::Copy => Some(("shot-copy", ICON)),
                ShotButton::Save => Some(("shot-save", WHITE)),
                ShotButton::Cancel => Some(("shot-close", ICON)),
                ShotButton::Smaller => Some(("shot-minus", ICON)),
                ShotButton::Bigger => Some(("shot-plus", ICON)),
                _ => None,
            };
            if let Some((name, c)) = icon {
                let side = sx(k, 18.0);
                let c0 = r.center();
                ctx.draw_icon(canvas, name, RectF::new(c0.x - side / 2.0, c0.y - side / 2.0, side, side), an::color(c));
            }
            match *b {
                ShotButton::ColorMenu => {
                    // A cor atual numa amostra, e o ▾ do seletor.
                    let m = sx(k, 8.0);
                    let side = r.height - 2.0 * m;
                    let sw = RectF::new(r.x + m, r.y + m, side, side);
                    canvas.fill_rect(sw.inflate(1.0), 0.0, &solid(0xFFA0_A0A0));
                    canvas.fill_rect(sw, 0.0, &solid(s.color | 0xFF00_0000));
                    let cs = sx(k, 12.0);
                    let cx = (sw.right() + r.right()) / 2.0;
                    ctx.draw_icon(canvas, "shot-caret", RectF::new(cx - cs / 2.0, r.center().y - cs / 2.0, cs, cs), an::color(0xFFDD_DDDD));
                }
                ShotButton::Size => canvas.draw_text(&size, *r, &text_style(sx(k, 13.0), 0xFFDD_DDDD, TextAlign::Center)),
                ShotButton::Color(i) => {
                    // A escolhida com moldura branca; as outras, um fio cinza.
                    let on = an::PALETTE[i] == s.color;
                    let ring = if on || hov { sx(k, 2.0) } else { 1.0 };
                    canvas.fill_rect(r.inflate(ring), 0.0, &solid(if on { WHITE } else if hov { 0xFFA0_A0A0 } else { 0xFF60_6060 }));
                    canvas.fill_rect(*r, 0.0, &solid(an::PALETTE[i]));
                }
                ShotButton::Hue => {
                    let stops = (0..=6).map(|i| GradientStop { offset: i as f32 / 6.0, color: an::color(an::hsv(i as f32 * 60.0, 0.85, 0.95)) }).collect();
                    canvas.fill_rect(r.inflate(1.0), 0.0, &solid(0xFF60_6060));
                    canvas.fill_rect(*r, 0.0, &Brush::Linear { start: PointF::new(r.x, r.y), end: PointF::new(r.right(), r.y), stops });
                }
                _ => {}
            }
        }
        canvas.restore();
    });
}

/// O arrasto `a` como marca da ferramenta `tool`.
fn drag_mark(a: an::Arrow, tool: Option<Tool>) -> an::Mark {
    match tool {
        Some(Tool::Rect) => an::Mark::Rect(an::Frame::from_drag(&a)),
        Some(Tool::Line) => an::Mark::Line(a),
        _ => an::Mark::Arrow(a),
    }
}

// ---------------------------------------------------------------------------
// Textos (um Memo cada)
// ---------------------------------------------------------------------------

/// A cor, o tamanho e o estilo no Memo.
fn style_memo(ui: &mut Ui, t: &TextMark) {
    let m = t.memo;
    let _ = ui.set_prop(m, "font_family", "Segoe UI");
    let _ = ui.set_prop(m, "font_size", f64::from(t.size));
    let _ = ui.set_prop(m, "text_color", format!("#{:06X}", t.color & 0x00FF_FFFF));
    let _ = ui.set_prop(m, "font_weight", if t.style & an::STYLE_BOLD != 0 { "bold" } else { "normal" });
    let _ = ui.set_prop(m, "font_italic", t.style & an::STYLE_ITALIC != 0);
    let _ = ui.set_prop(m, "font_underline", t.style & an::STYLE_UNDERLINE != 0);
}

/// Um texto novo em `p` (a linha centrada no clique), já com o foco.
fn new_text(ui: &mut Ui, p: PointF) {
    let (layer, at, color, size, style) = with(|s| (s.texts, s.texts_at, s.color, s.font_size, s.text_style));
    let Some(layer) = layer else { return };
    let memo = match ui.create("Memo", Some(layer)) {
        Ok(m) => m,
        Err(e) => {
            log!("Print: o texto nao nasceu ({e}).");
            return;
        }
    };
    for (prop, on) in [("plain", true), ("spellcheck", false), ("wrap", false), ("autocomplete", false)] {
        let _ = ui.set_prop(memo, prop, on);
    }
    if let Some(cb) = ui.control_mut(memo) {
        cb.set_position(p.x - at.0, p.y - line_height(size) / 2.0 - at.1).set_auto_size(true);
    }
    let t = TextMark { memo, color, size, style };
    style_memo(ui, &t);
    wire_memo(ui, memo);
    with(|s| {
        s.items.push(Item::Text(t));
        s.typing = Some(memo);
    });
    ui.invalidate();
    ui.set_focus(memo);
    refresh_bar(ui);
}

/// O texto de um Memo.
fn memo_text(ui: &Ui, memo: ComponentId) -> String {
    match ui.get_prop(memo, "text") {
        Ok(PropValue::String(s)) => s,
        _ => String::new(),
    }
}

/// O Memo ganhou o foco (clicado, ou recém-criado): é o texto em edição, e a cor, o tamanho
/// e o estilo dele passam a ser os da barra.
fn text_entered(ui: &mut Ui, memo: ComponentId) {
    let known = with(|s| {
        let Some(t) = text_of(s, memo).copied() else { return false };
        s.typing = Some(memo);
        s.tool = Some(Tool::Text);
        s.color = t.color;
        s.font_size = t.size;
        s.text_style = t.style;
        true
    });
    if known {
        refresh_bar(ui);
    }
}

/// Termina o texto `memo`: sem a seleção; vazio, ele sai.
fn finish_text(ui: &mut Ui, memo: ComponentId) {
    let _ = ui.set_prop(memo, "sel_length", 0i64);
    let empty = memo_text(ui, memo).trim().is_empty();
    with(|s| {
        if s.typing == Some(memo) {
            s.typing = None;
        }
        if empty {
            s.items.retain(|i| !matches!(i, Item::Text(t) if t.memo == memo));
        }
    });
    if empty && ui.contains(memo) {
        ui.free(memo);
        ui.invalidate();
    }
}

/// Termina o texto sendo digitado (o foco volta ao palco).
fn end_typing(ui: &mut Ui) {
    let (memo, stage) = with(|s| (s.typing, s.stage));
    if let Some(memo) = memo {
        finish_text(ui, memo);
    }
    if let Some(stage) = stage {
        ui.set_focus(stage);
    }
    refresh_bar(ui);
}

fn wire_memo(ui: &mut Ui, memo: ComponentId) {
    ui.on(memo, "on_enter", move |ui, _, _| text_entered(ui, memo));
    ui.on(memo, "on_exit", move |ui, _, _| {
        if with(|s| s.open && s.typing == Some(memo)) {
            finish_text(ui, memo);
            refresh_bar(ui);
        }
    });
    ui.on(memo, "on_wheel", |ui, _, args| {
        if let EventArgs::Wheel(e) = args {
            wheel_steps(ui, wheel_notches(e));
        }
    });
}

// ---------------------------------------------------------------------------
// Ações
// ---------------------------------------------------------------------------

/// A cor das próximas marcas, e a do texto sendo digitado, que continua.
fn set_color(ui: &mut Ui, c: u32) {
    let text = with(|s| {
        s.color = c;
        let m = s.typing?;
        let t = text_mut(s, m)?;
        t.color = c;
        Some(*t)
    });
    if let Some(t) = text {
        style_memo(ui, &t);
    }
    refresh_bar(ui);
}

/// Ctrl+B, Ctrl+I e Ctrl+S: liga e desliga o negrito, o itálico e o sublinhado do texto em
/// edição e dos próximos.
fn toggle_text_style(ui: &mut Ui, bit: i32) {
    let text = with(|s| {
        s.text_style ^= bit;
        let style = s.text_style;
        let m = s.typing?;
        let t = text_mut(s, m)?;
        t.style = style;
        Some(*t)
    });
    if let Some(t) = text {
        style_memo(ui, &t);
    }
    refresh_bar(ui);
}

/// Desfaz a última marca (o texto em edição termina antes).
fn undo(ui: &mut Ui) {
    end_typing(ui);
    match with(|s| s.items.pop()) {
        Some(Item::Mark(m)) => damage_stage_part(ui, m.bounds()),
        Some(Item::Text(t)) => {
            if ui.contains(t.memo) {
                ui.free(t.memo);
                ui.invalidate();
            }
        }
        None => {}
    }
    refresh_bar(ui);
}

/// Os dentes da roda (o touchpad manda frações: conta 1).
fn wheel_notches(e: &WheelEvent) -> i32 {
    let d = e.delta.y / WHEEL_LINE;
    if d.abs() >= 1.0 {
        d.round() as i32
    } else if e.delta.y == 0.0 {
        0
    } else {
        e.delta.y.signum() as i32
    }
}

/// A roda do mouse (e o − e o +): no texto sendo digitado (ou com a ferramenta Texto), o
/// tamanho da fonte, 2 px por dente; nas outras, a espessura, 1 px por dente (a marca sendo
/// arrastada acompanha).
fn wheel_steps(ui: &mut Ui, steps: i32) {
    if steps == 0 {
        return;
    }
    let text = with(|s| {
        if let Some(t) = s.typing.and_then(|m| text_mut(s, m)) {
            t.size = (t.size + steps as f32 * an::SIZE_STEP).clamp(an::SIZE_MIN, an::SIZE_MAX);
            let t = *t;
            s.font_size = t.size;
            Some(Some(t))
        } else if s.tool == Some(Tool::Text) && s.drawing.is_none() && s.stroke.is_none() {
            s.font_size = (s.font_size + steps as f32 * an::SIZE_STEP).clamp(an::SIZE_MIN, an::SIZE_MAX);
            Some(None)
        } else {
            None
        }
    });
    match text {
        Some(Some(t)) => style_memo(ui, &t),
        Some(None) => {}
        None => {
            with(|s| {
                s.width = (s.width + steps as f32).clamp(an::WIDTH_MIN, an::WIDTH_MAX);
                let w = s.width;
                if let Some(d) = s.drawing.as_mut() {
                    d.width = w;
                }
                if let Some(k) = s.stroke.as_mut() {
                    k.width = w;
                }
            });
            damage_stage(ui);
        }
    }
    refresh_bar(ui);
}

fn run_button(ui: &mut Ui, b: ShotButton, p: PointF) {
    match b {
        ShotButton::Cancel => cancel(ui),
        ShotButton::Copy => finish(ui, ShotAction::Copy),
        ShotButton::Save => finish(ui, ShotAction::Save),
        ShotButton::Tool(t) => {
            // A ativa de novo: solta (sem ferramenta, arrastar dentro move a área).
            end_typing(ui);
            with(|s| s.tool = if s.tool == Some(t) { None } else { Some(t) });
            refresh_bar(ui);
        }
        ShotButton::Undo => undo(ui),
        ShotButton::ColorMenu => {
            with(|s| s.picker = !s.picker);
            refresh_bar(ui);
        }
        ShotButton::Color(i) => {
            with(|s| s.picker = false);
            set_color(ui, an::PALETTE[i]);
        }
        ShotButton::Hue => {
            // Arrastar na faixa escolhe a matiz; soltar fecha o seletor.
            let Some(r) = with(|s| s.bar.as_ref().and_then(|b| b.button(ShotButton::Hue))) else { return };
            with(|s| s.hue_drag = true);
            set_color(ui, hue_at(r, p.x));
        }
        ShotButton::Smaller => wheel_steps(ui, -1),
        ShotButton::Bigger => wheel_steps(ui, 1),
        ShotButton::Size => {}
    }
}

/// O botão da barra (ou do seletor aberto) sob `p` (form).
fn button_at(p: PointF) -> Option<ShotButton> {
    with(|s| s.bar.as_ref().and_then(|b| b.buttons.iter().find(|(_, r)| r.contains(p)).map(|(b, _)| *b)))
}

// ---------------------------------------------------------------------------
// Mouse e teclado
// ---------------------------------------------------------------------------

/// Apertou no palco (fora da barra e dos textos).
fn stage_down(ui: &mut Ui, e: &PointerEvent) {
    if e.button != PointerButton::Primary {
        return;
    }
    let p = e.position;
    let busy = with(|s| s.anchor.is_some() || s.drawing.is_some() || s.stroke.is_some() || s.adjust.is_some() || s.hue_drag);
    if busy {
        return;
    }
    // Seletor de cores aberto: o clique fora dele só fecha.
    if with(|s| std::mem::replace(&mut s.picker, false)) {
        refresh_bar(ui);
        return;
    }
    let (editing, sel, width, color, tool, k) = with(|s| (s.editing, s.sel, s.width, s.color, s.tool, s.k));
    // Uma alça (com qualquer ferramenta) puxa os lados; dentro, sem ferramenta, arrastar
    // move a área inteira.
    let grab = sel.filter(|_| editing).and_then(|r| handle_at(r, p, k).or_else(|| (tool.is_none() && r.contains(p)).then_some(Grab::Move)).map(|g| (g, r)));
    if let Some((g, r)) = grab {
        end_typing(ui);
        with(|s| s.adjust = Some((g, p, r)));
        return;
    }
    if editing && sel.is_some_and(|r| r.contains(p)) {
        let (x, y) = (p.x, p.y);
        match tool {
            Some(Tool::Text) => {
                end_typing(ui);
                new_text(ui, p);
                return;
            }
            // O lápis e o marca-texto: o traço segue o mouse até soltar.
            Some(t) if t.freehand() => {
                end_typing(ui);
                with(|s| s.stroke = Some(an::Stroke { points: vec![(x, y)], width, color, marker: t == Tool::Marker }));
            }
            // A linha, a seta e o retângulo: do ponto inicial ao final.
            _ => {
                end_typing(ui);
                with(|s| s.drawing = Some(an::Arrow { x1: x, y1: y, x2: x, y2: y, width, color }));
            }
        }
        damage_stage(ui);
        return;
    }
    // Fora: outra área (a de antes volta se for só um clique). A barra sai.
    end_typing(ui);
    with(|s| {
        s.prev_sel = s.sel;
        s.editing = false;
        s.anchor = Some(p);
    });
    refresh_bar(ui);
    damage_stage(ui);
    set_selection(ui, Some(RectF::new(p.x, p.y, 0.0, 0.0)));
}

/// O cursor no ponto `p` (sem arrasto).
fn cursor_at(p: PointF) -> &'static str {
    with(|s| {
        let Some(r) = s.sel.filter(|_| s.editing) else { return "crosshair" };
        if let Some(g) = handle_at(r, p, s.k) {
            return grab_cursor(g);
        }
        match (r.contains(p), s.tool) {
            (true, None) => "move",
            (true, Some(Tool::Text)) => "text",
            _ => "crosshair",
        }
    })
}

fn stage_move(ui: &mut Ui, e: &PointerEvent) {
    let p = e.position;
    // Movendo a área ou puxando uma alça: a barra vai junto.
    if let Some((g, from, r0)) = with(|s| s.adjust) {
        let (w, h) = with(|s| s.size);
        set_selection(ui, Some(adjusted(r0, g, p.x - from.x, p.y - from.y, w, h)));
        refresh_bar(ui);
        return;
    }
    // O traço livre: um ponto a cada passo do mouse; repinta só o pedaço novo.
    let piece = with(|s| {
        let k = s.stroke.as_mut()?;
        let &(lx, ly) = k.points.last()?;
        if (p.x - lx).abs() < 1.0 && (p.y - ly).abs() < 1.0 {
            return Some(None);
        }
        // O marca-texto anda em trechos retos: o fim do trecho atual muda com o mouse, então
        // repinta o traço de antes e o de agora.
        let before = k.marker.then(|| k.bounds());
        k.points.push((p.x, p.y));
        Some(Some(match before {
            Some(b) => b.union(k.bounds()),
            None => an::points_bounds(&[(lx, ly), (p.x, p.y)], k.drawn_width()),
        }))
    });
    if let Some(piece) = piece {
        if let Some(r) = piece {
            damage_stage_part(ui, r);
        }
        return;
    }
    if let Some((old, new)) = with(|s| {
        let tool = s.tool;
        let a = s.drawing.as_mut()?;
        let old = drag_mark(*a, tool).bounds();
        (a.x2, a.y2) = (p.x, p.y);
        Some((old, drag_mark(*a, tool).bounds()))
    }) {
        damage_stage_part(ui, old);
        damage_stage_part(ui, new);
        return;
    }
    if let Some(a) = with(|s| s.anchor) {
        set_selection(ui, Some(RectF::from_points(a, p)));
        return;
    }
    set_cursor(ui, cursor_at(p));
}

fn stage_up(ui: &mut Ui, e: &PointerEvent) {
    if e.button == PointerButton::Secondary {
        cancel(ui);
        return;
    }
    let p = e.position;
    if with(|s| s.adjust.take()).is_some() {
        refresh_bar(ui);
        return;
    }
    // O traço livre entra como está (um clique sem arrasto vira um pingo no lápis; no
    // marca-texto, que só pinta para o lado do arrasto, não vira nada).
    if let Some(k) = with(|s| s.stroke.take()) {
        if k.marker && k.points.len() < 2 {
            damage_stage_part(ui, k.bounds());
            return;
        }
        let area = k.bounds();
        with(|s| s.items.push(Item::Mark(an::Mark::Stroke(k))));
        damage_stage_part(ui, area);
        return;
    }
    if let Some(mut a) = with(|s| s.drawing.take()) {
        (a.x2, a.y2) = (p.x, p.y);
        let mark = drag_mark(a, with(|s| s.tool));
        let tiny = match &mark {
            an::Mark::Rect(f) => f.is_tiny(),
            _ => a.is_tiny(),
        };
        let area = mark.bounds();
        if !tiny {
            with(|s| s.items.push(Item::Mark(mark)));
        }
        damage_stage_part(ui, area);
        return;
    }
    let Some(a) = with(|s| s.anchor.take()) else { return };
    let r = RectF::from_points(a, p);
    if r.width >= MIN_SIZE && r.height >= MIN_SIZE {
        // A área do print fica, com a barra; as marcas vêm depois.
        let r = RectF::new(r.x.round(), r.y.round(), r.width.round(), r.height.round());
        with(|s| {
            s.editing = true;
            s.prev_sel = None;
        });
        set_selection(ui, Some(r));
        log!("Print: area {}x{} em ({}, {}).", r.width, r.height, r.x, r.y);
    } else {
        // Clique sem arrasto: a de antes volta (se havia).
        let prev = with(|s| s.prev_sel.take());
        set_selection(ui, prev);
        with(|s| s.editing = prev.is_some());
    }
    damage_stage(ui);
    refresh_bar(ui);
    set_cursor(ui, cursor_at(p));
}

/// A barra: `p` local → form.
fn bar_point(p: PointF) -> PointF {
    with(|s| s.bar.as_ref().map_or(p, |b| {
        let a = b.area();
        PointF::new(p.x + a.x, p.y + a.y)
    }))
}

fn bar_down(ui: &mut Ui, e: &PointerEvent) {
    if e.button != PointerButton::Primary {
        return;
    }
    let p = bar_point(e.position);
    let Some(b) = button_at(p) else { return };
    // Seletor aberto e um botão da barra: fecha ele e o botão funciona (menos o da cor,
    // que era o que fechava).
    let picker = with(|s| s.picker);
    if picker && !matches!(b, ShotButton::Color(_) | ShotButton::Hue) {
        with(|s| s.picker = false);
        refresh_bar(ui);
        if b == ShotButton::ColorMenu {
            return;
        }
    }
    run_button(ui, b, p);
}

fn bar_move(ui: &mut Ui, e: &PointerEvent) {
    let p = bar_point(e.position);
    if with(|s| s.hue_drag) {
        if let Some(r) = with(|s| s.bar.as_ref().and_then(|b| b.button(ShotButton::Hue))) {
            set_color(ui, hue_at(r, p.x));
        }
        return;
    }
    let b = button_at(p);
    if let Some(bar) = with(|s| s.bar_id) {
        let _ = ui.set_prop(bar, "cursor", if b.is_some_and(|b| b != ShotButton::Size) { "pointer" } else { "default" });
    }
    set_hover(ui, b);
}

fn bar_up(ui: &mut Ui, _e: &PointerEvent) {
    if with(|s| std::mem::replace(&mut s.hue_drag, false)) {
        with(|s| s.picker = false);
        refresh_bar(ui);
    }
}

/// O teclado do form (antes do controle com o foco). Engole o que é do editor; o resto
/// vai ao Memo em edição.
fn key_down(ui: &mut Ui, k: &KeyEvent) {
    let (ctrl, shift) = (k.modifiers.ctrl, k.modifiers.shift);
    let (editing, typing, picker, tool) = with(|s| (s.editing, s.typing.is_some(), s.picker, s.tool));
    let style_bit = match k.key {
        Key::Char('b') => Some(an::STYLE_BOLD),
        Key::Char('i') => Some(an::STYLE_ITALIC),
        Key::Char('s') => Some(an::STYLE_UNDERLINE),
        _ => None,
    };
    match k.key {
        // Com o seletor de cores aberto, o Esc só fecha ele.
        Key::Escape if picker => {
            with(|s| s.picker = false);
            refresh_bar(ui);
        }
        // Digitando, o Esc e o Enter só terminam o texto (Shift+Enter: nova linha, no Memo).
        Key::Escape if typing => end_typing(ui),
        Key::Enter if typing && !shift => end_typing(ui),
        _ if ctrl && style_bit.is_some() && (typing || tool == Some(Tool::Text)) => toggle_text_style(ui, style_bit.unwrap_or(0)),
        // O resto do teclado é do texto (as setas, Home, End, Ctrl+A/C/X/V/Z...).
        _ if typing => return,
        Key::Escape => with(|s| s.esc_down = true),
        Key::Enter if editing => finish(ui, ShotAction::Save),
        Key::Char('c') if ctrl && editing => finish(ui, ShotAction::Copy),
        Key::Char('z') if ctrl => undo(ui),
        _ => return,
    }
    ui.suppress_key();
}

/// Liga os eventos do form recém-criado.
fn wire(ui: &mut Ui, f: &crate::generated::FrmPrintComponents) {
    let (stage, bar, form) = (f.pnlShotStage.id, f.pnlShotBar.id, f.form.id);
    ui.on_paint(stage, |canvas, ctx, _| paint_stage(canvas, ctx));
    ui.on_paint(bar, |canvas, ctx, _| paint_bar(canvas, ctx));
    ui.on(stage, "on_pointer_down", |ui, _, args| {
        if let EventArgs::Pointer(e) = args {
            stage_down(ui, e);
        }
    });
    ui.on(stage, "on_pointer_move", |ui, _, args| {
        if let EventArgs::Pointer(e) = args {
            stage_move(ui, e);
        }
    });
    ui.on(stage, "on_pointer_up", |ui, _, args| {
        if let EventArgs::Pointer(e) = args {
            stage_up(ui, e);
        }
    });
    for id in [stage, bar] {
        ui.on(id, "on_wheel", |ui, _, args| {
            if let EventArgs::Wheel(e) = args {
                wheel_steps(ui, wheel_notches(e));
            }
        });
    }
    ui.on(bar, "on_pointer_down", |ui, _, args| {
        if let EventArgs::Pointer(e) = args {
            bar_down(ui, e);
        }
    });
    ui.on(bar, "on_pointer_move", |ui, _, args| {
        if let EventArgs::Pointer(e) = args {
            bar_move(ui, e);
        }
    });
    ui.on(bar, "on_pointer_up", |ui, _, args| {
        if let EventArgs::Pointer(e) = args {
            bar_up(ui, e);
        }
    });
    ui.on(bar, "on_pointer_leave", |ui, _, _| {
        if !with(|s| s.hue_drag) {
            set_hover(ui, None);
        }
    });
    ui.on(form, "on_key_down", |ui, _, args| {
        if let EventArgs::Key(k) = args {
            key_down(ui, k);
        }
    });
    // O Esc cancela ao subir: a descida e a subida ficam as duas com o editor (um Esc já
    // apertado quando o print abriu só sobe aqui, e não cancela).
    ui.on(form, "on_key_up", |ui, _, args| {
        if let EventArgs::Key(k) = args {
            if k.key == Key::Escape && with(|s| std::mem::take(&mut s.esc_down)) {
                ui.suppress_key();
                cancel(ui);
            }
        }
    });
}

#[cfg(test)]
#[path = "print_editor_tests.rs"]
mod tests;
