//! O seletor "Capturar..." (janela ou área), uma camada por cima da tela
//! principal (`pnlPicker` no `forms/main.json`): abas Janela/Área, busca, a
//! grade das janelas abertas com miniatura, o som do app, as áreas salvas e o
//! timelapse. Aberto por um atalho de janela que achou várias, mostra só elas
//! (sem abas) e o atraso do atalho conta depois da escolha.

use crate::generated::{FraAreaButton, FraWindowCard, FrmPicker};
use std::cell::RefCell;
use std::collections::HashMap;

use aylis::prelude::*;

use crate::app_view::{main_form, with_ui};
use crate::view::{WindowItem, WindowMatches};
use crate::{bridge, config, lang};

#[derive(Default)]
struct Picker {
    open: bool,
    area: bool,
    items: Vec<WindowItem>,
    thumbs: HashMap<String, Image>,
    /// A miniatura de cada cartão na grade (id da janela → `ImageView`).
    cards: HashMap<String, ComponentId>,
    seq: u32,
    search: String,
    loading: bool,
    audio_supported: bool,
    /// Aberto por um atalho: (trecho do título, atraso).
    matches: Option<(String, u32)>,
}

thread_local! {
    static P: RefCell<Picker> = RefCell::new(Picker::default());
}

fn with<R>(f: impl FnOnce(&mut Picker) -> R) -> R {
    P.with(|p| f(&mut p.borrow_mut()))
}

/// Largura de um cartão: três por linha.
const CARD_W: f32 = 205.0;
const CARD_THUMB_H: f32 = CARD_W * 10.0 / 16.0;
const CARD_H: f32 = CARD_THUMB_H + 6.0 + 16.0 + 6.0 + 14.0 + 16.0;

fn subtitle(p: &Picker) -> String {
    if let Some((pattern, _)) = &p.matches {
        lang::tr_with("Mais de uma janela com \"{{title}}\" — escolha qual gravar", &[("title", pattern)])
    } else if p.area {
        lang::tr("Grave um pedaço da tela, em qualquer monitor. O áudio segue os dispositivos marcados.")
    } else {
        lang::tr("Só a janela escolhida entra no vídeo, mesmo coberta por outras. O áudio segue os dispositivos marcados.")
    }
}

fn apply_texts(ui: &mut Ui) {
    if let Some(frm) = FrmPicker::components(ui) {
        frm.edPickerSearch.set_placeholder(ui, lang::tr("Filtrar janelas..."));
        frm.segPickerMode.set_items(ui, format!("{};{}", lang::tr("Janela"), lang::tr("Área")));
    }
}

/// Grade filtrada pela busca (título ou exe), o texto de vazio, as abas e as
/// áreas salvas.
fn render(ui: &mut Ui) {
    let (area, matches, loading, supported, sub) = with(|p| (p.area, p.matches.is_some(), p.loading, p.audio_supported, subtitle(p)));
    if let Some(frm) = FrmPicker::components(ui) {
        frm.lblPickerSubtitle.set_text(ui, sub);
        frm.lyPickerMode.set_visible(ui, !matches);
        frm.btnPickerRefresh.set_visible(ui, !area && !matches);
        frm.segPickerMode.set_item_index(ui, if area { 1i64 } else { 0 });
        frm.lyPickerWindow.set_visible(ui, !area);
        frm.lyPickerArea.set_visible(ui, area);
        frm.chipAppAudio.set_visible(ui, supported);
    }
    fit_dialog(ui);
    if area {
        render_saved_areas(ui);
        ui.layout();
        return;
    }
    let cards: Vec<WindowItem> = with(|p| {
        let q = p.search.trim().to_lowercase();
        p.items.iter().filter(|it| q.is_empty() || it.title.to_lowercase().contains(&q) || it.exe.to_lowercase().contains(&q)).cloned().collect()
    });
    let empty = if loading {
        Some(lang::tr("Procurando janelas abertas") + "...")
    } else if cards.is_empty() {
        Some(lang::tr(if with(|p| p.items.is_empty()) { "Nenhuma janela aberta para gravar." } else { "Nenhuma janela com esse nome." }))
    } else {
        None
    };
    if let Some(frm) = FrmPicker::components(ui) {
        frm.lblPickerEmpty.set_visible(ui, empty.is_some());
        frm.lblPickerEmpty.set_text(ui, empty.unwrap_or_default());
    }
    let Some(grid) = FrmPicker::components(ui).map(|frm| frm.flowPickerGrid.id) else { return };
    for child in ui.children(grid).to_vec() {
        ui.free(child);
    }
    let mut views = HashMap::new();
    for it in &cards {
        if let Some(view) = add_card(ui, grid, it) {
            views.insert(it.id.clone(), view);
        }
    }
    let rows = cards.len().div_ceil(3).max(1);
    if let Some(cb) = ui.control_mut(grid) {
        cb.set_height(rows as f32 * (CARD_H + 12.0));
    }
    with(|p| p.cards = views);
    ui.layout();
}

