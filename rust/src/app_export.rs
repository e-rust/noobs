//! Presenter da tela de exportação: telas, prévia com recorte e zoom, linha do
//! tempo com cortes e forma de onda, faixas de áudio, opções de vídeo,
//! nome/formato, a exportação e a prévia gerada, e a barra de progresso no
//! cartão da gravação de origem.
//!
//! A tela inteira está em `forms/export.json` (cabeçalho, a área rolável `lyExport` e
//! as seções do formulário numa coluna de até 880 px); o código lê e escreve os
//! componentes tipados (`FrmExport::components`). As peças repetidas são frames de
//! `forms/export/`: as pílulas das telas e das faixas (`fraExPill`), as marcas da régua
//! (`fraExTick`) e a duração dos trechos (`fraExPartLabel`), estas duas reaproveitadas
//! entre um desenho e outro. A prévia é o motor do player (`playback`) aberto PARADO na
//! gravação, num `FrameView`, com o `CropBox` do iron por cima; a linha do tempo
//! (trechos, divisões e forma de onda) é desenhada aqui numa imagem da janela visível,
//! com o cursor e a régua por cima.

use crate::generated::{self as g, FraExPartLabel, FraExPill, FraExTick, FrmExport, FrmExportComponents as Frm};
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::sync::Arc;

use iron::prelude::*;

use crate::app_view::{main_form, with_ui};
use crate::bridge::export::{self as bexp, ExportRequest};
use crate::playback::{Out, Playback, Region, ViewSpec};
use crate::view::{ExportEncoderInfo, StreamInfo, Transcript, Turn, VideoInfo, VideoLayout};
use crate::{bridge, config, lang, log};

const CRF_MAX: i32 = 51;
const CRF_DEFAULT: i32 = 23;
const FPS_MAX: i32 = 1000;
const FPS_MAX_SVT: i32 = 240;
/// A partir deste CRF o SVT-AV1 já está no quantizador máximo (o arquivo sai igual).
const SVT_FLOOR_CRF: i32 = 31;
const SPEED_MAX: f64 = 512.0;
const SPEED_STEPS: [f64; 25] =
    [1.0, 1.25, 1.5, 1.75, 2.0, 2.5, 3.0, 4.0, 5.0, 6.0, 8.0, 10.0, 12.0, 16.0, 20.0, 24.0, 32.0, 48.0, 64.0, 96.0, 128.0, 192.0, 256.0, 384.0, 512.0];
const PREVIEW_SECS: [f64; 3] = [5.0, 10.0, 30.0];
/// Altura da faixa dos trechos (a régua, de 17, e a barra de rolagem, de 8, estão no JSON).
const TL_H: f32 = 46.0;
const TICK_STEPS: [f64; 13] = [1.0, 2.0, 5.0, 10.0, 15.0, 30.0, 60.0, 120.0, 300.0, 600.0, 900.0, 1800.0, 3600.0];
/// Largura máxima da coluna do formulário.
const COL_MAX: f32 = 880.0;
/// A folga preta em volta do quadro no palco da prévia.
const STAGE_GAP: f32 = 15.0;
const SCALES: [&str; 3] = ["bicubic", "bilinear", "area"];

