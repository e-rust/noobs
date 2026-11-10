//! Presenter do player: abre o motor (`playback`), põe os quadros no palco
//! (`FrameView`), mantém barra/tempo/relógio, e cuida de zoom, volume,
//! velocidade, teclado, do esconder dos controles e dos painéis de
//! informações e de transcrição (com a legenda). O desenho é o `forms/player.json`; as
//! peças repetidas dos painéis são os frames de `forms/player/` (seção, linha, opção da
//! visualização, cartão de faixa, fala e o campo do falante).

use crate::generated as g;
use crate::generated::{FraInfoRow, FraInfoSection, FraSpeakerEdit, FraTrackCard, FraTurn, FraTurnComponents, FraViewOption, FrmPlayer, FrmPlayerComponents};
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::sync::Arc;
use std::time::{Duration, Instant};

use aylis::prelude::*;

use crate::app_view::{main_form, with_ui};
use crate::playback::{Out, Playback, Region, ViewSpec, TRICK_MIN_RATE};
use crate::view::{Transcript, Turn, VideoInfo};
use crate::{bridge, config, lang, log};

/// Os controles somem depois disto sem mexer o mouse, tocando.
const IDLE_MS: u64 = 2000;
/// Zoom máximo da roda.
const ZOOM_MAX: f64 = 8.0;
const WAVE_BUCKETS: i64 = 2000;
/// As velocidades do menu (de 8× pra cima anda por saltos de keyframe).
const SPEEDS: [f64; 9] = [0.25, 0.5, 1.0, 1.5, 2.0, 3.0, 8.0, 16.0, 32.0];
/// O pulso do indicador central na pausa (entra rápido, segura, sai).
const FLASH_MS: f64 = 1200.0;
/// Os ícones cheios do player (grade 24).
const ICONS: [(&str, &str); 10] = [
    ("pl-play", "M8 5v14l11-7z"),
    ("pl-pause", "M6 5h4v14H6zM14 5h4v14h-4z"),
    ("pl-vol-high", "M3 9v6h4l5 5V4L7 9H3zm13.5 3a4.5 4.5 0 0 0-2.5-4.03v8.05A4.5 4.5 0 0 0 16.5 12zM14 3.23v2.06c2.89.86 5 3.54 5 6.71s-2.11 5.85-5 6.71v2.06c4.01-.91 7-4.49 7-8.77S18.01 4.14 14 3.23z"),
    ("pl-vol-low", "M7 9v6h4l5 5V4l-5 5H7zm9.5 3a4.5 4.5 0 0 0-2.5-4.03v8.05A4.5 4.5 0 0 0 16.5 12z"),
    ("pl-vol-off", "M16.5 12A4.5 4.5 0 0 0 14 7.97v2.21l2.45 2.45c.03-.2.05-.41.05-.63zm2.5 0c0 .94-.2 1.82-.54 2.64l1.51 1.51A8.92 8.92 0 0 0 21 12c0-4.28-2.99-7.86-7-8.77v2.06c2.89.86 5 3.54 5 6.71zM4.27 3L3 4.27 7.73 9H3v6h4l5 5v-6.73l4.25 4.25c-.67.52-1.42.93-2.25 1.18v2.06a8.99 8.99 0 0 0 3.69-1.81L19.73 21 21 19.73l-9-9L4.27 3zM12 4L9.91 6.09 12 8.18V4z"),
    ("pl-cc", "M19 4H5a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h14c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm-8 7H9.5v-.5h-2v3h2V13H11v1c0 .55-.45 1-1 1H7c-.55 0-1-.45-1-1v-4c0-.55.45-1 1-1h3c.55 0 1 .45 1 1v1zm7 0h-1.5v-.5h-2v3h2V13H18v1c0 .55-.45 1-1 1h-3c-.55 0-1-.45-1-1v-4c0-.55.45-1 1-1h3c.55 0 1 .45 1 1v1z"),
    ("pl-info", "M12 2a10 10 0 1 0 10 10A10 10 0 0 0 12 2zm1 15h-2v-6h2zm0-8h-2V7h2z"),
    ("pl-fs-enter", "M7 14H5v5h5v-2H7v-3zm-2-4h2V7h3V5H5v5zm12 7h-3v2h5v-5h-2v3zM14 5v2h3v3h2V5h-5z"),
    ("pl-fs-exit", "M5 16h3v3h2v-5H5v2zm3-8H5v2h5V5H8v3zm6 11h2v-3h3v-2h-5v5zm2-11V5h-2v5h5V8h-3z"),
    ("pl-clock", "M12 2a10 10 0 1 0 0 20 10 10 0 0 0 0-20zm0 18a8 8 0 1 1 0-16 8 8 0 0 1 0 16zm.5-13H11v6l5.2 3.1.8-1.3-4.5-2.6z"),
];
/// Os ícones de traço do painel de informações (grade 24).
const LINE_ICONS: [(&str, &str); 5] = [
    ("pl-expand", "M15 3h6v6 M9 21H3v-6 M21 3l-7 7 M3 21l7-7"),
    ("pl-monitor", "M4 4h16a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2z M8 22h8 M12 18v4"),
    ("pl-webcam", "M15.5 12A3.5 3.5 0 1 1 8.5 12A3.5 3.5 0 1 1 15.5 12Z M21 12A9 9 0 1 1 3 12A9 9 0 1 1 21 12Z"),
    ("pl-tvol", "M11 5L6 9H2v6h4l5 4V5z M15.54 8.46a5 5 0 0 1 0 7.07 M19.07 4.93a10 10 0 0 1 0 14.14"),
    ("pl-tvol-off", "M11 5L6 9H2v6h4l5 4V5z M23 9l-6 6 M17 9l6 6"),
];
/// As cores dos falantes ficam na paleta do projeto (`speaker_1`…`speaker_6`, uma por modo).
const SPEAKER_COLORS: usize = 6;

#[derive(Default)]
struct State {
    id: String,
    pb: Option<Playback>,
    /// Palco em px lógicos e a escala da janela.
    stage: (f32, f32),
    scale: f32,
    zoom: f64,
    pan: (f64, f64),
    pan_start: Option<((f64, f64), PointF)>,
    /// (zoom, pan_x, pan_y) do quadro na tela, em px lógicos.
    frame_view: (f64, f64, f64),
    master: i32,
    muted: bool,
    speed: f64,
    duration: f64,
    start_clock: i32,
    playing: bool,
    last_activity: Option<Instant>,
    idle: bool,
    fullscreen: bool,
    tick_job: Option<u64>,
    peaks: Option<(String, Vec<f32>)>,
    /// O painel aberto: "info", "transcript" ou vazio.
    side: String,
    info: Option<VideoInfo>,
    info_requested: bool,
    regions: BTreeSet<usize>,
    track_vols: Vec<i32>,
    pre_mute: Vec<i32>,
    transcript: Option<Transcript>,
    tr_requested: bool,
    tr_active: i32,
    tr_pinned: i32,
    tr_query: String,
    /// As falas da transcrição na tela (índice do turno → instância do `fraTurn`).
    tr_rows: Vec<(usize, FraTurnComponents)>,
    /// O turno com o nome do falante em edição (-1 = nenhum).
    tr_editing: i32,
    /// A lateral acompanha a fala atual. Sai quando o usuário rola para longe dela (fica
    /// onde ele deixou); volta com um clique numa fala ou rolando até a atual de novo.
    tr_follow: bool,
    captions_on: bool,
    cap_cache: Option<(i32, f32, Vec<(String, f64, f64)>)>,
    /// Texto, fonte, largura do palco e folga de baixo da legenda na tela (refaz só se mudar).
    cap_layout: Option<(String, f32, f32, f32)>,
    cap_notify: bool,
    /// O menu de velocidade aberto (em cima do botão).
    speed_open: bool,
    /// Quando começou o pulso de pausa (e o timer que o anima).
    flash: Option<(Instant, u64)>,
}

thread_local! {
    static ST: RefCell<State> = RefCell::new(State {
        zoom: 1.0,
        master: 100,
        speed: 1.0,
        scale: 1.0,
        tr_active: -1,
        tr_pinned: -1,
        tr_editing: -1,
        tr_follow: true,
        ..Default::default()
    });
}

fn with<R>(f: impl FnOnce(&mut State) -> R) -> R {
    ST.with(|s| f(&mut s.borrow_mut()))
}

/// A escala da janela (DPI / 96): o motor trabalha em pixels físicos.
pub(crate) fn window_scale() -> f32 {
    use windows::Win32::Foundation::HWND;
    use windows::Win32::UI::HiDpi::GetDpiForWindow;
    let Some(h) = main_form().and_then(aylis::platform::win::hwnd) else { return 1.0 };
    let dpi = unsafe { GetDpiForWindow(HWND(h as *mut _)) };
    if dpi == 0 { 1.0 } else { dpi as f32 / 96.0 }
}

/// `mm:ss` ou `h:mm:ss`.
fn fmt_time(s: f64) -> String {
    let s = s.max(0.0).round() as u64;
    let (h, m, ss) = (s / 3600, (s % 3600) / 60, s % 60);
    if h > 0 {
        format!("{h}:{m:02}:{ss:02}")
    } else {
        format!("{m:02}:{ss:02}")
    }
}

/// Hora do relógio, `HH:MM:SS`, volta à meia-noite.
fn fmt_clock(sec: i64) -> String {
    let s = sec.rem_euclid(86400);
    format!("{:02}:{:02}:{:02}", s / 3600, (s % 3600) / 60, s % 60)
}

/// `1×`, `0,5×`, `1,5×` (vírgula).
fn speed_label(r: f64) -> String {
    if (r - 1.0).abs() < 1e-9 {
        return "1×".into();
    }
    format!("{}×", format!("{r}").replace('.', ","))
}

fn view_spec(st: &State) -> ViewSpec {
    let k = st.scale as f64;
    ViewSpec {
        stage_w: (st.stage.0 as f64 * k).round() as i32,
        stage_h: (st.stage.1 as f64 * k).round() as i32,
        zoom: st.zoom,
        pan_x: st.pan.0 * k,
        pan_y: st.pan.1 * k,
        regions: selected_regions(st),
    }
}

/// As regiões escolhidas, em pixels do VÍDEO (o layout é do canvas da
/// gravação; um arquivo exportado menor tem o vídeo reduzido).
fn selected_regions(st: &State) -> Vec<Region> {
    let Some(info) = &st.info else { return Vec::new() };
    let Some(lay) = &info.layout else { return Vec::new() };
    let (vw, vh) = info.streams.iter().find(|s| s.kind == "video").map(|s| (s.width.unwrap_or(0), s.height.unwrap_or(0))).unwrap_or((0, 0));
    let sx = if lay.canvas_w > 0 && vw > 0 { vw as f64 / lay.canvas_w as f64 } else { 1.0 };
    let sy = if lay.canvas_h > 0 && vh > 0 { vh as f64 / lay.canvas_h as f64 } else { 1.0 };
    st.regions
        .iter()
        .filter_map(|&i| lay.regions.get(i))
        .map(|r| Region {
            x: (r.x as f64 * sx).round() as i32,
            y: (r.y as f64 * sy).round() as i32,
            w: (r.w as f64 * sx).round() as i32,
            h: (r.h as f64 * sy).round() as i32,
            webcam: r.kind == "webcam",
        })
        .collect()
}

fn push_view(ui: &mut Ui) {
    let (zoom, stage_w) = with(|st| (st.zoom, st.stage.0));
    if let Some(frm) = FrmPlayer::components(ui) {
        frm.lblPlayerZoom.set_visible(ui, zoom > 1.001);
    }
    if zoom > 1.001 {
        let text = format!("{}%", (zoom * 100.0).round());
        let w = text.chars().count() as f32 * 7.0 + 16.0;
        if let Some(frm) = FrmPlayer::components(ui) {
            frm.lblPlayerZoom.set_text(ui, text);
        }
        if let Some(cb) = FrmPlayer::components(ui).map(|frm| frm.lblPlayerZoom.id).and_then(|id| ui.control_mut(id)) {
            cb.set_width(w);
            cb.set_position(stage_w - 12.0 - w, 56.0);
        }
    }
    let v = with(|st| view_spec(st));
    with(|st| {
        if let Some(pb) = &st.pb {
            pb.set_view(v);
        }
    });
    sync_frame_shift(ui);
}

