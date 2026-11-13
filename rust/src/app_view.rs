//! A janela no aylis: implementa o [`View`] (core → interface) e liga os eventos
//! dos forms direto nas funções do core (interface → core), sem JSON.
//!
//! O `Ui` do aylis pertence ao laço de eventos: o core roda na main thread, mas
//! fora de um evento do aylis, e não o recebe. Cada chamada do `View` vira uma
//! função na fila ([`with_ui`]) que o aylis roda logo em seguida, com o `Ui`, na
//! ordem em que chegou.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use aylis::prelude::*;

use crate::generated as g;

use crate::view::{
    AudioMeter, Device, ExportEncoderInfo, Library, LocalAsrInfo, MonitorThumb, PendingKind, QueueItem, RecItem, RecState, ReplayState,
    RootFolder, TranscribeState, Transcript, UpdateInfo, VideoInfo, VideoTestState, View, WindowItem, WindowMatches,
};
use crate::{app_main, app_settings, lang, log};

/// Tempo de vida padrão de um aviso (o `ttl` do `Toast.show` da web).
const TOAST_TTL_MS: u32 = 6000;

/// Os temas do NoOBS (`themes/` do projeto): o escuro e o claro.
const THEME_DARK: &str = include_str!("../../themes/noobs-dark.json");
const THEME_LIGHT: &str = include_str!("../../themes/noobs-light.json");

type UiJob = Box<dyn FnOnce(&mut Ui)>;

thread_local! {
    static PENDING: RefCell<Vec<UiJob>> = const { RefCell::new(Vec::new()) };
    static POSTER: RefCell<Option<Poster>> = const { RefCell::new(None) };
    static MAIN_FORM: Cell<Option<ComponentId>> = const { Cell::new(None) };
}

/// Roda `f` com o `Ui`, na fila da interface (logo depois do que estiver
/// rodando agora). Só da main thread; as workers chegam pelo `view::call`.
/// Saída do motor de vídeo para a tela: os quadros vão para uma vaga que guarda só o
/// mais recente, e a interface é acordada só quando a vaga estava vazia — o motor
/// entrega todos, nada se acumula no laço e nenhum quadro se perde porque a
/// interface estava pintando. O resto (`Info`, `Ended`, `Error`) vai na ordem.
pub fn frame_sink(id: String, apply: fn(&mut Ui, &str, crate::playback::Out)) -> std::sync::Arc<dyn Fn(crate::playback::Out) + Send + Sync> {
    use crate::playback::Out;
    let slot: std::sync::Arc<std::sync::Mutex<Option<Out>>> = std::sync::Arc::default();
    let poster = poster();
    std::sync::Arc::new(move |o: Out| {
        let Some(p) = poster.as_ref() else { return };
        let id = id.clone();
        if matches!(o, Out::Frame { .. }) {
            let was_empty = slot.lock().map(|mut s| s.replace(o).is_none()).unwrap_or(false);
            if was_empty {
                let slot = slot.clone();
                p.post(move |ui| {
                    let frame = slot.lock().ok().and_then(|mut s| s.take());
                    if let Some(f) = frame {
                        apply(ui, &id, f);
                    }
                });
            }
        } else {
            p.post(move |ui| apply(ui, &id, o));
        }
    })
}

/// A fila da interface do aylis, para mandar trabalho direto de outra thread (os
/// quadros do motor de vídeo: um salto só até a tela).
pub fn poster() -> Option<Poster> {
    POSTER.with(|p| p.borrow().clone())
}

pub fn with_ui(f: impl FnOnce(&mut Ui) + 'static) {
    PENDING.with(|p| p.borrow_mut().push(Box::new(f)));
    POSTER.with(|p| {
        if let Some(p) = p.borrow().as_ref() {
            p.post(run_pending);
        }
    });
}

/// Esvazia a fila (o que uma função enfileirar entra na mesma volta).
fn run_pending(ui: &mut Ui) {
    loop {
        let jobs = PENDING.with(|p| std::mem::take(&mut *p.borrow_mut()));
        if jobs.is_empty() {
            break;
        }
        for job in jobs {
            job(ui);
        }
    }
}

