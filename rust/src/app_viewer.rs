//! O visualizador de prints: a imagem dentro do NoOBS (no lugar do visualizador do
//! Windows), no jeito do player. O fundo preto, o nome em cima, e:
//!
//! * a roda do mouse aproxima e afasta, ancorada no ponteiro (como no player);
//! * com a imagem maior que a tela, arrastar anda por ela;
//! * o duplo clique alterna entre "ajustada à tela" e o tamanho real (100 %, um pixel
//!   da imagem num pixel da tela), no ponto clicado;
//! * ← e → (ou ‹ ›) passam pelos prints da lista, na ordem dela; Esc fecha;
//! * Ctrl+C (ou "Copiar") põe a imagem na área de transferência.
//!
//! O zoom mostrado é o do tamanho real: 100 % é pixel a pixel, e "Ajustar" nunca amplia
//! um print menor que a tela.

use std::cell::RefCell;
use std::path::Path;

use aylis::prelude::*;

use crate::generated::FrmViewer;
use crate::{bridge, lang, log};

/// Até 8× o tamanho real.
const ZOOM_MAX: f64 = 8.0;
/// Um dente da roda (ou o + e o −).
const STEP: f64 = 1.25;

#[derive(Default)]
struct State {
    /// O print aberto (vazio: fechado).
    id: String,
    /// O tamanho do PNG, em pixels.
    size: (f64, f64),
    /// Pixels lógicos da tela por pixel da imagem.
    k: f64,
    /// Ajustada à tela: acompanha o redimensionar.
    fitted: bool,
    /// O canto de cima à esquerda da imagem no palco (px lógicos).
    pos: (f64, f64),
    /// Arrastando: a posição da imagem e a do ponteiro no começo.
    drag: Option<((f64, f64), PointF)>,
    /// Onde o ponteiro está no palco (o duplo clique amplia ali).
    pointer: Option<(f64, f64)>,
}

thread_local! {
    static ST: RefCell<State> = RefCell::new(State::default());
}

fn with<R>(f: impl FnOnce(&mut State) -> R) -> R {
    ST.with(|s| f(&mut s.borrow_mut()))
}

pub fn is_open() -> bool {
    with(|st| !st.id.is_empty())
}

/// O tamanho de um PNG pelo cabeçalho (`IHDR`).
fn png_size(bytes: &[u8]) -> Option<(f64, f64)> {
    if bytes.len() < 24 || &bytes[..8] != b"\x89PNG\r\n\x1a\n" || &bytes[12..16] != b"IHDR" {
        return None;
    }
    let w = u32::from_be_bytes(bytes[16..20].try_into().ok()?);
    let h = u32::from_be_bytes(bytes[20..24].try_into().ok()?);
    (w > 0 && h > 0).then_some((w as f64, h as f64))
}

fn fmt_size(b: u64) -> String {
    let b = b as f64;
    if b >= 1024.0 * 1024.0 {
        format!("{:.1} MB", b / (1024.0 * 1024.0)).replace('.', ",")
    } else {
        format!("{:.0} KB", (b / 1024.0).max(1.0))
    }
}

/// O tamanho do palco (px lógicos).
fn stage(ui: &mut Ui) -> (f64, f64) {
    FrmViewer::components(ui).and_then(|f| ui.bounds(f.lyViewStage.id)).map_or((1280.0, 720.0), |b| (b.width as f64, b.height as f64))
}

/// O tamanho real: um pixel da imagem num pixel da tela.
fn real_k() -> f64 {
    1.0 / f64::from(crate::app_player::window_scale())
}

/// Ajustada à tela, sem ampliar um print menor que ela.
fn fit_k(stage: (f64, f64), size: (f64, f64)) -> f64 {
    (stage.0 / size.0).min(stage.1 / size.1).min(real_k())
}

/// Centraliza no eixo em que a imagem cabe; no outro, não deixa sobrar borda.
fn clamp(st: &mut State, stage: (f64, f64)) {
    let (w, h) = (st.size.0 * st.k, st.size.1 * st.k);
    st.pos.0 = if w <= stage.0 { (stage.0 - w) / 2.0 } else { st.pos.0.clamp(stage.0 - w, 0.0) };
    st.pos.1 = if h <= stage.1 { (stage.1 - h) / 2.0 } else { st.pos.1.clamp(stage.1 - h, 0.0) };
}

/// Põe a imagem onde o estado diz e atualiza o zoom e o cursor.
fn place(ui: &mut Ui) {
    let stage = stage(ui);
    let (pos, w, h, pct, bigger) = with(|st| {
        clamp(st, stage);
        let (w, h) = (st.size.0 * st.k, st.size.1 * st.k);
        (st.pos, w, h, (st.k / real_k() * 100.0).round(), w > stage.0 + 0.5 || h > stage.1 + 0.5)
    });
    let Some(frm) = FrmViewer::components(ui) else { return };
    if let Some(cb) = ui.control_mut(frm.imgView.id) {
        cb.set_rect(pos.0 as f32, pos.1 as f32, w as f32, h as f32);
    }
    frm.lblViewZoom.set_text(ui, format!("{pct}%"));
    frm.lyViewStage.set_cursor(ui, if bigger { "grab" } else { "default" });
    ui.invalidate();
}