/// Desloca o quadro mostrado pela diferença entre o deslocamento atual e o da
/// vista que o gerou (só com o mesmo zoom).
fn sync_frame_shift(ui: &mut Ui) {
    let (dx, dy) = with(|st| {
        let (z, px, py) = st.frame_view;
        if (z - st.zoom).abs() < 1e-9 { (st.pan.0 - px, st.pan.1 - py) } else { (0.0, 0.0) }
    });
    if let Some(v) = FrmPlayer::components(ui).map(|frm| frm.frmVideo.id).and_then(|id| ui.get_mut::<FrameView>(id)) {
        v.set_shift(dx as f32, dy as f32);
    }
    ui.invalidate();
}

fn push_volumes(ui: &mut Ui) {
    let (m, muted) = with(|st| {
        let master = if st.muted { 0.0 } else { st.master as f32 / 100.0 };
        let tracks: Vec<f32> = if st.track_vols.is_empty() { vec![1.0] } else { st.track_vols.iter().map(|v| *v as f32 / 100.0).collect() };
        if let Some(pb) = &st.pb {
            pb.set_volumes(master, tracks);
        }
        (st.master, st.muted)
    });
    let v = if muted { 0 } else { m };
    if let Some(frm) = FrmPlayer::components(ui) {
        frm.trkVolume.set_value(ui, v as f64);
        frm.lblVolPct.set_text(ui, format!("{v}%"));
        frm.lblVolPct.set_text_color(ui, if v > 100 { "danger_2" } else { "" });
        frm.btnMute.set_icon(ui, if v == 0 { "pl-vol-off" } else if v < 50 { "pl-vol-low" } else { "pl-vol-high" });
    }
}

// ---------------------------------------------------------------------------
// Abrir / fechar
// ---------------------------------------------------------------------------

/// Abre a gravação no player (clique no cartão).
pub fn open(ui: &mut Ui, id: &str) {
    if !bridge::check_recording_for_player(id) {
        return;
    }
    close_engine(ui);
    let title = std::path::Path::new(id).file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
    let start_clock = bridge::recording_start_clock_sec(id);
    crate::app_view::show_form(ui, FrmPlayer::components(ui).map(|frm| frm.form), true);
    if let Some(frm) = FrmPlayer::components(ui) {
        frm.lblPlayerTitle.set_text(ui, title);
        frm.lblPlayerLoading.set_text(ui, lang::tr("Preparando vídeo..."));
        frm.pnlPlayerLoading.set_visible(ui, true);
        frm.lblPlayerTime.set_text(ui, "00:00 / 00:00");
        frm.lblPlayerClock.set_text(ui, "");
        frm.lyPlayerClock.set_visible(ui, start_clock >= 0);
        frm.pnlPlayerPaused.set_visible(ui, false);
        frm.btnPlay.set_icon(ui, "pl-pause");
        frm.wavPlayer.set_paused(ui, false);
        frm.lblCaption.set_visible(ui, false);
        frm.lblPlayerZoom.set_visible(ui, false);
        frm.pnlPauseFlash.set_visible(ui, false);
    }
    close_popups(ui);
    if let Some(frm) = FrmPlayer::components(ui) {
        frm.pnlPlayerSide.set_visible(ui, false);
    }
    set_controls_visible(ui, true);
    if let Some(v) = FrmPlayer::components(ui).map(|frm| frm.frmVideo.id).and_then(|id| ui.get_mut::<FrameView>(id)) {
        v.set_frame(None);
    }
    if let Some(w) = FrmPlayer::components(ui).map(|frm| frm.wavPlayer.id).and_then(|id| ui.get_mut::<Waveform>(id)) {
        w.set_peaks(Vec::new());
        w.position = 0.0;
    }
    ui.layout();
    let stage = FrmPlayer::components(ui).map(|frm| frm.frmVideo.id).and_then(|id| ui.bounds(id)).map_or((1280.0, 720.0), |b| (b.width, b.height));
    let captions = config::get_config_bool("playerCaptions", false);
    let view = with(|st| {
        st.id = id.to_owned();
        st.stage = stage;
        st.scale = window_scale();
        st.zoom = 1.0;
        st.pan = (0.0, 0.0);
        // Volume geral volta a 100 % em toda abertura.
        st.master = 100;
        st.muted = false;
        st.speed = 1.0;
        st.duration = 0.0;
        st.start_clock = start_clock;
        st.playing = true;
        st.last_activity = Some(Instant::now());
        st.idle = false;
        // Tudo do vídeo anterior sai (a região, a transcrição).
        st.side.clear();
        st.info = None;
        st.info_requested = false;
        st.regions.clear();
        st.track_vols.clear();
        st.pre_mute.clear();
        st.transcript = None;
        st.tr_requested = false;
        st.tr_active = -1;
        st.tr_pinned = -1;
        st.tr_rows.clear();
        st.cap_cache = None;
        st.cap_layout = None;
        st.captions_on = captions;
        view_spec(st)
    });
    set_speed(ui, 1.0);
    sync_cc(ui);
    if captions {
        request_transcript();
    }
    let id_out = id.to_owned();
    let out: Arc<dyn Fn(Out) + Send + Sync> = crate::app_view::frame_sink(id_out, on_out);
    let pb = Playback::open(id, view, out);
    with(|st| st.pb = Some(pb));
    push_volumes(ui);
    tick(ui);
    // Forma de onda: pedida um instante depois (o vídeo tem prioridade).
    let wid = id.to_owned();
    ui.set_timeout(100, move |ui| {
        if with(|st| st.id == wid) {
            if with(|st| st.peaks.as_ref().is_some_and(|(i, _)| i == &wid)) {
                render_waveform(ui);
            } else {
                bridge::handle_request_waveform(&wid, WAVE_BUCKETS, false);
            }
        }
    });
    if let Some(stage) = FrmPlayer::components(ui).map(|frm| frm.frmVideo.id) {
        ui.set_focus(stage);
    }
    bridge::set_player_open(true);
    log!("Player: abrindo {id}");
}

fn close_engine(ui: &mut Ui) {
    let (pb, job) = with(|st| (st.pb.take(), st.tick_job.take()));
    if let Some(job) = job {
        ui.clear_timeout(job);
    }
    // Soltar o handle fecha os canais; as threads saem sozinhas.
    drop(pb);
}

pub fn close(ui: &mut Ui) {
    close_engine(ui);
    if with(|st| st.fullscreen) {
        toggle_fullscreen(ui);
    }
    crate::app_view::show_form(ui, FrmPlayer::components(ui).map(|frm| frm.form), false);
    if let Some(v) = FrmPlayer::components(ui).map(|frm| frm.frmVideo.id).and_then(|id| ui.get_mut::<FrameView>(id)) {
        v.set_frame(None);
    }
    with(|st| {
        st.id.clear();
        st.tr_query.clear();
    });
    bridge::set_player_open(false);
}

pub fn is_open() -> bool {
    with(|st| !st.id.is_empty())
}

// ---------------------------------------------------------------------------
// Do motor
// ---------------------------------------------------------------------------

fn on_out(ui: &mut Ui, id: &str, o: Out) {
    if with(|st| st.id != id) {
        return;
    }
    match o {
        Out::Info(i) => {
            with(|st| st.duration = i.duration);
            if let Some(frm) = FrmPlayer::components(ui) {
                frm.pnlPlayerLoading.set_visible(ui, false);
            }
        }
        Out::Frame { buf, x, y, view } => {
            let k = with(|st| st.scale);
            let (bw, bh) = (buf.width, buf.height);
            let image = Image::bgra(bw, bh, buf.pixels);
            let rect = RectF::new(x as f32 / k, y as f32 / k, bw as f32 / k, bh as f32 / k);
            if let Some(v) = FrmPlayer::components(ui).map(|frm| frm.frmVideo.id).and_then(|id| ui.get_mut::<FrameView>(id)) {
                v.set_frame(Some((image, rect)));
            }
            with(|st| st.frame_view = (view.0, view.1 / k as f64, view.2 / k as f64));
            sync_frame_shift(ui);
            if let Some(frm) = FrmPlayer::components(ui) {
                frm.pnlPlayerLoading.set_visible(ui, false);
            }
            with(|st| {
                if let Some(pb) = &st.pb {
                    pb.shared.ui_pending.store(false, std::sync::atomic::Ordering::Release);
                }
            });
        }
        Out::Ended => {
            with(|st| {
                if let Some(pb) = &st.pb {
                    pb.pause();
                }
            });
            set_playing_ui(ui, false);
        }
        Out::Error(e) => {
            if let Some(frm) = FrmPlayer::components(ui) {
                frm.lblPlayerLoading.set_text(ui, lang::tr("Erro ao carregar vídeo."));
                frm.pnlPlayerLoading.set_visible(ui, true);
            }
            log!("Player: erro {e}");
        }
    }
}

fn set_playing_ui(ui: &mut Ui, playing: bool) {
    let was = with(|st| {
        let was = st.playing;
        st.playing = playing;
        st.last_activity = Some(Instant::now());
        was
    });
    if let Some(frm) = FrmPlayer::components(ui) {
        frm.btnPlay.set_icon(ui, if playing { "pl-pause" } else { "pl-play" });
        frm.pnlPlayerPaused.set_visible(ui, !playing);
        frm.wavPlayer.set_paused(ui, !playing);
    }
    if was && !playing {
        start_flash(ui);
    }
    set_controls_visible(ui, true);
}

/// O pulso do indicador central na pausa: sobe em 15 %, segura até 58 %, some.
fn start_flash(ui: &mut Ui) {
    if let Some((_, job)) = with(|st| st.flash.take()) {
        ui.clear_timeout(job);
    }
    if let Some(frm) = FrmPlayer::components(ui) {
        frm.pnlPauseFlash.set_visible(ui, true);
    }
    let job = ui.set_timeout(0, flash_step);
    with(|st| st.flash = Some((Instant::now(), job)));
}

fn flash_step(ui: &mut Ui) {
    let Some((at, _)) = with(|st| st.flash) else { return };
    let k = (at.elapsed().as_secs_f64() * 1000.0 / FLASH_MS).min(1.0);
    let op = if k < 0.15 { k / 0.15 * 0.95 } else if k < 0.58 { 0.95 } else { (1.0 - k) / 0.42 * 0.95 };
    if let Some(frm) = FrmPlayer::components(ui) {
        frm.pnlPauseFlash.set_opacity(ui, op);
    }
    if k >= 1.0 {
        if let Some(frm) = FrmPlayer::components(ui) {
            frm.pnlPauseFlash.set_visible(ui, false);
        }
        with(|st| st.flash = None);
        return;
    }
    let job = ui.set_timeout(16, flash_step);
    with(|st| {
        if let Some(f) = st.flash.as_mut() {
            f.1 = job;
        }
    });
}

fn set_controls_visible(ui: &mut Ui, on: bool) {
    let was_idle = with(|st| std::mem::replace(&mut st.idle, !on));
    if was_idle == !on {
        return;
    }
    if let Some(frm) = FrmPlayer::components(ui) {
        frm.lyChrome.set_visible(ui, on);
    }
    if !on {
        close_popups(ui);
    }
    // A legenda desce quando os controles somem.
    with(|st| st.cap_layout = None);
    sync_caption(ui);
    ui.layout();
    if let Some(cb) = FrmPlayer::components(ui).map(|frm| frm.frmVideo.id).and_then(|id| ui.control_mut(id)) {
        cb.set_cursor(if on { CursorShape::Default } else { CursorShape::None });
    }
}