/// O form da janela principal (`frmMain`), depois do [`create`].
pub fn main_form() -> Option<ComponentId> {
    MAIN_FORM.with(Cell::get)
}

/// As telas que cobrem a inicial, cada uma no seu form, na ordem em que se sobrepõem
/// (a última fica por cima).
type CreateScreen = fn(&mut Ui) -> Result<ComponentId, Error>;

const SCREENS: [(&str, CreateScreen); 5] = [
    ("Capturar", g::FrmPicker::create),
    ("Configurações", g::FrmSettings::create),
    ("Exportar", g::FrmExport::create),
    ("Player", g::FrmPlayer::create),
    ("Visualizador", g::FrmViewer::create),
];

/// Põe uma tela dentro da janela, por cima da inicial e escondida: o `frmMain`
/// fica só com a tela inicial no designer.
fn embed_screen(ui: &mut Ui, main: ComponentId, label: &str, create: CreateScreen) {
    let form = match create(ui) {
        Ok(f) => g::Form { id: f },
        Err(e) => {
            log!("Tela {label}: {e}");
            return;
        }
    };
    let _ = ui.set_parent(form.id, Some(main));
    form.set_align(ui, "contents");
    // Sem fundo próprio, a tela deixa ver a de baixo.
    if form.fill(ui).is_empty() {
        form.set_fill(ui, "transparent");
    }
    form.set_visible(ui, false);
}

/// Uma tela de fundo próprio cobre a principal (Configurações, player, exportação): o que
/// só aparece na principal (medidores, prévia das telas) não precisa ser atualizado nem
/// repintado. O seletor de captura não conta: é um diálogo sobre a principal, que continua à
/// vista atrás dele.
pub fn main_covered(ui: &Ui) -> bool {
    let shown = |form: Option<g::Form>| form.is_some_and(|f| f.visible(ui));
    shown(g::FrmSettings::components(ui).map(|frm| frm.form))
        || shown(g::FrmPlayer::components(ui).map(|frm| frm.form))
        || shown(g::FrmExport::components(ui).map(|frm| frm.form))
        || shown(g::FrmViewer::components(ui).map(|frm| frm.form))
}

/// Mostra ou esconde uma tela (`FrmPlayer::components(ui).map(|frm| frm.form)`...).
pub fn show_form(ui: &mut Ui, form: Option<g::Form>, on: bool) {
    if let Some(form) = form {
        form.set_visible(ui, on);
        // Escondida, a tela fica fora do layout: ao aparecer, o tamanho já vale para quem
        // lê logo depois (a coluna das Configurações).
        ui.layout();
    }
}

/// Troca o tema inteiro (o escuro ou o claro do NoOBS).
fn apply_theme(ui: &mut Ui, dark: bool) {
    match aylis::json::load_theme(if dark { THEME_DARK } else { THEME_LIGHT }) {
        Ok(theme) => ui.set_theme(theme),
        Err(e) => log!("UI: tema do NoOBS inválido: {e}"),
    }
}

/// Um aviso no canto de cima à direita (âmbar nos avisos). Um aviso com o
/// MESMO título já na tela só renova o prazo (o hot-plug de USB dispara o
/// refresh de áudio e o de webcam juntos).
pub fn show_toast(title: &str, msg: &str, warn: bool, ttl_ms: u32) {
    let (title, msg) = (title.to_owned(), msg.to_owned());
    let ttl = if ttl_ms == 0 { TOAST_TTL_MS } else { ttl_ms };
    with_ui(move |ui| {
        notify(ui, &title, &msg, if warn { ToastKind::Warning } else { ToastKind::Info }, ttl);
    });
}

pub struct AylisView;