/// Ajustada à tela (o "Ajustar", o 0).
pub fn fit(ui: &mut Ui) {
    let stage = stage(ui);
    with(|st| {
        st.k = fit_k(stage, st.size);
        st.fitted = true;
    });
    place(ui);
}

/// O zoom `k`, mantendo parado o ponto `at` do palco (sem ele, o centro).
fn zoom_to(ui: &mut Ui, k: f64, at: Option<(f64, f64)>) {
    let stage = stage(ui);
    with(|st| {
        let low = fit_k(stage, st.size);
        let k = k.clamp(low, real_k() * ZOOM_MAX);
        let (cx, cy) = at.unwrap_or((stage.0 / 2.0, stage.1 / 2.0));
        let (px, py) = ((cx - st.pos.0) / st.k, (cy - st.pos.1) / st.k);
        st.pos = (cx - px * k, cy - py * k);
        st.k = k;
        st.fitted = (k - low).abs() < 1e-9;
    });
    place(ui);
}

fn zoom_by(ui: &mut Ui, factor: f64, at: Option<(f64, f64)>) {
    let k = with(|st| st.k) * factor;
    zoom_to(ui, k, at);
}

/// O duplo clique: do ajustado (ou de outro zoom) para o tamanho real no ponto, e do
/// tamanho real de volta para o ajustado.
fn toggle_real(ui: &mut Ui) {
    let (k, at) = with(|st| (st.k, st.pointer));
    if (k - real_k()).abs() < 1e-9 {
        fit(ui);
    } else {
        zoom_to(ui, real_k(), at);
    }
}

/// Abre um print da lista. Só um PNG da pasta de gravações (a mesma conferência de antes,
/// quando ia para o visualizador do Windows).
pub fn open(ui: &mut Ui, id: &str) {
    if !bridge::check_print(id) {
        return;
    }
    let bytes = match std::fs::read(id) {
        Ok(b) => b,
        Err(e) => {
            log!("Visualizador: nao abriu {id} ({e})");
            crate::app_view::show_toast(&lang::tr("Erro"), &lang::tr("Não foi possível abrir o print."), true, 0);
            return;
        }
    };
    let Some(size) = png_size(&bytes) else {
        log!("Visualizador: {id} nao e um PNG valido");
        crate::app_view::show_toast(&lang::tr("Erro"), &lang::tr("Não foi possível abrir o print."), true, 0);
        return;
    };
    let title = Path::new(id).file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
    let info = format!("{} × {} px  ·  {}", size.0, size.1, fmt_size(bytes.len() as u64));
    crate::app_view::show_form(ui, FrmViewer::components(ui).map(|f| f.form), true);
    let Some(frm) = FrmViewer::components(ui) else { return };
    if let Some(v) = ui.get_mut::<ImageView>(frm.imgView.id) {
        v.set_picture(Some(Image::encoded(bytes)));
    }
    frm.lblViewTitle.set_text(ui, title);
    frm.lblViewInfo.set_text(ui, info);
    with(|st| {
        st.id = id.to_owned();
        st.size = size;
        st.k = 1.0;
        st.pos = (0.0, 0.0);
        st.drag = None;
        st.pointer = None;
    });
    fit(ui);
    sync_nav(ui);
    ui.set_focus(frm.lyViewStage.id);
    log!("Visualizador: abrindo {id}");
}

pub fn close(ui: &mut Ui) {
    if !is_open() {
        return;
    }
    crate::app_view::show_form(ui, FrmViewer::components(ui).map(|f| f.form), false);
    // Solta a imagem (um print 4K são dezenas de MB na placa).
    if let Some(id) = FrmViewer::components(ui).map(|f| f.imgView.id) {
        if let Some(v) = ui.get_mut::<ImageView>(id) {
            v.set_picture(None);
        }
    }
    with(|st| st.id.clear());
}

/// Onde o print aberto está entre os da lista, e quantos são.
fn nav_position() -> (Option<usize>, Vec<String>) {
    let ids = crate::app_library::print_ids();
    let at = with(|st| ids.iter().position(|i| i.eq_ignore_ascii_case(&st.id)));
    (at, ids)
}

fn sync_nav(ui: &mut Ui) {
    let (at, ids) = nav_position();
    let Some(frm) = FrmViewer::components(ui) else { return };
    let n = ids.len();
    frm.lblViewPos.set_text(ui, at.map(|i| format!("{} / {n}", i + 1)).unwrap_or_default());
    frm.btnViewPrev.set_enabled(ui, at.is_some_and(|i| i > 0));
    frm.btnViewNext.set_enabled(ui, at.is_some_and(|i| i + 1 < n));
}