#[derive(Clone, Copy, Debug)]
struct Part {
    start: f64,
    end: f64,
    keep: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct Rect {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

struct PanDrag {
    sx: f32,
    sy: f32,
    px: f32,
    py: f32,
    moved: f32,
}

#[derive(Default)]
struct State {
    id: String,
    loaded: bool,
    ready: bool,
    duration: f64,
    src_w: i32,
    src_h: i32,
    src_fps: f64,
    layout: Option<VideoLayout>,
    audio: Vec<StreamInfo>,
    sel_regions: BTreeSet<usize>,
    no_video: bool,
    turns: Option<Vec<Turn>>,
    transcribed: bool,
    captions_on: bool,
    cap_cache: Option<(usize, i32, Vec<(String, f64, f64)>)>,
    peaks: Option<Vec<f32>>,
    wave_max: f32,
    crop: Option<Rect>,
    crop_zoom: f32,
    pan: (f32, f32),
    pan_drag: Option<PanDrag>,
    sel_audio: BTreeSet<i64>,
    original_audio: bool,
    mix_checked: bool,
    parts: Vec<Part>,
    play_sec: f64,
    zoom: f64,
    scroll: f64,
    view_w: f64,
    cursor_part: i32,
    tl_drag: bool,
    sb_drag: Option<(f32, f64)>,
    encoders: Vec<ExportEncoderInfo>,
    enc_id: String,
    res_heights: Vec<i32>,
    res_h: i32,
    scale_algo: String,
    speed_text: String,
    fps_val: i32,
    q_pos: i32,
    burn: bool,
    container: String,
    video_container: String,
    prevsec: usize,
    /// O palco (o quadro sem zoom) na tela: posição dentro do painel e tamanho.
    frame_at: (f32, f32),
    frame: (f32, f32),
    scale: f32,
    pb: Option<Playback>,
    playing: bool,
    user_muted: bool,
    user_vol: i32,
    timer: Option<u64>,
    col_w: f32,
    // ---- exportação em curso (vive fora da tela: ela pode ser fechada) ----
    running: bool,
    running_id: String,
    pct: f64,
    canceling: bool,
    previewing: bool,
    preview_pct: f64,
}

fn fresh(id: &str, encoders: Vec<ExportEncoderInfo>, keep: &State) -> State {
    State {
        id: id.to_owned(),
        crop_zoom: 1.0,
        zoom: 1.0,
        user_vol: 100,
        q_pos: CRF_MAX - CRF_DEFAULT,
        scale_algo: "bicubic".into(),
        speed_text: "1".into(),
        container: "mp4".into(),
        video_container: "mp4".into(),
        prevsec: 1,
        cursor_part: -1,
        original_audio: true,
        captions_on: config::get_config_bool("exportCaptions", false),
        encoders,
        running: keep.running,
        running_id: keep.running_id.clone(),
        pct: keep.pct,
        canceling: keep.canceling,
        scale: 1.0,
        ..Default::default()
    }
}

thread_local! {
    static ST: RefCell<State> = RefCell::new(State { crop_zoom: 1.0, zoom: 1.0, scale: 1.0, ..Default::default() });
}

fn with<R>(f: impl FnOnce(&mut State) -> R) -> R {
    ST.with(|s| f(&mut s.borrow_mut()))
}

/// Os componentes da tela (`forms/export.json`).
fn frm(ui: &Ui) -> Option<Frm> {
    FrmExport::components(ui)
}

fn tw(key: &str, args: &[(&str, &str)]) -> String {
    lang::tr_with(key, args)
}

fn fmt_time(sec: f64) -> String {
    let s = sec.max(0.0).round() as u64;
    let (h, m, ss) = (s / 3600, (s % 3600) / 60, s % 60);
    if h > 0 {
        format!("{h}:{m:02}:{ss:02}")
    } else {
        format!("{m:02}:{ss:02}")
    }
}

fn parse_time(txt: &str) -> Option<f64> {
    let parts: Vec<&str> = txt.trim().split(':').map(str::trim).collect();
    let mut sec = 0.0;
    for p in parts {
        let v: f64 = p.parse().ok()?;
        sec = sec * 60.0 + v;
    }
    Some(sec.round())
}

pub fn is_open() -> bool {
    with(|st| !st.id.is_empty())
}

/// A tela está aberta NESTE id (as respostas de probe/transcrição vêm pra cá).
fn waiting_for(id: &str) -> bool {
    with(|st| !st.id.is_empty() && st.id == id)
}

// ---------------------------------------------------------------------------
// Abrir / fechar
// ---------------------------------------------------------------------------

/// Abre a tela pra uma gravação (botão "Exportar" ou menu de contexto).
pub fn open_for(ui: &mut Ui, rec: &str) {
    if rec.is_empty() {
        return;
    }
    // Uma de cada vez (o backend também recusa a segunda).
    if with(|st| st.running) || bexp::export_busy() {
        crate::app_view::show_toast(&lang::tr("Exportação em andamento"), &lang::tr("Espere a exportação atual terminar (ou cancele) para começar outra."), true, 5000);
        return;
    }
    if !bridge::check_recording_for_player(rec) {
        return;
    }
    close_engine(ui);
    with(|st| {
        let enc = std::mem::take(&mut st.encoders);
        let next = fresh(rec, enc, st);
        *st = next;
        st.scale = crate::app_player::window_scale();
    });
    crate::app_view::show_form(ui, FrmExport::components(ui).map(|frm| frm.form), true);
    build(ui);
    let base = std::path::Path::new(rec).file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
    let base = if base.is_empty() { lang::tr("Gravação") } else { base };
    if let Some(f) = frm(ui) {
        f.edtExName.set_text(ui, format!("{base} - {}", lang::tr("Exportado")));
        f.lblExMsg.set_text(ui, lang::tr("Carregando prévia…"));
        f.lblExMsg.set_visible(ui, true);
        if let Some(sb) = ui.control_mut(f.scrExport.id) {
            sb.set_scroll_offset(PointF::ZERO);
        }
    }
    render_all(ui);

    bridge::handle_request_video_info(rec);
    bridge::handle_request_transcript(rec);

    // Prévia: o motor do player, parado no começo.
    let view = with(|st| view_spec(st));
    let id_out = rec.to_owned();
    let out: Arc<dyn Fn(Out) + Send + Sync> = crate::app_view::frame_sink(id_out, on_out);
    let pb = Playback::open_paused(rec, view, out);
    with(|st| st.pb = Some(pb));
    push_volumes();
    tick_later(ui);
    log!("Export: tela aberta para {rec}");
}

fn tick_later(ui: &mut Ui) {
    let job = ui.set_timeout(33, |ui| {
        if !is_open() {
            return;
        }
        tick(ui);
        tick_later(ui);
    });
    with(|st| st.timer = Some(job));
}

fn close_engine(ui: &mut Ui) {
    let (pb, job) = with(|st| (st.pb.take(), st.timer.take()));
    if let Some(job) = job {
        ui.clear_timeout(job);
    }
    drop(pb);
}

pub fn close(ui: &mut Ui) {
    // A prévia gerada é desta tela: fechar a cancela (a exportação de verdade não).
    if with(|st| st.previewing) {
        bexp::handle_cancel_export();
        with(|st| st.previewing = false);
    }
    close_engine(ui);
    with(|st| {
        st.id.clear();
        st.crop = None;
        st.pan_drag = None;
    });
    crate::app_view::show_form(ui, FrmExport::components(ui).map(|frm| frm.form), false);
    // A tela fica montada (é do JSON): só as imagens grandes saem.
    clear_pictures(ui);
    sync_card();
}

/// Tira o quadro da prévia e a imagem da linha do tempo (a memória e o que sobraria da
/// gravação anterior ao abrir outra).
fn clear_pictures(ui: &mut Ui) {
    let Some(f) = frm(ui) else { return };
    if let Some(v) = ui.get_mut::<FrameView>(f.fvExFrame.id) {
        v.set_frame(None);
    }
    if let Some(iv) = ui.get_mut::<ImageView>(f.imgExTl.id) {
        iv.set_picture(None);
    }
}

// ---------------------------------------------------------------------------
// Montagem do formulário
// ---------------------------------------------------------------------------

/// A largura da coluna do formulário (até 880 px, centrada).
fn col_width(ui: &mut Ui) -> f32 {
    let w = frm(ui).and_then(|f| ui.bounds(f.lyExport.id)).map_or(900.0, |b| b.width);
    ((w - 8.0).min(COL_MAX) - 36.0).max(320.0)
}

fn wrapped_height(ui: &Ui, l: ComponentId, text: &str, w: f32) -> f32 {
    let style = TextStyle { font: ui.font_of(l), wrap: true, ..TextStyle::default() };
    ui.measure_text_wrapped(text, &style, w.max(40.0)).height.ceil() + 2.0
}

/// As dicas (`.export-hint`: 11 px) quebradas na largura da coluna: a altura é a do
/// texto, e sem texto a dica some.
fn hints(f: &Frm) -> [g::Label; 10] {
    [
        f.lblExRegionsHint,
        f.lblExCropHint,
        f.lblExRangeHint,
        f.lblExResHint,
        f.lblExScaleHint,
        f.lblExSpeedHint,
        f.lblExFpsHint,
        f.lblExQualityHint,
        f.lblExCaptionHint,
        f.lblExContHint,
    ]
}

/// A fonte dos botões e campos (`Form.control-font`: Arial).
fn ctl_text(size: f32) -> TextStyle {
    let mut s = TextStyle::default();
    s.font.family = "Arial".into();
    s.font.size = size;
    s
}

/// A largura de um botão do formulário (`.export-btn`) pelo texto, mais o respiro da web:
/// `ex_sm` (11 px, +24) ou `ex`/`ex_primary` (12 px, +34).
fn button_width(ui: &Ui, text: &str, small: bool) -> f32 {
    (ui.measure_text(text, &ctl_text(if small { 11.0 } else { 12.0 })).width + if small { 24.0 } else { 34.0 }).ceil()
}

/// Ajusta a largura de um botão `ex_sm` ao texto que ele tem.
fn fit_button(ui: &mut Ui, b: g::Button) {
    let w = button_width(ui, &b.text(ui), true);
    if let Some(cb) = ui.control_mut(b.id) {
        cb.set_width(w);
    }
}

/// A largura de uma pílula de caixa (`.export-check`): o texto (12 px), a caixa de 13, 6
/// entre eles e o respiro dos lados.
fn chk_width(ui: &Ui, text: &str) -> f32 {
    let mut font = ctl_text(12.0);
    font.font.family = "Segoe UI".into();
    (ui.measure_text(text, &font).width + 13.0 + 6.0 + 20.0 + 2.0).ceil()
}

/// Ajusta a largura de uma pílula fixa do JSON ao texto dela.
fn fit_chk(ui: &mut Ui, p: g::Panel, l: g::Label) {
    let w = chk_width(ui, &l.text(ui));
    if let Some(cb) = ui.control_mut(p.id) {
        cb.set_width(w);
    }
}

/// Marca/desmarca uma pílula (e a habilita); a cor de marcada só nas que não são `plain`.
fn set_chk(ui: &mut Ui, p: g::Panel, b: g::CheckBox, checked: bool, enabled: bool, plain: bool) {
    b.set_checked(ui, checked);
    p.set_style(ui, if checked && !plain { "ex_chk_on" } else { "ex_chk" });
    p.set_enabled(ui, enabled);
    p.set_opacity(ui, if enabled { 1.0 } else { 0.4 });
}

/// Troca o texto de um botão `ex` e a largura dele junto (como na web).
fn put_btn(ui: &mut Ui, b: g::Button, text: &str) {
    let w = button_width(ui, text, false);
    if let Some(cb) = ui.control_mut(b.id) {
        cb.set_width(w);
    }
    b.set_text(ui, text);
}

/// O valor à direita de um controle deslizante, na largura do próprio texto (11 px).
fn put_value(ui: &mut Ui, l: g::Label, text: &str) {
    let mut font = TextStyle::default();
    font.font.family = "Segoe UI".into();
    font.font.size = 11.0;
    let w = ui.measure_text(text, &font).width.ceil() + 2.0;
    if let Some(cb) = ui.control_mut(l.id) {
        cb.set_width(w);
    }
    l.set_text(ui, text);
}

fn set_items(ui: &mut Ui, b: g::ComboBox, items: &[String], index: usize) {
    let joined = items.iter().map(|s| s.replace(';', ",")).collect::<Vec<_>>().join(";");
    if b.items(ui) != joined {
        b.set_items(ui, joined);
    }
    if b.item_index(ui) != index as i64 {
        b.set_item_index(ui, index as i64);
    }
}

/// Prepara a tela (do JSON) para uma gravação: a coluna, as larguras que dependem do texto
/// e o que a abertura anterior deixou.
fn build(ui: &mut Ui) {
    let Some(f) = frm(ui) else { return };
    let col = col_width(ui);
    with(|st| {
        st.col_w = col;
        st.view_w = (col - 2.0) as f64;
    });
    center_column(ui);
    // Os botões na largura do texto (já traduzido); o "Tirar trecho" volta ao texto inicial.
    f.btnExToggle.set_text(ui, lang::tr("Tirar trecho"));
    for b in [f.btnExCropReset, f.btnExZoomReset, f.btnExCut, f.btnExUncut, f.btnExToggle] {
        fit_button(ui, b);
    }
    for (p, l) in [(f.pnlExOrig, f.lblExOrig), (f.pnlExMix, f.lblExMix), (f.pnlExBurn, f.lblExBurn)] {
        fit_chk(ui, p, l);
    }
    // Resolução e codificador: duas colunas, 10 entre elas.
    let half = (col - 10.0) / 2.0;
    if let Some(cb) = ui.control_mut(f.cmbExRes.id) {
        cb.set_width(half);
    }
    if let Some(cb) = ui.control_mut(f.lblExResHead.id) {
        cb.set_width(half + 10.0);
    }
    if f.cmbExPrevSec.item_index(ui) != 1 {
        f.cmbExPrevSec.set_item_index(ui, 1);
    }
    clear_pictures(ui);
    f.lblExCaption.set_text(ui, "");
    f.lblExCaption.set_visible(ui, false);
    layout_page(ui);
}

/// Centraliza a coluna (padding lateral) na largura atual.
fn center_column(ui: &mut Ui) {
    let Some(p) = frm(ui).map(|f| f.lyExport.id) else { return };
    let w = ui.bounds(p).map_or(900.0, |b| b.width);
    let col = col_width(ui);
    let avail = w - 8.0;
    let left = ((avail - avail.min(COL_MAX)) / 2.0 + 18.0).floor();
    if let Some(cb) = ui.control_mut(p) {
        cb.set_padding([left, 18.0, (w - left - col).max(0.0), 36.0]);
    }
}

/// Altura de um contêiner de seção = a soma dos filhos visíveis.
fn fit(ui: &mut Ui, p: ComponentId) -> f32 {
    let h: f32 = ui
        .children(p)
        .to_vec()
        .iter()
        .filter_map(|&i| ui.control(i).filter(|cb| cb.visible()).map(|cb| cb.height() + cb.margins().top + cb.margins().bottom))
        .sum();
    if let Some(cb) = ui.control_mut(p) {
        cb.set_height(h);
    }
    h
}

/// Alturas das dicas, das seções e da página (a largura mudou ou algo apareceu).
fn layout_page(ui: &mut Ui) {
    let Some(f) = frm(ui) else { return };
    let col = with(|st| st.col_w);
    for h in hints(&f) {
        let text = h.text(ui);
        let height = if text.is_empty() { 0.0 } else { wrapped_height(ui, h.id, &text, col) };
        if let Some(cb) = ui.control_mut(h.id) {
            cb.set_height(height).set_visible(height > 0.0);
        }
    }
    for flow in [f.flwExRegions, f.flwExTracks] {
        flow_height(ui, flow.id, col);
    }
    for sec in [f.lyExAudio, f.lyExScale, f.lyExFps] {
        fit(ui, sec.id);
    }
    let h = fit(ui, f.lyExport.id);
    if let Some(cb) = ui.control_mut(f.lyExport.id) {
        cb.set_height(h + 70.0);
    }
    ui.layout();
}

/// A altura de um `FlowLayout` de pílulas pela quantidade de linhas.
fn flow_height(ui: &mut Ui, f: ComponentId, col: f32) {
    let mut rows = 0;
    let mut x = 0.0f32;
    for ch in ui.children(f).to_vec() {
        let w = ui.control(ch).map_or(0.0, |cb| cb.width());
        if rows == 0 || x + w > col {
            rows += 1;
            x = 0.0;
        }
        x += w + 6.0;
    }
    let h = rows.max(1) as f32 * 30.0 + (rows.max(1) - 1) as f32 * 6.0;
    if let Some(cb) = ui.control_mut(f) {
        cb.set_height(h);
    }
}

pub fn resized(ui: &mut Ui) {
    if !is_open() {
        return;
    }
    let col = col_width(ui);
    let changed = with(|st| {
        let ch = (st.col_w - col).abs() > 0.5;
        st.col_w = col;
        st.view_w = (col - 2.0) as f64;
        ch
    });
    if !changed {
        // A coluna já está no máximo (880): só as margens mudam, para ela continuar no meio.
        // Antes nada era refeito, e as margens velhas deixavam o conteúdo esticar na tela cheia.
        ui.set_timeout(0, center_column);
        return;
    }
    ui.set_timeout(0, |ui| {
        center_column(ui);
        let col = with(|st| st.col_w);
        if let Some(f) = frm(ui) {
            if let Some(b) = ui.control_mut(f.cmbExRes.id) {
                b.set_width((col - 12.0) / 2.0);
            }
            if let Some(cb) = ui.control_mut(f.lblExResHead.id) {
                cb.set_width((col - 12.0) / 2.0 + 12.0);
            }
        }
        render_stage(ui);
        render_timeline(ui);
        layout_page(ui);
    });
}

// ---------------------------------------------------------------------------
// Respostas do core
// ---------------------------------------------------------------------------

/// `video_info`: devolve true se era desta tela.
pub fn on_video_info(info: &VideoInfo) -> bool {
    if !waiting_for(&info.id) {
        return false;
    }
    with(|st| {
        st.loaded = true;
        st.duration = info.duration.max(0.0);
        st.audio.clear();
        st.src_w = 0;
        st.src_h = 0;
        st.src_fps = 0.0;
        for s in &info.streams {
            if s.kind == "video" && st.src_w == 0 {
                st.src_w = s.width.unwrap_or(0) as i32;
                st.src_h = s.height.unwrap_or(0) as i32;
                st.src_fps = s.frame_rate.unwrap_or(0.0);
            } else if s.kind == "audio" {
                st.audio.push(s.clone());
            }
        }
        st.layout = info.layout.clone().filter(|l| !l.regions.is_empty());
        if let Some(first) = st.audio.first() {
            st.sel_audio.insert(first.index);
        }
        st.parts = vec![Part { start: 0.0, end: st.duration, keep: true }];
        st.play_sec = 0.0;
        st.zoom = 1.0;
        st.scroll = 0.0;
        st.fps_val = max_fps(st);
    });
    request_wave();
    with_ui(|ui| {
        render_all(ui);
        push_view();
        push_volumes();
    });
    true
}

pub fn on_transcript(t: &Transcript) -> bool {
    if !waiting_for(&t.id) {
        return false;
    }
    with(|st| {
        st.transcribed = t.transcribed;
        st.turns = Some(t.turns.clone());
        st.cap_cache = None;
    });
    with_ui(|ui| {
        render_captions(ui);
        sync_caption(ui);
        layout_page(ui);
    });
    true
}

/// Forma de onda em alta resolução (`hi`): só a exportação pede.
pub fn on_waveform(id: &str, peaks: &[f32]) {
    if !waiting_for(id) {
        return;
    }
    let max = peaks.iter().cloned().filter(|p| p.is_finite() && *p >= 0.0).fold(0.0f32, f32::max);
    with(|st| {
        st.peaks = Some(peaks.iter().map(|p| if p.is_finite() && *p >= 0.0 { *p } else { 0.0 }).collect());
        st.wave_max = max.max(0.0001);
    });
    with_ui(render_timeline);
}

fn request_wave() {
    let (id, dur, n) = with(|st| (st.id.clone(), st.duration, st.audio.len()));
    if id.is_empty() || dur <= 0.0 || n == 0 {
        return;
    }
    let buckets = ((dur * 25.0).round() as i64).clamp(2000, 150_000);
    bridge::handle_request_waveform(&id, buckets, true);
}

/// Encoders da exportação (do warmup).
pub fn set_encoders(list: &[ExportEncoderInfo]) {
    with(|st| st.encoders = list.to_vec());
    if is_open() {
        with_ui(render_encoders);
    }
}

// ---------------------------------------------------------------------------
// Do motor da prévia
// ---------------------------------------------------------------------------

fn on_out(ui: &mut Ui, id: &str, o: Out) {
    if !waiting_for(id) {
        return;
    }
    match o {
        Out::Info(_) => {}
        Out::Frame { buf, x, y, .. } => {
            let k = with(|st| st.scale);
            let (bw, bh) = (buf.width, buf.height);
            let image = Image::bgra(bw, bh, buf.pixels);
            let rect = RectF::new(x as f32 / k, y as f32 / k, bw as f32 / k, bh as f32 / k);
            let Some(f) = frm(ui) else { return };
            if let Some(v) = ui.get_mut::<FrameView>(f.fvExFrame.id) {
                v.set_frame(Some((image, rect)));
            }
            ui.invalidate();
            if !with(|st| std::mem::replace(&mut st.ready, true)) {
                f.lblExMsg.set_visible(ui, false);
                render_playbar(ui);
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
                st.playing = false;
            });
            render_playbar(ui);
        }
        Out::Error(e) => {
            // Sem prévia a exportação continua funcionando.
            with(|st| st.ready = false);
            if let Some(f) = frm(ui) {
                f.lblExMsg.set_text(ui, lang::tr("Prévia indisponível para este codec — a exportação funciona normalmente."));
                f.lblExMsg.set_visible(ui, true);
            }
            log!("Export: previa indisponivel ({e})");
        }
    }
}

fn tick(ui: &mut Ui) {
    let playing = with(|st| st.playing);
    if !playing {
        return;
    }
    let pos = with(|st| st.pb.as_ref().map_or(st.play_sec, |p| p.shared.position()));
    set_play_sec(ui, pos, false);
}

/// Vista do motor: o palco é o QUADRO (proporção da origem ou da composição).
fn view_spec(st: &State) -> ViewSpec {
    let k = st.scale as f64;
    let (fw, fh) = (st.frame.0 as f64, st.frame.1 as f64);
    let z = st.crop_zoom as f64;
    // Zoom com origem no CENTRO; o do motor tem origem no canto.
    let pan_x = st.pan.0 as f64 + fw / 2.0 * (1.0 - z);
    let pan_y = st.pan.1 as f64 + fh / 2.0 * (1.0 - z);
    ViewSpec {
        stage_w: (fw * k).round().max(2.0) as i32,
        stage_h: (fh * k).round().max(2.0) as i32,
        zoom: z,
        pan_x: pan_x * k,
        pan_y: pan_y * k,
        regions: if compose_on(st) { video_regions(st) } else { Vec::new() },
    }
}

/// As telas escolhidas, em pixels do VÍDEO (a composição centraliza todas na vertical).
fn video_regions(st: &State) -> Vec<Region> {
    let Some(lay) = &st.layout else { return Vec::new() };
    let sx = if lay.canvas_w > 0 && st.src_w > 0 { st.src_w as f64 / lay.canvas_w as f64 } else { 1.0 };
    let sy = if lay.canvas_h > 0 && st.src_h > 0 { st.src_h as f64 / lay.canvas_h as f64 } else { 1.0 };
    st.sel_regions
        .iter()
        .filter_map(|&i| lay.regions.get(i))
        .map(|r| Region {
            x: (r.x as f64 * sx).round() as i32,
            y: (r.y as f64 * sy).round() as i32,
            w: (r.w as f64 * sx).round() as i32,
            h: (r.h as f64 * sy).round() as i32,
            webcam: false,
        })
        .collect()
}

fn push_view() {
    with(|st| {
        if st.frame.0 <= 0.0 {
            return;
        }
        let v = view_spec(st);
        if let Some(pb) = &st.pb {
            pb.set_view(v);
        }
    });
}

/// O que a prévia toca: a mistura (posição 0) no original ou com ela marcada;
/// senão as isoladas marcadas, somadas.
fn audible(st: &State) -> Vec<usize> {
    let n = st.audio.len();
    if n == 0 {
        return Vec::new();
    }
    if use_original(st) || st.sel_audio.contains(&mix_index(st)) {
        return vec![0];
    }
    st.audio.iter().enumerate().filter(|(_, s)| st.sel_audio.contains(&s.index)).map(|(k, _)| k).collect()
}

fn push_volumes() {
    with(|st| {
        let aud = audible(st);
        let tracks: Vec<f32> = (0..st.audio.len().max(1)).map(|k| if aud.contains(&k) { 1.0 } else { 0.0 }).collect();
        let master = if st.user_muted { 0.0 } else { st.user_vol as f32 / 100.0 };
        if let Some(pb) = &st.pb {
            pb.set_volumes(master, tracks);
        }
    });
}

// ---------------------------------------------------------------------------
// Contas
// ---------------------------------------------------------------------------

fn mix_index(st: &State) -> i64 {
    st.audio.first().map_or(-1, |s| s.index)
}

fn use_original(st: &State) -> bool {
    st.original_audio && st.audio.len() > 1
}

fn audio_selection(st: &State) -> Vec<i32> {
    if use_original(st) {
        st.audio.iter().map(|s| s.index as i32).collect()
    } else {
        st.sel_audio.iter().map(|&i| i as i32).collect()
    }
}

fn audio_is_complete(st: &State) -> bool {
    if use_original(st) {
        return true;
    }
    let mix = mix_index(st);
    if st.sel_audio.contains(&mix) {
        return true;
    }
    let isolated = st.audio.iter().filter(|s| s.index != mix).count();
    let picked = st.sel_audio.iter().filter(|&&i| i != mix).count();
    isolated > 0 && picked == isolated
}

fn compose_on(st: &State) -> bool {
    !st.no_video && st.layout.is_some() && !st.sel_regions.is_empty()
}

fn crop_available(st: &State) -> bool {
    !st.id.is_empty() && st.src_w > 0 && st.src_h > 0 && !st.no_video && st.sel_regions.is_empty()
}

fn crop_rect(st: &State) -> Rect {
    st.crop.unwrap_or(Rect { x: 0, y: 0, w: st.src_w, h: st.src_h })
}

fn has_crop(st: &State) -> bool {
    st.crop.is_some_and(|c| c.x > 0 || c.y > 0 || c.w < st.src_w || c.h < st.src_h)
}

fn ordered_regions(st: &State) -> Vec<(i64, i64, i64, i64)> {
    let Some(lay) = &st.layout else { return Vec::new() };
    let mut out: Vec<(i64, i64, i64, i64)> = st.sel_regions.iter().filter_map(|&i| lay.regions.get(i)).map(|r| (r.x, r.y, r.w, r.h)).collect();
    out.sort_by_key(|r| r.0);
    out
}

/// Tamanho nativo do que sai.
fn composed_size(st: &State) -> (i32, i32) {
    if st.layout.is_none() || st.sel_regions.is_empty() {
        let c = crop_rect(st);
        return (if c.w > 0 { c.w } else { st.src_w }, if c.h > 0 { c.h } else { st.src_h });
    }
    let (mut w, mut h) = (0i64, 0i64);
    for r in ordered_regions(st) {
        w += r.2;
        h = h.max(r.3);
    }
    (if w > 0 { w as i32 } else { st.src_w }, if h > 0 { h as i32 } else { st.src_h })
}

fn stage_dims(st: &State) -> (i32, i32) {
    if compose_on(st) {
        composed_size(st)
    } else {
        (st.src_w, st.src_h)
    }
}

fn output_size(st: &State) -> (i32, i32) {
    let (cw, ch) = composed_size(st);
    if st.res_h <= 0 || st.res_h >= ch {
        return (cw, ch);
    }
    let s = st.res_h as f64 / ch as f64;
    ((((cw as f64 * s).round() as i32) & !1).max(2), (((ch as f64 * s).round() as i32) & !1).max(2))
}

fn is_resizing(st: &State) -> bool {
    output_size(st) != composed_size(st)
}

fn kept_segments(st: &State) -> Vec<(f64, f64)> {
    let mut out: Vec<(f64, f64)> = Vec::new();
    for p in &st.parts {
        if !p.keep {
            continue;
        }
        match out.last_mut() {
            Some(last) if (last.1 - p.start).abs() < 0.001 => last.1 = p.end,
            _ => out.push((p.start, p.end)),
        }
    }
    out
}

fn kept_duration(st: &State) -> f64 {
    kept_segments(st).iter().map(|s| s.1 - s.0).sum()
}

fn part_at_cursor(st: &State) -> i32 {
    if st.parts.is_empty() {
        return -1;
    }
    for (i, p) in st.parts.iter().enumerate() {
        if st.play_sec >= p.start && st.play_sec < p.end {
            return i as i32;
        }
    }
    st.parts.len() as i32 - 1
}

fn nearest_cut(st: &State) -> i32 {
    if st.parts.len() < 2 {
        return -1;
    }
    let mut best = -1;
    let mut dist = f64::MAX;
    for i in 1..st.parts.len() {
        let d = (st.parts[i].start - st.play_sec).abs();
        if d < dist {
            dist = d;
            best = i as i32;
        }
    }
    best
}

fn speed(st: &State) -> f64 {
    let raw = st.speed_text.trim().replace(',', ".");
    match raw.parse::<f64>() {
        Ok(v) if v.is_finite() && v >= 1.0 => v.min(SPEED_MAX),
        _ => 1.0,
    }
}

fn fmt_speed(v: f64) -> String {
    let s = format!("{:.2}", v);
    let s = s.trim_end_matches('0').trim_end_matches('.').to_owned();
    if lang::current_language().starts_with("en") { s } else { s.replace('.', ",") }
}

fn max_fps(st: &State) -> i32 {
    let src = st.src_fps.round() as i32;
    if src <= 0 {
        return 0;
    }
    let s = speed(st);
    let mut max = if s > 1.0 { (src as f64 * s + 1e-6).floor() as i32 } else { src };
    let cap = if st.enc_id == "av1-sw" { FPS_MAX_SVT } else { FPS_MAX };
    if s > 1.0 {
        max = max.min(cap);
    }
    max
}

fn target_fps(st: &State) -> i32 {
    let max = max_fps(st);
    if max <= 20 {
        return 0;
    }
    st.fps_val.clamp(20, max)
}

fn crf(st: &State) -> i32 {
    (CRF_MAX - st.q_pos).clamp(0, CRF_MAX)
}

fn crf_band_key(crf: i32) -> &'static str {
    match crf {
        ..=17 => "Praticamente sem perdas — arquivo bem grande.",
        18..=23 => "Alta qualidade — recomendado.",
        24..=30 => "Equilíbrio entre qualidade e tamanho.",
        31..=40 => "Arquivo pequeno, com perda visível.",
        _ => "Arquivo mínimo, qualidade baixa.",
    }
}