/// A altura do diálogo: FIXA, a de duas linhas de cartões na aba Janela (a grade rola
/// quando há mais), igual nas duas abas — antes ela seguia o conteúdo e o diálogo mudava
/// de tamanho ao trocar de aba. No máximo a da janela menos 48 px.
fn fit_dialog(ui: &mut Ui) {
    let header = 52.0 + 23.0 + 48.0;
    let content = 44.0 + 2.0 * (CARD_H + 12.0);
    let max = main_form().and_then(|f| ui.bounds(f)).map_or(800.0, |b| b.height - 48.0);
    let h = (16.0 + header + content + 20.0).min(max).max(260.0);
    if let Some(cb) = FrmPicker::components(ui).map(|frm| frm.pnlPickerDialog.id).and_then(|d| ui.control_mut(d)) {
        cb.set_height(h);
    }
}

/// Um cartão de janela: a miniatura, o título e "exe · 1920×1080". O clique grava.
fn add_card(ui: &mut Ui, grid: ComponentId, it: &WindowItem) -> Option<ComponentId> {
    // O cartão é o frame `fraWindowCard` (forms/picker/window_card.json).
    let card = FraWindowCard::create_in(ui, grid).ok()?;
    let id = it.id.clone();
    ui.on(card.pnlCard.id, "on_click", move |ui, _, _| pick(ui, &id));
    let view = card.imgThumb.id;
    if let Some(image) = with(|p| p.thumbs.get(&it.id).cloned()) {
        if let Some(v) = ui.get_mut::<ImageView>(view) {
            v.set_picture(Some(image));
        }
    }
    let size = if it.w > 0 && it.h > 0 { format!(" · {}×{}", it.w, it.h) } else { String::new() };
    card.lblTitle.set_text(ui, it.title.as_str());
    card.lblInfo.set_text(ui, format!("{}{size}", it.exe));
    Some(view)
}

/// Os botões das áreas salvas (atalhos de área com retângulo), depois do
/// "Selecionar área".
fn render_saved_areas(ui: &mut Ui) {
    let Some(row) = FrmPicker::components(ui).map(|frm| frm.lyAreaButtons.id) else { return };
    let first = FrmPicker::components(ui).map(|frm| frm.btnSelectArea.id);
    for child in ui.children(row).to_vec() {
        if Some(child) != first {
            ui.free(child);
        }
    }
    let mut width = 170.0;
    for (i, aw, ah, hk) in bridge::saved_areas() {
        let mut label = lang::tr_with("Gravar a área salva ({{w}} × {{h}})", &[("w", &aw.to_string()), ("h", &ah.to_string())]);
        if !hk.is_empty() {
            label = format!("{label} · {hk}");
        }
        let Ok(frame) = FraAreaButton::create_in(ui, row) else { continue };
        let w = ui.measure_text(&label, &TextStyle::default()).width + 40.0;
        frame.btnArea.set_text(ui, label);
        frame.form.set_width(ui, f64::from(w));
        width += w + 8.0;
        ui.on(frame.btnArea.id, "on_click", move |ui, _, _| {
            close(ui);
            bridge::trigger_hotkey(i);
        });
    }
    if let Some(cb) = ui.control_mut(row) {
        cb.set_height(if width > 620.0 { 80.0 } else { 36.0 });
    }
}