/// O print anterior (-1) ou o próximo (+1) da lista.
pub fn step(ui: &mut Ui, dir: i32) {
    let (at, ids) = nav_position();
    let Some(i) = at else { return };
    let j = i as i64 + i64::from(dir);
    if let Some(next) = usize::try_from(j).ok().and_then(|j| ids.get(j)).cloned() {
        open(ui, &next);
    }
}

/// A imagem aberta na área de transferência (Ctrl+C, "Copiar").
pub fn copy() {
    let id = with(|st| st.id.clone());
    if !id.is_empty() {
        crate::screenshot::copy_file(id);
    }
}

/// O teclado com o visualizador aberto. `true`: a tecla era dele.
pub fn key(ui: &mut Ui, k: &KeyEvent) -> bool {
    if !is_open() {
        return false;
    }
    let plain = !k.modifiers.ctrl && !k.modifiers.alt;
    match k.key {
        Key::Char('c') | Key::Char('C') if k.modifiers.ctrl => copy(),
        Key::Escape => close(ui),
        Key::Left => step(ui, -1),
        Key::Right => step(ui, 1),
        Key::Char('0') if plain => fit(ui),
        Key::Char('1') if plain => zoom_to(ui, real_k(), None),
        Key::Char('+') | Key::Char('=') => zoom_by(ui, STEP, None),
        Key::Char('-') => zoom_by(ui, 1.0 / STEP, None),
        _ => return false,
    }
    true
}

/// Liga os eventos (uma vez, na subida).
pub fn setup(ui: &mut Ui) {
    let Some(frm) = FrmViewer::components(ui) else { return };
    ui.on(frm.btnViewClose.id, "on_click", |ui, _, _| close(ui));
    ui.on(frm.btnViewPrev.id, "on_click", |ui, _, _| step(ui, -1));
    ui.on(frm.btnViewNext.id, "on_click", |ui, _, _| step(ui, 1));
    ui.on(frm.btnViewCopy.id, "on_click", |_, _, _| copy());
    ui.on(frm.btnViewFit.id, "on_click", |ui, _, _| fit(ui));
    ui.on(frm.btnView100.id, "on_click", |ui, _, _| zoom_to(ui, real_k(), None));
    ui.on(frm.btnViewIn.id, "on_click", |ui, _, _| zoom_by(ui, STEP, None));
    ui.on(frm.btnViewOut.id, "on_click", |ui, _, _| zoom_by(ui, 1.0 / STEP, None));
    let stage_id = frm.lyViewStage.id;
    ui.on(stage_id, "on_resize", |ui, _, _| {
        if !is_open() {
            return;
        }
        if with(|st| st.fitted) {
            fit(ui);
        } else {
            place(ui);
        }
    });
    ui.on(stage_id, "on_wheel", |ui, _, args| {
        if let EventArgs::Wheel(e) = args {
            let factor = if e.delta.y > 0.0 { STEP } else { 1.0 / STEP };
            zoom_by(ui, factor, Some((f64::from(e.position.x), f64::from(e.position.y))));
        }
    });
    ui.on(stage_id, "on_pointer_down", move |ui, _, args| {
        if let EventArgs::Pointer(e) = args {
            if e.button == PointerButton::Primary {
                with(|st| st.drag = Some((st.pos, e.position)));
            }
            ui.set_focus(stage_id);
        }
    });
    ui.on(stage_id, "on_pointer_move", |ui, _, args| {
        let EventArgs::Pointer(e) = args else { return };
        let moved = with(|st| {
            st.pointer = Some((f64::from(e.position.x), f64::from(e.position.y)));
            let Some((pos, from)) = st.drag else { return false };
            st.pos = (pos.0 + f64::from(e.position.x - from.x), pos.1 + f64::from(e.position.y - from.y));
            true
        });
        if moved {
            place(ui);
        }
    });
    ui.on(stage_id, "on_pointer_up", |_, _, _| with(|st| st.drag = None));
    ui.on(stage_id, "on_dbl_click", |ui, _, _| toggle_real(ui));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tamanho_do_png_pelo_cabecalho() {
        let mut png = b"\x89PNG\r\n\x1a\n\0\0\0\x0dIHDR".to_vec();
        png.extend_from_slice(&3840u32.to_be_bytes());
        png.extend_from_slice(&2160u32.to_be_bytes());
        assert_eq!(png_size(&png), Some((3840.0, 2160.0)));
        assert_eq!(png_size(b"GIF89a................."), None);
    }

    /// Menor que a tela: centralizada nos dois eixos. Maior: sem borda sobrando.
    #[test]
    fn a_imagem_fica_dentro_do_palco() {
        let mut st = State { size: (100.0, 50.0), k: 1.0, pos: (-30.0, 40.0), ..Default::default() };
        clamp(&mut st, (400.0, 300.0));
        assert_eq!(st.pos, (150.0, 125.0));
        st.k = 10.0;
        st.pos = (50.0, -900.0);
        clamp(&mut st, (400.0, 300.0));
        assert_eq!(st.pos, (0.0, -200.0));
    }
}