impl View for AylisView {
    fn theme(&self, dark: bool, _pref: &str) {
        with_ui(move |ui| apply_theme(ui, dark));
    }
    fn language_changed(&self, language: &str, _pref: &str) {
        // Os textos dos forms (o Aylis) e, abaixo, os que o código põe.
        let language = language.to_owned();
        with_ui(move |ui| aylis::Application::set_language(ui, &language));
        with_ui(app_main::on_language_changed);
        with_ui(crate::app_library::on_language_changed);
        app_settings::on_language_changed();
    }
    fn toast(&self, title: &str, msg: &str, warn: bool, ttl_ms: u32) {
        show_toast(title, msg, warn, if ttl_ms == 0 { TOAST_TTL_MS } else { ttl_ms });
    }
    fn error(&self, message: &str) {
        let msg = if message.is_empty() { lang::tr("Erro desconhecido") } else { message.to_owned() };
        show_toast(&lang::tr("Erro"), &msg, true, 8000);
    }
    fn open_settings(&self) {
        with_ui(|ui| app_settings::open(ui, None));
    }
    fn window_hidden(&self) {
        // Reabrir o app começa limpo, sem um player fantasma.
        with_ui(|ui| {
            if crate::app_player::is_open() {
                crate::app_player::close(ui);
            }
            crate::app_viewer::close(ui);
            // Escondida, a janela não desenha quadros: a onda do botão pararia no meio.
            crate::app_main::end_ripple(ui);
        });
    }

    fn monitors(&self, list: &[Device], changes: &[String]) {
        app_settings::on_devices("monitors", list);
        app_main::on_monitors(list, changes);
    }
    fn webcams(&self, list: &[Device], changes: &[String]) {
        app_settings::on_devices("webcams", list);
        app_main::on_webcams(list, changes);
    }
    fn audio_sources(&self, mics: &[Device], speakers: &[Device], silent: bool, changes: &[String]) {
        app_settings::on_devices("mics", mics);
        app_settings::on_devices("speakers", speakers);
        app_main::on_audio_sources(mics, speakers, silent, changes);
    }
    fn audio_meters(&self, items: &[AudioMeter]) {
        app_main::on_audio_meters(items);
    }
    fn monitor_thumbs(&self, items: &[MonitorThumb]) {
        app_main::on_monitor_thumbs(items);
    }
    fn device_change_pending(&self, kind: PendingKind, pending: bool) {
        app_main::on_device_change_pending(kind, pending);
    }
    fn refresh_busy(&self, busy: bool, what: &str) {
        app_main::on_refresh_busy(busy, what);
    }
    fn recording_state(&self, st: &RecState) {
        app_main::on_recording_state(st);
    }
    fn recording_finalizing(&self, active: bool) {
        app_main::on_recording_finalizing(active);
    }
    fn recording_stopping(&self) {
        // Atalho/bandeja: o clique na tela já tocou (o debounce evita o 2º).
        app_main::play_stop_sound();
    }
    fn replay_state(&self, st: &ReplayState) {
        app_settings::on_replay_state(st);
        app_main::on_replay_state(st);
    }
    fn replay_saved(&self, _id: &str, name: &str, duration_sec: i32) {
        app_main::on_replay_saved(name, duration_sec);
    }
    fn replay_restarted(&self) {
        app_main::on_replay_restarted();
    }