fn show(ui: &mut Ui) {
    crate::app_view::show_form(ui, FrmPicker::components(ui).map(|frm| frm.form), true);
    if let Some(frm) = crate::generated::FrmMain::components(ui) {
        frm.btnRecord.set_glow(ui, false);
    }
    if let Some(frm) = FrmPicker::components(ui) {
        frm.edPickerSearch.set_text(ui, "");
    }
    // Os dois desligados por padrão; o do cursor só vale (e só fica marcado) com o Timelapse.
    let timelapse = config::get_config_bool("areaTimelapse", false);
    if !timelapse {
        config::set_config_bool("areaTimelapseCursor", false);
    }
    if let Some(frm) = FrmPicker::components(ui) {
        frm.chipTimelapse.set_checked(ui, timelapse);
        frm.chipTimelapseCursor.set_checked(ui, timelapse && config::get_config_bool("areaTimelapseCursor", false));
        frm.chipTimelapseCursor.set_enabled(ui, timelapse);
    }
    apply_texts(ui);
    fit_chips(ui);
    if let Some(edit) = FrmPicker::components(ui).map(|frm| frm.edPickerSearch.id) {
        ui.set_focus(edit);
    }
}

/// O botão "Capturar..." do cartão.
pub fn open(ui: &mut Ui, recording: bool) {
    if recording {
        return;
    }
    let area = config::get_config_str("windowPickerMode", "window") == "area";
    with(|p| {
        let (seq, supported) = (p.seq, p.audio_supported);
        *p = Picker { seq, audio_supported: supported, open: true, ..Default::default() };
    });
    if let Some(frm) = FrmPicker::components(ui) {
        frm.chipAppAudio.set_checked(ui, config::get_config_bool("windowPickerAppAudio", false));
    }
    show(ui);
    set_mode(ui, area, false);
}

/// Troca de aba. A lista de janelas só é pedida quando a aba Janela aparece
/// (abrir direto na Área não gasta o PrintWindow de todas as janelas à toa).
fn set_mode(ui: &mut Ui, area: bool, remember: bool) {
    let matches = with(|p| {
        p.area = area;
        p.matches.is_some()
    });
    if remember && !matches {
        config::set_config_str("windowPickerMode", if area { "area" } else { "window" });
    }
    let need_list = with(|p| !area && p.items.is_empty() && !p.loading && p.matches.is_none());
    if need_list {
        refresh(ui);
    } else {
        render(ui);
    }
}

pub fn refresh(ui: &mut Ui) {
    with(|p| {
        p.loading = true;
        p.items.clear();
        p.thumbs.clear();
    });
    render(ui);
    bridge::handle_list_capture_windows();
}

pub fn is_open() -> bool {
    with(|p| p.open)
}

pub fn close(ui: &mut Ui) {
    with(|p| {
        p.open = false;
        p.matches = None;
    });
    crate::app_view::show_form(ui, FrmPicker::components(ui).map(|frm| frm.form), false);
    if let Some(frm) = crate::generated::FrmMain::components(ui) {
        frm.btnRecord.set_glow(ui, true);
    }
}

fn pick(ui: &mut Ui, id: &str) {
    if id.is_empty() {
        return;
    }
    let (delay, supported) = with(|p| (p.matches.as_ref().map_or(0, |m| m.1), p.audio_supported));
    close(ui);
    crate::app_main::start_ripple(ui);
    let app_audio = supported && config::get_config_bool("windowPickerAppAudio", false);
    bridge::handle_record_window(id, app_audio, delay);
}

pub fn on_capture_windows(seq: u32, items: &[WindowItem], app_audio: bool) {
    let items = items.to_vec();
    with_ui(move |ui| {
        with(|p| {
            p.seq = seq;
            p.items = items;
            p.thumbs.clear();
            p.loading = false;
            p.audio_supported = app_audio;
        });
        if with(|p| p.open) {
            render(ui);
        }
    });
}

pub fn on_capture_window_thumb(seq: u32, id: &str, width: u32, height: u32, rgb: &[u8]) {
    if rgb.len() != (width * height * 3) as usize {
        return;
    }
    let mut rgba = Vec::with_capacity(rgb.len() / 3 * 4);
    for px in rgb.chunks_exact(3) {
        rgba.extend_from_slice(&[px[0], px[1], px[2], 255]);
    }
    let image = Image::rgba(width, height, rgba);
    let id = id.to_owned();
    with_ui(move |ui| {
        if with(|p| p.seq) != seq {
            return;
        }
        // Só troca a imagem do cartão (refazer a grade faria o hover piscar).
        let view = with(|p| {
            p.thumbs.insert(id.clone(), image.clone());
            p.cards.get(&id).copied()
        });
        if let Some(view) = view.and_then(|v| ui.get_mut::<ImageView>(v)) {
            view.set_picture(Some(image));
        }
        ui.invalidate();
    });
}