fn enc_label(id: &str) -> String {
    match id {
        "h264-hw" => lang::tr("H.264 — hardware"),
        "h264-sw" => lang::tr("H.264 — software (x264, CPU)"),
        "av1-hw" => lang::tr("AV1 — hardware"),
        "av1-sw" => lang::tr("AV1 — software (SVT-AV1, CPU)"),
        "hevc-hw" => lang::tr("HEVC / H.265 — hardware"),
        _ => id.to_owned(),
    }
}

// ---------------------------------------------------------------------------
// Desenho
// ---------------------------------------------------------------------------

fn render_all(ui: &mut Ui) {
    render_regions(ui);
    render_stage(ui);
    render_playbar(ui);
    render_timeline(ui);
    render_audio(ui);
    render_resolutions(ui);
    render_encoders(ui);
    render_scale(ui);
    render_speed(ui);
    render_fps(ui);
    render_quality(ui);
    render_captions(ui);
    render_container(ui);
    render_actions(ui);
    sync_caption(ui);
    layout_page(ui);
}

/// As instâncias de um frame dentro de `parent`, criando as que faltam para chegar a `n`
/// (as que sobram ficam: quem chama esconde ou libera).
fn instances<T>(ui: &mut Ui, parent: ComponentId, n: usize, of: fn(&Ui, ComponentId) -> Option<T>, make: fn(&mut Ui, ComponentId) -> Result<T, Error>) -> Vec<T> {
    let mut have: Vec<T> = ui.children(parent).iter().filter_map(|&c| of(ui, c)).collect();
    while have.len() < n {
        match make(ui, parent) {
            Ok(t) => have.push(t),
            Err(_) => break,
        }
    }
    have
}

/// As pílulas de um `FlowLayout` (`fraExPill`): (índice, texto, marcada, desligada). As que
/// já existem são reaproveitadas (o índice vai no `tag`, que o clique lê); as que sobram saem.
fn pills(ui: &mut Ui, flow: ComponentId, items: Vec<(i32, String, bool, bool)>, f: fn(&mut Ui, i32)) {
    let mut list: Vec<g::FraExPillComponents> = ui.children(flow).iter().filter_map(|&c| FraExPill::components_of(ui, c)).collect();
    while list.len() < items.len() {
        let Ok(p) = FraExPill::create_in(ui, flow) else { break };
        ui.on(p.pnlPill.id, "on_click", move |ui, _, _| {
            if p.pnlPill.enabled(ui) {
                f(ui, p.pnlPill.tag(ui) as i32);
            }
        });
        list.push(p);
    }
    if list.len() > items.len() {
        let extra: Vec<ComponentId> = list.drain(items.len()..).map(|p| p.form.id).collect();
        ui.free_many(&extra);
    }
    for (p, (idx, text, checked, disabled)) in list.into_iter().zip(items) {
        if p.lblPill.text(ui) != text {
            p.lblPill.set_text(ui, text.as_str());
        }
        let w = chk_width(ui, &text);
        if let Some(cb) = ui.control_mut(p.form.id) {
            cb.set_width(w);
        }
        p.pnlPill.set_tag(ui, idx as i64);
        set_chk(ui, p.pnlPill, p.chkPill, checked, !disabled, false);
    }
}

fn render_regions(ui: &mut Ui) {
    let items = with(|st| {
        let regs = st.layout.as_ref().map(|l| l.regions.clone()).unwrap_or_default();
        let mut v = vec![(0, if regs.len() > 1 { lang::tr("Todas") } else { lang::tr("Tela inteira") }, !st.no_video && st.sel_regions.is_empty(), false)];
        if regs.len() > 1 {
            for (i, r) in regs.iter().enumerate() {
                let name = if r.name.is_empty() { tw("Região {{n}}", &[("n", &(i + 1).to_string())]) } else { r.name.clone() };
                v.push((i as i32 + 1, name, !st.no_video && st.sel_regions.contains(&i), false));
            }
        }
        v.push((-1, lang::tr("Nenhuma (só áudio)"), st.no_video, st.loaded && st.audio.is_empty()));
        v
    });
    if let Some(f) = frm(ui) {
        pills(ui, f.flwExRegions.id, items, region_clicked);
    }
}

fn region_clicked(ui: &mut Ui, idx: i32) {
    with(|st| {
        match idx {
            0 => {
                st.sel_regions.clear();
                st.no_video = false;
            }
            -1 => {
                if st.loaded && st.audio.is_empty() {
                    return;
                }
                st.no_video = !st.no_video;
                if st.no_video {
                    st.sel_regions.clear();
                }
            }
            i => {
                let i = (i - 1) as usize;
                st.no_video = false;
                if !st.sel_regions.remove(&i) {
                    st.sel_regions.insert(i);
                }
            }
        }
        // Recorte e telas são dois jeitos de dizer o que sai: o último vale.
        st.crop = None;
        if compose_on(st) || !crop_available(st) {
            st.crop_zoom = 1.0;
            st.pan = (0.0, 0.0);
        }
        if st.no_video {
            if st.container != "mp3" {
                st.video_container = st.container.clone();
            }
            st.container = "mp3".into();
        } else if st.container == "mp3" {
            st.container = st.video_container.clone();
        }
    });
    render_all(ui);
    push_view();
}

/// Proporção e tamanho do palco, recorte, zoom, mensagem de só áudio.
fn render_stage(ui: &mut Ui) {
    let (dims, crop_on, r, sw, sh, zoom, pan, no_video, has, col) =
        with(|st| (stage_dims(st), crop_available(st), crop_rect(st), st.src_w, st.src_h, st.crop_zoom, st.pan, st.no_video, has_crop(st), st.col_w));
    // O palco (como no original): o teto de altura (58 % da janela, até 560) vira teto de
    // largura; o quadro fica com 15 px de folga em volta, e o palco centrado na coluna.
    let aspect = if dims.0 > 0 && dims.1 > 0 { dims.0 as f32 / dims.1 as f32 } else { 16.0 / 9.0 };
    let win_h = main_form().and_then(|f| ui.bounds(f)).map_or(900.0, |b| b.height);
    let max_h = (win_h * 0.58).min(560.0);
    let stage_w = col.min((max_h * aspect + 28.0).round());
    let fw = stage_w - 2.0 * STAGE_GAP;
    let fh = fw / aspect;
    let side = ((col - stage_w) / 2.0).max(0.0).floor();
    let fx = STAGE_GAP;
    let changed = with(|st| {
        let ch = (st.frame.0 - fw).abs() > 0.5 || (st.frame.1 - fh).abs() > 0.5;
        st.frame = (fw, fh);
        st.frame_at = (fx, STAGE_GAP);
        if ch {
            clamp_pan(st);
            st.cap_cache = None;
        }
        ch
    });
    if changed {
        push_view();
    }
    let Some(f) = frm(ui) else { return };
    if let Some(cb) = ui.control_mut(f.pnlExStage.id) {
        cb.set_height(fh + 2.0 * STAGE_GAP).set_margins([side, 0.0, side, 0.0]);
    }
    // Sem recorte o palco tem cantos arredondados; com recorte, retos (como na web).
    f.pnlExStage.set_style(ui, if crop_on { "ex_stage_crop" } else { "ex_stage" });
    f.lyExCrop.set_visible(ui, crop_on);
    f.lblExCropHint.set_visible(ui, crop_on);
    if let Some(cb) = ui.control_mut(f.fvExFrame.id) {
        cb.set_rect(fx, STAGE_GAP, fw, fh).set_visible(!no_video);
    }
    f.lblExAudioOnly.set_visible(ui, no_video);
    // A moldura acompanha o zoom (origem no centro) e o deslocamento.
    let cr = f.crpExCrop.id;
    let (zx, zy) = (fx + pan.0 + fw / 2.0 * (1.0 - zoom), STAGE_GAP + pan.1 + fh / 2.0 * (1.0 - zoom));
    if let Some(cb) = ui.control_mut(cr) {
        cb.set_rect(zx, zy, fw * zoom, fh * zoom).set_visible(crop_on && !no_video);
    }
    if let Some(b) = ui.get_mut::<CropBox>(cr) {
        if !b.is_dragging() && sw > 0 && sh > 0 {
            b.source_width = sw as f32;
            b.source_height = sh as f32;
            b.set_selection(r.x as f32, r.y as f32, r.w as f32, r.h as f32);
        }
    }
    f.lblExZoom.set_text(ui, format!("{}%", (zoom * 100.0).round() as i32));
    f.btnExZoomReset.set_enabled(ui, zoom > 1.001);
    f.btnExZoomIn.set_enabled(ui, crop_on);
    f.btnExZoomOut.set_enabled(ui, crop_on && zoom > 1.001);
    f.btnExCropReset.set_enabled(ui, has);
    f.lblExCropInfo.set_text(ui, crop_info(has, r, sw, sh));
    // Legenda proporcional ao quadro.
    if fh > 0.0 {
        let font = (fh * 0.045).max(9.0);
        let w = (fw * 0.86).min(font * 32.0);
        f.lblExCaption.set_font_size(ui, font as f64);
        if let Some(cb) = ui.control_mut(f.lblExCaption.id) {
            cb.set_rect(fx + (fw - w) / 2.0, STAGE_GAP + fh - font * 3.2 - 12.0, w, font * 3.2);
        }
    }
    ui.invalidate();
}