    // As telas entram por fase; até lá, o que é delas não
    // aparece.
    fn capture_windows(&self, seq: u32, items: &[WindowItem], app_audio: bool) {
        crate::app_picker::on_capture_windows(seq, items, app_audio);
    }
    fn capture_window_thumb(&self, seq: u32, id: &str, width: u32, height: u32, rgb: &[u8]) {
        crate::app_picker::on_capture_window_thumb(seq, id, width, height, rgb);
    }
    fn window_picker_matches(&self, m: &WindowMatches) {
        crate::app_picker::on_window_picker_matches(m);
    }
    fn record_countdown(&self, sec: u32, total: u32) {
        app_main::on_record_countdown(sec, total);
    }
    fn library(&self, lib: &Library) {
        crate::app_library::on_library(lib);
    }
    fn recording_added(&self, item: &RecItem, free_bytes: i64) {
        crate::app_library::on_recording_added(item, free_bytes);
    }
    fn recording_meta(&self, id: &str, duration: i32, thumb: &str) {
        crate::app_library::on_recording_meta(id, duration, thumb);
    }
    fn recording_renamed(&self, old_id: &str, new_id: &str, new_name: &str) {
        crate::app_library::on_recording_renamed(old_id, new_id, new_name);
    }
    fn recording_removed(&self, id: &str, free_bytes: i64) {
        crate::app_library::on_recording_removed(id, free_bytes);
    }
    fn folder_created(&self, id: &str) {
        crate::app_library::on_folder_created(id);
    }
    fn transcript_search(&self, query: &str, ids: &[String]) {
        crate::app_library::on_transcript_search(query, ids);
    }
    fn merge_pending(&self) {
        crate::app_library::on_merge_pending();
    }
    fn merge_done(&self, ok: bool) {
        crate::app_library::on_merge_done(ok);
    }
    fn waveform(&self, id: &str, peaks: &[f32], hi: bool) {
        // A de alta resolução é da linha do tempo da exportação.
        if hi {
            crate::app_export::on_waveform(id, peaks);
        } else {
            crate::app_player::on_waveform(id, peaks);
        }
    }
    fn video_info(&self, info: &VideoInfo) {
        if !crate::app_export::on_video_info(info) {
            crate::app_player::on_video_info(info);
        }
    }
    fn transcript(&self, t: &Transcript) {
        if !crate::app_export::on_transcript(t) {
            crate::app_player::on_transcript(t);
        }
    }
    fn settings_changed(&self) {
        app_settings::on_settings_changed();
    }
    fn record_dir_picked(&self, path: &str) {
        app_settings::on_record_dir_picked(path);
    }
    fn update_result(&self, r: &UpdateInfo) {
        app_settings::on_update_result(r);
    }
    fn encoder_caps(&self, _caps: &crate::types::EncoderCaps, export: &[ExportEncoderInfo]) {
        app_settings::on_encoder_caps();
        crate::app_export::set_encoders(export);
    }
    fn hotkeys_state(&self, items: &[crate::hotkeys::Item], status: &[&str], delay_max: u32, defining: Option<usize>) {
        let status: Vec<&'static str> = status.iter().map(|s| static_status(s)).collect();
        app_settings::on_hotkeys_state(items, &status, delay_max, defining);
    }
    fn video_test(&self, st: &VideoTestState) {
        app_settings::on_video_test(st);
    }
    fn transcribe_state(&self, st: &TranscribeState) {
        app_settings::on_transcribe_state(st);
    }
    fn transcribe_pending(&self, count: usize, cloud: usize) {
        app_settings::on_transcribe_pending(count, cloud);
    }
    fn transcribe_queue(&self, items: &[QueueItem]) {
        app_settings::on_transcribe_queue(items);
    }
    fn local_asr_state(&self, st: &LocalAsrInfo) {
        app_settings::on_local_asr_state(st);
    }
    fn root_folders(&self, folders: &[RootFolder]) {
        app_settings::on_root_folders(folders);
    }
    fn export_progress(&self, pct: f64, source: &str) {
        crate::app_export::on_progress(pct, source);
    }
    fn export_done(&self, ok: bool, canceled: bool, path: &str, _source: &str) {
        crate::app_export::on_done(ok, canceled, path);
    }
    fn export_preview_progress(&self, pct: f64) {
        crate::app_export::on_preview_progress(pct);
    }
    fn export_preview_done(&self, ok: bool, canceled: bool, path: &str) {
        crate::app_export::on_preview_done(ok, canceled, path);
    }
}

fn static_status(s: &str) -> &'static str {
    match s {
        "empty" => "empty",
        "duplicate" => "duplicate",
        "invalid" => "invalid",
        "reserved" => "reserved",
        "inactive" => "inactive",
        "busy" => "busy",
        _ => "",
    }
}