/// Barra, tempo e relógio a ~30 Hz.
fn tick(ui: &mut Ui) {
    let (pos, dur, clock, playing, idle_due, hold) = with(|st| {
        let pos = st.pb.as_ref().map_or(0.0, |p| p.shared.position());
        let dur = st.pb.as_ref().map_or(st.duration, |p| p.shared.duration().max(st.duration));
        let due = st.last_activity.is_some_and(|t| t.elapsed() >= Duration::from_millis(IDLE_MS));
        (pos, dur, st.start_clock, st.playing, due, !st.side.is_empty() || st.speed_open)
    });
    let hold = hold || FrmPlayer::components(ui).is_some_and(|frm| hover_in(ui, &[frm.pnlPlayerBar.id, frm.pnlPlayerHead.id, frm.lyVolPop.id, frm.lyCcPop.id]));
    sync_popups(ui);
    if let Some(w) = FrmPlayer::components(ui).map(|frm| frm.wavPlayer.id).and_then(|id| ui.get_mut::<Waveform>(id)) {
        w.duration = dur.max(0.001);
        w.position = pos;
    }
    if let Some(frm) = FrmPlayer::components(ui) {
        frm.lblPlayerTime.set_text(ui, format!("{} / {}", fmt_time(pos), fmt_time(dur)));
        frm.lblPlayerClock.set_text(ui, if clock >= 0 { fmt_clock(clock as i64 + pos.floor() as i64) } else { String::new() });
    }
    // Os controles somem parado o mouse (só tocando, sem painel aberto).
    if playing && idle_due && !hold && !with(|st| st.idle) {
        set_controls_visible(ui, false);
    }
    sync_transcript_active(ui, false);
    sync_caption(ui);
    let job = ui.set_timeout(33, tick);
    with(|st| st.tick_job = Some(job));
}

// ---------------------------------------------------------------------------
// Forma de onda
// ---------------------------------------------------------------------------

pub fn on_waveform(id: &str, peaks: &[f32]) {
    let (id, peaks) = (id.to_owned(), peaks.to_vec());
    with_ui(move |ui| {
        let current = with(|st| {
            st.peaks = Some((id.clone(), peaks));
            st.id == id
        });
        if current {
            render_waveform(ui);
        }
    });
}

/// Reagrega os picos pro número de barras que cabem (raiz quadrada: realça
/// as variações; mínimo de 4 % pra silêncio ter linha).
fn render_waveform(ui: &mut Ui) {
    let Some((id, peaks)) = with(|st| st.peaks.clone()) else { return };
    if with(|st| st.id != id) || peaks.is_empty() {
        return;
    }
    let width = FrmPlayer::components(ui).map(|frm| frm.wavPlayer.id).and_then(|w| ui.bounds(w)).map_or(600.0, |b| b.width);
    let bars = ((width * window_scale() / 2.0) as usize).clamp(1, peaks.len());
    let agg: Vec<f32> = (0..bars)
        .map(|i| {
            let a = i * peaks.len() / bars;
            let b = ((i + 1) * peaks.len() / bars).max(a + 1);
            peaks[a..b.min(peaks.len())].iter().cloned().fold(0.0, f32::max)
        })
        .collect();
    let maxp = agg.iter().cloned().fold(0.0001f32, f32::max);
    let norm: Vec<f32> = agg.iter().map(|p| (p / maxp).sqrt().max(0.04)).collect();
    if let Some(w) = FrmPlayer::components(ui).map(|frm| frm.wavPlayer.id).and_then(|id| ui.get_mut::<Waveform>(id)) {
        w.set_peaks(norm);
    }
    ui.invalidate();
}

// ---------------------------------------------------------------------------
// Gestos
// ---------------------------------------------------------------------------

pub fn toggle_play(ui: &mut Ui) {
    let playing = with(|st| {
        let pb = st.pb.as_ref()?;
        if pb.playing() {
            pb.pause();
            Some(false)
        } else {
            pb.play();
            Some(true)
        }
    });
    if let Some(p) = playing {
        set_playing_ui(ui, p);
    }
}

pub fn seek_to(_ui: &mut Ui, t: f64) {
    with(|st| {
        if let Some(pb) = &st.pb {
            let d = pb.shared.duration().max(st.duration);
            pb.seek(t.clamp(0.0, d));
        }
    });
}

fn seek_by(delta: f64) {
    with(|st| {
        if let Some(pb) = &st.pb {
            let d = pb.shared.duration().max(st.duration);
            pb.seek((pb.shared.position() + delta).clamp(0.0, d));
        }
    });
}

fn set_speed(ui: &mut Ui, r: f64) {
    with(|st| {
        st.speed = r;
        if let Some(pb) = st.pb.as_mut() {
            pb.set_rate(r);
        }
    });
    // De 8× pra cima (saltos entre quadros-chave): âmbar com um ponto.
    let trick = r >= TRICK_MIN_RATE;
    let label = if trick { format!("{} •", speed_label(r)) } else { speed_label(r) };
    let width = (label.chars().count() as f32 * 7.5 + 16.0).max(42.0);
    if let Some(cb) = FrmPlayer::components(ui).map(|frm| frm.btnSpeed.id).and_then(|id| ui.control_mut(id)) {
        cb.set_width(width);
    }
    if let Some(frm) = FrmPlayer::components(ui) {
        frm.btnSpeed.set_text(ui, label);
        frm.btnSpeed.set_style(ui, if trick { "pl_trick" } else { "pl_speed" });
    }
    let hint = if trick { lang::tr("Velocidade de execução — a partir de 8× sem som; se a placa de vídeo não acompanhar, o vídeo avança por saltos entre quadros-chave") } else { lang::tr("Velocidade de execução") };
    if let Some(frm) = FrmPlayer::components(ui) {
        frm.btnSpeed.set_hint(ui, hint);
    }
    close_speed(ui);
}

/// O menu de velocidades, em cima do botão (abre e fecha no clique).
pub fn speed_menu(ui: &mut Ui, _sender: ComponentId) {
    if with(|st| st.speed_open) {
        close_speed(ui);
        return;
    }
    let Some(frm) = FrmPlayer::components(ui) else { return };
    let current = with(|st| st.speed);
    for (item, s) in speed_items(&frm).into_iter().zip(SPEEDS) {
        item.set_style(ui, if (s - current).abs() < 1e-9 { "pl_item_on" } else { "pl_item" });
    }
    with(|st| st.speed_open = true);
    frm.btnSpeed.set_checked(ui, true);
    place_popup(ui, frm.pnlSpeedPop.id, frm.btnSpeed.id, 8.0);
}

/// Os itens do menu de velocidade (declarados no form), na ordem de `SPEEDS`.
fn speed_items(frm: &FrmPlayerComponents) -> [g::Button; SPEEDS.len()] {
    [frm.btnSpeed025, frm.btnSpeed05, frm.btnSpeed1, frm.btnSpeed15, frm.btnSpeed2, frm.btnSpeed3, frm.btnSpeed8, frm.btnSpeed16, frm.btnSpeed32]
}

fn close_speed(ui: &mut Ui) {
    with(|st| st.speed_open = false);
    if let Some(frm) = FrmPlayer::components(ui) {
        frm.pnlSpeedPop.set_visible(ui, false);
        frm.btnSpeed.set_checked(ui, false);
    }
}

/// Põe o popup centrado em cima do botão, `gap` acima dele (coordenadas do palco).
fn place_popup(ui: &mut Ui, p: ComponentId, b: ComponentId, gap: f32) {
    let Some(screen) = FrmPlayer::components(ui).map(|frm| frm.form.id) else { return };
    // Por cima de tudo da tela do player (o palco e os painéis laterais de legendas e de
    // informações): filho da própria tela, o último (desenhado por cima). Dentro do palco, o
    // popup ficava preso à largura dele e era empurrado para a esquerda com um painel aberto.
    if ui.parent(p) != Some(screen) {
        let _ = ui.set_parent(p, Some(screen));
    }
    let last = ui.children(screen).len();
    let _ = ui.set_child_index(p, last);
    // Em dia (o botão pode ter acabado de mudar de lugar com um painel abrindo).
    let _ = ui.bounds(b);
    let (Some(br), Some(sr)) = (ui.absolute_rect(b), ui.absolute_rect(screen)) else { return };
    let Some(cb) = ui.control_mut(p) else { return };
    let (w, h) = (cb.width(), cb.height());
    // Centrado no botão; só encosta na borda da tela, nunca na do palco.
    let x = (br.x - sr.x + (br.width - w) / 2.0).min(sr.width - w - 4.0).max(4.0);
    cb.set_align(Align::None).set_position(x, br.y - sr.y - gap - h);
    cb.set_visible(true);
    ui.layout();
}

fn close_popups(ui: &mut Ui) {
    close_speed(ui);
    if let Some(frm) = FrmPlayer::components(ui) {
        frm.lyVolPop.set_visible(ui, false);
        frm.lyCcPop.set_visible(ui, false);
    }
}

/// O ponteiro está sobre algum destes (ou dentro deles)?
fn hover_in(ui: &Ui, targets: &[ComponentId]) -> bool {
    let Some(form) = main_form() else { return false };
    let mut cur = ui.hovered(form);
    while let Some(id) = cur {
        if targets.contains(&id) {
            return true;
        }
        cur = ui.parent(id);
    }
    false
}

/// Volume e legendas abrem no hover do botão e ficam abertos com o mouse no popup
/// (o popup cobre o vão até o botão).
fn sync_popups(ui: &mut Ui) {
    let Some(frm) = FrmPlayer::components(ui) else { return };
    for (button, popup) in [(frm.btnMute.id, frm.lyVolPop), (frm.btnCaptions.id, frm.lyCcPop)] {
        let open = popup.visible(ui);
        let hover = hover_in(ui, &[button, popup.id]);
        if hover && !open && !with(|st| st.idle) {
            if popup == frm.lyCcPop {
                sync_cc(ui);
            }
            place_popup(ui, popup.id, button, 0.0);
        } else if !hover && open {
            popup.set_visible(ui, false);
        }
    }
}

/// O botão CC aceso (legenda ou painel de transcrição) e as marcas do menu.
fn sync_cc(ui: &mut Ui) {
    let (cap, tr) = with(|st| (st.captions_on, st.side == "transcript"));
    let Some(frm) = FrmPlayer::components(ui) else { return };
    frm.btnCaptions.set_checked(ui, cap || tr);
    for (on, check, label, key) in [(cap, frm.lblCcCheck1, frm.lblCcCaptions, Some(frm.lblCcKey1)), (tr, frm.lblCcCheck2, frm.lblCcTranscript, None)] {
        check.set_text(ui, if on { "✓" } else { "" });
        let style = if on { "pl_opt_on" } else { "pl_opt" };
        check.set_style(ui, style);
        label.set_style(ui, style);
        if let Some(key) = key {
            key.set_style(ui, if on { "pl_key_on" } else { "pl_key" });
        }
    }
}

pub fn set_volume(ui: &mut Ui, v: f64) {
    with(|st| {
        st.master = (v.round() as i32).clamp(0, 200);
        st.muted = st.master == 0;
    });
    push_volumes(ui);
}

pub fn toggle_mute(ui: &mut Ui) {
    with(|st| st.muted = !st.muted);
    push_volumes(ui);
}

pub fn toggle_fullscreen(ui: &mut Ui) {
    crate::ui::toggle_fullscreen();
    let on = with(|st| {
        st.fullscreen = !st.fullscreen;
        st.fullscreen
    });
    if let Some(frm) = FrmPlayer::components(ui) {
        frm.btnFullscreen.set_icon(ui, if on { "pl-fs-exit" } else { "pl-fs-enter" });
    }
}

fn clamp_pan(st: &mut State) {
    let (w, h) = (st.stage.0 as f64, st.stage.1 as f64);
    st.pan.0 = st.pan.0.clamp((w - w * st.zoom).min(0.0), 0.0);
    st.pan.1 = st.pan.1.clamp((h - h * st.zoom).min(0.0), 0.0);
}

/// O palco mudou de tamanho (janela, tela cheia, painel).
pub fn stage_resized(ui: &mut Ui) {
    let Some(b) = FrmPlayer::components(ui).map(|frm| frm.frmVideo.id).and_then(|id| ui.bounds(id)) else { return };
    let changed = with(|st| {
        let new = (b.width, b.height);
        let changed = st.stage != new;
        st.stage = new;
        st.scale = window_scale();
        clamp_pan(st);
        changed && !st.id.is_empty()
    });
    if changed {
        push_view(ui);
        render_waveform(ui);
    }
}