/// O tamanho do recorte, se houver.
fn crop_info(has: bool, r: Rect, sw: i32, sh: i32) -> String {
    if has {
        tw("Recorte de {{w}}×{{h}}, de {{sw}}×{{sh}} da origem", &[("w", &r.w.to_string()), ("h", &r.h.to_string()), ("sw", &sw.to_string()), ("sh", &sh.to_string())])
    } else {
        String::new()
    }
}

fn render_playbar(ui: &mut Ui) {
    let (playing, muted, vol, has, no_video, on, ready) = with(|st| {
        (st.playing, st.user_muted || st.user_vol == 0, st.user_vol, st.turns.as_ref().is_some_and(|t| !t.is_empty()), st.no_video, st.captions_on, st.ready)
    });
    let Some(f) = frm(ui) else { return };
    f.btnExPlay.set_enabled(ui, ready);
    f.btnExPlay.set_icon(ui, if playing { "pl-pause" } else { "pl-play" });
    f.btnExPlay.set_hint(ui, lang::tr(if playing { "Pausar" } else { "Tocar" }));
    f.btnExMute.set_icon(ui, if muted { "pl-vol-off" } else { "pl-vol-high" });
    f.btnExMute.set_hint(ui, lang::tr(if muted { "Ativar som" } else { "Silenciar" }));
    if (f.trkExVolume.value(ui) - vol as f64).abs() > 0.5 {
        f.trkExVolume.set_value(ui, vol as f64);
    }
    f.btnExCc.set_enabled(ui, has && !no_video);
    f.btnExCc.set_style(ui, if on && has && !no_video { "ex_cc_on" } else { "ex_cc" });
    f.btnExCc.set_opacity(ui, if has && !no_video { 1.0 } else { 0.45 });
    f.btnExCc.set_hint(
        ui,
        lang::tr(if !has {
            "Esta gravação não tem transcrição"
        } else if on {
            "Esconder legenda"
        } else {
            "Mostrar legenda"
        }),
    );
}

// ---- linha do tempo ----

fn max_zoom(st: &State) -> f64 {
    (st.duration / 2.0).min(2000.0).max(1.0)
}

fn total_w(st: &State) -> f64 {
    st.view_w.max(1.0) * st.zoom.max(1.0)
}

fn clamp_scroll(st: &mut State) {
    let max = (total_w(st) - st.view_w).max(0.0);
    st.scroll = st.scroll.clamp(0.0, max);
}

fn render_timeline(ui: &mut Ui) {
    let Some(f) = frm(ui) else { return };
    let dark = crate::bridge::resolved_theme() == "dark";
    struct Marks {
        ticks: Vec<(f32, String)>,
        labels: Vec<(f32, f32, String)>,
        sb: (f32, f32),
        img: Option<Image>,
        ph: f32,
        view: f32,
    }
    let m = with(|st| {
        clamp_scroll(st);
        st.cursor_part = part_at_cursor(st);
        let dur = st.duration.max(0.001);
        let total = total_w(st);
        let view = st.view_w;
        let x_of = |sec: f64| (sec / dur * total - st.scroll) as f32;
        // Régua com passo de ~90 px, só a janela visível.
        let mut ticks = Vec::new();
        if st.duration > 0.0 {
            let step = TICK_STEPS.iter().copied().find(|&s| s / st.duration * total >= 90.0).unwrap_or(3600.0);
            let spp = st.duration / total;
            let from = (st.scroll * spp).max(0.0);
            let to = ((st.scroll + view) * spp).min(st.duration);
            let mut t = (from / step).ceil() * step;
            while t <= to {
                if t / st.duration > 0.985 {
                    break;
                }
                ticks.push((x_of(t), fmt_time(t)));
                t += step;
            }
        }
        // A duração de cada trecho no meio dele, só sem a forma de onda (como no original).
        let has_wave = st.peaks.as_ref().is_some_and(|p| !p.is_empty());
        let labels = st
            .parts
            .iter()
            .filter(|p| !has_wave && (p.end - p.start) / dur * total > 60.0)
            .map(|p| (x_of(p.start), ((p.end - p.start) / dur * total) as f32, fmt_time(p.end - p.start)))
            .collect();
        let sb_w = if total > 0.0 { (view * view / total).max(20.0).min(view) } else { view };
        let sb_x = if total > view { st.scroll / (total - view) * (view - sb_w) } else { 0.0 };
        let img = timeline_image(st, dark, st.scale);
        Marks { ticks, labels, sb: (sb_x as f32, sb_w as f32), img, ph: x_of(st.play_sec), view: view as f32 }
    });
    if let Some(cb) = ui.control_mut(f.imgExTl.id) {
        cb.set_rect(1.0, 1.0, m.view, TL_H);
    }
    if let Some(pic) = m.img {
        if let Some(iv) = ui.get_mut::<ImageView>(f.imgExTl.id) {
            iv.set_picture(Some(pic));
        }
    }
    // A duração dos trechos (`fraExPartLabel`), na faixa: os frames ficam de um desenho
    // para o outro, e os que sobram se escondem.
    let labels: Vec<(f32, f32, String)> = m
        .labels
        .into_iter()
        .filter_map(|(x, w, text)| {
            let (lx, lw) = (x.max(0.0), (x + w).min(m.view) - x.max(0.0));
            (lw >= 50.0).then_some((lx, lw, text))
        })
        .collect();
    let pool = instances(ui, f.lyExTlLabels.id, labels.len(), FraExPartLabel::components_of, FraExPartLabel::create_in);
    for (i, p) in pool.iter().enumerate() {
        let shown = labels.get(i);
        if let Some((_, _, text)) = shown {
            if p.lblPart.text(ui) != *text {
                p.lblPart.set_text(ui, text.as_str());
            }
        }
        if let Some(cb) = ui.control_mut(p.form.id) {
            if let Some((lx, lw, _)) = shown {
                cb.set_rect(*lx, 0.0, *lw, TL_H);
            }
            cb.set_visible(shown.is_some());
        }
    }
    // A régua (`fraExTick`): o fio na altura toda e o tempo (9 px) 4 px depois.
    let pool = instances(ui, f.lyExTicks.id, m.ticks.len(), FraExTick::components_of, FraExTick::create_in);
    for (i, t) in pool.iter().enumerate() {
        let shown = m.ticks.get(i);
        if let Some((_, text)) = shown {
            if t.lblTick.text(ui) != *text {
                t.lblTick.set_text(ui, text.as_str());
            }
        }
        if let Some(cb) = ui.control_mut(t.form.id) {
            if let Some((x, _)) = shown {
                cb.set_left(*x);
            }
            cb.set_visible(shown.is_some());
        }
    }
    // O cursor por cima de tudo: 4 px escuros e 2 px claros.
    let on = (-2.0..=m.view + 2.0).contains(&m.ph);
    for (bar, dx) in [(f.shpExPlayShade, 2.0), (f.shpExPlayLine, 1.0)] {
        if let Some(cb) = ui.control_mut(bar.id) {
            if on {
                cb.set_left(1.0 + m.ph - dx);
            }
            cb.set_visible(on);
        }
    }
    for c in [f.pnlExSb.id, f.shpExRuler.id, f.shpExRulerLine.id, f.lyExTlLabels.id, f.lyExTicks.id] {
        if let Some(cb) = ui.control_mut(c) {
            cb.set_width(m.view);
        }
    }
    if let Some(cb) = ui.control_mut(f.shpExSbThumb.id) {
        cb.set_left(m.sb.0).set_width(m.sb.1);
    }
    render_tlbar(ui);
    ui.invalidate();
}

/// Fundo dos trechos (mantido verde, tirado listrado, realce do trecho do cursor), a forma
/// de onda e as divisões, só na janela visível.
fn timeline_image(st: &State, dark: bool, k: f32) -> Option<Image> {
    let view = st.view_w as f32;
    if view <= 1.0 {
        return None;
    }
    let (wpx, hpx) = ((view * k).round().max(1.0) as u32, (TL_H * k).round().max(1.0) as u32);
    let mut px = vec![0u8; (wpx * hpx * 4) as usize];
    let rgb = |c: u32| ((c >> 16) as u8, (c >> 8) as u8, c as u8);
    let (p2, p3, b2, succ, t3) = if dark {
        (rgb(0x1a1a1e), rgb(0x202024), rgb(0x36363c), rgb(0x10b981), rgb(0x71717a))
    } else {
        (rgb(0xfafafa), rgb(0xf4f4f5), rgb(0xd4d4d8), rgb(0x059669), rgb(0x71717a))
    };
    let blend = |a: (u8, u8, u8), b: (u8, u8, u8), t: f32| {
        let m = |x: u8, y: u8| (x as f32 + (y as f32 - x as f32) * t).round() as u8;
        (m(a.0, b.0), m(a.1, b.1), m(a.2, b.2))
    };
    let keep_bg = blend(p2, (16, 185, 129), 0.16);
    let dur = st.duration.max(0.001);
    let total = total_w(st) as f32;
    let scroll = st.scroll as f32;
    let cur = st.cursor_part;
    let put = |px: &mut [u8], x: u32, y: u32, c: (u8, u8, u8)| {
        let i = ((y * wpx + x) * 4) as usize;
        px[i..i + 4].copy_from_slice(&[c.0, c.1, c.2, 255]);
    };
    let get = |px: &[u8], x: u32, y: u32| {
        let i = ((y * wpx + x) * 4) as usize;
        (px[i], px[i + 1], px[i + 2])
    };
    // Fundo, coluna por coluna (cada coluna cai num trecho só).
    let sec_at = |xp: u32| ((scroll + (xp as f32 + 0.5) / k) / total) as f64 * dur;
    let mut pi = 0usize;
    for xp in 0..wpx {
        let s = sec_at(xp);
        while pi + 1 < st.parts.len() && s >= st.parts[pi].end {
            pi += 1;
        }
        let Some(p) = st.parts.get(pi) else { break };
        let x_css = scroll + xp as f32 / k - (p.start / dur) as f32 * total;
        for yp in 0..hpx {
            let c = if p.keep {
                keep_bg
            } else {
                let d = (x_css + yp as f32 / k) / std::f32::consts::SQRT_2;
                if d.rem_euclid(12.0) < 6.0 { p3 } else { p2 }
            };
            put(&mut px, xp, yp, c);
        }
    }
    // Realce do trecho sob o cursor.
    if cur >= 0 {
        if let Some(p) = st.parts.get(cur as usize) {
            let x0 = ((p.start / dur) as f32 * total - scroll) * k;
            let x1 = ((p.end / dur) as f32 * total - scroll) * k;
            let t = k.round().max(1.0) as i32;
            let (xa, xb) = (x0.round() as i32, x1.round() as i32);
            for xp in xa.max(0)..xb.min(wpx as i32) {
                for d in 0..t {
                    put(&mut px, xp as u32, d as u32, b2);
                    put(&mut px, xp as u32, hpx - 1 - d as u32, b2);
                }
            }
            for d in 0..t {
                for edge in [xa + d, xb - 1 - d] {
                    if edge >= 0 && edge < wpx as i32 {
                        for yp in 0..hpx {
                            put(&mut px, edge as u32, yp, b2);
                        }
                    }
                }
            }
        }
    }
    // Forma de onda: pico por fatia, raiz quadrada, barras de ~2 px.
    if let Some(peaks) = st.peaks.as_ref().filter(|p| !p.is_empty()) {
        let n = peaks.len();
        let step = (2.0 * k).max(1.0);
        let bar_w = (step * 0.7).max(1.0);
        let mut x = 0.0f32;
        while x < wpx as f32 {
            let f0 = (scroll + x / k) / total;
            let f1 = (scroll + (x + step) / k) / total;
            if f0 >= 1.0 {
                break;
            }
            let i0 = ((f0 * n as f32).floor() as usize).min(n - 1);
            let i1 = ((f1 * n as f32).ceil() as usize).clamp(i0 + 1, n);
            let m = peaks[i0..i1].iter().cloned().fold(0.0f32, f32::max);
            let bh = ((m / st.wave_max).sqrt() * hpx as f32 * 0.82).max(k);
            let mid = ((f0 + f1) / 2.0) as f64 * dur;
            let kept = st.parts.iter().find(|p| mid >= p.start && mid < p.end).is_none_or(|p| p.keep);
            let (col, a) = if kept { (succ, 0.85) } else { (t3, 0.45) };
            let y0 = ((hpx as f32 - bh) / 2.0).max(0.0);
            let y1 = (y0 + bh).min(hpx as f32);
            let xa = x.floor() as u32;
            let xb = ((x + bar_w).ceil() as u32).min(wpx);
            for xp in xa..xb {
                let cov_x = ((xp as f32 + 1.0).min(x + bar_w) - (xp as f32).max(x)).clamp(0.0, 1.0);
                for yp in (y0.floor() as u32)..(y1.ceil() as u32).min(hpx) {
                    let cov_y = ((yp as f32 + 1.0).min(y1) - (yp as f32).max(y0)).clamp(0.0, 1.0);
                    let b = get(&px, xp, yp);
                    put(&mut px, xp, yp, blend(b, col, a * cov_x * cov_y));
                }
            }
            x += step;
        }
    }
    // As divisões, por cima: uma faixa de 5 px escura (`black/55`) com a linha de 3 px no
    // meio, azul (`blue_2`) ou vermelha (`danger_2`) na mais próxima do cursor. Cada coluna
    // recebe a cor na proporção em que a faixa a cobre (a borda fracionária, como o
    // retângulo antialiasado).
    let band = |px: &mut [u8], a: f32, b: f32, c: (u8, u8, u8), alpha: f32| {
        let (a, b) = (a * k, b * k);
        for xp in (a.floor().max(0.0) as u32)..(b.ceil().max(0.0) as u32).min(wpx) {
            let cov = ((xp as f32 + 1.0).min(b) - (xp as f32).max(a)).clamp(0.0, 1.0);
            if cov <= 0.0 {
                continue;
            }
            for yp in 0..hpx {
                let under = get(px, xp, yp);
                put(px, xp, yp, blend(under, c, alpha * cov));
            }
        }
    };
    let target = nearest_cut(st);
    let total64 = total_w(st);
    for i in 0..st.parts.len().saturating_sub(1) {
        let x = (st.parts[i].end / dur * total64 - st.scroll) as f32;
        if !(-3.0..=view + 3.0).contains(&x) {
            continue;
        }
        let line = if target == i as i32 + 1 { rgb(0xf87171) } else { rgb(0x60a5fa) };
        band(&mut px, x - 2.5, x + 2.5, (0, 0, 0), 0.55);
        band(&mut px, x - 1.5, x + 1.5, line, 1.0);
    }
    Some(Image::rgba(wpx, hpx, px))
}