/// Atalho de janela com várias janelas casando o título.
pub fn on_window_picker_matches(m: &WindowMatches) {
    let m = m.clone();
    with_ui(move |ui| {
        with(|p| {
            *p = Picker {
                open: true,
                seq: m.seq,
                items: m.items.clone(),
                audio_supported: m.app_audio,
                matches: Some((m.pattern.clone(), m.delay_sec)),
                ..Default::default()
            };
        });
        if let Some(frm) = FrmPicker::components(ui) {
            frm.chipAppAudio.set_checked(ui, m.want_app_audio);
        }
        show(ui);
        render(ui);
    });
}

// ----- eventos do form -----

pub fn on_mode(ui: &mut Ui, area: bool) {
    set_mode(ui, area, true);
}

pub fn on_search(ui: &mut Ui, text: String) {
    with(|p| p.search = text);
    render(ui);
}

pub fn on_audio(on: bool) {
    config::set_config_bool("windowPickerAppAudio", on);
}

pub fn on_timelapse(ui: &mut Ui, on: bool) {
    config::set_config_bool("areaTimelapse", on);
    if let Some(frm) = FrmPicker::components(ui) {
        frm.chipTimelapseCursor.set_enabled(ui, on);
    }
    // Sem o Timelapse, o "cursor conta como movimento" não tem o que fazer: desmarca.
    if !on {
        config::set_config_bool("areaTimelapseCursor", false);
        if let Some(frm) = FrmPicker::components(ui) {
            frm.chipTimelapseCursor.set_checked(ui, false);
        }
    }
    fit_chips(ui);
}

pub fn on_timelapse_cursor(ui: &mut Ui, on: bool) {
    config::set_config_bool("areaTimelapseCursor", on);
    fit_chips(ui);
}

/// Os chips da aba Área do tamanho do conteúdo, com a conta do próprio Chip: 28 de respiro,
/// o texto e o ícone (o ✓ do marcado conta como ícone). Com largura fixa, o "Cursor conta
/// como movimento" desmarcado sobrava espaço do ✓ que não aparece; também cabe nas traduções.
fn fit_chips(ui: &mut Ui) {
    let Some(frm) = FrmPicker::components(ui) else { return };
    for handle in [frm.chipTimelapse, frm.chipTimelapseCursor] {
        let chip = handle.id;
        let text = handle.text(ui);
        let has_icon = !handle.icon(ui).is_empty();
        let checked = handle.checked(ui);
        let style = TextStyle { font: ui.font_of(chip), ..TextStyle::default() };
        let tw = ui.measure_text(&text, &style).width.ceil();
        let height = ui.control(chip).map_or(30.0, |cb| cb.height());
        let side = (height * 0.55).round();
        let w = 28.0 + tw + if has_icon || checked { side + 4.0 } else { 0.0 };
        if let Some(cb) = ui.control_mut(chip) {
            cb.set_width(w);
        }
    }
}

pub fn on_select_area(ui: &mut Ui) {
    close(ui);
    bridge::handle_select_capture_area();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Os cartões são instâncias do frame `fraWindowCard`, cada uma com os seus
    /// componentes tipados, lado a lado no mesmo pai.
    #[test]
    fn the_cards_are_frames_with_their_own_components() {
        let mut app = aylis::Application::bundled(crate::PROJECT).unwrap();
        crate::generated::register(&mut app.ui);
        let ui = &mut app.ui;
        let grid = ui.create("FlowLayout", None).unwrap();
        let item = |id: &str, title: &str| WindowItem { id: id.into(), title: title.into(), exe: "app.exe".into(), w: 800, h: 600, ..Default::default() };
        let a = add_card(ui, grid, &item("1", "Primeira")).unwrap();
        let b = add_card(ui, grid, &item("2", "Segunda")).unwrap();
        assert_ne!(a, b);
        let cards: Vec<_> = ui.children(grid).iter().map(|&c| FraWindowCard::components_of(ui, c).unwrap()).collect();
        assert_eq!(cards.len(), 2);
        assert_eq!(cards[0].lblTitle.text(ui), "Primeira");
        assert_eq!(cards[1].lblTitle.text(ui), "Segunda");
        assert_eq!(cards[1].lblInfo.text(ui), "app.exe · 800×600");
        assert_eq!(cards[0].imgThumb.id, a);
        assert_eq!(ui.bounds(cards[0].form.id).map(|r| r.width), Some(CARD_W));
    }
}