/// Roda sobre o palco: zoom ancorado no ponto do mouse.
pub fn wheel(ui: &mut Ui, e: &WheelEvent) {
    activity(ui);
    with(|st| {
        let (cx, cy) = (e.position.x as f64, e.position.y as f64);
        let vx = (cx - st.pan.0) / st.zoom;
        let vy = (cy - st.pan.1) / st.zoom;
        let factor = if e.delta.y > 0.0 { 1.15 } else { 1.0 / 1.15 };
        st.zoom = (st.zoom * factor).clamp(1.0, ZOOM_MAX);
        st.pan = (cx - vx * st.zoom, cy - vy * st.zoom);
        clamp_pan(st);
    });
    push_view(ui);
}

pub fn reset_zoom(ui: &mut Ui) {
    with(|st| {
        st.zoom = 1.0;
        st.pan = (0.0, 0.0);
    });
    push_view(ui);
}

/// Botão no palco: com zoom arrasta; sem, o clique pausa/toca.
pub fn stage_down(ui: &mut Ui, e: &PointerEvent) {
    if with(|st| st.speed_open) {
        close_speed(ui);
    }
    activity(ui);
    with(|st| st.pan_start = Some((st.pan, e.position)));
}

pub fn stage_move(ui: &mut Ui, e: &PointerEvent) {
    activity(ui);
    let moved = with(|st| {
        let Some((pan, from)) = st.pan_start else { return false };
        if st.zoom <= 1.0 {
            return false;
        }
        st.pan = (pan.0 + (e.position.x - from.x) as f64, pan.1 + (e.position.y - from.y) as f64);
        clamp_pan(st);
        true
    });
    if moved {
        push_view(ui);
    }
}

pub fn stage_up(ui: &mut Ui, e: &PointerEvent) {
    let click = with(|st| {
        let start = st.pan_start.take();
        start.is_some_and(|(_, from)| (from.x - e.position.x).abs() < 4.0 && (from.y - e.position.y).abs() < 4.0)
    });
    if click && e.button == PointerButton::Primary {
        toggle_play(ui);
    }
}

pub fn activity(ui: &mut Ui) {
    with(|st| st.last_activity = Some(Instant::now()));
    if with(|st| st.idle) {
        set_controls_visible(ui, true);
    }
}

/// O teclado do player (Esc, espaço, setas, M, F, 0, C).
pub fn key(ui: &mut Ui, k: &KeyEvent) -> bool {
    if !is_open() {
        return false;
    }
    let plain = !k.modifiers.ctrl && !k.modifiers.alt;
    match k.key {
        Key::Escape => {
            if with(|st| st.fullscreen) {
                toggle_fullscreen(ui);
            } else {
                close(ui);
            }
        }
        Key::Space => toggle_play(ui),
        Key::Char('m') | Key::Char('M') if plain => toggle_mute(ui),
        Key::Char('f') | Key::Char('F') if plain => toggle_fullscreen(ui),
        Key::Char('0') if plain => reset_zoom(ui),
        Key::Char('c') | Key::Char('C') if plain => toggle_captions(ui),
        Key::Left | Key::Right => {
            let dir = if k.key == Key::Right { 1 } else { -1 };
            let playing = with(|st| st.pb.as_ref().is_some_and(|p| p.playing()));
            if playing {
                // Tocando: 5 s. Pausado: um quadro.
                seek_by(5.0 * dir as f64);
            } else {
                with(|st| {
                    if let Some(pb) = &st.pb {
                        pb.step(dir);
                    }
                });
            }
        }
        _ => return false,
    }
    activity(ui);
    true
}

// ---------------------------------------------------------------------------
// Painéis (informações e transcrição)
// ---------------------------------------------------------------------------

fn fmt_bitrate(bps: i64) -> String {
    if bps <= 0 {
        "—".into()
    } else if bps >= 1_000_000 {
        format!("{:.2} Mbps", bps as f64 / 1e6)
    } else if bps >= 1000 {
        format!("{} kbps", (bps as f64 / 1000.0).round())
    } else {
        format!("{bps} bps")
    }
}

fn fmt_fps(fps: f64) -> String {
    if fps <= 0.0 {
        return "—".into();
    }
    let r = fps.round();
    if (fps - r).abs() < 0.01 { format!("{r} fps") } else { format!("{fps:.2} fps") }
}

fn fmt_bytes(b: i64) -> String {
    const KB: f64 = 1024.0;
    let f = b as f64;
    if f >= KB * KB * KB {
        format!("{:.1} GB", f / (KB * KB * KB))
    } else if f >= KB * KB {
        format!("{} MB", (f / (KB * KB)).round())
    } else if f >= KB {
        format!("{} KB", (f / KB).round())
    } else {
        format!("{b} B")
    }
}

fn fmt_duration(sec: i64) -> String {
    if sec <= 0 {
        return String::new();
    }
    let (h, m, s) = (sec / 3600, (sec % 3600) / 60, sec % 60);
    if h > 0 { format!("{h:02}:{m:02}:{s:02}") } else { format!("{m:02}:{s:02}") }
}

/// Limpa o painel lateral para um conteúdo novo: as peças (frames) do anterior saem numa
/// passada só; as mensagens fixas são do form e só se escondem.
fn side_panel(ui: &mut Ui) -> Option<ComponentId> {
    let frm = FrmPlayer::components(ui)?;
    let p = frm.lySide.id;
    let fixed = [frm.lblSideLoading.id, frm.lblSideEmpty.id];
    let old: Vec<ComponentId> = ui.children(p).iter().copied().filter(|c| !fixed.contains(c)).collect();
    ui.free_many(&old);
    frm.lblSideLoading.set_visible(ui, false);
    frm.lblSideEmpty.set_visible(ui, false);
    // O respiro do form (a transcrição troca por um próprio, e as informações abertas depois
    // dela ficavam encostadas nas bordas).
    if let Some(cb) = ui.control_mut(p) {
        cb.set_padding([14.0, 14.0, 14.0, 14.0]);
    }
    Some(p)
}

/// Uma linha "chave ... valor" (o valor à direita): 4 + 16,8 + 4. É o frame `fraInfoRow`;
/// aqui só os textos e a largura da chave.
fn add_row(ui: &mut Ui, parent: ComponentId, key: &str, val: &str) {
    let Ok(row) = FraInfoRow::create_in(ui, parent) else { return };
    row.lblKey.set_text(ui, key);
    // Do tamanho do texto (a fonte do `pl_ikey`): o valor fica com o resto da linha (um
    // nome de arquivo longo cortava cedo com a coluna fixa de 110).
    let mut st = TextStyle::default();
    st.font.family = "Segoe UI".into();
    st.font.size = 11.0;
    let w = (ui.measure_text(key, &st).width + 2.0).ceil().min(110.0);
    row.lblKey.set_width(ui, f64::from(w));
    row.lblValue.set_text(ui, val);
    // Cortado, o valor inteiro aparece com o mouse em cima.
    row.lblValue.set_hint(ui, val);
}

/// Título de seção (o frame `fraInfoSection`): maiúsculo, linha fina 4 px abaixo e 8 de
/// folga; `top` é o vão desde a seção anterior.
fn add_section(ui: &mut Ui, parent: ComponentId, title: &str, top: f32) {
    let Ok(sec) = FraInfoSection::create_in(ui, parent) else { return };
    sec.lblTitle.set_text(ui, title);
    if let Some(cb) = ui.control_mut(sec.form.id) {
        cb.set_margins([0.0, top, 0.0, 0.0]);
    }
}

/// A altura do conteúdo do painel (só o que está visível: as mensagens do form ficam
/// escondidas entre as peças).
fn fit_side(ui: &mut Ui) {
    let Some(side) = FrmPlayer::components(ui).map(|frm| frm.lySide.id) else { return };
    let h: f32 = ui
        .children(side)
        .iter()
        .filter_map(|&id| ui.control(id).filter(|cb| cb.visible()).map(|cb| cb.height() + cb.margins().top + cb.margins().bottom))
        .sum();
    if let Some(cb) = ui.control_mut(side) {
        cb.set_height(h + 24.0);
    }
    ui.layout();
}

fn open_side(ui: &mut Ui, which: &str) {
    let opening = with(|st| {
        let opening = st.side != which;
        st.side = if opening { which.to_owned() } else { String::new() };
        opening
    });
    if let Some(frm) = FrmPlayer::components(ui) {
        frm.pnlPlayerSide.set_visible(ui, opening);
    }
    if let Some(cb) = FrmPlayer::components(ui).map(|frm| frm.pnlPlayerSide.id).and_then(|id| ui.control_mut(id)) {
        cb.set_width(if which == "info" { 320.0 } else { 360.0 });
    }
    if let Some(frm) = FrmPlayer::components(ui) {
        frm.btnInfo.set_checked(ui, opening && which == "info");
    }
    sync_cc(ui);
    if let Some(frm) = FrmPlayer::components(ui) {
        frm.lyTrSearch.set_visible(ui, which == "transcript");
    }
    ui.layout();
    stage_resized(ui);
}

pub fn toggle_info(ui: &mut Ui) {
    open_side(ui, "info");
    if with(|st| st.side != "info") {
        return;
    }
    if let Some(frm) = FrmPlayer::components(ui) {
        frm.lblSideTitle.set_text(ui, lang::tr("Informações do vídeo"));
    }
    let (need, id) = with(|st| (st.info.is_none() && !st.info_requested, st.id.clone()));
    if need {
        with(|st| st.info_requested = true);
        if side_panel(ui).is_some() {
            if let Some(frm) = FrmPlayer::components(ui) {
                frm.lblSideLoading.set_visible(ui, true);
            }
        }
        bridge::handle_request_video_info(&id);
    } else {
        render_info(ui);
    }
}

pub fn on_video_info(info: &VideoInfo) {
    let info = info.clone();
    with_ui(move |ui| {
        if with(|st| st.id != info.id) {
            return;
        }
        with(|st| {
            let n_audio = info.streams.iter().filter(|s| s.kind == "audio").count();
            if st.track_vols.len() != n_audio {
                st.track_vols = (0..n_audio).map(|i| if i == 0 { 100 } else { 0 }).collect();
                st.pre_mute = vec![0; n_audio];
            }
            let n_regions = info.layout.as_ref().map_or(0, |l| l.regions.len());
            st.regions.retain(|&i| i < n_regions);
            st.info = Some(info);
        });
        if with(|st| st.side == "info") {
            render_info(ui);
        }
    });
}