fn render_tlbar(ui: &mut Ui) {
    let (cut_ok, uncut, uncut_at, toggle, toggle_ok, zoom, play, total, kept, spd) = with(|st| {
        let idx = part_at_cursor(st);
        let p = (idx >= 0).then(|| st.parts[idx as usize]);
        let t = st.play_sec;
        let cut_ok = p.is_some_and(|p| t > p.start + 0.05 && t < p.end - 0.05);
        let ci = nearest_cut(st);
        let at = if ci > 0 { st.parts[ci as usize].start } else { 0.0 };
        (cut_ok, ci, at, p.is_none_or(|p| p.keep), p.is_some(), st.zoom, st.play_sec, st.duration, kept_duration(st), speed(st))
    });
    let Some(f) = frm(ui) else { return };
    f.btnExCut.set_enabled(ui, cut_ok);
    f.btnExUncut.set_enabled(ui, uncut > 0);
    f.btnExUncut.set_hint(ui, if uncut > 0 { tw("Remove a divisão em {{at}} (junta os dois trechos)", &[("at", &fmt_time(uncut_at))]) } else { lang::tr("Não há divisão para remover") });
    f.btnExToggle.set_text(ui, lang::tr(if toggle { "Tirar trecho" } else { "Devolver trecho" }));
    f.btnExToggle.set_enabled(ui, toggle_ok);
    f.lblExTlZoom.set_text(ui, format!("{}×", if zoom < 10.0 { format!("{zoom:.1}") } else { format!("{}", zoom.round()) }));
    // O campo do tempo não muda enquanto o usuário digita nele.
    let focused = main_form().and_then(|m| ui.focused(m)) == Some(f.edtExTime.id);
    if !focused {
        f.edtExTime.set_text(ui, fmt_time(play));
    }
    f.lblExRangeText.set_text(ui, format!("/   {}", fmt_time(total)));
    let mut len = tw("final: {{len}}", &[("len", &fmt_time(kept))]);
    if spd > 1.0 {
        len.push_str(&format!("   ·   {}", tw("Duração final: {{len}}", &[("len", &fmt_time(kept / spd))])));
    }
    let mut st = TextStyle::default();
    st.font.family = "Segoe UI".into();
    st.font.size = 11.0;
    let w = ui.measure_text(&len, &st).width + 4.0;
    if let Some(cb) = ui.control_mut(f.lblExRangeLen.id) {
        cb.set_width(w.ceil());
    }
    f.lblExRangeLen.set_text(ui, len);
}

/// Move o cursor (barra, tempo, botões); `seek` = também reposiciona o vídeo.
fn set_play_sec(ui: &mut Ui, sec: f64, seek: bool) {
    let (changed, ph) = with(|st| {
        let scroll0 = st.scroll;
        st.play_sec = sec.clamp(0.0, st.duration.max(0.0));
        if seek {
            if let Some(pb) = &st.pb {
                pb.seek(st.play_sec);
            }
        }
        // Mantém o cursor na vista.
        let total = total_w(st);
        if st.duration > 0.0 {
            let x = st.play_sec / st.duration * total;
            let margin = (st.view_w * 0.15).min(60.0);
            if x < st.scroll + margin {
                st.scroll = (x - margin).max(0.0);
            } else if x > st.scroll + st.view_w - margin {
                st.scroll = x - st.view_w + margin;
            }
        }
        let changed = part_at_cursor(st) != st.cursor_part || (st.scroll - scroll0).abs() > 0.01;
        let ph = if st.duration > 0.0 { (st.play_sec / st.duration * total - st.scroll) as f32 } else { 0.0 };
        (changed, ph)
    });
    // A imagem só muda quando o trecho do cursor ou a rolagem muda.
    if changed {
        render_timeline(ui);
    } else {
        // O cursor anda sozinho (escondido, fora da vista, continua escondido).
        if let Some(f) = frm(ui) {
            for (bar, dx) in [(f.shpExPlayShade, 2.0), (f.shpExPlayLine, 1.0)] {
                if let Some(cb) = ui.control_mut(bar.id) {
                    cb.set_left(1.0 + ph - dx);
                }
            }
        }
        render_tlbar(ui);
        ui.invalidate();
    }
    sync_caption(ui);
}

fn tl_seek(ui: &mut Ui, x: f32) {
    let sec = with(|st| {
        let total = total_w(st);
        ((st.scroll + x as f64) / total * st.duration).clamp(0.0, st.duration)
    });
    set_play_sec(ui, sec, true);
}

fn set_zoom(ui: &mut Ui, z: f64, anchor: Option<f64>) {
    let ok = with(|st| {
        let prev = st.zoom.max(1.0);
        let next = z.clamp(1.0, max_zoom(st));
        if (next - prev).abs() < 0.0001 || st.duration <= 0.0 {
            return false;
        }
        let view = st.view_w.max(1.0);
        let anchor = anchor.unwrap_or_else(|| (st.scroll + view / 2.0) / (view * prev) * st.duration);
        let off = anchor / st.duration * (view * prev) - st.scroll;
        st.zoom = next;
        st.scroll = anchor / st.duration * (view * next) - off;
        clamp_scroll(st);
        true
    });
    if ok {
        render_timeline(ui);
    }
}

pub(crate) fn tl_wheel(ui: &mut Ui, x: f32, dy: f32) {
    let (anchor, z) = with(|st| ((st.scroll + x as f64) / total_w(st) * st.duration, st.zoom));
    // Roda pra cima (delta positivo) aproxima.
    set_zoom(ui, z * if dy > 0.0 { 1.25 } else { 1.0 / 1.25 }, Some(anchor));
}

fn sb_drag(ui: &mut Ui, x: f32) {
    let ok = with(|st| {
        let Some((from, scroll0)) = st.sb_drag else { return false };
        let total = total_w(st);
        let view = st.view_w;
        let sb_w = (view * view / total).max(20.0).min(view);
        if total > view && view > sb_w {
            st.scroll = scroll0 + (x - from) as f64 / (view - sb_w) * (total - view);
        }
        clamp_scroll(st);
        true
    });
    if ok {
        render_timeline(ui);
    }
}

/// O campo do tempo ao sair dele ou com Enter: vai para o tempo digitado (inválido, volta).
pub(crate) fn time_commit(ui: &mut Ui, text: &str) {
    match parse_time(text) {
        Some(v) => set_play_sec(ui, v, true),
        None => render_tlbar(ui),
    }
}

/// Apertar na linha do tempo leva o cursor ao ponto, e arrastar o acompanha.
pub(crate) fn tl_down(ui: &mut Ui, x: f32) {
    with(|st| st.tl_drag = true);
    tl_seek(ui, x);
}

pub(crate) fn tl_move(ui: &mut Ui, x: f32) {
    if with(|st| st.tl_drag) {
        tl_seek(ui, x);
    }
}

pub(crate) fn tl_up() {
    with(|st| st.tl_drag = false);
}

/// Os botões − e + do zoom da linha do tempo.
pub(crate) fn tl_zoom(ui: &mut Ui, closer: bool) {
    let z = with(|st| st.zoom);
    set_zoom(ui, if closer { z * 1.6 } else { z / 1.6 }, None);
}

/// A posição do polegar da barra de rolagem (o ponteiro vem nas coordenadas dele).
fn thumb_left(ui: &Ui) -> f32 {
    frm(ui).and_then(|f| ui.control(f.shpExSbThumb.id)).map_or(0.0, |cb| cb.left())
}

/// Arrastar o polegar da barra de rolagem rola a linha do tempo.
pub(crate) fn sb_down(ui: &mut Ui, x: f32) {
    let x0 = thumb_left(ui);
    with(|st| st.sb_drag = Some((x + x0, st.scroll)));
}

pub(crate) fn sb_move(ui: &mut Ui, x: f32) {
    let x0 = thumb_left(ui);
    sb_drag(ui, x + x0);
}

pub(crate) fn sb_up() {
    with(|st| st.sb_drag = None);
}

pub(crate) fn cut_here(ui: &mut Ui) {
    with(|st| {
        let t = st.play_sec;
        if let Some(i) = st.parts.iter().position(|p| t > p.start + 0.05 && t < p.end - 0.05) {
            let p = st.parts[i];
            st.parts.splice(i..=i, [Part { start: p.start, end: t, keep: p.keep }, Part { start: t, end: p.end, keep: p.keep }]);
        }
    });
    render_timeline(ui);
}

pub(crate) fn remove_cut(ui: &mut Ui) {
    with(|st| {
        let i = nearest_cut(st);
        if i < 1 {
            return;
        }
        let i = i as usize;
        let (l, r) = (st.parts[i - 1], st.parts[i]);
        st.parts.splice(i - 1..=i, [Part { start: l.start, end: r.end, keep: l.keep || r.keep }]);
    });
    render_timeline(ui);
}

pub(crate) fn toggle_part(ui: &mut Ui) {
    let refused = with(|st| {
        let i = part_at_cursor(st);
        if i < 0 {
            return false;
        }
        let i = i as usize;
        // Não deixa zerar tudo.
        if st.parts[i].keep && st.parts.iter().filter(|p| p.keep).count() == 1 {
            return true;
        }
        st.parts[i].keep = !st.parts[i].keep;
        false
    });
    if refused {
        crate::app_view::show_toast(&lang::tr("Erro"), &lang::tr("Pelo menos um trecho precisa entrar no vídeo final."), true, 4000);
    }
    render_timeline(ui);
}

// ---- reprodução ----

pub(crate) fn toggle_play(ui: &mut Ui) {
    if !with(|st| st.ready) {
        return;
    }
    with(|st| {
        let Some(pb) = &st.pb else { return };
        if st.playing {
            pb.pause();
            st.playing = false;
            st.play_sec = pb.shared.position();
        } else {
            pb.seek(st.play_sec);
            pb.play();
            st.playing = true;
        }
    });
    render_playbar(ui);
}

pub(crate) fn toggle_mute(ui: &mut Ui) {
    with(|st| {
        st.user_muted = !st.user_muted;
        if !st.user_muted && st.user_vol == 0 {
            st.user_vol = 100;
        }
    });
    push_volumes();
    render_playbar(ui);
}

pub(crate) fn set_volume(ui: &mut Ui, v: i32) {
    let same = with(|st| {
        let same = st.user_vol == v.clamp(0, 100);
        st.user_vol = v.clamp(0, 100);
        if st.user_vol > 0 {
            st.user_muted = false;
        }
        same
    });
    if !same {
        push_volumes();
        render_playbar(ui);
    }
}

pub(crate) fn toggle_cc(ui: &mut Ui) {
    with(|st| st.captions_on = !st.captions_on);
    config::set_config_bool("exportCaptions", with(|st| st.captions_on));
    render_playbar(ui);
    sync_caption(ui);
}

fn sync_caption(ui: &mut Ui) {
    let (font, width) = with(|st| {
        let fh = st.frame.1;
        let font = (fh * 0.045).max(9.0);
        (font, (st.frame.0 * 0.86).min(font * 32.0))
    });
    let text = with(|st| {
        if !st.captions_on || st.no_video {
            return None;
        }
        let turns = st.turns.as_ref().filter(|t| !t.is_empty())?;
        // Turnos sobrepostos: vale o que começou por último.
        let t = st.play_sec;
        let mut best: Option<usize> = None;
        for (i, tr) in turns.iter().enumerate() {
            if t >= tr.start && t < tr.end && best.is_none_or(|b| tr.start >= turns[b].start) {
                best = Some(i);
            }
        }
        let idx = best?;
        let key = (width * 10.0) as i32 + (font * 1000.0) as i32;
        let valid = st.cap_cache.as_ref().is_some_and(|(i, k, _)| *i == idx && *k == key);
        if !valid {
            let tr = Transcript { turns: vec![turns[idx].clone()], ..Default::default() };
            st.cap_cache = Some((idx, key, crate::app_player::caption_chunks(&tr, 0, width, font)));
        }
        let chunks = &st.cap_cache.as_ref()?.2;
        let mut pick = chunks.first()?;
        for c in chunks {
            if t >= c.1 {
                pick = c;
            } else {
                break;
            }
        }
        Some(pick.0.clone())
    });
    let text = text.unwrap_or_default();
    if let Some(f) = frm(ui) {
        if f.lblExCaption.text(ui) != text {
            f.lblExCaption.set_text(ui, text.as_str());
            f.lblExCaption.set_visible(ui, !text.is_empty());
        }
    }
}

// ---- recorte e zoom da prévia ----