/// Abre o projeto aylis (forms e temas embutidos no exe), registra os
/// handlers e o [`View`]. A janela só nasce quando o laço começa.
pub fn create() -> Result<aylis::Application, aylis::Error> {
    let mut app = aylis::Application::bundled(crate::PROJECT)?;
    // O Application sobe no idioma do projeto (o português): o escolhido vale também na tela.
    aylis::Application::set_language(&mut app.ui, &crate::lang::current_language());
    crate::generated::register(&mut app.ui);
    POSTER.with(|p| *p.borrow_mut() = Some(app.ui.poster()));
    MAIN_FORM.with(|m| m.set(Some(app.main_form)));
    for (label, create) in SCREENS {
        embed_screen(&mut app.ui, app.main_form, label, create);
    }
    apply_theme(&mut app.ui, crate::bridge::resolved_theme() == "dark");
    crate::view::set(Rc::new(AylisView));
    with_ui(app_main::setup);
    with_ui(crate::app_library::apply_texts);
    with_ui(crate::app_player::setup);
    with_ui(crate::app_viewer::setup);
    // Os botões extras do mouse (voltar e avançar), em qualquer ponto da janela: a galeria
    // anda pelo histórico de pastas, só com ela à vista.
    app.ui.add_observer(|_, activity| {
        let aylis::prelude::activity::Activity::PointerUp { event, .. } = activity else { return };
        let forward = match event.button {
            PointerButton::Back => false,
            PointerButton::Forward => true,
            _ => return,
        };
        with_ui(move |ui| {
            let covered = main_covered(ui) || confirm_open() || crate::app_picker::is_open();
            if !covered {
                crate::app_library::mouse_history(ui, forward);
            }
        });
    });
    Ok(app)
}

// ---------------------------------------------------------------------------
// Confirmação (o diálogo da versão web): um cartão sobre o fundo escurecido,
// dentro da janela. Esc ou clique no fundo cancelam; Enter confirma.
// ---------------------------------------------------------------------------

/// Ícone do diálogo: a lixeira vermelha ou o "unir" azul.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ConfirmIcon {
    Delete,
    Merge,
}

type ConfirmFn = Box<dyn FnOnce(&mut Ui)>;

thread_local! {
    static CONFIRM: RefCell<Option<(ComponentId, ConfirmFn)>> = const { RefCell::new(None) };
}


pub fn confirm_open() -> bool {
    CONFIRM.with(|c| c.borrow().is_some())
}

/// Fecha o diálogo; com `ok`, roda a ação.
pub fn confirm_close(ui: &mut Ui, ok: bool) {
    let Some((scrim, f)) = CONFIRM.with(|c| c.borrow_mut().take()) else { return };
    if ui.contains(scrim) {
        ui.free(scrim);
    }
    ui.invalidate();
    if ok {
        f(ui);
    }
}