fn render_info(ui: &mut Ui) {
    let Some(info) = with(|st| st.info.clone()) else { return };
    let Some(p) = side_panel(ui) else { return };
    add_section(ui, p, &lang::tr("Arquivo"), 0.0);
    add_row(ui, p, &lang::tr("Nome"), &info.file_name);
    add_row(ui, p, &lang::tr("Container"), &info.format);
    add_row(ui, p, &lang::tr("Duração"), &fmt_duration(info.duration.round() as i64));
    add_row(ui, p, &lang::tr("Tamanho"), &fmt_bytes(info.size));
    if info.bitrate > 0 {
        add_row(ui, p, &lang::tr("Bitrate total"), &fmt_bitrate(info.bitrate));
    }
    let videos: Vec<_> = info.streams.iter().filter(|s| s.kind == "video").collect();
    if !videos.is_empty() {
        add_section(ui, p, &lang::tr("Vídeo"), 18.0);
        for (i, s) in videos.iter().enumerate() {
            if videos.len() > 1 {
                add_row(ui, p, &lang::tr_with("Stream {{n}}", &[("n", &(i + 1).to_string())]), "");
            }
            add_row(ui, p, &lang::tr("Codec"), &s.codec.to_uppercase());
            if let (Some(wd), Some(ht)) = (s.width, s.height) {
                if wd > 0 && ht > 0 {
                    add_row(ui, p, &lang::tr("Resolução"), &format!("{wd} × {ht}"));
                }
            }
            if s.frame_rate.unwrap_or(0.0) > 0.0 {
                add_row(ui, p, &lang::tr("Taxa de quadros"), &fmt_fps(s.frame_rate.unwrap_or(0.0)));
            }
            if s.bitrate > 0 {
                add_row(ui, p, &lang::tr("Bitrate"), &fmt_bitrate(s.bitrate));
            }
        }
    }
    // Visualização: tela cheia (rádio) + uma caixa por região.
    let mut gap = 18.0;
    if let Some(lay) = info.layout.as_ref().filter(|l| !l.regions.is_empty()) {
        add_section(ui, p, &lang::tr("Visualização"), gap);
        let regions = with(|st| st.regions.clone());
        let mut opts = vec![(-1i32, lang::tr("Tela cheia"), "full".to_owned(), regions.is_empty())];
        for (i, r) in lay.regions.iter().enumerate() {
            let label = if r.name.is_empty() { lang::tr_with("Região {{n}}", &[("n", &(i + 1).to_string())]) } else { r.name.clone() };
            let kind = if r.kind.is_empty() { "monitor".to_owned() } else { r.kind.clone() };
            opts.push((i as i32, label, kind, regions.contains(&i)));
        }
        for (idx, label, kind, on) in opts {
            view_option(ui, p, idx, &label, &kind, on);
        }
        gap = 14.0;
    }
    // Áudio: um cartão por faixa, com o volume dela.
    let audios: Vec<_> = info.streams.iter().filter(|s| s.kind == "audio").collect();
    if !audios.is_empty() {
        let key = if audios.len() == 1 { "Áudio ({{count}} faixa)" } else { "Áudio ({{count}} faixas)" };
        add_section(ui, p, &lang::tr_with(key, &[("count", &audios.len().to_string())]), gap);
        let vols = with(|st| st.track_vols.clone());
        for (i, s) in audios.iter().enumerate() {
            let n = (i + 1).to_string();
            let title = if !s.title.is_empty() {
                s.title.clone()
            } else if i == 0 {
                lang::tr_with("Faixa {{n}} (mix)", &[("n", &n)])
            } else {
                lang::tr_with("Faixa {{n}}", &[("n", &n)])
            };
            let mut rows = vec![(lang::tr("Codec"), s.codec.to_uppercase())];
            if let Some(ch) = s.channels.filter(|c| *c > 0) {
                let v = match ch {
                    1 => lang::tr("Mono"),
                    2 => lang::tr("Estéreo"),
                    n => lang::tr_with("{{count}} canais", &[("count", &n.to_string())]),
                };
                rows.push((lang::tr("Canais"), v));
            }
            if let Some(sr) = s.sample_rate.filter(|r| *r > 0) {
                rows.push((lang::tr("Sample rate"), format!("{:.1} kHz", sr as f64 / 1000.0)));
            }
            if s.bitrate > 0 {
                rows.push((lang::tr("Bitrate"), fmt_bitrate(s.bitrate)));
            }
            track_card(ui, p, i, &title, &rows, vols.get(i).copied().unwrap_or(0));
        }
    }
    fit_side(ui);
}

/// Uma opção da "Visualização" (o frame `fraViewOption`): rádio (tela cheia) ou caixa
/// (região), ícone e nome.
fn view_option(ui: &mut Ui, parent: ComponentId, idx: i32, label: &str, kind: &str, on: bool) {
    let Ok(opt) = FraViewOption::create_in(ui, parent) else { return };
    let full = kind == "full";
    if on {
        opt.pnlOption.set_style(ui, "pl_view_on");
        opt.btnIcon.set_style(ui, "pl_vicon_on");
    }
    let mark = match (full, on) {
        (true, true) => "pl_radio_on",
        (true, false) => "pl_radio",
        (false, true) => "pl_box_on",
        (false, false) => "pl_box",
    };
    opt.pnlMark.set_style(ui, mark);
    // Marcada: a bolinha do rádio ou o ✓ da caixa (os dois estão no frame, escondidos).
    opt.cirDot.set_visible(ui, on && full);
    opt.btnCheck.set_visible(ui, on && !full);
    let icon = match kind {
        "webcam" => "pl-webcam",
        "monitor" => "pl-monitor",
        _ => "pl-expand",
    };
    opt.btnIcon.set_icon(ui, icon);
    opt.lblLabel.set_text(ui, label);
    ui.on(opt.pnlOption.id, "on_click", move |ui, _, _| view_clicked(ui, idx));
}

/// O cartão de uma faixa de áudio (o frame `fraTrackCard`): título, linhas e o volume
/// (0–200 %, mudo).
fn track_card(ui: &mut Ui, parent: ComponentId, i: usize, title: &str, rows: &[(String, String)], vol: i32) {
    let Ok(card) = FraTrackCard::create_in(ui, parent) else { return };
    let muted = vol == 0;
    let rows_h = rows.len() as f32 * 24.8;
    let h = 8.0 + 19.4 + rows_h + 6.0 + 1.0 + 6.0 + 22.0 + 8.0;
    card.form.set_height(ui, f64::from(h));
    if muted {
        card.pnlCard.set_opacity(ui, 0.55);
    }
    card.lblTitle.set_text(ui, title);
    card.lblTitle.set_font_strikeout(ui, muted);
    card.lyRows.set_height(ui, f64::from(rows_h));
    for (k, v) in rows {
        add_row(ui, card.lyRows.id, k, v);
    }
    if muted {
        card.btnMute.set_style(ui, "pl_tmute_off");
        card.btnMute.set_icon(ui, "pl-tvol-off");
        card.btnMute.set_hint(ui, lang::tr("Desmutar"));
    }
    ui.on(card.btnMute.id, "on_click", move |ui, _, _| track_mute(ui, i));
    card.lblPct.set_text(ui, format!("{vol}%"));
    card.trkVolume.set_value(ui, vol as f64);
    let (trk, pct) = (card.trkVolume, card.lblPct);
    ui.on(trk.id, "on_change", move |ui, _, _| {
        let v = trk.value(ui) as i32;
        pct.set_text(ui, format!("{v}%"));
        set_track_volume(ui, i, v);
    });
    ui.on(trk.id, "on_dbl_click", move |ui, _, _| {
        set_track_volume(ui, i, 100);
        ui.set_timeout(0, render_info);
    });
}

fn view_clicked(ui: &mut Ui, idx: i32) {
    with(|st| {
        if idx < 0 {
            st.regions.clear();
        } else if !st.regions.remove(&(idx as usize)) {
            st.regions.insert(idx as usize);
        }
        // Mudou a área visível: o zoom anterior não vale mais.
        st.zoom = 1.0;
        st.pan = (0.0, 0.0);
    });
    push_view(ui);
    // As caixas acompanham (tela cheia desmarca as regiões e vice-versa).
    ui.set_timeout(0, render_info);
}

fn set_track_volume(ui: &mut Ui, i: usize, v: i32) {
    with(|st| {
        if i < st.track_vols.len() {
            st.track_vols[i] = v.clamp(0, 200);
        }
    });
    push_volumes(ui);
}

/// Mutar zera guardando o volume; desmutar volta pra ele (ou 100).
fn track_mute(ui: &mut Ui, i: usize) {
    with(|st| {
        if i >= st.track_vols.len() {
            return;
        }
        if st.track_vols[i] > 0 {
            st.pre_mute[i] = st.track_vols[i];
            st.track_vols[i] = 0;
        } else {
            st.track_vols[i] = if st.pre_mute[i] > 0 { st.pre_mute[i] } else { 100 };
            st.pre_mute[i] = 0;
        }
    });
    push_volumes(ui);
    ui.set_timeout(0, render_info);
}

// ----- transcrição e legenda -----

fn request_transcript() {
    let (need, id) = with(|st| (st.transcript.is_none() && !st.tr_requested, st.id.clone()));
    if need && !id.is_empty() {
        with(|st| st.tr_requested = true);
        bridge::handle_request_transcript(&id);
    }
}

pub fn toggle_transcript(ui: &mut Ui) {
    open_side(ui, "transcript");
    if with(|st| st.side != "transcript") {
        return;
    }
    if let Some(frm) = FrmPlayer::components(ui) {
        frm.lblSideTitle.set_text(ui, lang::tr("Transcrição"));
    }
    // Aberta, a lateral acompanha a fala atual.
    with(|st| st.tr_follow = true);
    render_turns(ui);
    request_transcript();
}

pub fn toggle_captions(ui: &mut Ui) {
    with(|st| st.cap_layout = None);
    let on = with(|st| {
        st.captions_on = !st.captions_on;
        st.captions_on
    });
    config::set_config_bool("playerCaptions", on);
    sync_cc(ui);
    if !on {
        if let Some(frm) = FrmPlayer::components(ui) {
            frm.lblCaption.set_visible(ui, false);
        }
        return;
    }
    if with(|st| st.transcript.is_none()) {
        with(|st| st.cap_notify = true);
        request_transcript();
    } else {
        notify_no_captions();
    }
    sync_caption(ui);
}

/// Legenda ligada numa gravação sem fala: sem aviso parece defeito.
fn notify_no_captions() {
    let (on, empty, transcribed) = with(|st| {
        let t = st.transcript.as_ref();
        (st.captions_on, t.is_some_and(|t| t.turns.is_empty()), t.is_some_and(|t| t.transcribed))
    });
    if on && empty {
        let msg = if transcribed { lang::tr("Esta gravação foi transcrita, mas nenhuma fala foi detectada no áudio.") } else { lang::tr("Esta gravação ainda não foi transcrita. Use \"Transcrever\" no menu de contexto da lista.") };
        crate::app_view::show_toast(&lang::tr("Legenda no vídeo"), &msg, false, 0);
    }
}

pub fn on_transcript(t: &Transcript) {
    let t = t.clone();
    with_ui(move |ui| {
        if with(|st| st.id != t.id) {
            return;
        }
        let notify = with(|st| {
            st.transcript = Some(t);
            st.tr_active = -1;
            st.tr_pinned = -1;
            st.cap_cache = None;
            std::mem::take(&mut st.cap_notify)
        });
        if with(|st| st.side == "transcript") {
            render_turns(ui);
        }
        if notify {
            notify_no_captions();
        }
        sync_caption(ui);
    });
}

/// "SPEAKER_00" vira "Falante 1"; o nome dado pelo usuário vence.
fn speaker_label(t: &Transcript, spk: &str) -> String {
    if let Some(n) = t.speakers.get(spk).filter(|n| !n.is_empty()) {
        return n.clone();
    }
    match speaker_number(spk) {
        Some(n) => lang::tr_with("Falante {{n}}", &[("n", &(n + 1).to_string())]),
        None if spk.is_empty() => lang::tr("Falante"),
        None => spk.to_owned(),
    }
}

fn speaker_number(spk: &str) -> Option<u32> {
    let s = spk.to_ascii_uppercase();
    let rest = s.strip_prefix("SPEAKER")?;
    rest.trim_start_matches(['_', '-']).parse().ok()
}

/// Cor estável por falante (seis matizes).
fn speaker_color(spk: &str) -> String {
    let i = match speaker_number(spk) {
        Some(n) => n as usize % SPEAKER_COLORS,
        None => {
            let mut h: i32 = 0;
            for c in spk.chars() {
                h = h.wrapping_mul(31).wrapping_add(c as i32);
            }
            h.unsigned_abs() as usize % SPEAKER_COLORS
        }
    };
    format!("speaker_{}", i + 1)
}