/// A moldura mudou (arrastando ou ao soltar).
pub(crate) fn crop_changed(ui: &mut Ui, sender: ComponentId, end: bool) {
    let Some((x, y, w, h)) = ui.get::<CropBox>(sender).map(|b| b.selection()) else { return };
    with(|st| {
        let r = Rect { x: x as i32, y: y as i32, w: w as i32, h: h as i32 };
        let full = r.x == 0 && r.y == 0 && r.w >= st.src_w - 1 && r.h >= st.src_h - 1;
        st.crop = if full { None } else { Some(r) };
    });
    if end {
        render_stage(ui);
        render_resolutions(ui);
        render_scale(ui);
        layout_page(ui);
    } else {
        let (r, sw, sh, has) = with(|st| (crop_rect(st), st.src_w, st.src_h, has_crop(st)));
        if let Some(f) = frm(ui) {
            f.lblExCropInfo.set_text(ui, crop_info(has, r, sw, sh));
        }
    }
}

fn clamp_pan(st: &mut State) {
    let (fw, fh) = st.frame;
    let mx = (fw * (st.crop_zoom - 1.0) / 2.0).max(0.0);
    let my = (fh * (st.crop_zoom - 1.0) / 2.0).max(0.0);
    st.pan.0 = st.pan.0.clamp(-mx, mx);
    st.pan.1 = st.pan.1.clamp(-my, my);
}

fn set_crop_zoom(ui: &mut Ui, z: f32, anchor: Option<(f32, f32, f32, f32)>) {
    with(|st| {
        let next = z.clamp(1.0, 12.0);
        if (next - st.crop_zoom).abs() < 0.0001 {
            return;
        }
        st.crop_zoom = next;
        if let Some((cx, cy, vx, vy)) = anchor {
            st.pan = (cx - vx * next, cy - vy * next);
        }
        if next <= 1.001 {
            st.pan = (0.0, 0.0);
        }
        clamp_pan(st);
    });
    push_view();
    render_stage(ui);
}

pub(crate) fn crop_zoom_by(ui: &mut Ui, f: f32) {
    let (z, pan) = with(|st| (st.crop_zoom, st.pan));
    set_crop_zoom(ui, z * f, Some((0.0, 0.0, -pan.0 / z, -pan.1 / z)));
}

/// "Usar tela inteira": desfaz o recorte.
pub(crate) fn crop_reset(ui: &mut Ui) {
    with(|st| st.crop = None);
    render_stage(ui);
    render_resolutions(ui);
    render_scale(ui);
}

pub(crate) fn crop_zoom_reset(ui: &mut Ui) {
    with(|st| {
        st.crop_zoom = 1.0;
        st.pan = (0.0, 0.0);
    });
    push_view();
    render_stage(ui);
}

/// Roda sobre a prévia (Ctrl): aproxima no ponto do mouse.
pub(crate) fn stage_wheel(ui: &mut Ui, sender: ComponentId, at: PointF, dy: f32) {
    if !with(|st| crop_available(st)) {
        return;
    }
    // O ponto em relação ao centro do quadro.
    let origin = ui.control(sender).map_or((0.0, 0.0), |cb| (cb.left(), cb.top()));
    let (z, pan, (fx, fy), (fw, fh)) = with(|st| (st.crop_zoom, st.pan, st.frame_at, st.frame));
    let (cx, cy) = (origin.0 + at.x - fx - fw / 2.0, origin.1 + at.y - fy - fh / 2.0);
    let (vx, vy) = ((cx - pan.0) / z, (cy - pan.1) / z);
    set_crop_zoom(ui, z * if dy > 0.0 { 1.2 } else { 1.0 / 1.2 }, Some((cx, cy, vx, vy)));
}

pub(crate) fn pan_down(ui: &mut Ui, x: f32, y: f32) {
    let started = with(|st| {
        if st.crop_zoom <= 1.001 {
            return false;
        }
        st.pan_drag = Some(PanDrag { sx: x, sy: y, px: st.pan.0, py: st.pan.1, moved: 0.0 });
        true
    });
    // Sem zoom (ou com as telas compostas), clicar na prévia toca/pausa.
    if !started {
        toggle_play(ui);
    }
}

pub(crate) fn pan_move(ui: &mut Ui, x: f32, y: f32) {
    let ok = with(|st| {
        let Some(d) = st.pan_drag.as_mut() else { return false };
        d.moved = d.moved.max((x - d.sx).abs() + (y - d.sy).abs());
        st.pan = (d.px + x - d.sx, d.py + y - d.sy);
        clamp_pan(st);
        true
    });
    if ok {
        push_view();
        render_stage(ui);
    }
}

pub(crate) fn pan_up(ui: &mut Ui) {
    let still = with(|st| st.pan_drag.take().is_some_and(|d| d.moved < 4.0));
    if still {
        toggle_play(ui);
    }
}

// ---- áudio ----

fn render_audio(ui: &mut Ui) {
    let (vis, orig_vis, orig, items, mix_checked, can_mix) = with(|st| {
        let orig = use_original(st);
        let mix = mix_index(st);
        let has_iso = st.sel_audio.iter().any(|&i| i != mix);
        let has_mix = st.sel_audio.contains(&mix);
        let items: Vec<(i32, String, bool, bool)> = st
            .audio
            .iter()
            .enumerate()
            .map(|(i, s)| {
                let is_mix = s.index == mix;
                let label = if !s.title.is_empty() {
                    s.title.clone()
                } else if is_mix {
                    tw("Faixa {{n}} (mixagem)", &[("n", &(i + 1).to_string())])
                } else {
                    tw("Faixa {{n}}", &[("n", &(i + 1).to_string())])
                };
                (i as i32, label, orig || st.sel_audio.contains(&s.index), orig || if is_mix { has_iso } else { has_mix })
            })
            .collect();
        let can_mix = !orig && st.sel_audio.len() > 1;
        if !can_mix {
            st.mix_checked = false;
        }
        (!st.audio.is_empty(), st.audio.len() > 1, orig, items, st.mix_checked, can_mix)
    });
    let Some(f) = frm(ui) else {
        push_volumes();
        return;
    };
    f.lyExAudio.set_visible(ui, vis);
    f.lyExOrigRow.set_visible(ui, orig_vis);
    set_chk(ui, f.pnlExOrig, f.chkExOrig, orig, true, true);
    pills(ui, f.flwExTracks.id, items, track_clicked);
    set_chk(ui, f.pnlExMix, f.chkExMix, mix_checked, can_mix, true);
    push_volumes();
    let col = with(|st| st.col_w);
    flow_height(ui, f.flwExTracks.id, col);
    fit(ui, f.lyExAudio.id);
}

/// As pílulas fixas do JSON (o áudio original, juntar as faixas, gravar a legenda): o
/// clique passa o novo estado.
pub(crate) fn orig_clicked(ui: &mut Ui, on: bool) {
    with(|st| st.original_audio = on);
    render_audio(ui);
}

pub(crate) fn mix_clicked(ui: &mut Ui, on: bool) {
    with(|st| st.mix_checked = on);
    render_audio(ui);
}

pub(crate) fn burn_clicked(ui: &mut Ui, on: bool) {
    with(|st| {
        st.burn = on;
        // Quem vai gravar a legenda quer ver como ela sai.
        if on && !st.captions_on {
            st.captions_on = true;
        }
    });
    if on {
        config::set_config_bool("exportCaptions", true);
    }
    render_captions(ui);
    sync_caption(ui);
}

fn track_clicked(ui: &mut Ui, i: i32) {
    with(|st| {
        let Some(s) = st.audio.get(i as usize) else { return };
        let idx = s.index;
        if !st.sel_audio.remove(&idx) {
            st.sel_audio.insert(idx);
        }
    });
    render_audio(ui);
    layout_page(ui);
}

// ---- vídeo ----

fn render_resolutions(ui: &mut Ui) {
    let (opts, idx, no_video) = with(|st| {
        let (cw, ch) = composed_size(st);
        let mut hs = vec![0];
        let mut opts = vec![tw("Original ({{w}} × {{h}})", &[("w", &cw.to_string()), ("h", &ch.to_string())])];
        for h in [1440, 1080, 720, 480] {
            if h < ch {
                hs.push(h);
                opts.push(format!("{h}p"));
            }
        }
        if !hs.contains(&st.res_h) {
            st.res_h = 0;
        }
        let idx = hs.iter().position(|&h| h == st.res_h).unwrap_or(0);
        st.res_heights = hs;
        (opts, idx, st.no_video)
    });
    let Some(f) = frm(ui) else { return };
    set_items(ui, f.cmbExRes, &opts, idx);
    f.cmbExRes.set_enabled(ui, !no_video);
    f.cmbExEnc.set_enabled(ui, !no_video);
}

/// A resolução escolhida (o índice na lista de alturas).
pub(crate) fn res_changed(ui: &mut Ui, i: usize) {
    with(|st| st.res_h = st.res_heights.get(i).copied().unwrap_or(0));
    render_resolutions(ui);
    render_scale(ui);
}

/// O codificador escolhido.
pub(crate) fn enc_changed(ui: &mut Ui, i: usize) {
    with(|st| {
        if let Some(e) = st.encoders.get(i) {
            st.enc_id = e.id.clone();
        }
    });
    render_encoders(ui);
    render_fps(ui);
    render_quality(ui);
}

fn render_encoders(ui: &mut Ui) {
    let (opts, idx) = with(|st| {
        if st.encoders.is_empty() {
            st.enc_id = "auto".into();
            return (vec![lang::tr("Automático (recomendado)")], 0);
        }
        // Padrão: o codec das Configurações, se estiver na lista.
        if st.enc_id.is_empty() || !st.encoders.iter().any(|e| e.id == st.enc_id) {
            let want = config::get_config_str("codec", "auto");
            st.enc_id = if st.encoders.iter().any(|e| e.id == want) { want } else { st.encoders[0].id.clone() };
        }
        let idx = st.encoders.iter().position(|e| e.id == st.enc_id).unwrap_or(0);
        (st.encoders.iter().map(|e| enc_label(&e.id)).collect(), idx)
    });
    if let Some(f) = frm(ui) {
        set_items(ui, f.cmbExEnc, &opts, idx);
    }
}

fn render_scale(ui: &mut Ui) {
    let (vis, algo) = with(|st| (!st.id.is_empty() && st.loaded && is_resizing(st), st.scale_algo.clone()));
    let Some(f) = frm(ui) else { return };
    let was = f.lyExScale.visible(ui);
    f.lyExScale.set_visible(ui, vis);
    let labels: Vec<String> = SCALES.iter().map(|i| lang::tr_of("export.scaleOpt", &i.to_string())).collect();
    set_items(ui, f.cmbExScale, &labels, SCALES.iter().position(|i| *i == algo).unwrap_or(0));
    f.lblExScaleHint.set_text(ui, lang::tr_of("export.scaleHint", &algo.to_string()));
    if was != vis {
        layout_page(ui);
    }
}

/// O algoritmo de redimensionamento escolhido.
pub(crate) fn scale_changed(ui: &mut Ui, i: usize) {
    with(|st| st.scale_algo = SCALES.get(i).copied().unwrap_or("bicubic").into());
    render_scale(ui);
}

pub(crate) fn speed_edit(ui: &mut Ui, text: &str) {
    let changed = with(|st| {
        if st.speed_text == text {
            return false;
        }
        st.speed_text = text.to_owned();
        // A taxa acompanha a velocidade: o seletor vai pro teto.
        st.fps_val = max_fps(st);
        true
    });
    if changed {
        render_speed(ui);
        render_fps(ui);
    }
}

pub(crate) fn speed_commit(ui: &mut Ui) {
    with(|st| st.speed_text = fmt_speed(speed(st)));
    render_speed(ui);
    render_fps(ui);
}

pub(crate) fn speed_step(ui: &mut Ui, dir: i32) {
    with(|st| {
        let cur = speed(st);
        let next = if dir > 0 {
            SPEED_STEPS.iter().copied().find(|&s| s > cur + 1e-9).unwrap_or(SPEED_MAX)
        } else {
            SPEED_STEPS.iter().rev().copied().find(|&s| s < cur - 1e-9).unwrap_or(1.0)
        };
        st.speed_text = fmt_speed(next);
        st.fps_val = max_fps(st);
    });
    render_speed(ui);
    render_fps(ui);
}

fn render_speed(ui: &mut Ui) {
    let (text, invalid) = with(|st| {
        let raw = st.speed_text.trim().replace(',', ".");
        let bad = !raw.is_empty() && raw.parse::<f64>().map_or(true, |v| !v.is_finite() || !(1.0..=SPEED_MAX).contains(&v));
        (st.speed_text.clone(), bad)
    });
    if let Some(f) = frm(ui) {
        if f.edtExSpeed.text(ui) != text {
            f.edtExSpeed.set_text(ui, text.as_str());
        }
        // Sem a cor própria (texto vazio), volta a do tema.
        f.edtExSpeed.set_border_color(ui, if invalid { "red" } else { "" });
    }
    render_tlbar(ui);
}

fn render_fps(ui: &mut Ui) {
    let (max, val, src) = with(|st| {
        let max = max_fps(st);
        st.fps_val = if max > 0 { st.fps_val.clamp(20.min(max), max) } else { 0 };
        (max, st.fps_val, st.src_fps)
    });
    let Some(f) = frm(ui) else { return };
    let was = f.lyExFps.visible(ui);
    f.lyExFps.set_visible(ui, max > 20);
    f.trkExFps.set_max(ui, max.max(21) as f64);
    f.trkExFps.set_min(ui, 20.0);
    if (f.trkExFps.value(ui) - val as f64).abs() > 0.5 {
        f.trkExFps.set_value(ui, val as f64);
    }
    let shown = if max > 20 { val } else { src.round() as i32 };
    put_value(ui, f.lblExFps, &tw("{{fps}} fps", &[("fps", &shown.to_string())]));
    if was != (max > 20) {
        layout_page(ui);
    }
}

/// O trilho dos quadros por segundo.
pub(crate) fn fps_changed(ui: &mut Ui, v: f64) {
    with(|st| st.fps_val = v as i32);
    render_fps(ui);
}