/// Pergunta antes de uma ação. `danger` pinta o botão de vermelho.
pub fn confirm(ui: &mut Ui, title: &str, message: &str, ok: &str, danger: bool, icon: ConfirmIcon, on_ok: impl FnOnce(&mut Ui) + 'static) {
    confirm_close(ui, false);
    let Some(form) = main_form() else { return };
    let Some(fb) = ui.bounds(form) else { return };
    // O diálogo é o frame `fraConfirm` (forms/view/confirm.json): aqui só o que muda por
    // pergunta (textos, ícone, botão) e as medidas, que dependem do texto e da janela.
    let Ok(dlg) = g::FraConfirm::create_in(ui, form) else { return };
    ui.on(dlg.pnlScrim.id, "on_click", |ui, _, _| confirm_close(ui, false));
    let w = 420.0f32.min(fb.width - 40.0);
    let text_w = w - 44.0 - 48.0 - 14.0;
    dlg.lblTitle.set_text(ui, title);
    dlg.lblMessage.set_text(ui, message);
    let measure = |ui: &Ui, l: ComponentId, text: &str| -> f32 {
        let st = TextStyle { font: ui.font_of(l), wrap: true, ..TextStyle::default() };
        ui.measure_text_wrapped(text, &st, text_w).height.ceil() + 1.0
    };
    let ht = measure(ui, dlg.lblTitle.id, title);
    let hm = measure(ui, dlg.lblMessage.id, message) * 1.13;
    let text_h = ht + 4.0 + hm;
    let top_h = 22.0 + text_h.max(48.0) + 8.0;
    let h = top_h + 14.0 + 34.0 + 18.0;
    for (l, y, hh) in [(dlg.lblTitle.id, 22.0, ht), (dlg.lblMessage.id, 22.0 + ht + 4.0, hm)] {
        if let Some(cb) = ui.control_mut(l) {
            cb.set_rect(22.0 + 48.0 + 14.0, y, text_w, hh);
        }
    }
    // O ícone: a lixeira vermelha (o padrão do frame) ou o "unir" azul.
    if icon == ConfirmIcon::Merge {
        dlg.cirIcon.set_fill(ui, "blue/10");
        dlg.pthTrash.set_visible(ui, false);
        dlg.pthMerge.set_visible(ui, true);
    }
    // Os botões, à direita: confirmar e, antes dele, cancelar.
    dlg.btnOk.set_text(ui, ok);
    if danger {
        dlg.btnOk.set_style(ui, "cf_danger");
    }
    let mut x = w - 22.0;
    for (b, is_ok) in [(dlg.btnOk, true), (dlg.btnCancel, false)] {
        let text = b.text(ui);
        let tw = ui.measure_text(&text, &TextStyle { font: ui.font_of(b.id), ..TextStyle::default() }).width.ceil();
        let bw = tw + 32.0;
        x -= bw;
        if let Some(cb) = ui.control_mut(b.id) {
            cb.set_rect(x, top_h + 14.0, bw, 34.0);
        }
        x -= 8.0;
        ui.on(b.id, "on_click", move |ui, _, _| confirm_close(ui, is_ok));
    }
    if let Some(cb) = ui.control_mut(dlg.pnlCard.id) {
        cb.set_rect((fb.width - w) / 2.0, (fb.height - h) / 2.0, w, h);
    }
    CONFIRM.with(|c| *c.borrow_mut() = Some((dlg.form.id, Box::new(on_ok))));
    ui.invalidate();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// O diálogo é o frame `fraConfirm`: o ícone e o botão seguem o pedido, Cancelar fecha
    /// sem rodar a ação e o frame é liberado.
    #[test]
    fn the_confirmation_is_a_frame() {
        let mut app = aylis::Application::bundled(crate::PROJECT).unwrap();
        crate::generated::register(&mut app.ui);
        MAIN_FORM.with(|m| m.set(Some(app.main_form)));
        let ui = &mut app.ui;
        let ran = Rc::new(Cell::new(false));
        let flag = ran.clone();
        confirm(ui, "Unir", "Unir as duas gravações?", "Unir", false, ConfirmIcon::Merge, move |_| flag.set(true));
        let root = CONFIRM.with(|c| c.borrow().as_ref().map(|(id, _)| *id)).unwrap();
        let dlg = g::FraConfirm::components_of(ui, root).unwrap();
        assert_eq!(dlg.btnOk.text(ui), "Unir");
        assert!(dlg.pthMerge.visible(ui) && !dlg.pthTrash.visible(ui));
        assert_eq!(ui.parent(root), Some(app.main_form));
        ui.fire(dlg.btnCancel.id, "on_click", &EventArgs::None);
        assert!(!ran.get() && !confirm_open() && !ui.contains(root));
        confirm(ui, "Excluir", "Excluir?", "Excluir", true, ConfirmIcon::Delete, |_| {});
        let root = CONFIRM.with(|c| c.borrow().as_ref().map(|(id, _)| *id)).unwrap();
        let dlg = g::FraConfirm::components_of(ui, root).unwrap();
        assert_eq!(dlg.btnOk.style(ui), "cf_danger");
        ui.fire(dlg.btnOk.id, "on_click", &EventArgs::None);
        assert!(!confirm_open());
        MAIN_FORM.with(|m| m.set(None));
    }
}