/// A lista de falas: uma instância do frame `fraTurn` por fala (só o que varia sai daqui)
/// ou, sem falas, a mensagem do form.
fn render_turns(ui: &mut Ui) {
    let (t, q, active, editing) = with(|st| (st.transcript.clone(), st.tr_query.clone(), st.tr_active, st.tr_editing));
    let Some(p) = side_panel(ui) else { return };
    // A lista encosta nas bordas (o turno tem o próprio respiro).
    if let Some(cb) = ui.control_mut(p) {
        cb.set_padding([0.0, 6.0, 8.0, 6.0]);
    }
    with(|st| st.tr_rows.clear());
    let empty = |ui: &mut Ui, msg: &str| {
        if let Some(frm) = FrmPlayer::components(ui) {
            frm.lblSideEmpty.set_text(ui, msg);
            frm.lblSideEmpty.set_visible(ui, true);
        }
    };
    let Some(t) = t else {
        empty(ui, &lang::tr("Carregando transcrição…"));
        fit_side(ui);
        return;
    };
    if t.turns.is_empty() {
        let msg = if t.transcribed { lang::tr("Esta gravação foi transcrita, mas nenhuma fala foi detectada no áudio.") } else { lang::tr("Esta gravação ainda não foi transcrita. Use \"Transcrever\" no menu de contexto da lista.") };
        empty(ui, &msg);
        fit_side(ui);
        return;
    }
    let width = FrmPlayer::components(ui).map(|frm| frm.pnlPlayerSide.id).and_then(|s| ui.control(s)).map_or(360.0, |cb| cb.width()) - 8.0;
    let mut body = TextStyle::default();
    body.font.family = "Segoe UI".into();
    body.font.size = 12.0;
    body.wrap = true;
    let line_h = (12.0 * 1.45f32).round();
    let natural = ui.measure_text("Ág", &body).height.max(1.0);
    let mut spk_style = body.clone();
    spk_style.font.size = 11.0;
    spk_style.font.weight = FontWeight::SemiBold;
    let mut rows = Vec::new();
    for (i, tr) in t.turns.iter().enumerate().filter(|(_, tr)| q.is_empty() || tr.text.to_lowercase().contains(&q)) {
        let Ok(row) = FraTurn::create_in(ui, p) else { continue };
        let lines = (ui.measure_text_wrapped(&tr.text, &body, width - 30.0).height / natural).round().max(1.0);
        let h = f64::from(8.0 + 15.4 + 3.0 + lines * line_h + 8.0);
        row.form.set_height(ui, h);
        // A marca do turno ativo, na borda esquerda, da altura da fala.
        row.recMark.set_height(ui, h);
        if i as i32 == active {
            row.pnlTurn.set_style(ui, "pl_turn_on");
            row.recMark.set_visible(ui, true);
        }
        let s = fmt_duration(tr.start.floor() as i64);
        if !s.is_empty() {
            row.lblTime.set_text(ui, s);
        }
        let name = speaker_label(&t, &tr.speaker);
        if editing == i as i32 {
            row.lblSpeaker.set_visible(ui, false);
            speaker_editor(ui, row.lyHead.id, i, &name);
        } else {
            let w = ui.measure_text(&name, &spk_style).width + 0.2 * name.chars().count() as f32 + 7.0;
            row.lblSpeaker.set_width(ui, f64::from(w));
            row.lblSpeaker.set_text(ui, name.as_str());
            row.lblSpeaker.set_text_color(ui, speaker_color(&tr.speaker));
            ui.on(row.lblSpeaker.id, "on_click", move |ui, _, _| {
                with(|st| st.tr_editing = i as i32);
                render_turns(ui);
            });
        }
        row.lblText.set_text(ui, tr.text.as_str());
        if !q.is_empty() {
            row.lblText.set_highlight(ui, q.as_str());
        }
        ui.on(row.pnlTurn.id, "on_click", move |ui, _, _| turn_clicked(ui, i));
        rows.push((i, row));
    }
    if rows.is_empty() {
        empty(ui, &lang::tr("Nenhum trecho encontrado."));
    }
    with(|st| st.tr_rows = rows);
    fit_side(ui);
}

/// O campo que troca o nome do falante (o frame `fraSpeakerEdit`), no lugar do nome
/// (Enter grava, Esc desiste, sair do campo grava; um clique numa fala desiste).
fn speaker_editor(ui: &mut Ui, head: ComponentId, idx: usize, name: &str) {
    let Ok(frame) = FraSpeakerEdit::create_in(ui, head) else { return };
    let ed = frame.edName;
    ed.set_text(ui, name);
    ui.on(ed.id, "on_key_down", move |ui, _, args| {
        let EventArgs::Key(k) = args else { return };
        match k.key {
            Key::Enter => {
                let text = ed.text(ui);
                finish_rename(ui, idx, Some(text));
            }
            Key::Escape => finish_rename(ui, idx, None),
            _ => {}
        }
    });
    ui.on(ed.id, "on_exit", move |ui, _, _| {
        if with(|st| st.tr_editing == idx as i32) {
            let text = ed.text(ui);
            finish_rename(ui, idx, Some(text));
        }
    });
    ui.set_focus(ed.id);
    if let Some(e) = ui.get_mut::<Edit>(ed.id) {
        e.select_all();
    }
}

fn finish_rename(ui: &mut Ui, idx: usize, name: Option<String>) {
    with(|st| st.tr_editing = -1);
    match name {
        Some(n) => rename_speaker(ui, idx, &n),
        None => render_turns(ui),
    }
    // O foco volta ao palco (as teclas do player) depois desta tecla.
    ui.set_timeout(0, |ui| {
        if let Some(stage) = FrmPlayer::components(ui).map(|frm| frm.frmVideo.id) {
            ui.set_focus(stage);
        }
    });
}

/// A fala em reprodução no instante `pos` (-1: nenhuma). É a última que começou, na ordem da
/// lista, e a marca só anda para a frente: com falas sobrepostas (o microfone e o alto-falante
/// pegando a mesma conversa), a mais curta terminava dentro da mais longa e a marca voltava
/// para a de cima. Depois que a última termina, ela continua marcada enquanto uma anterior
/// ainda fala; em silêncio (nenhuma fala no instante), nenhuma.
fn playing_turn(turns: &[Turn], pos: f64) -> i32 {
    let started = turns.iter().take_while(|tr| tr.start <= pos).count();
    if started > 0 && turns[..started].iter().any(|tr| pos < tr.end) { started as i32 - 1 } else { -1 }
}

/// O turno em reprodução. Turnos SOBREPOSTOS são normais: o clicado vence enquanto o tempo
/// está nele; fora disso, o de [`playing_turn`].
fn sync_transcript_active(ui: &mut Ui, force: bool) {
    let changed = with(|st| {
        let t = st.transcript.as_ref()?;
        if t.turns.is_empty() {
            return None;
        }
        let pos = st.pb.as_ref().map_or(0.0, |p| p.shared.position());
        let pin = st.tr_pinned;
        let idx = if pin >= 0 && (pin as usize) < t.turns.len() && pos >= t.turns[pin as usize].start - 0.5 && pos < t.turns[pin as usize].end {
            pin
        } else {
            st.tr_pinned = -1;
            playing_turn(&t.turns, pos)
        };
        if idx == st.tr_active && !force {
            return None;
        }
        st.tr_active = idx;
        Some((idx, st.tr_rows.clone()))
    });
    if let Some((idx, rows)) = changed {
        for (i, row) in rows {
            let on = i as i32 == idx;
            row.pnlTurn.set_style(ui, if on { "pl_turn_on" } else { "pl_turn" });
            row.recMark.set_visible(ui, on);
        }
    }
    follow_transcript(ui);
}

/// A fala atual fica inteira à vista enquanto a lateral acompanha: quando ela sai da área (o
/// vídeo avançou, a lista mudou), a lateral rola o mínimo até ela (`scroll_to` do
/// `ScrollBox`). O usuário rolando para longe dela (o `on_scroll`, que só o usuário dispara)
/// faz a lateral ficar onde ele deixou; rolar de volta até a atual ou clicar numa fala faz
/// acompanhar de novo.
fn follow_transcript(ui: &mut Ui) {
    if !with(|st| st.tr_follow) {
        return;
    }
    let (Some(frm), Some(row)) = (FrmPlayer::components(ui), active_row()) else { return };
    if !ScrollBox::visible_children(ui, frm.scrPlayerSide.id, frm.lySide.id, true).contains(&row) {
        if let Err(e) = frm.scrPlayerSide.scroll_to(ui, g::Form { id: row }) {
            log!("Player: acompanhar a transcricao falhou ({e}).");
        }
    }
}

/// A raiz do `fraTurn` da fala atual, se ela está na lista.
fn active_row() -> Option<ComponentId> {
    with(|st| st.tr_rows.iter().find(|(i, _)| *i as i32 == st.tr_active).map(|(_, r)| r.form.id))
}

/// O usuário rolou a lateral (o `on_scroll` do `scrPlayerSide`): acompanha enquanto a fala
/// atual continuar inteira à vista; longe dela, a lateral fica onde ele deixou.
pub fn side_scrolled(ui: &mut Ui) {
    let (Some(frm), Some(row)) = (FrmPlayer::components(ui), active_row()) else { return };
    let shown = ScrollBox::visible_children(ui, frm.scrPlayerSide.id, frm.lySide.id, true).contains(&row);
    with(|st| st.tr_follow = shown);
}

/// Parte o turno em blocos de até duas linhas pela largura da legenda e
/// divide o tempo pela proporção de caracteres.
pub(crate) fn caption_chunks(t: &Transcript, idx: usize, width_px: f32, font_px: f32) -> Vec<(String, f64, f64)> {
    let Some(turn) = t.turns.get(idx) else { return Vec::new() };
    let per_line = ((width_px / (font_px * 0.55)).floor() as usize).max(10);
    let max = per_line * 2;
    let mut texts: Vec<String> = Vec::new();
    let mut cur = String::new();
    for wd in turn.text.split_whitespace() {
        let next = if cur.is_empty() { wd.to_owned() } else { format!("{cur} {wd}") };
        if !cur.is_empty() && next.chars().count() > max {
            texts.push(std::mem::take(&mut cur));
            cur = wd.to_owned();
        } else {
            cur = next;
        }
    }
    if !cur.is_empty() {
        texts.push(cur);
    }
    let dur = (turn.end - turn.start).max(0.0);
    let total: usize = texts.iter().map(|s| s.chars().count()).sum::<usize>().max(1);
    let mut acc = 0usize;
    texts
        .into_iter()
        .map(|s| {
            let a = turn.start + dur * acc as f64 / total as f64;
            acc += s.chars().count();
            let b = turn.start + dur * acc as f64 / total as f64;
            (s, a, b)
        })
        .collect()
}

fn sync_caption(ui: &mut Ui) {
    if !with(|st| st.captions_on) {
        return;
    }
    // Como no original: 86 % do palco (até 960), fonte de 16 a 30 px pela largura.
    let (width, fs) = with(|st| ((st.stage.0 * 0.86).min(960.0), (st.stage.0 * 0.022).clamp(16.0, 30.0)));
    let text = with(|st| {
        let idx = st.tr_active;
        let t = st.transcript.as_ref()?;
        if idx < 0 {
            return None;
        }
        let valid = st.cap_cache.as_ref().is_some_and(|(i, wd, _)| *i == idx && (*wd - width).abs() < 1.0);
        if !valid {
            st.cap_cache = Some((idx, width, caption_chunks(t, idx as usize, width, fs)));
        }
        let pos = st.pb.as_ref().map_or(0.0, |p| p.shared.position());
        let chunks = &st.cap_cache.as_ref()?.2;
        let mut pick = chunks.first()?;
        for c in chunks {
            if pos >= c.1 {
                pick = c;
            } else {
                break;
            }
        }
        Some(pick.0.clone())
    });
    let text = text.unwrap_or_default();
    let (stage_w, idle) = with(|st| (st.stage.0, st.idle));
    let bottom = if idle { 36.0 } else { 92.0 };
    let key = (text.clone(), fs, stage_w, bottom);
    if with(|st| st.cap_layout.as_ref() == Some(&key)) {
        return;
    }
    with(|st| st.cap_layout = Some(key));
    if let Some(frm) = FrmPlayer::components(ui) {
        frm.lblCaption.set_visible(ui, !text.is_empty());
        frm.lblCaption.set_font_size(ui, f64::from(fs));
    }
    let mut style = TextStyle::default();
    style.font.family = "Segoe UI".into();
    style.font.size = fs;
    style.font.weight = FontWeight::SemiBold;
    let h = ui.measure_text_wrapped(&text, &style, width).height.ceil() + 8.0;
    if let Some(cb) = FrmPlayer::components(ui).map(|frm| frm.lblCaption.id).and_then(|id| ui.control_mut(id)) {
        let side = ((stage_w - width) / 2.0).max(0.0);
        cb.set_height(h).set_margins([side, 0.0, side, bottom]);
    }
    if let Some(frm) = FrmPlayer::components(ui) {
        frm.lblCaption.set_text(ui, text);
    }
    ui.layout();
}