fn render_quality(ui: &mut Ui) {
    let (pos, c, svt) = with(|st| (st.q_pos, crf(st), st.enc_id == "av1-sw"));
    let Some(f) = frm(ui) else { return };
    if (f.trkExQuality.value(ui) - pos as f64).abs() > 0.5 {
        f.trkExQuality.set_value(ui, pos as f64);
    }
    put_value(ui, f.lblExQuality, &tw("CRF {{crf}}", &[("crf", &c.to_string())]));
    let mut hint = tw("{{band}} A escala vai de 0 (sem perdas) a 51 (pior). O bitrate é variável (VBR): o codificador gasta os bits que o conteúdo pedir, então o tamanho final depende do vídeo.", &[("band", &lang::tr(crf_band_key(c)))]);
    let warn = svt && c >= SVT_FLOOR_CRF;
    if warn {
        hint = format!("{} {}", tw("Com AV1 (CPU), a partir de {{crf}} a qualidade não cai mais: o encoder já está no mínimo dele e o arquivo sai do mesmo tamanho.", &[("crf", &SVT_FLOOR_CRF.to_string())]), hint);
    }
    let old = f.lblExQualityHint.text(ui);
    f.lblExQualityHint.set_text(ui, hint.as_str());
    f.lblExQualityHint.set_text_color(ui, if warn { "amber" } else { "" });
    if old != hint {
        relayout_hint(ui, f.lblExQualityHint);
    }
}

/// O trilho da qualidade (a posição; o CRF corre ao contrário).
pub(crate) fn quality_changed(ui: &mut Ui, v: f64) {
    with(|st| st.q_pos = (v as i32).clamp(0, CRF_MAX));
    render_quality(ui);
}

/// Uma dica mudou de texto: a altura dela (e a da página) acompanha.
fn relayout_hint(ui: &mut Ui, h: g::Label) {
    let col = with(|st| st.col_w);
    let text = h.text(ui);
    let height = if text.is_empty() { 0.0 } else { wrapped_height(ui, h.id, &text, col) };
    let old = ui.control(h.id).map_or(0.0, |cb| cb.height());
    if (old - height).abs() > 0.5 {
        layout_page(ui);
    }
}

fn render_captions(ui: &mut Ui) {
    let (has, loading, transcribed, no_video, burn) = with(|st| {
        let has = st.turns.as_ref().is_some_and(|t| !t.is_empty());
        if !(has && !st.no_video) {
            st.burn = false;
        }
        (has, st.turns.is_none(), st.transcribed, st.no_video, st.burn)
    });
    if let Some(f) = frm(ui) {
        set_chk(ui, f.pnlExBurn, f.chkExBurn, burn, has && !no_video, false);
        f.lblExCaptionHint.set_text(
            ui,
            lang::tr(if loading {
                "Carregando a transcrição…"
            } else if has {
                "A transcrição é desenhada nos quadros do vídeo, igual à legenda da prévia com o CC ligado. Ela vira parte da imagem: não dá para desligar depois em outro player."
            } else if transcribed {
                "Esta gravação foi transcrita, mas nenhuma fala foi detectada no áudio."
            } else {
                "Transcreva a gravação para poder gravar a legenda no vídeo."
            }),
        );
        relayout_hint(ui, f.lblExCaptionHint);
    }
    render_playbar(ui);
}

fn render_container(ui: &mut Ui) {
    let c = with(|st| st.container.clone());
    let Some(f) = frm(ui) else { return };
    if c == "mp3" {
        set_items(ui, f.cmbExCont, &["MP3".to_owned()], 0);
        f.cmbExCont.set_enabled(ui, false);
    } else {
        set_items(ui, f.cmbExCont, &["MP4".to_owned(), "MKV".to_owned()], if c == "mkv" { 1 } else { 0 });
        f.cmbExCont.set_enabled(ui, true);
    }
    f.lblExContHint.set_text(
        ui,
        lang::tr(match c.as_str() {
            "mkv" => "MKV é o formato nativo do app e aguenta HEVC/AV1 sem problema, mas nem todo player ou site aceita. Salvo na pasta de gravações.",
            "mp3" => "Só áudio sai em MP3, que abre em qualquer aparelho. MP3 guarda uma faixa só: com o áudio original vai a mistura de todas, e faixas escolhidas uma a uma são juntadas. A qualidade é a da aba Áudio das Configurações.",
            _ => "MP4 abre em qualquer player, site ou celular. Salvo na pasta de gravações.",
        }),
    );
    relayout_hint(ui, f.lblExContHint);
}

/// O formato escolhido (MP4 ou MKV; o MP3 do só áudio não se escolhe).
pub(crate) fn cont_changed(ui: &mut Ui, i: usize) {
    with(|st| {
        if st.container != "mp3" {
            st.container = if i == 1 { "mkv".into() } else { "mp4".into() };
            st.video_container = st.container.clone();
        }
    });
    render_container(ui);
}

/// Quantos segundos a prévia gerada tem.
pub(crate) fn prevsec_changed(i: usize) {
    with(|st| st.prevsec = i.min(2));
}

fn render_actions(ui: &mut Ui) {
    let (running, canceling, previewing, ppct, pct, loaded) = with(|st| (st.running, st.canceling, st.previewing, st.preview_pct, st.pct, st.loaded));
    let Some(f) = frm(ui) else { return };
    put_btn(ui, f.btnExRun, &lang::tr(if running { "Cancelar" } else { "Exportar" }));
    f.btnExRun.set_style(ui, if running { "ex" } else { "ex_primary" });
    f.btnExRun.set_enabled(ui, loaded && !canceling && (running || !previewing));
    put_btn(ui, f.btnExPreview, &if previewing { tw("Gerando prévia… {{pct}}%", &[("pct", &(ppct.round() as i32).to_string())]) } else { lang::tr("Gerar prévia") });
    f.btnExPreview.set_enabled(ui, loaded && !running);
    f.cmbExPrevSec.set_enabled(ui, !previewing);
    f.lyExProgress.set_visible(ui, running);
    f.prgExProgress.set_value(ui, pct);
    f.lblExProgress.set_text(
        ui,
        if !running {
            String::new()
        } else if canceling {
            lang::tr("Cancelando…")
        } else {
            tw("Exportando… {{pct}}%", &[("pct", &(pct.round() as i32).to_string())])
        },
    );
}

// ---------------------------------------------------------------------------
// Execução
// ---------------------------------------------------------------------------

/// O pedido, do jeito que a tela está agora. `None` = nada a exportar.
fn build_request(name: &str) -> Option<ExportRequest> {
    let r = with(|st| {
        if st.id.is_empty() || !st.loaded {
            return Err(false);
        }
        let segs = kept_segments(st);
        if segs.is_empty() || kept_duration(st) <= 0.0 {
            return Err(false);
        }
        if st.no_video && audio_selection(st).is_empty() {
            return Err(true);
        }
        let orig = use_original(st);
        Ok(ExportRequest {
            id: st.id.clone(),
            preview: None,
            no_video: st.no_video,
            segments: segs.iter().map(|s| ((s.0 * 1000.0).round() as i32, (s.1 * 1000.0).round() as i32)).collect(),
            target_height: st.res_h,
            fps: target_fps(st),
            speed: speed(st),
            mix_audio: !orig && st.mix_checked,
            crf: crf(st),
            scale_algo: if is_resizing(st) { st.scale_algo.clone() } else { "bicubic".into() },
            encoder: if st.enc_id.is_empty() { "auto".into() } else { st.enc_id.clone() },
            regions: if st.no_video { Vec::new() } else { st.sel_regions.iter().map(|&i| i as i32).collect() },
            crop: if !st.no_video && has_crop(st) { st.crop.map(|c| (c.x, c.y, c.w, c.h)) } else { None },
            audio_streams: audio_selection(st),
            mix_track_index: mix_index(st) as i32,
            original_audio: orig,
            burn_captions: !st.no_video && st.burn && st.turns.as_ref().is_some_and(|t| !t.is_empty()),
            keep_transcript: audio_is_complete(st),
            container: st.container.clone(),
            name: name.to_owned(),
        })
    });
    match r {
        Ok(req) => Some(req),
        Err(true) => {
            crate::app_view::show_toast(&lang::tr("Erro"), &lang::tr("Escolha pelo menos uma tela ou uma faixa de áudio para exportar."), true, 5000);
            None
        }
        Err(false) => None,
    }
}

fn file_name(ui: &Ui) -> String {
    frm(ui).map(|f| f.edtExName.text(ui)).unwrap_or_default()
}

fn pause_preview(ui: &mut Ui) {
    with(|st| {
        if let Some(pb) = &st.pb {
            if st.playing {
                pb.pause();
                st.play_sec = pb.shared.position();
            }
        }
        st.playing = false;
    });
    render_playbar(ui);
}

pub(crate) fn run_or_cancel(ui: &mut Ui) {
    if with(|st| st.running) {
        cancel(ui);
        return;
    }
    if with(|st| st.previewing) {
        return;
    }
    let Some(req) = build_request(&file_name(ui)) else { return };
    pause_preview(ui);
    with(|st| {
        st.running = true;
        st.running_id = st.id.clone();
        st.pct = 0.0;
        st.canceling = false;
    });
    render_actions(ui);
    sync_card();
    bexp::handle_export(req);
}

/// Cancelar (botão da tela ou o × do cartão).
pub fn cancel(ui: &mut Ui) {
    let go = with(|st| {
        if !st.running || st.canceling {
            return false;
        }
        st.canceling = true;
        true
    });
    if go {
        bexp::handle_cancel_export();
        render_actions(ui);
        sync_card();
    }
}

pub(crate) fn preview_action(ui: &mut Ui) {
    if with(|st| st.previewing) {
        bexp::handle_cancel_export();
        if let Some(f) = frm(ui) {
            put_btn(ui, f.btnExPreview, &lang::tr("Cancelando…"));
            f.btnExPreview.set_enabled(ui, false);
        }
        return;
    }
    if with(|st| st.running) {
        return;
    }
    let Some(mut req) = build_request(&file_name(ui)) else { return };
    let (from, sec) = with(|st| (st.play_sec, PREVIEW_SECS[st.prevsec.min(2)]));
    req.preview = Some((from, sec));
    pause_preview(ui);
    with(|st| {
        st.previewing = true;
        st.preview_pct = 0.0;
    });
    render_actions(ui);
    bexp::handle_export(req);
}

pub fn on_progress(pct: f64, source: &str) {
    with(|st| {
        // Exportação adotada (a tela pode ter sido reaberta noutro estado).
        st.running = true;
        if !source.is_empty() {
            st.running_id = source.to_owned();
        }
        st.pct = pct.clamp(0.0, 100.0);
    });
    if is_open() {
        with_ui(render_actions);
    }
    sync_card();
}

pub fn on_done(ok: bool, canceled: bool, _path: &str) {
    with(|st| {
        st.running = false;
        st.running_id.clear();
        st.pct = 0.0;
        st.canceling = false;
    });
    sync_card();
    with_ui(move |ui| {
        if ok {
            if is_open() {
                close(ui);
            }
            crate::app_library::clear_selection();
            crate::app_view::show_toast(&lang::tr("Exportação concluída"), &lang::tr("O arquivo já está na lista de gravações."), false, 6000);
        } else if canceled {
            crate::app_view::show_toast(&lang::tr("Exportação cancelada"), &lang::tr("O arquivo parcial foi descartado."), false, 4000);
        }
        if is_open() {
            render_actions(ui);
        }
    });
}

pub fn on_preview_progress(pct: f64) {
    with(|st| st.preview_pct = pct);
    if with(|st| st.previewing) {
        with_ui(render_actions);
    }
}

pub fn on_preview_done(ok: bool, canceled: bool, path: &str) {
    with(|st| st.previewing = false);
    let path = path.to_owned();
    with_ui(move |ui| {
        render_actions(ui);
        if ok && !path.is_empty() {
            crate::app_player::open(ui, &path);
        } else if canceled {
            crate::app_view::show_toast(&lang::tr("Prévia cancelada"), "", false, 3000);
        }
    });
}

/// Exportação desta gravação em curso? (bloqueia excluir/mover/renomear na UI).
pub fn running_id() -> String {
    with(|st| if st.running { st.running_id.clone() } else { String::new() })
}

/// Barra de progresso no cartão da gravação de origem.
fn sync_card() {
    thread_local! {
        static LAST: RefCell<String> = const { RefCell::new(String::new()) };
    }
    let (rec, pct, canceling) = with(|st| (if st.running { st.running_id.clone() } else { String::new() }, st.pct, st.canceling));
    // A barra sai do cartão anterior quando a exportação acaba.
    let last = LAST.with(|l| std::mem::replace(&mut *l.borrow_mut(), rec.clone()));
    if !last.is_empty() && last != rec {
        crate::app_library::set_export_progress(&last, None, "");
    }
    if !rec.is_empty() {
        let label = if canceling { lang::tr("Cancelando…") } else { tw("Exportando… {{pct}}%", &[("pct", &(pct.round() as i32).to_string())]) };
        crate::app_library::set_export_progress(&rec, Some(pct), &label);
    }
    crate::app_library::sync_export_button();
}

// ---------------------------------------------------------------------------
// Teclado
// ---------------------------------------------------------------------------