fn turn_clicked(ui: &mut Ui, idx: usize) {
    // Um nome de falante em edição é cancelado: o clique numa fala não tira o foco do campo
    // (o painel da fala não recebe foco), então o "sair do campo grava" não acontecia e a
    // edição ficava aberta.
    let editing = with(|st| st.tr_editing);
    if editing >= 0 {
        finish_rename(ui, editing as usize, None);
    }
    let start = with(|st| st.transcript.as_ref().and_then(|t| t.turns.get(idx)).map(|t| t.start));
    let Some(start) = start else { return };
    with(|st| {
        st.tr_pinned = idx as i32;
        // Clicou numa fala: a lateral volta a acompanhar dali em diante.
        st.tr_follow = true;
        if let Some(pb) = &st.pb {
            pb.seek(start.max(0.0));
        }
    });
    sync_transcript_active(ui, true);
    sync_caption(ui);
}

pub fn tr_search(ui: &mut Ui, text: &str) {
    with(|st| st.tr_query = text.to_lowercase().trim().to_owned());
    render_turns(ui);
    sync_transcript_active(ui, true);
}

pub fn close_side(ui: &mut Ui) {
    let which = with(|st| st.side.clone());
    if !which.is_empty() {
        open_side(ui, &which);
    }
}

/// Renomeia o falante (todos os turnos dele mudam de uma vez).
pub fn rename_speaker(ui: &mut Ui, idx: usize, name: &str) {
    let nome: String = name.replace(['\r', '\n'], " ").trim().chars().take(60).collect();
    let change = with(|st| {
        let id = st.id.clone();
        let t = st.transcript.as_mut()?;
        let spk = t.turns.get(idx)?.speaker.clone();
        if nome == speaker_label(t, &spk) {
            return None;
        }
        if nome.is_empty() {
            t.speakers.remove(&spk);
        } else {
            t.speakers.insert(spk.clone(), nome.clone());
        }
        Some((id, spk))
    });
    if let Some((id, spk)) = change {
        bridge::handle_set_speaker_name(&id, &spk, &nome);
    }
    render_turns(ui);
}

/// Liga os gestos do palco e o teclado do form (uma vez, com o form criado).
pub fn setup(ui: &mut Ui) {
    for (name, d) in ICONS {
        let _ = ui.register_icon_svg(name, "", d, 24.0);
    }
    for (name, d) in LINE_ICONS {
        let _ = ui.register_icon_svg(name, d, "", 24.0);
    }
    // A barra da biblioteca usa os desenhos do original (pasta com +, mesclar).
    for (name, d) in [
        ("lib-folder", "M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"),
        ("lib-folder-plus", "M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z M12 11v6 M9 14h6"),
        ("lib-download", "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4 M7 10l5 5 5-5 M12 15V3"),
        ("lib-trash", "M3 6h2h16 M19 6l-1 14a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2L5 6 M10 11v6 M14 11v6 M9 6V4a2 2 0 0 1 2-2h2a2 2 0 0 1 2 2v2"),
    ] {
        let _ = ui.register_icon_svg(name, d, "", 24.0);
    }
    let _ = ui.register_icon_svg("lib-merge", "", "M7.586 8.00366L4 8.00366C3.44772 8.00366 3 7.55595 3 7.00366C3 6.45138 3.44772 6.00366 4 6.00366L8 6.00366C8.26509 6.00366 8.51933 6.10892 8.70685 6.2963L13.414 11H18.5845L15.2931 7.71103C14.9025 7.32065 14.9023 6.68748 15.2926 6.29681C15.683 5.90615 16.3162 5.90592 16.7068 6.2963L21.7068 11.2926C21.8945 11.4802 22 11.7346 22 11.9998C22 12.2651 21.8947 12.5195 21.7071 12.7071L16.7071 17.7071C16.3166 18.0976 15.6834 18.0976 15.2929 17.7071C14.9024 17.3166 14.9024 16.6834 15.2929 16.2929L18.5858 13H13.4142L8.70711 17.7071C8.51957 17.8947 8.26522 18 8 18H4C3.44772 18 3 17.5523 3 17C3 16.4477 3.44772 16 4 16H7.58579L11.5855 12.0003L7.586 8.00366Z", 24.0);
    if let Some(frm) = FrmPlayer::components(ui) {
        // Os itens do menu de velocidade são do form; cada um escolhe a sua.
        for (item, s) in speed_items(&frm).into_iter().zip(SPEEDS) {
            ui.on(item.id, "on_click", move |ui, _, _| set_speed(ui, s));
        }
        for id in [frm.trkVolume.id, frm.pnlCcCaptions.id, frm.pnlCcTranscript.id, frm.pnlPlayerBar.id, frm.pnlPlayerHead.id] {
            ui.on(id, "on_pointer_move", |ui, _, _| activity(ui));
        }
    }
    if let Some(t) = FrmPlayer::components(ui).map(|frm| frm.trkVolume.id) {
        ui.on(t, "on_dbl_click", |ui, _, _| set_volume(ui, 100.0));
    }
    if let Some(stage) = FrmPlayer::components(ui).map(|frm| frm.frmVideo.id) {
        ui.on(stage, "on_wheel", |ui, _, args| {
            if let EventArgs::Wheel(e) = args {
                wheel(ui, e);
            }
        });
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
        ui.on(stage, "on_dbl_click", |ui, _, _| toggle_fullscreen(ui));
    }
    let Some(form) = main_form() else { return };
    ui.on(form, "on_key_down", |ui, _, args| {
        let EventArgs::Key(k) = args else { return };
        if crate::app_view::confirm_open() {
            match k.key {
                Key::Escape => crate::app_view::confirm_close(ui, false),
                Key::Enter => crate::app_view::confirm_close(ui, true),
                _ => {}
            }
            return;
        }
        if is_open() {
            let typing = ui.focused(form_of(ui)).is_some_and(|f| ui.get::<Edit>(f).is_some());
            if !typing {
                key(ui, k);
            }
            return;
        }
        if crate::app_export::is_open() {
            let typing = ui.focused(form_of(ui)).is_some_and(|f| ui.get::<Edit>(f).is_some());
            crate::app_export::key(ui, k, typing);
            return;
        }
        if crate::app_picker::is_open() {
            if k.key == Key::Escape {
                crate::app_picker::close(ui);
            }
            return;
        }
        if crate::app_settings::is_open() {
            if k.key == Key::Escape {
                crate::app_settings::close(ui);
            }
            return;
        }
        // Na biblioteca, menos com o cursor num campo (o Backspace é do texto).
        let typing = ui.focused(form_of(ui)).is_some_and(|f| ui.get::<Edit>(f).is_some());
        if !typing {
            crate::app_library::key(ui, k);
        }
    });
}