/// Teclas com a tela aberta (Esc fecha, espaço toca, Ctrl +/−/0 no zoom da linha do tempo).
pub fn key(ui: &mut Ui, k: &KeyEvent, typing: bool) {
    if k.key == Key::Escape {
        close(ui);
        return;
    }
    if k.modifiers.ctrl {
        let (ps, z) = with(|st| (st.play_sec, st.zoom));
        match k.key {
            Key::Char('+') | Key::Char('=') => set_zoom(ui, z * 1.6, Some(ps)),
            Key::Char('-') | Key::Char('_') => set_zoom(ui, z / 1.6, Some(ps)),
            Key::Char('0') => {
                set_zoom(ui, 1.0, Some(0.0));
                with(|st| st.scroll = 0.0);
                render_timeline(ui);
            }
            _ => {}
        }
        return;
    }
    if k.key == Key::Space && !typing {
        toggle_play(ui);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_like_the_web() {
        assert_eq!(fmt_time(0.0), "00:00");
        assert_eq!(fmt_time(199.5), "03:20");
        assert_eq!(fmt_time(3725.0), "1:02:05");
        assert_eq!(parse_time("1:30"), Some(90.0));
        assert_eq!(parse_time("x"), None);
    }

    #[test]
    fn neighbor_parts_merge() {
        let st = State {
            parts: vec![
                Part { start: 0.0, end: 5.0, keep: true },
                Part { start: 5.0, end: 9.0, keep: true },
                Part { start: 9.0, end: 12.0, keep: false },
                Part { start: 12.0, end: 20.0, keep: true },
            ],
            ..Default::default()
        };
        assert_eq!(kept_segments(&st), vec![(0.0, 9.0), (12.0, 20.0)]);
        assert!((kept_duration(&st) - 17.0).abs() < 1e-9);
    }

    #[test]
    fn crf_runs_backwards() {
        let st = State { q_pos: 28, ..Default::default() };
        assert_eq!(crf(&st), 23);
        let st = State { q_pos: 51, ..Default::default() };
        assert_eq!(crf(&st), 0);
        assert_eq!(crf_band_key(23), "Alta qualidade — recomendado.");
    }

    #[test]
    fn speed_takes_comma_and_clamps() {
        let mut st = State { speed_text: "1,5".into(), ..Default::default() };
        assert_eq!(speed(&st), 1.5);
        st.speed_text = "9999".into();
        assert_eq!(speed(&st), 512.0);
        st.speed_text = "0.5".into();
        assert_eq!(speed(&st), 1.0);
    }

    fn app() -> iron::Application {
        let mut app = iron::Application::bundled(crate::PROJECT).unwrap();
        crate::generated::register(&mut app.ui);
        app
    }

    /// Uma gravação de 10 min com duas telas, três faixas, cortes e a forma de onda.
    fn sample_state() {
        with(|st| {
            *st = fresh("C:/x/v.mkv", Vec::new(), &State::default());
            st.loaded = true;
            st.duration = 600.0;
            st.src_w = 3840;
            st.src_h = 1080;
            st.src_fps = 60.0;
            st.layout = Some(VideoLayout {
                canvas_w: 3840,
                canvas_h: 1080,
                regions: vec![
                    crate::view::LayoutRegion { name: "Monitor 1".into(), x: 0, y: 0, w: 1920, h: 1080, ..Default::default() },
                    crate::view::LayoutRegion { name: "Monitor 2".into(), x: 1920, y: 0, w: 1920, h: 1080, ..Default::default() },
                ],
            });
            st.audio = (0..3).map(|i| StreamInfo { index: i + 1, kind: "audio".into(), ..Default::default() }).collect();
            st.sel_audio.insert(1);
            st.parts = (0..6).map(|i| Part { start: i as f64 * 100.0, end: (i + 1) as f64 * 100.0, keep: i % 2 == 0 }).collect();
            st.peaks = Some((0..15_000).map(|i| ((i as f32) * 0.37).sin().abs()).collect());
            st.wave_max = 1.0;
            st.fps_val = 60;
        });
    }

    /// O `frmExport` sozinho, no tamanho da janela de 1600 × 900.
    fn screen(ui: &mut Ui) -> Frm {
        let form = ui.create_form("frmExport").unwrap();
        if let Some(cb) = ui.control_mut(form) {
            cb.set_rect(0.0, 0.0, 1600.0, 900.0);
        }
        ui.layout();
        frm(ui).unwrap()
    }

    fn pills_of(ui: &Ui, flow: ComponentId) -> Vec<g::FraExPillComponents> {
        ui.children(flow).iter().filter_map(|&c| FraExPill::components_of(ui, c)).collect()
    }

    fn visible_ticks(ui: &Ui, f: &Frm) -> Vec<String> {
        ui.children(f.lyExTicks.id).iter().filter_map(|&c| FraExTick::components_of(ui, c)).filter(|t| t.form.visible(ui)).map(|t| t.lblTick.text(ui)).collect()
    }

    fn visible_parts(ui: &Ui, f: &Frm) -> Vec<String> {
        ui.children(f.lyExTlLabels.id).iter().filter_map(|&c| FraExPartLabel::components_of(ui, c)).filter(|p| p.form.visible(ui)).map(|p| p.lblPart.text(ui)).collect()
    }

    /// A coluna tem no máximo 880 px e fica no meio: ao alargar a janela além disso (a tela
    /// cheia), as margens acompanham e o conteúdo não estica.
    #[test]
    fn the_column_keeps_its_maximum_width_centered() {
        let mut app = app();
        let ui = &mut app.ui;
        let f = screen(ui);
        sample_state();
        build(ui);
        render_all(ui);
        ui.layout();
        let column = |ui: &mut Ui, width: f32| {
            ui.control_mut(f.form.id).unwrap().set_rect(0.0, 0.0, width, 900.0);
            ui.layout();
            resized(ui);
            ui.run_timers();
            ui.layout();
            let page = ui.absolute_rect(f.lyExport.id).unwrap();
            let pad = ui.control(f.lyExport.id).unwrap().padding();
            (page.width - pad.left - pad.right, pad.left, pad.right)
        };
        for width in [1600.0, 2400.0, 3200.0] {
            let (w, left, right) = column(ui, width);
            assert!((w - (COL_MAX - 36.0)).abs() < 1.0, "a {width}: coluna de {w}");
            assert!((left - right).abs() < 20.0, "a {width}: no meio ({left} × {right})");
        }
        with(|st| st.id.clear());
    }

    /// A tela é a do `forms/export.json`; o código só cria as pílulas (`fraExPill`), as
    /// marcas da régua (`fraExTick`) e as durações dos trechos (`fraExPartLabel`), que se
    /// reaproveitam ao redesenhar. Os handlers do form chamam a tela.
    #[test]
    fn the_screen_comes_from_the_json_and_the_parts_are_frames() {
        let mut app = app();
        let ui = &mut app.ui;
        let f = screen(ui);
        sample_state();
        with(|st| st.play_sec = 150.0);
        build(ui);
        render_all(ui);
        ui.layout();
        // Telas: "Todas", as duas regiões e "Nenhuma", lado a lado, na largura do texto.
        let regions = pills_of(ui, f.flwExRegions.id);
        let texts: Vec<String> = regions.iter().map(|p| p.lblPill.text(ui)).collect();
        assert_eq!(texts, [lang::tr("Todas"), "Monitor 1".into(), "Monitor 2".into(), lang::tr("Nenhuma (só áudio)")]);
        assert!(regions[0].chkPill.checked(ui) && !regions[1].chkPill.checked(ui));
        assert_eq!((regions[0].pnlPill.style(ui), regions[1].pnlPill.style(ui)), ("ex_chk_on".to_owned(), "ex_chk".to_owned()));
        assert_eq!(ui.control(regions[0].form.id).unwrap().width(), chk_width(ui, &lang::tr("Todas")));
        let (r0, r1) = (ui.absolute_rect(regions[0].pnlPill.id).unwrap(), ui.absolute_rect(regions[1].pnlPill.id).unwrap());
        assert!((r1.x - (r0.x + r0.width + 6.0)).abs() < 0.01 && (r1.y - r0.y).abs() < 0.01 && (r0.height - 30.1).abs() < 0.01);
        // Clicar numa tela marca só ela, nas mesmas pílulas (não recria).
        let ids: Vec<ComponentId> = regions.iter().map(|p| p.form.id).collect();
        ui.fire(regions[1].pnlPill.id, "on_click", &EventArgs::None);
        assert_eq!(with(|st| st.sel_regions.iter().copied().collect::<Vec<_>>()), [0]);
        let again = pills_of(ui, f.flwExRegions.id);
        assert_eq!(again.iter().map(|p| p.form.id).collect::<Vec<_>>(), ids);
        assert_eq!((again[0].pnlPill.style(ui), again[1].pnlPill.style(ui)), ("ex_chk".to_owned(), "ex_chk_on".to_owned()));
        // Áudio: com o original, as três faixas marcadas e desligadas; a pílula fixa do JSON
        // ("Manter o áudio original") libera as faixas.
        let tracks = pills_of(ui, f.flwExTracks.id);
        assert_eq!(tracks.len(), 3);
        assert!(f.lyExAudio.visible(ui) && f.lyExOrigRow.visible(ui) && f.chkExOrig.checked(ui));
        assert!(tracks.iter().all(|p| p.chkPill.checked(ui) && !p.pnlPill.enabled(ui)));
        ui.fire(f.pnlExOrig.id, "on_click", &EventArgs::None);
        assert!(!with(|st| st.original_audio) && !f.chkExOrig.checked(ui));
        let tracks = pills_of(ui, f.flwExTracks.id);
        // Só a mixagem (a 1ª) marcada: ela fica livre e as isoladas, desligadas.
        assert!(tracks[0].chkPill.checked(ui) && tracks[0].pnlPill.enabled(ui));
        assert!(!tracks[1].chkPill.checked(ui) && !tracks[1].pnlPill.enabled(ui));
        // A régua: 600 s em 842 px, uma marca a cada 2 min; com a forma de onda, sem as durações.
        assert_eq!(with(|st| st.view_w), 842.0);
        assert_eq!(visible_ticks(ui, &f), ["00:00", "02:00", "04:00", "06:00", "08:00"]);
        assert!(visible_parts(ui, &f).is_empty());
        // Com zoom, mais marcas; de volta, as que sobram só se escondem.
        set_zoom(ui, 8.0, Some(0.0));
        assert_eq!(visible_ticks(ui, &f).first().map(String::as_str), Some("00:00"));
        let made = ui.children(f.lyExTicks.id).len();
        set_zoom(ui, 1.0, Some(0.0));
        assert_eq!(ui.children(f.lyExTicks.id).len(), made);
        assert_eq!(visible_ticks(ui, &f).len(), 5);
        // Sem a forma de onda, a duração de cada trecho no meio dele.
        with(|st| st.peaks = None);
        render_timeline(ui);
        assert_eq!(visible_parts(ui, &f), ["01:40"; 6]);
        // O cursor: os dois retângulos do JSON no tempo (150 s = 210,5 px).
        let left = |ui: &Ui, r: g::Rectangle| ui.control(r.id).map(|cb| (cb.left(), cb.visible()));
        assert_eq!(left(ui, f.shpExPlayShade), Some((1.0 + 210.5 - 2.0, true)));
        assert_eq!(left(ui, f.shpExPlayLine), Some((1.0 + 210.5 - 1.0, true)));
        // Os botões na largura do texto; as dicas na altura do texto quebrado.
        assert_eq!(ui.control(f.btnExCut.id).unwrap().width(), button_width(ui, &f.btnExCut.text(ui), true));
        for h in [f.lblExRegionsHint, f.lblExRangeHint, f.lblExQualityHint, f.lblExContHint] {
            assert!(h.visible(ui) && ui.control(h.id).unwrap().height() > 16.0, "{}", h.text(ui));
        }
        // Os handlers do form: "Dividir aqui" e o campo do tempo.
        ui.fire(f.btnExCut.id, "on_click", &EventArgs::None);
        assert_eq!(with(|st| st.parts.len()), 7);
        f.edtExTime.set_text(ui, "5:00");
        ui.fire(f.edtExTime.id, "on_exit", &EventArgs::None);
        assert_eq!(with(|st| st.play_sec), 300.0);
        // A página tem a altura do conteúdo mais o respiro do fim.
        let h = ui.control(f.lyExport.id).unwrap().height();
        assert!(h > 900.0, "{h}");
        with(|st| *st = State { crop_zoom: 1.0, zoom: 1.0, scale: 1.0, ..Default::default() });
    }

    /// As divisões vão na imagem da linha do tempo: a linha de 3 px no meio da faixa
    /// escura, vermelha na mais próxima do cursor e azul nas outras.
    #[test]
    fn the_cuts_are_drawn_in_the_timeline_image() {
        let st = State {
            duration: 100.0,
            view_w: 200.0,
            zoom: 1.0,
            cursor_part: -1,
            play_sec: 10.0,
            parts: vec![Part { start: 0.0, end: 50.5, keep: true }, Part { start: 50.5, end: 80.0, keep: true }, Part { start: 80.0, end: 100.0, keep: true }],
            ..Default::default()
        };
        let img = timeline_image(&st, true, 1.0).unwrap();
        let ImageSource::Rgba { width, pixels, .. } = img.source() else { panic!("a imagem é RGBA") };
        let px = |x: u32, y: u32| {
            let i = ((y * width + x) * 4) as usize;
            (pixels[i], pixels[i + 1], pixels[i + 2])
        };
        // 50,5 s = 101 px: a linha cobre de 99,5 a 102,5; 80 s = 160 px.
        assert_eq!((px(100, 20), px(101, 20)), ((0xf8, 0x71, 0x71), (0xf8, 0x71, 0x71)));
        assert_eq!((px(159, 0), px(160, 45)), ((0x60, 0xa5, 0xfa), (0x60, 0xa5, 0xfa)));
        // A faixa escura em volta: mais escura que o fundo do trecho, que fica igual longe dela.
        let (bg, shade) = (px(130, 20), px(98, 20));
        assert!(shade.1 < bg.1 && px(90, 20) == bg, "{shade:?} {bg:?}");
    }

    /// Quanto custa montar a tela e redesenhar a linha do tempo (rode em release, com
    /// `--ignored --nocapture`): criar o form (uma vez, ao iniciar o app), a montagem ao abrir
    /// e o `render_timeline` rolando e com zoom.
    #[test]
    #[ignore]
    fn bench_build_and_timeline() {
        use std::time::Instant;
        let mut app = app();
        let ui = &mut app.ui;
        let t = Instant::now();
        screen(ui);
        println!("criar o frmExport: {:.2} ms", t.elapsed().as_secs_f64() * 1000.0);
        let mut builds = Vec::new();
        for _ in 0..6 {
            sample_state();
            let t = Instant::now();
            build(ui);
            render_all(ui);
            builds.push(t.elapsed().as_secs_f64() * 1000.0);
        }
        // Com zoom (8×), rolando: a imagem e as marcas a cada passo.
        with(|st| st.zoom = 8.0);
        let mut tl = Vec::new();
        for i in 0..60 {
            with(|st| st.scroll = i as f64 * 50.0);
            let t = Instant::now();
            render_timeline(ui);
            tl.push(t.elapsed().as_secs_f64() * 1000.0);
        }
        tl.sort_by(|a, b| a.partial_cmp(b).unwrap());
        println!("montagem: primeira {:.2} ms; refazer {:?} ms", builds[0], builds[1..].iter().map(|t| format!("{t:.2}")).collect::<Vec<_>>());
        println!("render_timeline (60x, zoom 8x): mediana {:.3} ms; min {:.3}; max {:.3}", tl[30], tl[0], tl[59]);
        with(|st| *st = State { crop_zoom: 1.0, zoom: 1.0, scale: 1.0, ..Default::default() });
    }
}