fn form_of(_ui: &Ui) -> ComponentId {
    main_form().unwrap_or(ComponentId::from_u64(0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::view::Turn;

    fn app() -> aylis::Application {
        let mut app = aylis::Application::bundled(crate::PROJECT).unwrap();
        crate::generated::register(&mut app.ui);
        app
    }

    /// Uma transcrição de `n` falas, de três falantes e tamanhos variados.
    fn sample_transcript(n: usize) -> Transcript {
        let turns = (0..n)
            .map(|i| Turn {
                speaker: format!("SPEAKER_{:02}", i % 3),
                start: i as f64 * 4.0,
                end: i as f64 * 4.0 + 3.5,
                text: "uma fala de teste com algumas palavras ".repeat(1 + i % 5).trim().to_owned(),
            })
            .collect();
        Transcript { id: "v.mp4".into(), transcribed: true, turns, ..Default::default() }
    }

    /// Um vídeo com tudo o que o painel de informações mostra: um vídeo, duas regiões e duas
    /// faixas de áudio (a segunda muda).
    fn sample_info() -> VideoInfo {
        use crate::view::{LayoutRegion, StreamInfo, VideoLayout};
        let video = StreamInfo { kind: "video".into(), codec: "h264".into(), width: Some(1920), height: Some(1080), frame_rate: Some(60.0), bitrate: 8_000_000, ..Default::default() };
        let audio = |title: &str| StreamInfo { kind: "audio".into(), codec: "aac".into(), title: title.into(), channels: Some(2), sample_rate: Some(48000), ..Default::default() };
        let region = |name: &str, kind: &str| LayoutRegion { name: name.into(), kind: kind.into(), w: 100, h: 100, ..Default::default() };
        VideoInfo {
            id: "v.mp4".into(),
            file_name: "v.mp4".into(),
            format: "mp4".into(),
            duration: 65.0,
            bitrate: 9_000_000,
            size: 1024 * 1024,
            streams: vec![video, audio(""), audio("Microfone")],
            layout: Some(VideoLayout { canvas_w: 1920, canvas_h: 1080, regions: vec![region("Tela", "monitor"), region("", "webcam")] }),
        }
    }

    /// Os painéis são montados com os frames de `forms/player/` (só o que varia sai do
    /// código), as mensagens e o menu de velocidade são do form.
    #[test]
    fn the_side_panels_are_frames() {
        let mut app = app();
        let ui = &mut app.ui;
        ui.create_form("frmPlayer").unwrap();
        setup(ui);
        let frm = FrmPlayer::components(ui).unwrap();
        with(|st| {
            st.id = "v.mp4".into();
            st.side = "info".into();
            st.info = Some(sample_info());
            st.regions = BTreeSet::from([1]);
            st.track_vols = vec![100, 0];
            st.pre_mute = vec![0, 0];
        });

        // Carregando: só a mensagem do form.
        assert!(side_panel(ui).is_some());
        assert!(!frm.lblSideLoading.visible(ui) && !frm.lblSideEmpty.visible(ui));

        render_info(ui);
        let kids: Vec<ComponentId> = ui.children(frm.lySide.id).to_vec();
        assert_eq!(&kids[..2], &[frm.lblSideLoading.id, frm.lblSideEmpty.id], "as mensagens ficam no form");
        let kids = &kids[2..];
        let sections: Vec<_> = kids.iter().filter_map(|&c| FraInfoSection::components_of(ui, c)).collect();
        let rows: Vec<_> = kids.iter().filter_map(|&c| FraInfoRow::components_of(ui, c)).collect();
        let opts: Vec<_> = kids.iter().filter_map(|&c| FraViewOption::components_of(ui, c)).collect();
        let cards: Vec<_> = kids.iter().filter_map(|&c| FraTrackCard::components_of(ui, c)).collect();
        assert_eq!((sections.len(), rows.len(), opts.len(), cards.len()), (4, 9, 3, 2));
        assert_eq!(sections.len() + rows.len() + opts.len() + cards.len(), kids.len());
        assert_eq!(sections[0].lblTitle.text(ui), lang::tr("Arquivo"));
        assert_eq!(ui.control(sections[0].form.id).unwrap().margins().top, 0.0);
        assert_eq!(ui.control(sections[1].form.id).unwrap().margins().top, 18.0);
        assert_eq!(rows[0].lblKey.text(ui), lang::tr("Nome"));
        assert_eq!(rows[0].lblValue.text(ui), "v.mp4");
        assert_eq!(rows[0].lblValue.hint(ui), "v.mp4");
        assert!(rows[0].lblKey.width(ui) < 110.0, "a chave tem o tamanho do texto");
        assert_eq!(rows[6].lblValue.text(ui), "1920 × 1080");

        // Tela cheia desmarcada (há região escolhida); a webcam marcada, com o ✓.
        assert_eq!(opts[0].pnlMark.style(ui), "pl_radio");
        assert!(!opts[0].cirDot.visible(ui) && !opts[0].btnCheck.visible(ui));
        assert_eq!(opts[0].btnIcon.icon(ui), "pl-expand");
        assert_eq!(opts[1].lblLabel.text(ui), "Tela");
        assert_eq!(opts[1].pnlOption.style(ui), "pl_view");
        assert_eq!(opts[2].pnlOption.style(ui), "pl_view_on");
        assert_eq!(opts[2].pnlMark.style(ui), "pl_box_on");
        assert!(opts[2].btnCheck.visible(ui) && !opts[2].cirDot.visible(ui));
        assert_eq!(opts[2].btnIcon.icon(ui), "pl-webcam");

        // Os cartões: as linhas dentro do cartão, a altura pela conta de antes e o mudo.
        let card_rows = |ui: &Ui, card: &g::FraTrackCardComponents| ui.children(card.lyRows.id).iter().filter_map(|&c| FraInfoRow::components_of(ui, c)).count();
        assert_eq!(card_rows(ui, &cards[0]), 3);
        assert_eq!(cards[0].form.height(ui), f64::from(8.0 + 19.4 + 3.0 * 24.8f32 + 6.0 + 1.0 + 6.0 + 22.0 + 8.0));
        assert_eq!(cards[0].lblTitle.text(ui), lang::tr_with("Faixa {{n}} (mix)", &[("n", "1")]));
        assert_eq!(cards[0].lblPct.text(ui), "100%");
        assert_eq!(cards[0].trkVolume.value(ui), 100.0);
        assert_eq!(cards[0].btnMute.icon(ui), "pl-tvol");
        assert_eq!(cards[1].lblTitle.text(ui), "Microfone");
        assert!(cards[1].lblTitle.font_strikeout(ui));
        assert!((cards[1].pnlCard.opacity(ui) - 0.55).abs() < 1e-6);
        assert_eq!(cards[1].btnMute.icon(ui), "pl-tvol-off");
        assert_eq!(cards[1].btnMute.hint(ui), lang::tr("Desmutar"));
        assert_eq!(cards[1].trkVolume.value(ui), 0.0);
        // Mexer no volume da faixa acompanha o rótulo e o estado.
        cards[1].trkVolume.set_value(ui, 150.0);
        ui.fire(cards[1].trkVolume.id, "on_change", &EventArgs::None);
        assert_eq!(cards[1].lblPct.text(ui), "150%");
        assert_eq!(with(|st| st.track_vols.clone()), vec![100, 150]);

        // A altura do painel é a soma das peças visíveis.
        let sum: f32 = kids.iter().map(|&c| ui.control(c).map_or(0.0, |cb| cb.height() + cb.margins().top + cb.margins().bottom)).sum();
        assert_eq!(ui.control(frm.lySide.id).unwrap().height(), sum + 24.0);

        // A transcrição: uma instância de `fraTurn` por fala; refazer troca as peças.
        with(|st| {
            st.side = "transcript".into();
            st.transcript = Some(sample_transcript(3));
            st.tr_active = 1;
        });
        render_turns(ui);
        let turns = with(|st| st.tr_rows.clone());
        assert_eq!(ui.children(frm.lySide.id).len(), 2 + 3, "as peças das informações saíram");
        assert_eq!(turns.iter().map(|(i, _)| *i).collect::<Vec<_>>(), vec![0, 1, 2]);
        let (_, t1) = turns[1];
        assert_eq!(t1.pnlTurn.style(ui), "pl_turn_on");
        assert!(t1.recMark.visible(ui) && !turns[0].1.recMark.visible(ui));
        assert_eq!(t1.recMark.height(ui), t1.form.height(ui));
        assert_eq!(t1.lblSpeaker.text(ui), lang::tr_with("Falante {{n}}", &[("n", "2")]));
        assert_eq!(t1.lblSpeaker.text_color(ui), "speaker_2");
        assert_eq!(t1.lblTime.text(ui), "00:04");
        assert_eq!(turns[0].1.lblTime.text(ui), "00:00");
        assert_eq!(t1.lblText.text(ui), "uma fala de teste com algumas palavras uma fala de teste com algumas palavras");
        // A fala ativa muda (sem motor, a posição é 0: a primeira): o estilo e a marca acompanham.
        sync_transcript_active(ui, true);
        assert_eq!(with(|st| st.tr_active), 0);
        assert_eq!(turns[0].1.pnlTurn.style(ui), "pl_turn_on");
        assert!(turns[0].1.recMark.visible(ui));
        assert_eq!(t1.pnlTurn.style(ui), "pl_turn");
        assert!(!t1.recMark.visible(ui));

        // Clicar no nome põe o campo do falante no lugar dele.
        ui.fire(t1.lblSpeaker.id, "on_click", &EventArgs::None);
        let (_, t1) = with(|st| st.tr_rows[1]);
        assert!(!t1.lblSpeaker.visible(ui));
        let editor = ui.children(t1.lyHead.id).iter().find_map(|&c| FraSpeakerEdit::components_of(ui, c)).expect("o campo do falante");
        assert_eq!(editor.edName.text(ui), lang::tr_with("Falante {{n}}", &[("n", "2")]));
        // Clicar em outra fala desiste da edição: o nome volta, sem o campo e sem gravar.
        editor.edName.set_text(ui, "Outro nome");
        let (_, t3) = with(|st| st.tr_rows[2]);
        ui.fire(t3.pnlTurn.id, "on_click", &EventArgs::None);
        assert_eq!(with(|st| st.tr_editing), -1);
        let (_, t1) = with(|st| st.tr_rows[1]);
        assert!(t1.lblSpeaker.visible(ui));
        assert!(ui.children(t1.lyHead.id).iter().all(|&c| FraSpeakerEdit::components_of(ui, c).is_none()));
        assert_eq!(t1.lblSpeaker.text(ui), lang::tr_with("Falante {{n}}", &[("n", "2")]));

        // A busca sem resultado mostra a mensagem do form.
        with(|st| st.tr_query = "nada disso".into());
        render_turns(ui);
        assert!(with(|st| st.tr_rows.is_empty()));
        assert!(frm.lblSideEmpty.visible(ui));
        assert_eq!(frm.lblSideEmpty.text(ui), lang::tr("Nenhum trecho encontrado."));
        assert_eq!(ui.children(frm.lySide.id).len(), 2);

        // O menu de velocidade: os itens são do form, na ordem de `SPEEDS`.
        let items = speed_items(&frm);
        for (item, s) in items.iter().zip(SPEEDS) {
            assert_eq!(item.text(ui), speed_label(s));
        }
        speed_menu(ui, frm.btnSpeed.id);
        assert_eq!(items[2].style(ui), "pl_item_on");
        assert_eq!(items[4].style(ui), "pl_item");
        ui.fire(items[4].id, "on_click", &EventArgs::None);
        assert_eq!(with(|st| st.speed), 2.0);
        assert_eq!(frm.btnSpeed.text(ui), "2×");
        assert!(!frm.pnlSpeedPop.visible(ui));
        with(|st| st.id.clear());
    }

    /// Falas sobrepostas (o microfone longo e o alto-falante curto dentro dele, como no vídeo do
    /// autor): a marca anda para a frente e não volta para a de cima quando a curta termina.
    #[test]
    fn the_playing_turn_only_moves_forward() {
        let turn = |start: f64, end: f64| Turn { start, end, ..Default::default() };
        // 03:22–03:45 (outra), 03:42–03:58 (microfone), 03:46–03:50 (alto-falante), 04:10–04:20.
        let turns = [turn(202.0, 225.0), turn(222.0, 238.0), turn(226.0, 230.0), turn(250.0, 260.0)];
        let at = |pos: f64| playing_turn(&turns, pos);
        assert_eq!(at(100.0), -1, "antes da primeira");
        assert_eq!(at(210.0), 0);
        assert_eq!(at(223.0), 1, "a de baixo começou: anda");
        assert_eq!(at(227.0), 2);
        assert_eq!(at(232.0), 2, "a curta terminou, a longa ainda fala: não volta");
        assert_eq!(at(237.9), 2);
        assert_eq!(at(240.0), -1, "silêncio");
        assert_eq!(at(255.0), 3);
        assert_eq!(playing_turn(&[], 10.0), -1);
    }

    /// A lateral acompanha a fala atual; rolada para longe pelo usuário (a roda, que dispara o
    /// `on_scroll`), fica onde ele deixou; um clique numa fala ou rolar de volta até a atual
    /// faz acompanhar de novo. Posicionar pelo código não conta como o usuário rolando.
    #[test]
    fn the_transcript_follows_the_current_turn() {
        let mut app = app();
        let ui = &mut app.ui;
        let form = ui.create_form("frmPlayer").unwrap();
        setup(ui);
        with(|st| {
            st.id = "v.mp4".into();
            st.side.clear();
            st.transcript = Some(sample_transcript(200));
        });
        toggle_transcript(ui);
        ui.layout();
        let frm = FrmPlayer::components(ui).unwrap();
        let (sb, list) = (frm.scrPlayerSide.id, frm.lySide.id);
        let row_of = |i: usize| with(|st| st.tr_rows.iter().find(|(t, _)| *t == i).map(|(_, r)| r.form.id)).unwrap();
        let shown = |ui: &mut Ui, i: usize| ScrollBox::visible_children(ui, sb, list, true).contains(&row_of(i));
        let go = |ui: &mut Ui, i: i32| {
            with(|st| st.tr_active = i);
            follow_transcript(ui);
            ui.layout();
        };
        // A roda do usuário em cima da lateral.
        let wheel = |ui: &mut Ui, dy: f32| {
            let r = ui.absolute_rect(sb).unwrap();
            let at = PointF::new(r.x + r.width / 2.0, r.y + r.height / 2.0);
            ui.wheel(form, WheelEvent { position: at, delta: PointF::new(0.0, dy), ..Default::default() });
            ui.layout();
        };
        let position = |ui: &Ui| ui.get::<ScrollBox>(sb).unwrap().scroll();
        assert!(ui.get::<ScrollBox>(sb).unwrap().max_scroll() > 0.0, "a lista precisa rolar");

        // Avançando: a fala atual entra na área sozinha, inteira.
        go(ui, 0);
        go(ui, 120);
        assert!(position(ui) > 0.0 && shown(ui, 120));
        go(ui, 121);
        assert!(shown(ui, 121) && with(|st| st.tr_follow), "o código rolar não para o acompanhamento");

        // O usuário rola para o começo: a lateral fica lá enquanto o vídeo avança.
        wheel(ui, 1_000_000.0);
        assert_eq!(position(ui), 0.0);
        assert!(!with(|st| st.tr_follow));
        go(ui, 140);
        assert_eq!(position(ui), 0.0, "fica onde o usuário deixou");

        // Um clique numa fala: acompanha de novo dali em diante.
        turn_clicked(ui, 150);
        assert!(with(|st| st.tr_follow));
        go(ui, 150);
        assert!(shown(ui, 150));
        go(ui, 170);
        assert!(shown(ui, 170));

        // Rolar um pouco, com a fala atual ainda inteira à vista, continua acompanhando.
        let before = position(ui);
        wheel(ui, -1.0);
        assert!(position(ui) != before && shown(ui, 170) && with(|st| st.tr_follow));
        with(|st| st.id.clear());
    }

    /// Quanto custa montar a transcrição com 500 falas (rode em release, com `--ignored
    /// --nocapture`): a primeira montagem e o refazer (a busca refaz a lista a cada tecla).
    #[test]
    #[ignore]
    fn bench_500_turns() {
        let mut app = app();
        let ui = &mut app.ui;
        ui.create_form("frmPlayer").unwrap();
        with(|st| {
            st.id = "v.mp4".into();
            st.side = "transcript".into();
            st.transcript = Some(sample_transcript(500));
        });
        let mut times = Vec::new();
        for _ in 0..6 {
            let t = Instant::now();
            render_turns(ui);
            times.push(t.elapsed().as_secs_f64() * 1000.0);
        }
        assert_eq!(with(|st| st.tr_rows.len()), 500);
        println!("500 falas: primeira {:.1} ms; refazer {:?} ms", times[0], times[1..].iter().map(|t| format!("{t:.1}")).collect::<Vec<_>>());
        // Só criar (sem textos, medidas nem liberar): 500 instâncias do frame.
        let list = FrmPlayer::components(ui).unwrap().lySide.id;
        let t = Instant::now();
        for _ in 0..500 {
            FraTurn::create_in(ui, list).unwrap();
        }
        println!("500 fraTurn::create_in: {:.1} ms", t.elapsed().as_secs_f64() * 1000.0);
        with(|st| st.id.clear());
    }
}
