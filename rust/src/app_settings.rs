//! Presenter das Configurações: o form `frmSettings` (`forms/settings.json`:
//! cabeçalho, barra lateral com as abas e a área rolável) e um frame por aba
//! (`forms/settings/*.json`: `fraGeneral`, `fraVideo`...), com tudo o que a aba tem
//! declarado no JSON dela; aqui só se preenchem os valores e se mostra o que vale. As
//! peças repetidas (dispositivo, atalho, item da fila, nível do teste...) são frames de
//! `forms/settings/parts/`, criados por item com `create_in` no contêiner da lista.
//! Os handlers ligados no JSON ficam em `src/forms/<aba>.rs`, que lê os valores dos
//! componentes e chama os módulos daqui (`general`, `video`...).
//!
//! Lê o estado direto do core (config, caps, filas) e cada mudança chama o
//! handler do core na hora — sem botão Salvar. Os dados que o core empurra
//! (fila de transcrição, motor local, atalhos, teste de qualidade…) chegam
//! pelo `View` e são guardados aqui pra redesenhar.

use crate::generated as g;
use crate::generated::{FraAbout, FraAboutItem, FraAudio, FraDeviceGroup, FraQueueRow, FraDeviceRow, FraDevices, FraFiles, FraGeneral, FraHiddenFolder, FraHotkeyCard, FraHotkeys, FraOnRecord, FraReplay, FraSliderTick, FraVtLevel, FraTranscribe, FraUpdates, FraVideo, FrmSettings};
use std::cell::RefCell;
use std::collections::HashMap;

use aylis::prelude::*;

use crate::app_view::with_ui;
use crate::bridge::{self, hotkeys as bhk, recording, replay, settings, shell, transcribe as btr, video_test};
use crate::hotkeys::{Item, Mode, ShotTarget};
use crate::types::GpuVendor;
use crate::view::{Device, LocalAsrInfo, QueueItem, ReplayState, RootFolder, TranscribeState, UpdateInfo, VideoTestState};
use crate::{config, encoder, hotkey, lang, log};

const AUDIO_BITRATES: [i32; 6] = [96, 128, 160, 192, 256, 320];
const AUDIO_BITRATE_DEFAULT: i32 = 192;
const REPLAY_SEC_OPTIONS: [i64; 8] = [30, 60, 120, 300, 600, 900, 1800, 3600];
const REPLAY_MB_OPTIONS: [i64; 16] = [512, 1024, 2048, 3072, 4096, 5120, 6144, 7168, 8192, 10240, 12288, 14336, 16384, 20480, 24576, 28672];
const REPLAY_DEFAULT_SEC: i64 = 300;
const REPLAY_DEFAULT_MB: i64 = 2048;
const CORNERS: [&str; 4] = ["top-left", "top-right", "bottom-left", "bottom-right"];
/// Na ordem do combo: os de hardware juntos e depois os de software (cada grupo com a sua
/// cabeça na lista, por `codec_groups`).
const CODECS: [&str; 6] = ["auto", "h264-hw", "hevc-hw", "av1-hw", "h264-sw", "av1-sw"];
const LIBRARY_MODES: [&str; 3] = ["auto", "always", "off"];
const TABS: [&str; 11] = ["general", "onrecord", "files", "recording", "audio", "replay", "hotkeys", "devices", "transcribe", "updates", "about"];
/// O form da aba (o `fraX` que o frame da página mostra); uma aba desconhecida é a Geral.
fn tab_form(ui: &Ui, tab: &str) -> Option<ComponentId> {
    Some(match tab {
        "onrecord" => FraOnRecord::components(ui)?.form.id,
        "files" => FraFiles::components(ui)?.form.id,
        "recording" => FraVideo::components(ui)?.form.id,
        "audio" => FraAudio::components(ui)?.form.id,
        "replay" => FraReplay::components(ui)?.form.id,
        "hotkeys" => FraHotkeys::components(ui)?.form.id,
        "devices" => FraDevices::components(ui)?.form.id,
        "transcribe" => FraTranscribe::components(ui)?.form.id,
        "updates" => FraUpdates::components(ui)?.form.id,
        "about" => FraAbout::components(ui)?.form.id,
        _ => FraGeneral::components(ui)?.form.id,
    })
}
/// Idiomas oferecidos pra transcrição (o nome no próprio idioma).
const TRANSCRIBE_LANGS: [(&str, &str); 16] = [
    ("pt", "Português"),
    ("en", "English"),
    ("es", "Español"),
    ("fr", "Français"),
    ("de", "Deutsch"),
    ("it", "Italiano"),
    ("nl", "Nederlands"),
    ("pl", "Polski"),
    ("ru", "Русский"),
    ("uk", "Українська"),
    ("tr", "Türkçe"),
    ("ar", "العربية"),
    ("hi", "हिन्दी"),
    ("ja", "日本語"),
    ("ko", "한국어"),
    ("zh", "中文"),
];
const HK_MODES: [Mode; 5] = [Mode::Record, Mode::Window, Mode::Area, Mode::Screenshot, Mode::Replay];
const HK_DELAYS: [u32; 6] = [0, 3, 5, 10, 15, 30];
const VT_SVT_FLOOR_LEVEL: i32 = 3;

/// Teclas do seletor de atalho: (valor, rótulo, é título de grupo).
fn key_table() -> Vec<(String, String, bool)> {
    let mut v: Vec<(String, String, bool)> = Vec::new();
    let group = |v: &mut Vec<(String, String, bool)>, key: &str| v.push((String::new(), lang::tr(key), true));
    let plain = |v: &mut Vec<(String, String, bool)>, k: &str| v.push((k.to_owned(), k.to_owned(), false));
    let tr = |v: &mut Vec<(String, String, bool)>, k: &str, key: &str| v.push((k.to_owned(), lang::tr(key), false));
    group(&mut v, "Função");
    for i in 1..=12 {
        plain(&mut v, &format!("F{i}"));
    }
    group(&mut v, "Letras");
    for c in 'A'..='Z' {
        plain(&mut v, &c.to_string());
    }
    group(&mut v, "Números");
    for c in '0'..='9' {
        plain(&mut v, &c.to_string());
    }
    group(&mut v, "Especiais");
    tr(&mut v, "Space", "Espaço");
    for k in ["Enter", "Tab", "Esc", "Insert", "Delete", "Home", "End"] {
        plain(&mut v, k);
    }
    tr(&mut v, "PageUp", "Página acima");
    tr(&mut v, "PageDown", "Página abaixo");
    plain(&mut v, "Backspace");
    plain(&mut v, "Pause/Break");
    tr(&mut v, "PrintScreen", "Print Screen");
    group(&mut v, "Setas");
    tr(&mut v, "Left", "← Esquerda");
    tr(&mut v, "Right", "→ Direita");
    tr(&mut v, "Up", "↑ Cima");
    tr(&mut v, "Down", "↓ Baixo");
    group(&mut v, "Numpad");
    for i in 0..10 {
        v.push((format!("Numpad{i}"), format!("Num {i}"), false));
    }
    for (k, l) in [("NumpadAdd", "Num +"), ("NumpadSubtract", "Num -"), ("NumpadMultiply", "Num *"), ("NumpadDivide", "Num /"), ("NumpadDecimal", "Num .")] {
        v.push((k.to_owned(), l.to_owned(), false));
    }
    group(&mut v, "Caracteres");
    for k in ["-", "=", ",", ".", ";", "/", "`", "[", "\\", "]", "'"] {
        plain(&mut v, k);
    }
    group(&mut v, "Mídia");
    tr(&mut v, "MediaPlayPause", "⏯ Reproduzir/Pausar");
    tr(&mut v, "MediaStop", "⏹ Parar");
    tr(&mut v, "MediaPrev", "⏮ Faixa anterior");
    tr(&mut v, "MediaNext", "⏭ Próxima faixa");
    v
}

#[derive(Default)]
struct State {
    open: bool,
    tab: String,
    fps_presets: Vec<i32>,
    quality: i32,
    fps: i32,
    keyframe: i32,
    monitors: Vec<Device>,
    webcams: Vec<Device>,
    mics: Vec<Device>,
    speakers: Vec<Device>,
    replay: Option<ReplayState>,
    hk_items: Vec<Item>,
    hk_status: Vec<&'static str>,
    hk_delay_max: u32,
    hk_defining: Option<usize>,
    /// Modificadores marcados sem tecla ficam só na tela até a tecla chegar.
    hk_pending: HashMap<usize, (bool, bool, bool, bool)>,
    tr: TranscribeState,
    tr_pending: usize,
    tr_cloud: usize,
    tr_queue: Vec<QueueItem>,
    tr_cancel_clicked: bool,
    tr_eta_shown: i64,
    tr_eta_for: String,
    tr_last_toast_error: String,
    tr_job: Option<u64>,
    asr: Option<LocalAsrInfo>,
    asr_confirm_remove: bool,
    folders: Vec<RootFolder>,
    update: Option<UpdateInfo>,
    checking: bool,
    vt: VtState,
    /// Envio adiado de um controle que muda enquanto arrasta (slider, texto).
    debounce: HashMap<&'static str, u64>,
}

#[derive(Default)]
struct VtState {
    running: bool,
    capturing: bool,
    countdown: i32,
    pct: i64,
    level: i32,
    result: Option<VideoTestState>,
    a: i32,
    b: i32,
    job: Option<u64>,
    zoom: i32,
    split: f32,
    pan: (f32, f32),
    drag: Option<(f32, f32, (f32, f32), bool)>,
    /// A comparação montada (`None` sem ela).
    geom: Option<CompareGeom>,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State { tab: "general".into(), quality: 5, fps: 30, keyframe: 2, vt: VtState { split: 0.5, ..Default::default() }, ..Default::default() });
}

fn with<R>(f: impl FnOnce(&mut State) -> R) -> R {
    STATE.with(|s| f(&mut s.borrow_mut()))
}

fn tw(key: &str, args: &[(&str, &str)]) -> String {
    lang::tr_with(key, args)
}

fn fmt_dur(sec: i64) -> String {
    let sec = sec.max(0);
    let (m, s) = (sec / 60, sec % 60);
    if m >= 60 { format!("{}:{:02}:{:02}", m / 60, m % 60, s) } else { format!("{m}:{s:02}") }
}

/// Número com a vírgula decimal do idioma da interface.
fn fmt_num(v: f64, decimals: usize) -> String {
    let s = format!("{v:.decimals$}");
    let s = if decimals > 0 { s.trim_end_matches('0').trim_end_matches('.').to_owned() } else { s };
    if lang::current_language().starts_with("en") { s } else { s.replace('.', ",") }
}

// ---------------------------------------------------------------------------
// Construtor de linhas
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Abrir / fechar / abas
// ---------------------------------------------------------------------------

/// Abre as Configurações (na aba em que o usuário parou, ou em `tab`).
pub fn open(ui: &mut Ui, tab: Option<&str>) {
    if let Some(t) = tab {
        with(|s| s.tab = t.to_owned());
    }
    with(|s| s.open = true);
    load_values();
    crate::app_view::show_form(ui, FrmSettings::components(ui).map(|frm| frm.form), true);
    if let Some(frm) = FrmSettings::components(ui) {
        frm.lblSetTitle.set_text(ui, lang::tr("Configurações"));
    }
    let tab = with(|s| s.tab.clone());
    show_tab(ui, &tab);
}

pub fn is_open() -> bool {
    with(|s| s.open)
}

pub fn close(ui: &mut Ui) {
    with(|s| s.open = false);
    crate::app_view::show_form(ui, FrmSettings::components(ui).map(|frm| frm.form), false);
}

/// Os valores do core que as abas mostram (lidos ao abrir).
fn load_values() {
    let q = encoder::recording_quality_level().clamp(0, 10);
    let fps = config::get_config_int("recordingFps", 30).max(10);
    let kf = config::get_config_int("recordingKeyframeSec", 2).clamp(1, 10);
    let presets = fps_presets(crate::win_preview::max_monitor_refresh_rate());
    with(|s| {
        s.quality = q;
        s.fps = fps;
        s.keyframe = kf;
        s.fps_presets = presets;
    });
}

/// Pedidos ao core quando uma aba aparece.
fn enter_tab(tab: &str) {
    match tab {
        "files" => btr::push_root_folders(),
        "replay" => replay::push_replay_state(),
        "hotkeys" => bhk::push_state(),
        "transcribe" => {
            btr::handle_get_transcribe_state();
            btr::push_local_asr_state(false);
        }
        _ => {}
    }
}

/// Refaz a aba se for a aberta (dados novos do core).
fn refresh(tab: &'static str) {
    if is_open() && with(|s| s.tab == tab) {
        with_ui(render_tab);
    }
}

// ---------------------------------------------------------------------------
// Geral
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Gravação
// ---------------------------------------------------------------------------

fn corner_labels() -> Vec<String> {
    ["topLeft", "topRight", "bottomLeft", "bottomRight"].iter().map(|c| lang::tr_of("settings.recIndicator.corners", &c.to_string())).collect()
}

// ---------------------------------------------------------------------------
// Arquivos
// ---------------------------------------------------------------------------

/// Prévia do nome do arquivo: data/hora de agora e o codec/fps/qualidade.
fn filename_preview(pattern: &str) -> String {
    let codec = config::get_config_str("codec", "auto");
    let fam = if codec.starts_with("av1") {
        "AV1"
    } else if codec.starts_with("hevc") {
        "HEVC"
    } else {
        "H.264"
    };
    let (q, fps, presets) = with(|s| (s.quality, s.fps, s.fps_presets.clone()));
    let fps = presets.get(fps_index(&presets, fps)).copied().unwrap_or(fps);
    let now = chrono::Local::now();
    let rep = |s: &str, code: &str, val: &str| -> String {
        let low = s.to_lowercase();
        let code_l = code.to_lowercase();
        let mut out = String::new();
        let mut from = 0;
        while let Some(p) = low[from..].find(&code_l).map(|p| p + from) {
            out.push_str(&s[from..p]);
            out.push_str(val);
            from = p + code.len();
        }
        out.push_str(&s[from..]);
        out
    };
    use chrono::{Datelike, Timelike};
    let mut s = pattern.to_owned();
    s = rep(&s, "{CODEC}", fam);
    s = rep(&s, "{FPS}", &fps.to_string());
    s = rep(&s, "{QUALIDADE}", &q.to_string());
    s = rep(&s, "{AAAA}", &now.year().to_string());
    s = rep(&s, "{MM}", &format!("{:02}", now.month()));
    s = rep(&s, "{DD}", &format!("{:02}", now.day()));
    s = rep(&s, "{HH}", &format!("{:02}", now.hour()));
    s = rep(&s, "{NN}", &format!("{:02}", now.minute()));
    s = rep(&s, "{SS}", &format!("{:02}", now.second()));
    s = rep(&s, "{ZZZ}", &format!("{:03}", now.timestamp_subsec_millis()));
    let mut s: String = s.chars().filter(|c| *c != '{' && *c != '}').map(|c| if "\\/:*?\"<>|".contains(c) { '_' } else { c }).collect();
    s = s.trim().to_owned();
    if s.is_empty() {
        s = now.format("NoOBS_%Y-%m-%d_%H-%M-%S").to_string();
    }
    tw("Exemplo: {{name}}", &[("name", &format!("{s}.mkv"))])
}

fn library_hint(mode: &str) -> String {
    let mut txt = lang::tr_of("settings.library.hint", &mode.to_string());
    if mode != "off" && settings::is_path_under_onedrive(&recording::record_dir().to_string_lossy()) {
        txt.push(' ');
        txt.push_str(&lang::tr("Sua pasta de gravação parece estar no OneDrive."));
    }
    txt
}

// ---------------------------------------------------------------------------
// Vídeo
// ---------------------------------------------------------------------------

/// 20, 30 e múltiplos de 30 até o monitor mais rápido; se ele não for
/// múltiplo de 30, o último múltiplo dá lugar a ele.
fn fps_presets(max_hz: i32) -> Vec<i32> {
    let max = max_hz.max(20);
    let mut list = vec![20, 30];
    let mut v = 60;
    while v <= max {
        list.push(v);
        v += 30;
    }
    let mut p: Vec<i32> = list.into_iter().filter(|v| *v <= max).collect();
    p.dedup();
    if p.is_empty() {
        return vec![max];
    }
    if *p.last().unwrap_or(&0) != max {
        if p.len() >= 3 {
            p.pop();
        }
        p.push(max);
    }
    p
}

fn fps_index(presets: &[i32], fps: i32) -> usize {
    presets.iter().enumerate().min_by_key(|(_, p)| (*p - fps).abs()).map(|(i, _)| i).unwrap_or(0)
}

fn codec_options() -> Vec<(String, bool)> {
    let caps = bridge::last_encoder_caps();
    let tag = match caps.map(|c| c.vendor) {
        Some(GpuVendor::Nvidia) => lang::tr(" (NVIDIA NVENC)"),
        Some(GpuVendor::Amd) => lang::tr(" (AMD AMF)"),
        Some(GpuVendor::Intel) => lang::tr(" (Intel QSV)"),
        _ => String::new(),
    };
    let unavailable = format!(" {}", lang::tr("(indisponível)"));
    CODECS
        .iter()
        .map(|c| {
            let avail = caps.map_or(true, |k| match *c {
                "av1-hw" => k.av1_hw,
                "hevc-hw" => k.hevc_hw,
                "h264-hw" => k.h264_hw,
                "h264-sw" => k.h264_sw,
                "av1-sw" => k.av1_sw,
                _ => true,
            });
            let text = match *c {
                "auto" => lang::tr("Automático (recomendado)"),
                "av1-hw" | "hevc-hw" | "h264-hw" if caps.is_some() => {
                    let base = match *c {
                        "av1-hw" => "AV1",
                        "hevc-hw" => "HEVC / H.265",
                        _ => "H.264",
                    };
                    format!("{base}{}{}", lang::tr(" — hardware"), if avail { tag.clone() } else { unavailable.clone() })
                }
                "av1-hw" => lang::tr("AV1 — hardware"),
                "hevc-hw" => lang::tr("HEVC / H.265 — hardware"),
                "h264-hw" => lang::tr("H.264 — hardware"),
                "h264-sw" => format!("{}{}", lang::tr("H.264 — software (x264, CPU)"), if avail { "" } else { &unavailable }),
                _ => format!("{}{}", lang::tr("AV1 — software (SVT-AV1, CPU)"), if avail { "" } else { &unavailable }),
            };
            (text, avail)
        })
        .collect()
}

/// O grupo de cada codec no combo (`groups`, na ordem de `CODECS`): o automático fica sem.
fn codec_groups() -> String {
    let (hw, sw) = (lang::tr("Hardware"), lang::tr("Software"));
    CODECS
        .iter()
        .map(|c| match *c {
            "auto" => "",
            c if c.ends_with("-hw") => hw.as_str(),
            _ => sw.as_str(),
        })
        .collect::<Vec<_>>()
        .join(";")
}

/// As posições dos codecs que esta máquina não tem (`disabled_items`).
fn codec_disabled(options: &[(String, bool)]) -> String {
    options.iter().enumerate().filter(|(_, (_, ok))| !ok).map(|(i, _)| i.to_string()).collect::<Vec<_>>().join(";")
}

fn fps_hint(v: i32) -> String {
    let bucket = match v {
        v if v <= 24 => "low",
        v if v <= 30 => "smooth",
        v if v <= 60 => "good",
        v if v <= 120 => "high",
        _ => "veryHigh",
    };
    lang::tr_of_with("settings.fps.hint", &bucket.to_string(), &[("fps", &v.to_string())])
}

fn keyframe_hint(kf: i32) -> String {
    let kb = match kf {
        k if k <= 1 => "precise",
        k if k <= 2 => "balanced",
        k if k <= 5 => "medium",
        _ => "spaced",
    };
    lang::tr_of_with("settings.keyframe.hint", &kb.to_string(), &[("sec", &kf.to_string())])
}

// ----- teste de qualidade -----

fn per_hour(bytes: i64, dur: f64) -> f64 {
    if bytes <= 0 || dur <= 0.0 { 0.0 } else { bytes as f64 / dur * 3600.0 }
}

fn fmt_size(b: f64) -> String {
    if b >= 1e9 {
        format!("{} GB", fmt_num(b / 1e9, 1))
    } else if b >= 1e6 {
        format!("{} MB", fmt_num(b / 1e6, 0))
    } else {
        format!("{} KB", ((b / 1e3).round() as i64).max(1))
    }
}

fn vt_use(ui: &mut Ui) {
    let a = with(|s| s.vt.a);
    if a < 0 {
        return;
    }
    with(|s| {
        s.quality = a;
        if let Some(r) = s.vt.result.as_mut() {
            r.current_level = a;
        }
    });
    settings::handle_set_recording_quality(a);
    render_tab(ui);
    crate::app_view::show_toast(&tw("Qualidade da gravação: nível {{n}}", &[("n", &a.to_string())]), "", false, 0);
}

pub fn on_video_test(d: &VideoTestState) {
    let d = d.clone();
    with_ui(move |ui| {
        match d.state.as_str() {
            "recording" => {
                with(|s| {
                    s.vt.running = true;
                    s.vt.capturing = true;
                    s.vt.countdown = if d.seconds > 0 { d.seconds as i32 } else { 5 };
                });
                vt_tick(ui);
            }
            "encoding" => with(|s| {
                s.vt.running = true;
                s.vt.capturing = false;
                s.vt.pct = d.pct;
                s.vt.level = d.level;
            }),
            "done" => with(|s| {
                s.vt.running = false;
                s.vt.capturing = false;
                let cur = d.current_level;
                s.vt.a = if cur == 10 { 9 } else { cur };
                s.vt.b = 10;
                s.vt.result = Some(d.clone());
            }),
            st => {
                with(|s| {
                    s.vt.running = false;
                    s.vt.capturing = false;
                });
                if st == "error" {
                    let msg = if d.error.is_empty() { lang::tr("Não foi possível fazer o teste de qualidade. Veja o log para detalhes.") } else { d.error.clone() };
                    crate::app_view::show_toast(&lang::tr("Erro"), &msg, true, 6000);
                }
            }
        }
        if is_open() && with(|s| s.tab == "recording") {
            // Gerando: só o texto e a barra mudam; vindo da captura (sem a barra, texto em
            // vermelho), a aba é refeita.
            let frm = FraVideo::components(ui).filter(|frm| frm.prgVt.visible(ui));
            if let (Some(frm), true) = (frm, d.state == "encoding") {
                let status = tw("Gerando o nível {{n}}… {{pct}}%", &[("n", &d.level.to_string()), ("pct", &d.pct.to_string())]);
                frm.lblVtStatus.set_text(ui, status);
                frm.prgVt.set_value(ui, d.pct.clamp(0, 100) as f64);
            } else {
                render_tab(ui);
            }
        }
    });
}

fn vt_tick(ui: &mut Ui) {
    if let Some(job) = with(|s| s.vt.job.take()) {
        ui.clear_timeout(job);
    }
    let job = ui.set_timeout(1000, |ui| {
        let (capturing, sec) = with(|s| {
            s.vt.countdown = (s.vt.countdown - 1).max(0);
            (s.vt.capturing, s.vt.countdown)
        });
        if capturing {
            if let Some(frm) = FraVideo::components(ui) {
                frm.lblVtStatus.set_text(ui, tw("Gravando a amostra… {{sec}} s. Mexa em algo na tela.", &[("sec", &sec.to_string())]));
            }
            vt_tick(ui);
        }
    });
    with(|s| s.vt.job = Some(job));
}

// ---------------------------------------------------------------------------
// Áudio
// ---------------------------------------------------------------------------

fn audio_hint(k: i32) -> String {
    let band = if k <= 96 {
        "voice"
    } else if k <= 160 {
        "good"
    } else if k <= 192 {
        "default"
    } else {
        "high"
    };
    let mb = ((k as f64) * 3600.0 / 8.0 / 1000.0).round() as i64;
    format!("{} {}", lang::tr_of("settings.audioBitrate.hint", &band.to_string()), tw("Cerca de {{mb}} MB por hora, por faixa. Vale a partir da próxima gravação.", &[("mb", &mb.to_string())]))
}

// ---------------------------------------------------------------------------
// Buffer
// ---------------------------------------------------------------------------

fn sec_label(v: i64) -> String {
    if v < 60 {
        tw("{{n}} segundos", &[("n", &v.to_string())])
    } else {
        tw("{{n}} min", &[("n", &((v as f64 / 60.0).round() as i64).to_string())])
    }
}

fn mb_label(v: i64) -> String {
    if v < 1024 {
        return format!("{v} MB");
    }
    let gb = v as f64 / 1024.0;
    format!("{} GB", if gb.fract() == 0.0 { fmt_num(gb, 0) } else { fmt_num(gb, 1) })
}

/// Valores da lista com o atual incluído (config editado à mão continua lá).
fn with_current(mut list: Vec<i64>, cur: i64) -> (Vec<i64>, usize) {
    if cur > 0 && !list.contains(&cur) {
        list.push(cur);
        list.sort_unstable();
    }
    let i = list.iter().position(|v| *v == cur).unwrap_or(0);
    (list, i)
}

// ---------------------------------------------------------------------------
// Atalhos
// ---------------------------------------------------------------------------

fn split_spec(spec: &str) -> (bool, bool, bool, bool, String) {
    let mut m = (false, false, false, false);
    let mut key = String::new();
    for p in spec.split('+').map(str::trim).filter(|p| !p.is_empty()) {
        match p.to_lowercase().as_str() {
            "ctrl" => m.0 = true,
            "shift" => m.1 = true,
            "alt" => m.2 = true,
            "win" => m.3 = true,
            _ => key = p.to_owned(),
        }
    }
    (m.0, m.1, m.2, m.3, key)
}

fn hk_send(items: Vec<Item>) {
    bhk::handle_set_items(items);
}

fn hk_update(i: usize, f: impl FnOnce(&mut Item)) {
    let mut items = with(|s| s.hk_items.clone());
    if let Some(it) = items.get_mut(i) {
        f(it);
        hk_send(items);
    }
}

/// Modificador ou tecla mudou na linha `i`: monta a combinação e valida (não
/// repetida, não reservada) antes de mandar.
fn hk_combo(ui: &mut Ui, i: usize, mods: (bool, bool, bool, bool), key: String) {
    let mut parts: Vec<&str> = Vec::new();
    if mods.0 {
        parts.push("Ctrl");
    }
    if mods.1 {
        parts.push("Shift");
    }
    if mods.2 {
        parts.push("Alt");
    }
    if mods.3 {
        parts.push("Win");
    }
    if key.is_empty() {
        if !parts.is_empty() {
            // Espera a tecla: o que está marcado fica só na tela.
            with(|s| s.hk_pending.insert(i, mods));
            return;
        }
    } else {
        parts.push(&key);
    }
    with(|s| s.hk_pending.remove(&i));
    let spec = crate::hotkeys::normalize(&parts.join("+"));
    let items = with(|s| s.hk_items.clone());
    if items.get(i).is_some_and(|it| it.hotkey == spec) {
        return;
    }
    if !spec.is_empty() {
        if let Some(other) = items.iter().enumerate().find(|(j, x)| *j != i && !x.hotkey.is_empty() && x.hotkey.eq_ignore_ascii_case(&spec)) {
            crate::app_view::show_toast(
                &lang::tr("Atalho inválido"),
                &tw("Essa combinação já é do atalho \"{{mode}}\".", &[("mode", &lang::tr_of("hotkeys.mode", other.1.mode.as_str()))]),
                true,
                7000,
            );
            render_tab(ui);
            return;
        }
        let reason = match hotkey::parse_hotkey(&spec) {
            None => Some("Atalho inválido — selecione pelo menos uma tecla principal (letra, número, F1-F12, Pause, etc)."),
            Some(hk) => hotkey::reserved_reason(hk.modifiers, hk.vk),
        };
        if let Some(r) = reason {
            crate::app_view::show_toast(&lang::tr("Atalho inválido"), &format!("{} {}", lang::tr(r), lang::tr("Escolha outra combinação.")), true, 7000);
            render_tab(ui);
            return;
        }
    }
    hk_update(i, |it| it.hotkey = spec);
}

pub fn on_hotkeys_state(items: &[Item], status: &[&'static str], delay_max: u32, defining: Option<usize>) {
    with(|s| {
        s.hk_items = items.to_vec();
        s.hk_status = status.to_vec();
        s.hk_delay_max = delay_max;
        s.hk_defining = defining;
    });
    refresh("hotkeys");
}

// ---------------------------------------------------------------------------
// Dispositivos
// ---------------------------------------------------------------------------

/// Ordem "natural" sem caixa ("Monitor 2" < "Monitor 10").
fn natural_key(s: &str) -> Vec<(String, u64)> {
    let mut out = Vec::new();
    let mut text = String::new();
    let mut num = String::new();
    for c in s.to_lowercase().chars() {
        if c.is_ascii_digit() {
            num.push(c);
        } else {
            if !num.is_empty() {
                out.push((std::mem::take(&mut text), num.parse().unwrap_or(0)));
                num.clear();
            }
            text.push(c);
        }
    }
    out.push((text, num.parse().unwrap_or(0)));
    out
}

fn set_device_flag(id: &str, f: impl Fn(&mut Device)) {
    with(|s| {
        for d in s.monitors.iter_mut().chain(s.webcams.iter_mut()).chain(s.mics.iter_mut()).chain(s.speakers.iter_mut()) {
            if d.id == id {
                f(d);
            }
        }
    });
}

/// Listas de dispositivos da tela principal (hot-plug incluso).
pub fn on_devices(kind: &str, list: &[Device]) {
    with(|s| match kind {
        "monitors" => s.monitors = list.to_vec(),
        "webcams" => s.webcams = list.to_vec(),
        "mics" => s.mics = list.to_vec(),
        _ => s.speakers = list.to_vec(),
    });
    refresh("devices");
}

pub fn on_root_folders(folders: &[RootFolder]) {
    with(|s| s.folders = folders.to_vec());
    refresh("files");
}

// ---------------------------------------------------------------------------
// Transcrição
// ---------------------------------------------------------------------------

fn transcribe_langs() -> (Vec<String>, Vec<String>, usize) {
    let cur = match config::get_config_str("transcribeLanguage", "").trim().to_lowercase() {
        l if l.is_empty() => "app".to_owned(),
        l => l,
    };
    let ui_lang = lang::current_language();
    let base = ui_lang.split('-').next().unwrap_or("pt").to_lowercase();
    let ui_name = TRANSCRIBE_LANGS.iter().find(|(c, _)| *c == base).map(|(_, n)| n.to_string()).unwrap_or(base.clone());
    let mut vals: Vec<String> = vec!["app".into(), "auto".into()];
    let mut labels = vec![tw("Idioma do NoOBS ({{name}})", &[("name", &ui_name)]), lang::tr("Detectar automaticamente")];
    for (c, n) in TRANSCRIBE_LANGS {
        vals.push(c.into());
        labels.push(n.into());
    }
    if !vals.contains(&cur) {
        vals.push(cur.clone());
        labels.push(cur.clone());
    }
    let i = vals.iter().position(|v| *v == cur).unwrap_or(0);
    (vals, labels, i)
}

fn gb(bytes: i64) -> String {
    let g = bytes.max(0) as f64 / 1e9;
    format!("{} GB", fmt_num(g, if g >= 10.0 { 0 } else { 1 }))
}

/// (situação, detalhe, botão, botão perigoso, progresso).
fn asr_texts() -> (String, String, String, bool, f32) {
    let d = with(|s| s.asr.clone()).unwrap_or(LocalAsrInfo { status: "missing".into(), ..Default::default() });
    let confirm = with(|s| s.asr_confirm_remove);
    let l = |k: &str| lang::tr_of("settings.transcribe.local", &k.to_string());
    let lw = |k: &str, a: &[(&str, &str)]| lang::tr_of_with("settings.transcribe.local", k, a);
    match d.status.as_str() {
        "installing" => {
            let stage = if d.stage.is_empty() { String::new() } else { l(&format!("stages.{}", d.stage)) };
            (
                lw("installing", &[("stage", &stage)]),
                format!("{} · {}", lw("progress", &[("done", &gb(d.done)), ("total", &gb(d.total))]), l("resumeHint")),
                l("cancel"),
                false,
                if d.total > 0 { (d.done as f64 / d.total as f64).min(1.0) as f32 } else { 0.0 },
            )
        }
        "ready" => (
            if d.device.is_empty() { l("readyCpu") } else { lw("ready", &[("device", &d.device)]) },
            lw("readyHow", &[("size", &gb(d.total))]),
            if confirm { l("removeConfirm") } else { l("remove") },
            confirm,
            -1.0,
        ),
        s => (
            if s == "error" { lw("error", &[("error", &d.error)]) } else { l("missing") },
            lw("missingHow", &[("size", &gb(d.total))]),
            if s == "error" { l("retry") } else { l("install") },
            false,
            -1.0,
        ),
    }
}

fn asr_action(ui: &mut Ui) {
    let status = with(|s| s.asr.as_ref().map(|a| a.status.clone())).unwrap_or_else(|| "missing".into());
    match status.as_str() {
        "installing" => crate::local_asr::cancel_install(),
        "ready" => {
            // Dois cliques: apagar 3,7 GB por engano custaria outro download.
            if !with(|s| std::mem::replace(&mut s.asr_confirm_remove, false)) {
                with(|s| s.asr_confirm_remove = true);
                render_tab(ui);
                return;
            }
            render_tab(ui);
            btr::handle_local_asr_remove();
        }
        _ => crate::local_asr::start_install(),
    }
}

/// A linha de progresso da fila de transcrição.
fn transcribe_line() -> (bool, f32, String) {
    let (t, clicked, eta) = with(|s| (s.tr.clone(), s.tr_cancel_clicked, s.tr_eta_shown));
    let canceling = t.canceling || clicked;
    if !t.running && t.queue == 0 {
        if t.total > 0 {
            let txt = if t.failed > 0 {
                tw("{{done}} de {{total}} concluída(s), {{failed}} com falha.", &[("done", &t.done.to_string()), ("total", &t.total.to_string()), ("failed", &t.failed.to_string())])
            } else {
                tw("{{total}} transcrição(ões) concluída(s).", &[("total", &t.total.to_string())])
            };
            return (true, 1.0, txt);
        }
        return (false, 0.0, String::new());
    }
    if canceling && t.running {
        return (true, 0.0, tw("Cancelando a transcrição de “{{name}}”…", &[("name", if t.current.is_empty() { "—" } else { &t.current })]));
    }
    let pos = (t.done + t.failed + 1).min(t.total.max(1));
    let total = if t.total > 0 { t.total } else { pos };
    let mut txt = tw("{{pos}} de {{total}} · {{name}}", &[("pos", &pos.to_string()), ("total", &total.to_string()), ("name", if t.current.is_empty() { "—" } else { &t.current })]);
    if !t.stage.is_empty() {
        let stg = lang::tr_of("settings.transcribe.stage", &t.stage);
        if !stg.starts_with('[') {
            txt.push_str(&format!(" · {stg}"));
        }
    }
    if t.track_count > 1 {
        txt.push_str(&format!(" · {}", tw("faixa {{n}} de {{total}}", &[("n", &(t.track + 1).to_string()), ("total", &t.track_count.to_string())])));
    }
    if t.progress >= 0.0 {
        txt.push_str(&format!(" · {}%", (t.progress * 100.0).round() as i64));
    }
    if eta > 0 {
        txt.push_str(&format!(" · {}", tw("faltam ~{{eta}}", &[("eta", &fmt_dur(eta))])));
    } else {
        txt.push_str(&format!(" · {}", tw("{{elapsed}} decorrido", &[("elapsed", &fmt_dur(t.elapsed))])));
    }
    if t.failed > 0 {
        txt.push_str(&format!(" · {}", tw("{{failed}} com falha", &[("failed", &t.failed.to_string())])));
    }
    (true, if t.progress >= 0.0 { t.progress as f32 } else { 0.0 }, txt)
}

pub fn on_local_asr_state(st: &LocalAsrInfo) {
    if st.remove_refused {
        crate::app_view::show_toast(&lang::tr("Não dá para remover o motor local durante uma instalação ou transcrição."), "", true, 4000);
    }
    with(|s| s.asr = Some(st.clone()));
    refresh("transcribe");
    crate::app_main::on_transcribe_chip(transcribe_chip());
}

pub fn on_transcribe_state(t: &TranscribeState) {
    let t = t.clone();
    with_ui(move |ui| {
        let (failed_before, was_canceling) = with(|s| (s.tr.failed, s.tr.canceling || s.tr_cancel_clicked));
        let structure_changed = with(|s| s.tr.running != t.running || s.tr.queue != t.queue || s.tr.canceling != t.canceling);
        with(|s| {
            s.tr = t.clone();
            if t.canceling || !t.running {
                s.tr_cancel_clicked = false;
            }
            // O "faltam" só desce.
            let key = if t.running { format!("{}#{}", t.current, t.track) } else { String::new() };
            if key != s.tr_eta_for {
                s.tr_eta_for = key;
                s.tr_eta_shown = -1;
            }
            if t.eta > 0 {
                s.tr_eta_shown = if s.tr_eta_shown < 0 { t.eta } else { s.tr_eta_shown.min(t.eta) };
            }
        });
        // Aviso na BORDA do contador e só com motivo novo.
        if t.failed > failed_before && !t.last_error.is_empty() && with(|s| s.tr_last_toast_error != t.last_error) {
            with(|s| s.tr_last_toast_error = t.last_error.clone());
            let name = if t.last_error_name.is_empty() { "—".to_owned() } else { t.last_error_name.clone() };
            crate::app_view::show_toast(&tw("Falha ao transcrever \"{{name}}\"", &[("name", &name)]), &t.last_error, true, 12000);
        }
        if t.failed == 0 {
            with(|s| s.tr_last_toast_error.clear());
        }
        if was_canceling && !t.canceling && !with(|s| s.tr_cancel_clicked) && !t.running {
            crate::app_view::show_toast(&lang::tr("Transcrição cancelada"), "", false, 3000);
        }
        sync_tick(ui, t.running);
        if is_open() && with(|s| s.tab == "transcribe") {
            if structure_changed {
                render_tab(ui);
            } else {
                update_transcribe_line(ui);
            }
        }
        crate::app_main::on_transcribe_chip(transcribe_chip());
    });
}

fn update_transcribe_line(ui: &mut Ui) {
    let (_, p, line) = transcribe_line();
    if let Some(frm) = FraTranscribe::components(ui) {
        frm.lblTrLine.set_text(ui, line);
        frm.prgTrQueue.set_value(ui, (p * 100.0) as f64);
    }
}

/// O decorrido anda sozinho entre os pushes.
fn sync_tick(ui: &mut Ui, running: bool) {
    let has = with(|s| s.tr_job.is_some());
    if running && !has {
        tr_tick(ui);
    } else if !running && has {
        if let Some(job) = with(|s| s.tr_job.take()) {
            ui.clear_timeout(job);
        }
    }
}

fn tr_tick(ui: &mut Ui) {
    let job = ui.set_timeout(1000, |ui| {
        with(|s| {
            s.tr.elapsed += 1;
            if s.tr_eta_shown > 0 {
                s.tr_eta_shown -= 1;
            }
        });
        if is_open() && with(|s| s.tab == "transcribe") {
            update_transcribe_line(ui);
        }
        tr_tick(ui);
    });
    with(|s| s.tr_job = Some(job));
}

pub fn on_transcribe_pending(count: usize, cloud: usize) {
    with(|s| {
        s.tr_pending = count;
        s.tr_cloud = cloud;
    });
    refresh("transcribe");
}

pub fn on_transcribe_queue(items: &[QueueItem]) {
    with(|s| s.tr_queue = items.to_vec());
    refresh("transcribe");
}

/// Selo da tela inicial: (mostrar, rodando, número, dica).
pub fn transcribe_chip() -> (bool, bool, i32, String) {
    with(|s| {
        let ready = s.asr.as_ref().is_some_and(|a| a.status == "ready");
        let running = s.tr.running;
        let total = s.tr.queue;
        let waiting = (total - if running { 1 } else { 0 }).max(0);
        let show = ready && (running || waiting > 0);
        let canceling = s.tr.canceling || s.tr_cancel_clicked;
        let hint = if canceling {
            lang::tr("Cancelando…")
        } else if running {
            if waiting > 0 { tw("Transcrevendo, mais {{count}} na fila — clique para ver", &[("count", &waiting.to_string())]) } else { lang::tr("Transcrevendo — clique para ver") }
        } else {
            tw("{{count}} na fila de transcrição — clique para ver", &[("count", &waiting.to_string())])
        };
        (show, running, total.max(1) as i32, hint)
    })
}

// ---------------------------------------------------------------------------
// Atualizações e Sobre
// ---------------------------------------------------------------------------

fn update_status() -> (String, bool) {
    let (r, checking) = with(|s| (s.update.clone(), s.checking));
    if checking {
        return (lang::tr("Verificando..."), false);
    }
    let Some(r) = r else { return (String::new(), false) };
    if !r.ok {
        (lang::tr("Não foi possível verificar agora. Confira sua conexão e tente novamente."), false)
    } else if r.has_update {
        (tw("Nova versão disponível: {{version}}.", &[("version", &r.latest)]), !r.url.is_empty())
    } else {
        (lang::tr("Você está na versão mais recente."), false)
    }
}

pub fn on_update_result(r: &UpdateInfo) {
    let r = r.clone();
    with_ui(move |ui| {
        with(|s| {
            s.update = Some(r.clone());
            s.checking = false;
        });
        if r.ok && r.has_update {
            crate::app_main::show_update_banner(ui, &r.latest, &r.url);
        }
        if is_open() && with(|s| s.tab == "updates") {
            render_tab(ui);
        }
    });
}

/// As linhas de uma lista do idioma (`about.features.0`, `.1`…).
fn lang_list(key: &str) -> Vec<String> {
    let mut v = Vec::new();
    for i in 0.. {
        let t = lang::tr_of(key, &i.to_string());
        if t.is_empty() || t.starts_with('[') {
            break;
        }
        v.push(t);
    }
    v
}

// ---------------------------------------------------------------------------
// Restaurar padrões e chaves
// ---------------------------------------------------------------------------

fn restore_defaults(ui: &mut Ui) {
    crate::app_view::confirm(
        ui,
        &lang::tr("Restaurar padrão"),
        &lang::tr("Isso vai redefinir todos os campos pros valores padrão e aplicar na hora. Continuar?"),
        &lang::tr("Restaurar"),
        false,
        crate::app_view::ConfirmIcon::Merge,
        do_restore,
    );
}

fn do_restore(ui: &mut Ui) {
    bridge::handle_set_language("");
    settings::handle_set_record_dir("");
    settings::handle_set_codec("auto");
    shell::handle_set_window_title("NoOBS");
    settings::handle_set_filename_pattern(settings::DEFAULT_FILENAME_PATTERN);
    shell::handle_set_autostart(true);
    shell::handle_set_run_as_admin(false);
    recording::handle_set_fullscreen_detect(true);
    shell::handle_set_close_to_tray(true);
    shell::handle_set_hibernate(true);
    shell::handle_set_config_bool("minimizeOnRecord", true);
    recording::handle_set_rec_indicator(false);
    recording::handle_set_rec_indicator_corner("top-right");
    recording::handle_set_rec_indicator_opacity(90);
    recording::handle_set_rec_indicator_clickable(true);
    shell::handle_set_config_bool("playSoundOnRecord", true);
    shell::handle_set_config_bool("stopOnLock", true);
    settings::handle_set_mute_when_device_muted(true);
    btr::handle_set_transcribe_language("app");
    btr::handle_set_transcribe_on_stop(true);
    settings::handle_set_auto_record_on_mic(false);
    settings::handle_set_auto_record_mic_apps("");
    settings::handle_set_auto_record_mic_except("");
    settings::handle_set_check_updates(true);
    settings::handle_set_recording_quality(5);
    settings::handle_set_recording_fps(30);
    settings::handle_set_recording_keyframe(2);
    settings::handle_set_audio_bitrate(AUDIO_BITRATE_DEFAULT);
    settings::handle_set_library_thumbs("auto");
    btr::handle_reset_hidden_lists();
    // Buffer: limites e indicador voltam ao padrão; o liga/desliga é da tela
    // principal e fica como está.
    replay::handle_set_replay_autostart(false);
    replay::handle_set_replay_into_recording(true);
    replay::handle_set_replay_indicator(true);
    replay::handle_set_replay_indicator_corner("top-right");
    replay::handle_set_replay_indicator_opacity(90);
    replay::handle_set_replay_indicator_clickable(true);
    replay::handle_set_replay_apps("");
    replay::handle_set_replay_limits(REPLAY_DEFAULT_SEC as i32, REPLAY_DEFAULT_MB as i32);
    bhk::handle_reset();
    bridge::handle_set_theme("system");
    load_values();
    render_tab(ui);
    crate::app_view::show_toast(&lang::tr("Campos restaurados"), &lang::tr("As alterações já foram aplicadas."), false, 4000);
}

/// Uma chave de liga/desliga mudou.
fn check_changed(ui: &mut Ui, key: &str, on: bool) {
    match key {
        "muteWhenDeviceMuted" => settings::handle_set_mute_when_device_muted(on),
        "replayAutoStart" => replay::handle_set_replay_autostart(on),
        "replayIntoRecording" => replay::handle_set_replay_into_recording(on),
        "replayIndicator" => replay::handle_set_replay_indicator(on),
        "replayIndicatorClickable" => replay::handle_set_replay_indicator_clickable(on),
        "recIndicator" => recording::handle_set_rec_indicator(on),
        "recIndicatorClickable" => recording::handle_set_rec_indicator_clickable(on),
        "fullscreenDetect" => recording::handle_set_fullscreen_detect(on),
        "playSoundOnRecord" | "minimizeOnRecord" | "stopOnLock" => shell::handle_set_config_bool(key, on),
        "autoRecordOnMic" => settings::handle_set_auto_record_on_mic(on),
        "transcribeOnStop" => btr::handle_set_transcribe_on_stop(on),
        "checkUpdates" => settings::handle_set_check_updates(on),
        _ => log!("Configuracoes: opcao desconhecida \"{key}\"."),
    }
    // As que mostram ou escondem outras linhas refazem a aba.
    if matches!(key, "recIndicator" | "replayIndicator" | "autoRecordOnMic") {
        ui.set_timeout(0, render_tab);
    }
}

// ---------------------------------------------------------------------------
// Avisos do core
// ---------------------------------------------------------------------------

pub fn on_settings_changed() {
    with_ui(|ui| {
        if is_open() {
            // Com a tela aberta, a pasta é do core (ele resolve o vazio).
            set_record_dir_field(ui, &recording::record_dir().to_string_lossy());
        }
    });
}

pub fn on_record_dir_picked(path: &str) {
    let p = path.to_owned();
    with_ui(move |ui| set_record_dir_field(ui, &p));
    settings::handle_set_record_dir(path);
}

pub fn on_encoder_caps() {
    refresh("recording");
}

pub fn on_replay_state(r: &ReplayState) {
    with(|s| s.replay = Some(r.clone()));
    refresh("replay");
}

pub fn on_language_changed() {
    if is_open() {
        with_ui(|ui| {
            let tab = with(|s| s.tab.clone());
            show_tab(ui, &tab);
        });
    }
}

// ---------------------------------------------------------------------------
// A página (o `VerticalLayout` das telas originais)
// ---------------------------------------------------------------------------
//
// Cada aba é uma coluna, declarada no JSON dela: os campos (rótulo, controle, dica) com
// 6 px entre as partes, 10 px entre um campo e a divisória. As listas (dispositivos,
// atalhos, a fila...) são frames de `forms/settings/parts/` empilhados no contêiner dela.

/// Espaço entre as partes de um campo.
const FIELD_GAP: f32 = 6.0;
/// Espaço entre os campos e as divisórias.
const TAB_GAP: f32 = 10.0;
/// Largura máxima da coluna dos campos.
const COL_MAX: f32 = 760.0;
/// Largura mínima da coluna: abaixo dela os textos param de quebrar e o resto fica cortado.
const COL_MIN: f32 = 280.0;

thread_local! {
    /// (largura da coluna, largura das divisórias).
    static WIDTHS: std::cell::Cell<(f32, f32)> = const { std::cell::Cell::new((760.0, 760.0)) };
}

fn col_w() -> f32 {
    WIDTHS.with(|w| w.get().0)
}

/// A página da aba aberta: o form do frame dela (`fraGeneral`...), mostrada no
/// `tbcSettings`.
fn clear_page(ui: &mut Ui) -> Option<ComponentId> {
    let tab = with(|s| s.tab.clone());
    // Cada aba é uma página do `tbcSettings` (sem a barra: a navegação é a lateral).
    let index = TABS.iter().position(|t| *t == tab).unwrap_or(0);
    if let Some(frm) = FrmSettings::components(ui) {
        frm.tbcSettings.set_tab_index(ui, index as i64);
    }
    tab_form(ui, &tab)
}

// ----- os frames das abas -----

// Os helpers recebem o componente já achado (`FraX::components(ui).map(|frm| frm.x)`):
// `None` quando o frame não está aberto, e então não fazem nada.

/// O botão do tamanho do texto dele (os textos mudam com o idioma).
fn fit_button(ui: &mut Ui, b: Option<g::Button>) {
    let Some(b) = b else { return };
    let w = text_width(ui, b.id, &b.text(ui)) + 30.0;
    b.set_width(ui, f64::from(w));
}

/// Se o evento confirma o texto de um campo: sair dele (`on_exit`) ou o Enter
/// (`on_key_down`); as outras teclas não confirmam.
pub fn commits(args: &EventArgs) -> bool {
    !matches!(args, EventArgs::Key(k) if k.key != Key::Enter)
}

/// A posição do item escolhido num combo (0 sem nenhum).
fn item_of(ui: &Ui, combo: g::ComboBox) -> usize {
    combo.item_index(ui).max(0) as usize
}

/// Os itens de um combo numa linha só (`;` separa; um `;` no texto vira `,`).
fn combo_items(items: &[String]) -> String {
    items.iter().map(|s| s.replace(';', ",")).collect::<Vec<_>>().join(";")
}

/// Os itens de um combo e o escolhido.
fn fitems(ui: &mut Ui, combo: Option<g::ComboBox>, items: &[String], index: usize) {
    let Some(combo) = combo else { return };
    combo.set_items(ui, combo_items(items));
    combo.set_item_index(ui, index as i64);
}

/// Altura do texto do rótulo `l` quebrado na largura `w` (com a fonte do estilo dele).
fn wrapped_height(ui: &Ui, l: ComponentId, text: &str, w: f32) -> f32 {
    let style = TextStyle { font: ui.font_of(l), wrap: true, ..TextStyle::default() };
    ui.measure_text_wrapped(text, &style, w.max(40.0)).height.ceil() + 1.0
}

fn text_width(ui: &Ui, l: ComponentId, text: &str) -> f32 {
    let style = TextStyle { font: ui.font_of(l), ..TextStyle::default() };
    ui.measure_text(text, &style).width.ceil()
}

/// Linhas que um texto com `<b>` ocupa na largura `w` (os trechos em negrito medidos
/// em negrito).
fn md_lines(ui: &Ui, l: ComponentId, html: &str, w: f32) -> f32 {
    let font = ui.font_of(l);
    let mut total = 0.0;
    for (run, bold, _) in aylis::controls::markup_runs(html) {
        let mut f = font.clone();
        if bold {
            f.weight = FontWeight::Bold;
        }
        total += ui.measure_text(&run, &TextStyle { font: f, ..TextStyle::default() }).width;
    }
    if total <= w {
        return 1.0;
    }
    let plain = aylis::controls::markup_plain(html);
    let one = ui.measure_text("Ág", &TextStyle { font: font.clone(), ..TextStyle::default() }).height.max(1.0);
    (wrapped_height(ui, l, &plain, w * 0.97) / one).round().max(1.0)
}

/// Envio adiado (`ms`) — o slider manda a cada passo e o core regrava a config.
fn debounce(ui: &mut Ui, key: &'static str, ms: u64, f: impl FnOnce(&mut Ui) + 'static) {
    if let Some(job) = with(|s| s.debounce.remove(key)) {
        ui.clear_timeout(job);
    }
    let job = ui.set_timeout(ms, move |ui| {
        with(|s| s.debounce.remove(key));
        f(ui);
    });
    with(|s| s.debounce.insert(key, job));
}

/// A área rolável (`lySettings`) fica da altura da página da aba aberta.
fn fit_page(ui: &mut Ui) {
    let Some(area) = FrmSettings::components(ui).map(|frm| frm.lySettings.id) else { return };
    let Some(p) = tab_form(ui, &with(|s| s.tab.clone())) else { return };
    // A altura dos textos (`auto_size`) sai do layout.
    ui.layout();
    let h: f32 = ui
        .children(p)
        .to_vec()
        .iter()
        .filter_map(|&id| ui.control(id).filter(|cb| cb.visible()).map(|cb| cb.height() + cb.margins().top + cb.margins().bottom))
        .sum();
    let pad = ui.control(p).map_or(28.0, |cb| cb.padding().top + cb.padding().bottom);
    if let Some(cb) = ui.control_mut(area) {
        cb.set_height(h + pad);
    }
    ui.layout();
}

// ---------------------------------------------------------------------------
// Abas
// ---------------------------------------------------------------------------

/// A barra lateral (desenhada no `frmSettings`): o texto de cada aba no idioma e a aberta
/// em destaque.
fn render_sidebar(ui: &mut Ui) {
    let current = with(|s| s.tab.clone());
    let Some(frm) = FrmSettings::components(ui) else { return };
    // Na ordem de `TABS`.
    let rows = [(frm.pnlTabGeneral, frm.lblTabGeneral), (frm.pnlTabOnRecord, frm.lblTabOnRecord), (frm.pnlTabFiles, frm.lblTabFiles), (frm.pnlTabVideo, frm.lblTabVideo), (frm.pnlTabAudio, frm.lblTabAudio), (frm.pnlTabReplay, frm.lblTabReplay), (frm.pnlTabHotkeys, frm.lblTabHotkeys), (frm.pnlTabDevices, frm.lblTabDevices), (frm.pnlTabTranscribe, frm.lblTabTranscribe), (frm.pnlTabUpdates, frm.lblTabUpdates), (frm.pnlTabAbout, frm.lblTabAbout)];
    for (tab, (row, label)) in TABS.iter().zip(rows) {
        let style = if *tab == current { "s_tab_on" } else { "s_tab" };
        row.set_style(ui, style);
        label.set_style(ui, style);
        label.set_text(ui, lang::tr_of("settings.tabs", &tab.to_string()));
    }
}

/// O clique numa aba da barra lateral (o `tag` da linha é a posição em `TABS`).
pub fn show_tab_at(ui: &mut Ui, index: usize) {
    if let Some(tab) = TABS.get(index) {
        show_tab(ui, tab);
    }
}

pub fn show_tab(ui: &mut Ui, tab: &str) {
    with(|s| s.tab = tab.to_owned());
    render_sidebar(ui);
    render_tab(ui);
    enter_tab(tab);
    if let Some(sb) = FrmSettings::components(ui).map(|frm| frm.scrSettings.id).and_then(|id| ui.control_mut(id)) {
        sb.set_scroll_offset(PointF::ZERO);
    }
}

fn render_tab(ui: &mut Ui) {
    let tab = with(|s| s.tab.clone());
    let Some(page) = clear_page(ui) else { return };
    measure_column(ui);
    if let Some(cb) = ui.control_mut(page) {
        // A página não fica mais estreita que a coluna mínima (com as margens): numa janela
        // menor, o que passar é cortado, em vez de os textos quebrarem letra por letra.
        cb.set_min_width(COL_MIN + 36.0);
        // Na web o 1º campo de toda aba menos a Geral herda 10 px do campo escondido.
        cb.set_padding([18.0, if tab == "general" { 14.0 } else { 24.0 }, 18.0, 14.0]);
    }
    match tab.as_str() {
        "general" => load_general(ui),
        "onrecord" => load_onrecord(ui),
        "files" => load_files(ui),
        "recording" => load_video(ui),
        "audio" => load_audio(ui),
        "replay" => load_replay(ui),
        "hotkeys" => load_hotkeys(ui),
        "devices" => load_devices(ui),
        "transcribe" => load_transcribe(ui),
        "updates" => load_updates(ui),
        _ => load_about(ui),
    }
    limit_column(ui, page);
    fit_page(ui);
}

/// A largura que a coluna tem agora (até 760 px, no mínimo 280): só para o que ainda é
/// calculado à mão (as marcas dos sliders, as duas colunas do Buffer). Devolve se mudou.
fn measure_column(ui: &mut Ui) -> bool {
    let body = FrmSettings::components(ui).map(|frm| frm.pnlSettings.id).and_then(|s| ui.bounds(s)).map_or(1190.0, |b| b.width) - 190.0;
    let col = (body - 36.0).min(COL_MAX).max(COL_MIN);
    WIDTHS.with(|w| w.replace((col, (body - 36.0).max(col))).0 != col)
}

/// A coluna pelo próprio layout: cada campo da página ocupa a largura até 760 px, encostado
/// à esquerda (`self_align: start` com `max_width`), e não fica mais estreito que 280. As
/// divisórias (`lnRule*`) e os grupos que as têm esticam até a borda. Nada aqui depende da
/// largura da janela: redimensionar não mexe em nenhum campo.
fn limit_column(ui: &mut Ui, page: ComponentId) {
    fn has_rule(ui: &Ui, id: ComponentId) -> bool {
        ui.children(id).iter().any(|&c| ui.name(c).is_some_and(|n| n.starts_with("lnRule")) || has_rule(ui, c))
    }
    let mut stack = ui.children(page).to_vec();
    while let Some(id) = stack.pop() {
        let Some(cb) = ui.control(id) else { continue };
        if !matches!(cb.align(), Align::Top | Align::Bottom) {
            continue;
        }
        if ui.name(id).is_some_and(|n| n.starts_with("lnRule")) {
            continue;
        }
        if has_rule(ui, id) {
            // Um grupo com divisórias: ele estica e os campos dele é que ficam na coluna.
            stack.extend(ui.children(id).iter().copied());
            continue;
        }
        // Quem já escolheu o próprio tamanho (o fundo do tema) fica como está.
        if cb.self_align() != aylis::core::SelfAlign::Stretch {
            continue;
        }
        if let Some(cb) = ui.control_mut(id) {
            cb.set_self_align(aylis::core::SelfAlign::Start).set_max_width(COL_MAX).set_min_width(COL_MIN);
        }
    }
}

pub fn resized(ui: &mut Ui) {
    thread_local! {
        /// A aba inteira refeita quando a largura parar de mudar.
        static SETTLE: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
    }
    if !is_open() {
        return;
    }
    // A coluna e as divisórias acompanham sozinhas (o layout); aqui, só a altura da página
    // (os textos quebram em outra largura), a cada quadro do arrasto.
    ui.set_timeout(0, fit_page);
    // O que ainda é calculado pela largura da coluna (as marcas dos sliders, as duas colunas
    // do Buffer): a aba é refeita quando a largura parar, e só se a coluna mudou (acima de
    // 796 px ela fica em 760 e nada se refaz).
    SETTLE.with(|s| ui.clear_timeout(s.get()));
    let id = ui.set_timeout(150, |ui| {
        if measure_column(ui) {
            render_tab(ui);
        }
    });
    SETTLE.with(|s| s.set(id));
}

// ---------------------------------------------------------------------------
// Geral
// ---------------------------------------------------------------------------
/// Os botões do tema, na ordem do `tag` deles: (modo, texto, dica); os botões, em
/// `theme_buttons`.
const THEME_MODES: [(&str, &str, &str); 3] = [
    ("system", "Sistema", "Tema do sistema"),
    ("light", "Claro", "Tema claro"),
    ("dark", "Escuro", "Tema escuro"),
];

/// Os idiomas da interface: "Automático" e os embutidos (nome nativo); os códigos e o
/// escolhido.
fn language_options() -> (Vec<String>, Vec<String>, usize) {
    let pref = config::get_config_str("language", "");
    let mut labels = vec![lang::tr("Automático (sistema)")];
    let mut codes = vec![String::new()];
    let mut idx = 0;
    if let Some(list) = lang::available_languages().as_array() {
        for l in list {
            let code = l.get("code").and_then(|v| v.as_str()).unwrap_or_default();
            if code.is_empty() {
                continue;
            }
            let name = l.get("nativeName").or_else(|| l.get("name")).and_then(|v| v.as_str()).unwrap_or(code);
            if code.eq_ignore_ascii_case(&pref) {
                idx = labels.len();
            }
            labels.push(name.to_owned());
            codes.push(code.to_owned());
        }
    }
    (labels, codes, idx)
}

fn load_general(ui: &mut Ui) {
    let Some(frm) = FraGeneral::components(ui) else { return };
    // O tema: Sistema / Claro / Escuro lado a lado, do tamanho do texto, o escolhido em
    // destaque.
    let pref = config::get_config_str("theme", "system");
    let mut x = 4.0;
    let buttons = [frm.btnThemeSystem, frm.btnThemeLight, frm.btnThemeDark];
    for ((mode, key, tip), b) in THEME_MODES.into_iter().zip(buttons) {
        let text = lang::tr(key);
        b.set_style(ui, if pref == mode { "s_seg_on" } else { "s_seg" });
        b.set_text(ui, text.as_str());
        b.set_hint(ui, lang::tr(tip));
        let w = text_width(ui, b.id, &text) + 16.0 + 6.0 + 28.0;
        if let Some(cb) = ui.control_mut(b.id) {
            cb.set_rect(x, 4.0, w, 27.0);
        }
        x += w + 3.0;
    }
    // O fundo do tamanho dos botões, encostado à esquerda (não depende da largura da coluna).
    if let Some(cb) = ui.control_mut(frm.pnlTheme.id) {
        let m = cb.margins();
        cb.set_margins([0.0, m.top, 0.0, 0.0]).set_self_align(aylis::core::SelfAlign::Start).set_width(x + 1.0);
    }
    let (labels, _, idx) = language_options();
    fitems(ui, Some(frm.cmbLanguage), &labels, idx);
    frm.edtWindowTitle.set_text(ui, config::get_config_str("windowTitle", "NoOBS"));
    frm.chkAutostart.set_checked(ui, crate::autostart::is_autostart_enabled());
    frm.chkRunAsAdmin.set_checked(ui, config::get_config_bool("runAsAdmin", false));
    let state = if crate::elevate::is_process_elevated() { "Agora: rodando como administrador." } else { "Agora: rodando sem administrador." };
    frm.lblRunAsAdminState.set_text(ui, lang::tr(state));
    let tray = config::get_config_bool("closeToTray", true);
    frm.chkCloseToTray.set_checked(ui, tray);
    // Hibernar só vale indo para a bandeja.
    frm.chkHibernate.set_checked(ui, config::get_config_bool("hibernate", true));
    frm.chkHibernate.set_enabled(ui, tray);
    fit_button(ui, Some(frm.btnReset));
}

/// Os eventos do frame da aba Geral (ligados em `forms/general.rs`, que lê os valores).
pub mod general {
    use super::*;

    /// Um dos botões do tema (`i`, o `tag` dele, é a posição em `THEME_MODES`).
    pub fn theme(ui: &mut Ui, i: usize) {
        if let Some((mode, ..)) = THEME_MODES.get(i) {
            bridge::handle_set_theme(mode);
            ui.set_timeout(0, render_tab);
        }
    }

    /// O idioma escolhido (a posição na lista de `language_options`).
    pub fn language(i: usize) {
        let (_, codes, _) = language_options();
        bridge::handle_set_language(codes.get(i).map_or("", |s| s.as_str()));
    }

    pub fn window_title(t: &str) {
        if t != config::get_config_str("windowTitle", "NoOBS") {
            shell::handle_set_window_title(t);
        }
    }

    pub fn autostart(on: bool) {
        shell::handle_set_autostart(on);
    }

    pub fn run_as_admin(on: bool) {
        shell::handle_set_run_as_admin(on);
    }

    pub fn close_to_tray(ui: &mut Ui, on: bool) {
        shell::handle_set_close_to_tray(on);
        if let Some(frm) = FraGeneral::components(ui) {
            frm.chkHibernate.set_enabled(ui, on);
        }
    }

    /// "Liberar memória após 1 minuto na bandeja".
    pub fn hibernate(on: bool) {
        shell::handle_set_hibernate(on);
    }

    pub fn reset(ui: &mut Ui) {
        restore_defaults(ui);
    }
}

// ---------------------------------------------------------------------------
// Gravação
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Arquivos
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Vídeo
// ---------------------------------------------------------------------------

// ----- teste de qualidade -----

/// O teste de qualidade, no contêiner `lyVideoTest` do frame da aba Vídeo: os botões, o
/// andamento, os resultados e a comparação estão no JSON e aparecem conforme o estado;
/// cada nível do resultado é um `fraVtLevel` (`forms/settings/parts/vt_level.json`).
fn video_test_box(ui: &mut Ui, frm: &g::FraVideoComponents) {
    let (running, capturing, countdown, pct, level, result, a, b, zoom) =
        with(|s| (s.vt.running, s.vt.capturing, s.vt.countdown, s.vt.pct, s.vt.level, s.vt.result.clone(), s.vt.a, s.vt.b, s.vt.zoom));
    frm.btnVtRun.set_text(ui, lang::tr(if result.is_some() { "Testar de novo" } else { "Testar os níveis" }));
    fit_button(ui, Some(frm.btnVtRun));
    frm.btnVtRun.set_enabled(ui, !running);
    frm.btnVtCancel.set_visible(ui, running);
    fit_button(ui, Some(frm.btnVtCancel));
    frm.lblVtStatus.set_visible(ui, running);
    frm.prgVt.set_visible(ui, running && !capturing);
    frm.lyVtResult.set_visible(ui, !running && result.is_some());
    if running {
        let status = if capturing {
            tw("Gravando a amostra… {{sec}} s. Mexa em algo na tela.", &[("sec", &countdown.to_string())])
        } else if level >= 0 {
            tw("Gerando o nível {{n}}… {{pct}}%", &[("n", &level.to_string()), ("pct", &pct.to_string())])
        } else {
            lang::tr("Preparando…")
        };
        frm.lblVtStatus.set_text(ui, status);
        frm.lblVtStatus.set_style(ui, if capturing { "s_err" } else { "s_hint" });
        frm.prgVt.set_value(ui, pct.clamp(0, 100) as f64);
        return;
    }
    let Some(r) = result else { return };
    frm.lblVtSample.set_text(
        ui,
        tw(
            "Amostra de {{sec}} s em {{w}}×{{h}}, {{fps}} fps, codificada com {{encoder}}. Os tamanhos são estimativas por hora de gravação com esse mesmo conteúdo.",
            &[("w", &r.width.to_string()), ("h", &r.height.to_string()), ("fps", &(r.fps.round() as i64).to_string()), ("sec", &fmt_num(r.duration, 1)), ("encoder", &r.encoder)],
        ),
    );
    let mut levels = r.levels.clone();
    levels.sort_by_key(|l| -l.level);
    let max = levels.iter().map(|l| per_hour(l.bytes, r.duration)).fold(1.0f64, f64::max);
    let cur_h = levels.iter().find(|l| l.level == r.current_level).map(|l| per_hour(l.bytes, r.duration)).unwrap_or(0.0);
    clear_list(ui, frm.lyVtLevels.id);
    frm.lyVtLevels.set_visible(ui, !levels.is_empty());
    // As barras ganham a largura depois do layout (a fração da trilha).
    let mut bars = Vec::new();
    for (k, l) in levels.iter().enumerate() {
        let h = per_hour(l.bytes, r.duration);
        let current = l.level == r.current_level;
        let rel = if current {
            lang::tr("atual")
        } else if h > 0.0 && cur_h > 0.0 {
            let f = h / cur_h;
            format!("×{}", fmt_num(f, if f < 1.0 { 2 } else { 1 }))
        } else {
            String::new()
        };
        let Ok(line) = FraVtLevel::create_in(ui, frm.lyVtLevels.id) else { continue };
        if k > 0 {
            set_top(ui, line.form.id, 2.0);
        }
        line.pnlLevel.set_style(ui, if l.level == a { "s_vt_a" } else if l.level == b { "s_vt_b" } else { "s_vt" });
        let lv = l.level;
        ui.on(line.pnlLevel.id, "on_pointer_up", move |ui, _, args| {
            if let EventArgs::Pointer(e) = args {
                with(|s| {
                    if e.button == PointerButton::Secondary {
                        s.vt.b = lv;
                    } else {
                        s.vt.a = lv;
                    }
                });
                render_tab(ui);
            }
        });
        line.lblLevel.set_text(ui, tw("Nível {{n}}", &[("n", &l.level.to_string())]));
        line.lblRelative.set_text(ui, rel);
        if current {
            line.lblRelative.set_style(ui, "s_q_now12");
            line.pnlBarFill.set_style(ui, "s_bar_on");
        }
        line.lblSize.set_text(ui, if h > 0.0 { tw("{{size}}/hora", &[("size", &fmt_size(h))]) } else { "—".into() });
        bars.push((line.pnlBarFill, if h > 0.0 { (h / max).max(0.015) as f32 } else { 0.0 }));
    }
    // Trocar os lados | zoom: do tamanho do texto medido na fonte do botão "Testar os níveis"
    // (a do estilo `s`, sem os 12 px destes), e o zoom escolhido em destaque.
    for (bt, z) in [(frm.btnVtSwap, -1), (frm.btnVtZoom300, 3), (frm.btnVtZoom200, 2), (frm.btnVtZoom100, 1), (frm.btnVtZoomFit, 0)] {
        let w = text_width(ui, frm.btnVtRun.id, &bt.text(ui)) + 22.0;
        bt.set_width(ui, f64::from(w));
        bt.set_style(ui, if zoom == z { "s_on" } else { "s" });
    }
    compare_view(ui, frm, &r, a, b, zoom);
    frm.btnVtUse.set_text(ui, tw("Usar o nível {{n}} nas gravações", &[("n", &a.to_string())]));
    fit_button(ui, Some(frm.btnVtUse));
    frm.btnVtUse.set_enabled(ui, a >= 0 && a != r.current_level);
    ui.layout();
    for (fill, frac) in bars {
        let w = ui.parent(fill.id).and_then(|p| ui.bounds(p)).map_or(0.0, |b| b.width);
        fill.set_width(ui, f64::from(w * frac));
    }
}

/// A geometria da comparação montada (o quadro e as imagens no zoom): o arrasto só
/// reposiciona as peças, sem reler e decodificar as duas imagens a cada passo.
#[derive(Clone, Copy)]
struct CompareGeom {
    /// A largura e a altura da área das imagens (o quadro sem a borda).
    w: f32,
    h: f32,
    /// O tamanho das imagens no zoom.
    dw: f32,
    dh: f32,
    zoom: i32,
}

impl CompareGeom {
    /// O deslocamento só vale com zoom e até a borda da imagem encostar no quadro (antes, o
    /// de um zoom ficava no "Ajustar" e a imagem saía do quadro: tudo preto).
    fn limit(self, pan: (f32, f32)) -> (f32, f32) {
        if self.zoom == 0 {
            return (0.0, 0.0);
        }
        let (mx, my) = (((self.dw - self.w) / 2.0).max(0.0), ((self.dh - self.h) / 2.0).max(0.0));
        (pan.0.clamp(-mx, mx), pan.1.clamp(-my, my))
    }
}

/// O quadro de dois níveis lado a lado com a divisória (A à esquerda, B à direita): o
/// `pnlVtStage` do JSON, com as imagens dos níveis; sem a imagem de A, o quadro some.
fn compare_view(ui: &mut Ui, frm: &g::FraVideoComponents, r: &VideoTestState, a: i32, b: i32, zoom: i32) {
    let load = |lv: i32| r.levels.iter().find(|l| l.level == lv).and_then(|l| std::fs::read(&l.image).ok());
    let (img_a, img_b) = (load(a), load(b));
    frm.pnlVtStage.set_visible(ui, img_a.is_some());
    with(|s| s.vt.geom = None);
    let Some(img_a) = img_a else { return };
    let scale = crate::app_player::window_scale().max(0.5);
    let (iw, ih) = (r.width.max(1) as f32 / scale, r.height.max(1) as f32 / scale);
    let w = col_w() - 2.0;
    // Cabe na largura e na altura do quadro (até 420): só pela largura, uma imagem 16:9 na
    // coluna cheia ficava uns pixels mais alta que o quadro e passava em cima e embaixo.
    let fit = (w / iw).min(420.0 / ih).min(1.0);
    let k = match zoom {
        1 => 1.0,
        2 => 2.0,
        3 => 3.0,
        _ => fit,
    };
    let h = (ih * fit).min(420.0);
    frm.pnlVtStage.set_height(ui, f64::from(h + 2.0));
    // As imagens vêm em bytes (PNG do teste): sem propriedade para isso, pelo `ImageView`.
    let put = |ui: &mut Ui, v: ComponentId, bytes: Vec<u8>| {
        if let Some(iv) = ui.get_mut::<ImageView>(v) {
            iv.set_picture(Some(Image::encoded(bytes)));
        }
    };
    put(ui, frm.imgVtA.id, img_a);
    frm.lyVtB.set_visible(ui, img_b.is_some());
    if let Some(bytes) = img_b {
        put(ui, frm.imgVtB.id, bytes);
    }
    with(|s| s.vt.geom = Some(CompareGeom { w, h, dw: iw * k, dh: ih * k, zoom }));
    arrange_compare(ui);
    // As etiquetas dos lados.
    for (tag, label, side, lv) in [(frm.pnlVtTagA, frm.lblVtTagA, 0, a), (frm.pnlVtTagB, frm.lblVtTagB, 1, b)] {
        let text = tw("Nível {{n}}", &[("n", &lv.to_string())]);
        let tw_ = text_width(ui, label.id, &text) + 6.0 + 6.0 + 3.0 + 4.0;
        label.set_text(ui, text);
        let x = if side == 0 { 9.0 } else { w - tw_ - 8.0 };
        if let Some(cb) = ui.control_mut(tag.id) {
            cb.set_rect(x, 9.0, tw_, 24.0);
        }
    }
}

/// Onde fica cada parte da comparação com a divisória em `split` e o deslocamento `pan`.
fn arrange_compare(ui: &mut Ui) {
    let Some(frm) = FraVideo::components(ui) else { return };
    let Some(geom) = with(|s| s.vt.geom) else { return };
    let (split, pan) = with(|s| (s.vt.split, s.vt.pan));
    let (w, h, dw, dh) = (geom.w, geom.h, geom.dw, geom.dh);
    let pan = geom.limit(pan);
    let (ox, oy) = (1.0 + (w - dw) / 2.0 + pan.0, 1.0 + (h - dh) / 2.0 + pan.1);
    let cut = 1.0 + w * split;
    let mut set = |id: ComponentId, x: f32, y: f32, width: f32, height: f32| {
        if let Some(cb) = ui.control_mut(id) {
            cb.set_rect(x, y, width, height);
        }
    };
    set(frm.imgVtA.id, ox, oy, dw, dh);
    set(frm.lyVtB.id, cut, 1.0, w - (cut - 1.0), h);
    set(frm.imgVtB.id, ox - cut, oy, dw, dh);
    set(frm.shpVtSplit.id, cut - 1.0, 1.0, 2.0, h);
    set(frm.cirVtKnob.id, cut - 13.0, 1.0 + h / 2.0 - 13.0, 26.0, 26.0);
    set(frm.lblVtKnob.id, cut - 13.0, 1.0 + h / 2.0 - 13.0, 26.0, 26.0);
    ui.invalidate();
}

// ---------------------------------------------------------------------------
// Áudio
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Buffer
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Atalhos
// ---------------------------------------------------------------------------

/// Os cartões dos atalhos no `lyHotkeys` da aba Atalhos: cada um é um `fraHotkeyCard`
/// (`forms/settings/parts/hotkey_card.json`) com todas as linhas; aqui só aparecem as do
/// modo, com os valores. O cartão tem a altura das linhas à vista (`auto_size`).
fn hotkey_cards(ui: &mut Ui, frm: &g::FraHotkeysComponents) {
    let keys = key_table();
    let (items, status, delay_max, defining, pending) = with(|s| (s.hk_items.clone(), s.hk_status.clone(), s.hk_delay_max, s.hk_defining, s.hk_pending.clone()));
    let mut delays: Vec<u32> = HK_DELAYS.iter().copied().filter(|v| *v <= delay_max.max(1)).collect();
    for it in &items {
        if !delays.contains(&it.delay_sec) {
            delays.push(it.delay_sec);
        }
    }
    delays.sort_unstable();
    let delays = std::rc::Rc::new(delays);
    let mode_labels: Vec<String> = HK_MODES.iter().map(|m| lang::tr_of("hotkeys.mode", m.as_str())).collect();
    let key_values: std::rc::Rc<Vec<String>> = std::rc::Rc::new(std::iter::once(String::new()).chain(keys.iter().filter(|k| !k.2).map(|k| k.0.clone())).collect());
    let key_labels: Vec<String> = std::iter::once(lang::tr("Não definida")).chain(keys.iter().filter(|k| !k.2).map(|k| k.1.clone())).collect();
    let delay_labels: Vec<String> = delays.iter().map(|v| if *v == 0 { lang::tr("Sem atraso") } else { tw("{{sec}} segundos", &[("sec", &v.to_string())]) }).collect();
    let (mode_items, key_items, delay_items) = (combo_items(&mode_labels), combo_items(&key_labels), combo_items(&delay_labels));
    let list = frm.lyHotkeys.id;
    clear_list(ui, list);
    frm.lblHotkeysEmpty.set_visible(ui, items.is_empty());
    for (i, it) in items.iter().enumerate() {
        let (mut ctrl, mut shift, mut alt, mut win, key) = split_spec(&it.hotkey);
        if let Some(p) = pending.get(&i) {
            (ctrl, shift, alt, win) = *p;
        }
        let ki = if key.is_empty() { 0 } else { key_values.iter().position(|k| k.eq_ignore_ascii_case(&key)).unwrap_or(0) };
        let Ok(card) = FraHotkeyCard::create_in(ui, list) else { continue };
        set_top(ui, card.form.id, if i == 0 { FIELD_GAP + 6.0 } else { TAB_GAP });
        // Modo, modificadores, tecla, excluir.
        card.cmbMode.set_items(ui, mode_items.as_str());
        card.cmbMode.set_item_index(ui, HK_MODES.iter().position(|m| *m == it.mode).unwrap_or(0) as i64);
        let mode = card.cmbMode;
        ui.on(mode.id, "on_change", move |ui, _, _| {
            let mode = HK_MODES.get(item_of(ui, mode)).copied().unwrap_or(Mode::Record);
            hk_update(i, |it| {
                let hk = it.hotkey.clone();
                let delay = it.delay_sec;
                *it = Item::new(mode, &hk);
                it.hotkey = hk;
                if mode.records() {
                    it.delay_sec = delay;
                }
            });
        });
        // `.hotkey-mod`: caixa + nome, verde quando marcado, da largura do nome.
        let toggles = [(card.pnlCtrl, card.chkCtrl, ctrl), (card.pnlShift, card.chkShift, shift), (card.pnlAlt, card.chkAlt, alt), (card.pnlWin, card.chkWin, win)];
        // A tecla da combinação (a escolhida no combo, ou a do atalho se o combo não a tem).
        let key_now = std::rc::Rc::new(RefCell::new(key.clone()));
        for (panel, chk, on) in toggles {
            panel.set_style(ui, if on { "s_mod_on" } else { "s_mod" });
            chk.set_checked(ui, on);
            let w = text_width(ui, chk.id, &chk.text(ui)) + 13.0 + 5.0 + 22.0;
            panel.set_width(ui, f64::from(w));
            let key_now = key_now.clone();
            ui.on(chk.id, "on_change", move |ui, _, _| {
                let mods = (card.chkCtrl.checked(ui), card.chkShift.checked(ui), card.chkAlt.checked(ui), card.chkWin.checked(ui));
                hk_combo(ui, i, mods, key_now.borrow().clone());
                ui.set_timeout(0, render_tab);
            });
        }
        ui.on(card.btnRemove.id, "on_click", move |_, _, _| {
            let mut items = with(|s| s.hk_items.clone());
            if i < items.len() {
                items.remove(i);
                hk_send(items);
            }
        });
        card.cmbKey.set_items(ui, key_items.as_str());
        card.cmbKey.set_item_index(ui, ki as i64);
        let kv = key_values.clone();
        ui.on(card.cmbKey.id, "on_change", move |ui, _, _| {
            let key = kv.get(item_of(ui, card.cmbKey)).cloned().unwrap_or_default();
            *key_now.borrow_mut() = key.clone();
            let mods = (card.chkCtrl.checked(ui), card.chkShift.checked(ui), card.chkAlt.checked(ui), card.chkWin.checked(ui));
            hk_combo(ui, i, mods, key);
        });
        // Janela: o título (e o som do app); área: a área salva.
        if it.mode == Mode::Window {
            card.lyWindow.set_visible(ui, true);
            card.lyAppAudio.set_visible(ui, true);
            let edit = card.edtWindowTitle;
            edit.set_text(ui, it.title.as_str());
            for event in ["on_exit", "on_key_down"] {
                let cur = it.title.clone();
                ui.on(edit.id, event, move |ui, _, args| {
                    if commits(args) {
                        let t = edit.text(ui).trim().to_owned();
                        if t != cur {
                            hk_update(i, |it| it.title = t);
                        }
                    }
                });
            }
            card.chkAppAudio.set_checked(ui, it.app_audio);
            let chk = card.chkAppAudio;
            ui.on(chk.id, "on_change", move |ui, _, _| {
                let on = chk.checked(ui);
                hk_update(i, |it| it.app_audio = on);
            });
        }
        // Print: o que capturar (a seleção na hora, a área salva ou o monitor do mouse).
        if it.mode == Mode::Screenshot {
            card.lyShot.set_visible(ui, true);
            card.lyShotInfo.set_visible(ui, true);
            let w = text_width(ui, card.lblShotTarget.id, &card.lblShotTarget.text(ui)).max(140.0);
            card.lblShotTarget.set_width(ui, f64::from(w));
            let labels: Vec<String> = ShotTarget::ALL.iter().map(|t| lang::tr_of("hotkeys.shot", t.as_str())).collect();
            card.cmbShotTarget.set_items(ui, combo_items(&labels).as_str());
            card.cmbShotTarget.set_item_index(ui, ShotTarget::ALL.iter().position(|t| *t == it.target).unwrap_or(0) as i64);
            let cmb = card.cmbShotTarget;
            ui.on(cmb.id, "on_change", move |ui, _, _| {
                let target = ShotTarget::ALL.get(item_of(ui, cmb)).copied().unwrap_or_default();
                hk_update(i, |it| {
                    it.target = target;
                    // A área salva só vale com o alvo área.
                    if target != ShotTarget::Area {
                        it.area = None;
                    }
                });
            });
        }
        let shows_area = it.mode == Mode::Area || (it.mode == Mode::Screenshot && it.target == ShotTarget::Area);
        if shows_area {
            card.lyArea.set_visible(ui, true);
            card.lyTimelapse.set_visible(ui, it.mode == Mode::Area);
            // No print, "na hora" é um alvo do combo: sem o botão de voltar a ela.
            card.btnAreaClear.set_visible(ui, it.mode == Mode::Area && it.area.is_some());
            if it.mode == Mode::Screenshot && it.area.is_none() {
                card.lblArea.set_text(ui, lang::tr("Nenhuma área definida — até definir, o atalho abre a seleção"));
            }
            if it.mode == Mode::Area && it.area.is_some() {
                fit_button(ui, Some(card.btnAreaClear));
                ui.on(card.btnAreaClear.id, "on_click", move |_, _, _| hk_update(i, |it| it.area = None));
            }
            let define = lang::tr(if defining == Some(i) {
                "Escolhendo…"
            } else if it.area.is_some() {
                "Escolher outra"
            } else {
                "Definir área"
            });
            card.btnAreaDefine.set_text(ui, define);
            fit_button(ui, Some(card.btnAreaDefine));
            card.btnAreaDefine.set_enabled(ui, defining.is_none());
            ui.on(card.btnAreaDefine.id, "on_click", move |_, _, _| bhk::handle_define_area(i));
            if let Some(r) = it.area {
                card.lblArea.set_text(ui, tw("{{w}} × {{h}} em ({{x}}, {{y}})", &[("w", &r.w.to_string()), ("h", &r.h.to_string()), ("x", &r.x.to_string()), ("y", &r.y.to_string())]));
                card.lblArea.set_style(ui, "s_about_v");
            }
            // O Timelapse deste atalho (o do Capturar... não vale aqui).
            card.chkTimelapse.set_checked(ui, it.timelapse);
            let chk = card.chkTimelapse;
            ui.on(chk.id, "on_change", move |ui, _, _| {
                let on = chk.checked(ui);
                hk_update(i, |it| it.timelapse = on);
            });
        }
        card.lyDelay.set_visible(ui, it.mode.records());
        if it.mode.records() {
            let w = text_width(ui, card.lblDelay.id, &card.lblDelay.text(ui)).max(140.0);
            card.lblDelay.set_width(ui, f64::from(w));
            card.cmbDelay.set_items(ui, delay_items.as_str());
            card.cmbDelay.set_item_index(ui, delays.iter().position(|d| *d == it.delay_sec).unwrap_or(0) as i64);
            let dl = delays.clone();
            ui.on(card.cmbDelay.id, "on_change", move |ui, _, _| {
                let v = dl.get(item_of(ui, card.cmbDelay)).copied().unwrap_or(0);
                hk_update(i, |it| it.delay_sec = v);
            });
        }
        let st_key = status.get(i).copied().unwrap_or("");
        if !st_key.is_empty() {
            card.lyStatus.set_visible(ui, true);
            card.lblStatus.set_text(ui, lang::tr_of("hotkeys.status", st_key));
            card.lblStatus.set_style(ui, if st_key == "inactive" { "s_hint" } else { "s_err" });
        }
    }
}

// ---------------------------------------------------------------------------
// Dispositivos
// ---------------------------------------------------------------------------

/// Os grupos de dispositivos (`.devices-group`) na lista `list`: cada grupo é um
/// `fraDeviceGroup` (título em maiúsculas e a linha) com um `fraDeviceRow` por dispositivo
/// (`forms/settings/parts/`). `pick` diz se entra e se está marcado; sem nenhum, aparece o
/// aviso `empty` (fixo no JSON da aba).
fn device_groups(ui: &mut Ui, list: ComponentId, empty: g::Label, pick: impl Fn(&Device) -> Option<bool>, toggle: impl Fn(&mut Ui, &str, bool) + Clone + 'static) {
    let (mons, cams, mics, spks) = with(|s| (s.monitors.clone(), s.webcams.clone(), s.mics.clone(), s.speakers.clone()));
    clear_list(ui, list);
    let mut gi = 0;
    for (key, mut devices) in [("monitors", mons), ("webcams", cams), ("mics", mics), ("speakers", spks)] {
        devices.sort_by_key(|d| natural_key(&d.name));
        let rows: Vec<(Device, bool)> = devices.into_iter().filter_map(|d| pick(&d).map(|on| (d, on))).collect();
        if rows.is_empty() {
            continue;
        }
        let Ok(group) = FraDeviceGroup::create_in(ui, list) else { continue };
        // O primeiro grupo a 10 px do texto de cima; os outros, a 14 do grupo anterior.
        set_top(ui, group.form.id, if gi == 0 { FIELD_GAP + 4.0 } else { 14.0 });
        gi += 1;
        group.lblGroup.set_text(ui, lang::tr_of("devices.groups", key));
        for (d, on) in rows {
            let Ok(row) = FraDeviceRow::create_in(ui, group.lyRows.id) else { continue };
            row.chkOn.set_checked(ui, on);
            if !d.info.is_empty() {
                let w = text_width(ui, row.lblInfo.id, &d.info) + 2.0;
                row.lblInfo.set_text(ui, d.info.as_str());
                row.lblInfo.set_width(ui, f64::from(w));
                row.lblInfo.set_visible(ui, true);
            }
            row.lblName.set_text(ui, if d.name.is_empty() { d.id.as_str() } else { d.name.as_str() });
            let id = d.id.clone();
            let t = toggle.clone();
            ui.on(row.pnlRow.id, "on_click", move |ui, _, _| {
                t(ui, &id, !on);
                ui.set_timeout(0, render_tab);
            });
        }
    }
    empty.set_visible(ui, gi == 0);
}

// ---------------------------------------------------------------------------
// Transcrição
// ---------------------------------------------------------------------------

/// A fila (`.tq-item`) no quadro `pnlTrQueue`: cada item é um `fraQueueRow`
/// (`forms/settings/parts/queue_row.json`). A em curso no alto; as que esperam se arrastam
/// para reordenar.
fn transcribe_queue(ui: &mut Ui, frame: ComponentId, queue: &[QueueItem]) {
    clear_list(ui, frame);
    let mut waiting = 0i32;
    for (i, q) in queue.iter().enumerate() {
        let Ok(row) = FraQueueRow::create_in(ui, frame) else { continue };
        let r = row.pnlRow;
        // A linha entre os itens.
        row.shpLine.set_visible(ui, i > 0);
        if q.current {
            r.set_style(ui, "s_q_cur");
            row.lblDot.set_text(ui, "●");
            row.lblDot.set_style(ui, "s_q_dot");
            row.lblName.set_style(ui, "s_q_on");
            row.btnRemove.set_visible(ui, false);
            let w = text_width(ui, row.lblNow.id, &lang::tr("agora").to_uppercase()) + 4.0;
            row.lblNow.set_width(ui, f64::from(w));
            row.lblNow.set_visible(ui, true);
        } else {
            let id = q.id.clone();
            ui.on(row.btnRemove.id, "on_click", move |_, _, _| btr::handle_remove_transcribe_item(&id));
        }
        let dur = if q.duration > 0 { fmt_dur(q.duration as i64) } else { "—".into() };
        let w = text_width(ui, row.lblDuration.id, &dur) + 2.0;
        row.lblDuration.set_text(ui, dur);
        row.lblDuration.set_width(ui, f64::from(w));
        row.lblName.set_text(ui, q.name.as_str());
        // Arrastar para reordenar a espera (o aylis entrega o soltar na linha de destino).
        if !q.current {
            let at = waiting;
            let id = q.id.clone();
            r.set_draggable(ui, true);
            r.set_accepts_drops(ui, true);
            r.set_cursor(ui, "move");
            let name = q.name.clone();
            ui.on(r.id, "on_drag_start", move |ui, _, _| ui.begin_drag(id.clone(), name.clone()));
            ui.on(r.id, "on_drag_drop", move |_, _, args| {
                if let EventArgs::Text(id) = args {
                    btr::handle_move_transcribe_item(id, at);
                }
            });
            waiting += 1;
        }
    }
}

// ---------------------------------------------------------------------------
// Atualizações e Sobre
// ---------------------------------------------------------------------------

/// A lista com a bolinha verde (`.about-list`) em `list`: cada item é um `fraAboutItem`
/// (`forms/settings/parts/about_item.json`), da altura das linhas do texto dele.
fn about_list(ui: &mut Ui, list: ComponentId, items: &[String]) {
    clear_list(ui, list);
    let w = col_w() - 14.0;
    for it in items {
        let Ok(item) = FraAboutItem::create_in(ui, list) else { continue };
        item.lblText.set_text(ui, it.as_str());
        let h = md_lines(ui, item.lblText.id, it, w) * 12.0 * 1.55 + 2.0;
        item.form.set_height(ui, f64::from(h));
    }
}

// ---------------------------------------------------------------------------
// Peças comuns das abas desenhadas
// ---------------------------------------------------------------------------

/// O contêiner de uma lista que o código enche (`lyDevices`...), vazio: os frames da
/// montagem anterior saem.
fn clear_list(ui: &mut Ui, list: ComponentId) {
    let old = ui.children(list).to_vec();
    ui.free_many(&old);
}

/// O espaço acima de um item de lista (a margem de cima; as outras ficam as do JSON).
fn set_top(ui: &mut Ui, id: ComponentId, top: f32) {
    if let Some(cb) = ui.control_mut(id) {
        let m = cb.margins();
        cb.set_margins([m.left, top, m.right, m.bottom]);
    }
}

/// Um rótulo de uma linha do tamanho do texto dele (no mínimo `min`); devolve a largura.
fn fit_label(ui: &mut Ui, l: Option<g::Label>, min: f32) -> f32 {
    let Some(l) = l else { return min };
    let w = (text_width(ui, l.id, &l.text(ui)) + 2.0).max(min);
    l.set_width(ui, f64::from(w));
    w
}

/// Um texto que some quando vazio (a dica sem conteúdo não ocupa lugar).
fn ftext_or_hide(ui: &mut Ui, l: Option<g::Label>, text: &str) {
    let Some(l) = l else { return };
    l.set_text(ui, text);
    l.set_visible(ui, !text.is_empty());
}

/// A posição de um trilho (o passo, arredondado).
pub fn step_of(value: f64) -> usize {
    value.round().max(0.0) as usize
}

/// As peças de um trilho desenhado: o trilho, as legendas das pontas e a faixa das marcas.
#[derive(Clone, Copy)]
struct Slider {
    track: g::TrackBar,
    low: g::Label,
    high: g::Label,
    ticks: g::Layout,
}

/// As marcas embaixo de um trilho: as fixas do JSON (qualidade, keyframe) ou, quando o
/// número delas varia (o FPS, pelo monitor), um `fraSliderTick` para cada texto.
enum Ticks<'a> {
    Fixed(&'a [g::Label]),
    Made(&'a [String]),
}

/// Um trilho desenhado: as legendas do tamanho do texto, o valor e as marcas embaixo,
/// cada uma no lugar do passo dela.
fn load_slider(ui: &mut Ui, s: Option<Slider>, max: usize, value: usize, ticks: Ticks) {
    let Some(s) = s else { return };
    let mut middle = col_w();
    for end in [s.low, s.high] {
        middle -= fit_label(ui, Some(end), 44.0) + 10.0;
    }
    s.track.set_max(ui, max.max(1) as f64);
    s.track.set_value(ui, value as f64);
    // (o que se posiciona, o rótulo que se mede, o texto).
    let marks: Vec<(ComponentId, ComponentId, String)> = match ticks {
        Ticks::Fixed(labels) => labels.iter().map(|l| (l.id, l.id, l.text(ui))).collect(),
        Ticks::Made(texts) => {
            let old: Vec<ComponentId> = ui.children(s.ticks.id).iter().copied().filter(|&c| c != s.track.id).collect();
            ui.free_many(&old);
            texts
                .iter()
                .filter_map(|t| {
                    let tick = FraSliderTick::create_in(ui, s.ticks.id).ok()?;
                    tick.lblTick.set_text(ui, t.as_str());
                    Some((tick.form.id, tick.lblTick.id, t.clone()))
                })
                .collect()
        }
    };
    let count = marks.len();
    for (i, (target, label, text)) in marks.into_iter().enumerate() {
        let w = text_width(ui, label, &text) + 2.0;
        let x = 7.0 + if count > 1 { i as f32 / (count - 1) as f32 } else { 0.0 } * (middle - 14.0) - w / 2.0;
        if let Some(cb) = ui.control_mut(target) {
            cb.set_rect(x, 18.0, w, 14.0);
        }
    }
}

/// As peças de um indicador na tela (o da aba Gravação e o da Buffer): o canto, a
/// opacidade e a prévia da pílula.
#[derive(Clone, Copy)]
struct Indicator {
    corner: g::ComboBox,
    corner_label: g::Label,
    opacity_label: g::Label,
    opacity: g::TrackBar,
    pct: g::Label,
    preview: g::Panel,
    preview_label: g::Label,
}

fn rec_indicator(ui: &Ui) -> Option<Indicator> {
    let f = FraOnRecord::components(ui)?;
    Some(Indicator {
        corner: f.cmbRecCorner,
        corner_label: f.lblRecCorner,
        opacity_label: f.lblRecOpacity,
        opacity: f.trkRecOpacity,
        pct: f.lblRecOpacityPct,
        preview: f.pnlRecPreview,
        preview_label: f.lblRecPreview,
    })
}

fn replay_indicator(ui: &Ui) -> Option<Indicator> {
    let f = FraReplay::components(ui)?;
    Some(Indicator {
        corner: f.cmbRepCorner,
        corner_label: f.lblRepCorner,
        opacity_label: f.lblRepOpacity,
        opacity: f.trkRepOpacity,
        pct: f.lblRepOpacityPct,
        preview: f.pnlRepPreview,
        preview_label: f.lblRepPreview,
    })
}

/// O canto e a opacidade de um indicador na tela, com a prévia da pílula na opacidade
/// escolhida.
fn load_indicator(ui: &mut Ui, ind: Option<Indicator>, corner: &str, opacity: i32) {
    let Some(ind) = ind else { return };
    let i = CORNERS.iter().position(|c| *c == corner).unwrap_or(1);
    fitems(ui, Some(ind.corner), &corner_labels(), i);
    fit_label(ui, Some(ind.corner_label), 0.0);
    fit_label(ui, Some(ind.opacity_label), 0.0);
    ind.opacity.set_value(ui, ((opacity - 20) / 5) as f64);
    ind.pct.set_text(ui, format!("{opacity}%"));
    ind.preview.set_opacity(ui, opacity as f64 / 100.0);
    // A pílula do tamanho do tempo.
    let w = fit_label(ui, Some(ind.preview_label), 0.0);
    ind.preview.set_width(ui, f64::from(w + 30.0));
}

/// O trilho da opacidade mexeu (`step`, a posição dele): o número e a prévia na hora, o
/// core depois de parar.
fn indicator_opacity(ui: &mut Ui, ind: Option<Indicator>, step: usize, apply: fn(i32)) {
    let pct = 20 + step as i32 * 5;
    if let Some(ind) = ind {
        ind.pct.set_text(ui, format!("{pct}%"));
        ind.preview.set_opacity(ui, pct as f64 / 100.0);
    }
    debounce(ui, "opacity", 300, move |_| apply(pct));
}

// ---------------------------------------------------------------------------
// Gravação (`forms/settings/onrecord.json`)
// ---------------------------------------------------------------------------

fn load_onrecord(ui: &mut Ui) {
    let rec_on = config::get_config_bool("recIndicator", false);
    if let Some(frm) = FraOnRecord::components(ui) {
        frm.chkRecIndicator.set_checked(ui, rec_on);
        frm.lyRecOptions.set_visible(ui, rec_on);
    }
    if rec_on {
        if let Some(frm) = FraOnRecord::components(ui) {
            frm.chkRecClickable.set_checked(ui, config::get_config_bool("recIndicatorClickable", true));
        }
        let corner = config::get_config_str("recIndicatorCorner", "top-right");
        load_indicator(ui, rec_indicator(ui), &corner, config::get_config_int("recIndicatorOpacity", 90).clamp(20, 100));
    }
    // O texto da tela cheia diz o que some (os indicadores ligados).
    let replay_on = with(|s| s.replay.as_ref().map(|r| r.indicator)).unwrap_or(true);
    let which = match (rec_on, replay_on) {
        (true, true) => "labelBoth",
        (true, false) => "labelRecording",
        (false, true) => "labelBuffer",
        _ => "labelPreviews",
    };
    if let Some(frm) = FraOnRecord::components(ui) {
        frm.chkFullscreen.set_text(ui, lang::tr_of("settings.fullscreenDetect", &which.to_string()));
        frm.chkFullscreen.set_checked(ui, config::get_config_bool("fullscreenDetect", true));
    }
    let hint = if rec_on || replay_on { "Janelas por cima de uma aplicação em tela cheia derrubam o FPS dela. Desmarque se o indicador sumir ou as prévias congelarem sem nada em tela cheia aberto." } else { "Atualizar as prévias da tela inicial por cima de uma aplicação em tela cheia derruba o FPS dela. Desmarque se as prévias congelarem sem nada em tela cheia aberto." };
    if let Some(frm) = FraOnRecord::components(ui) {
        frm.lblFullscreenHint.set_text(ui, lang::tr(hint));
        frm.chkPlaySound.set_checked(ui, config::get_config_bool("playSoundOnRecord", true));
    }
    // Minimizar: para a bandeja ou para a barra de tarefas, conforme o "Minimizar para bandeja".
    let tray = config::get_config_bool("closeToTray", true);
    let (label_key, hint_key) =
        if tray { ("Minimizar para a bandeja ao iniciar a gravação", "Ao começar uma gravação, a janela do NoOBS é enviada pra bandeja do sistema — útil para gravar a tela sem a própria janela aparecer no vídeo. O atalho global continua funcionando pra parar.") } else { ("Minimizar ao iniciar a gravação", "Ao começar uma gravação, a janela do NoOBS é minimizada — útil para gravar a tela sem a própria janela aparecer no vídeo. O atalho global continua funcionando pra parar.") };
    if let Some(frm) = FraOnRecord::components(ui) {
        frm.chkMinimize.set_text(ui, lang::tr(label_key));
        frm.chkMinimize.set_checked(ui, config::get_config_bool("minimizeOnRecord", true));
        frm.lblMinimizeHint.set_text(ui, lang::tr(hint_key));
    }
    let auto = config::get_config_bool("autoRecordOnMic", false);
    let apps = config::get_config_str("autoRecordMicApps", "");
    if let Some(frm) = FraOnRecord::components(ui) {
        frm.chkAutoRecord.set_checked(ui, auto);
        frm.edtAutoApps.set_text(ui, apps.as_str());
    }
    // As exceções só valem monitorando todos os apps.
    if let Some(frm) = FraOnRecord::components(ui) {
        frm.lyAutoExcept.set_visible(ui, apps.is_empty());
        frm.edtAutoExcept.set_text(ui, config::get_config_str("autoRecordMicExcept", ""));
        frm.lyAutoDevices.set_visible(ui, auto);
    }
    if auto {
        if let Some(frm) = FraOnRecord::components(ui) {
            device_groups(ui, frm.lyAutoDeviceList.id, frm.lblAutoDevicesEmpty, |d| (!d.hidden).then_some(d.auto), |_, id, on| {
                set_device_flag(id, |d| d.auto = on);
                crate::bridge::devices::handle_set_auto_source(id, on);
            });
        }
    }
    if let Some(frm) = FraOnRecord::components(ui) {
        frm.chkStopOnLock.set_checked(ui, config::get_config_bool("stopOnLock", true));
    }
}

/// Os eventos do frame da aba Gravação (ligados em `forms/onrecord.rs`, que lê os valores).
pub mod onrecord {
    use super::*;

    /// Uma chave de liga/desliga: `key` é a da configuração.
    pub fn check(ui: &mut Ui, key: &str, on: bool) {
        check_changed(ui, key, on);
    }

    /// O canto escolhido (a posição em `CORNERS`).
    pub fn rec_corner(i: usize) {
        recording::handle_set_rec_indicator_corner(CORNERS.get(i).copied().unwrap_or("top-right"));
    }

    pub fn rec_opacity(ui: &mut Ui, step: usize) {
        indicator_opacity(ui, rec_indicator(ui), step, recording::handle_set_rec_indicator_opacity);
    }

    pub fn auto_apps(ui: &mut Ui, t: &str) {
        if t != config::get_config_str("autoRecordMicApps", "") {
            settings::handle_set_auto_record_mic_apps(t);
            ui.set_timeout(0, render_tab);
        }
    }

    pub fn auto_except(t: &str) {
        if t != config::get_config_str("autoRecordMicExcept", "") {
            settings::handle_set_auto_record_mic_except(t);
        }
    }
}

// ---------------------------------------------------------------------------
// Arquivos (`forms/settings/files.json`)
// ---------------------------------------------------------------------------

fn load_files(ui: &mut Ui) {
    let Some(frm) = FraFiles::components(ui) else { return };
    fit_button(ui, Some(frm.btnRecordDir));
    frm.edtRecordDir.set_text(ui, recording::record_dir().to_string_lossy().into_owned());
    // A pasta dos prints: vazio (o padrão) mostra o texto de ajuda "A mesma das gravações".
    fit_button(ui, Some(frm.btnPrintDir));
    frm.edtPrintDir.set_text(ui, config::get_config_str("printDir", "").as_str());
    frm.chkPrintsInLibrary.set_checked(ui, bridge::prints_in_library());
    let pattern = config::get_config_str("filenamePattern", settings::DEFAULT_FILENAME_PATTERN);
    frm.edtFilename.set_text(ui, pattern.as_str());
    frm.lblFilenamePreview.set_text(ui, filename_preview(&pattern));
    let lib = config::get_config_str("libraryThumbs", "auto");
    let li = LIBRARY_MODES.iter().position(|m| *m == lib).unwrap_or(0);
    let labels: Vec<String> = LIBRARY_MODES.iter().map(|m| lang::tr_of("settings.library", m)).collect();
    fitems(ui, Some(frm.cmbLibrary), &labels, li);
    frm.lblLibraryHint.set_text(ui, library_hint(LIBRARY_MODES[li]));
    hidden_folders(ui, &frm);
}

/// O campo da pasta de gravação, com a aba Arquivos à vista (com outra aba, ele é
/// preenchido ao abri-la).
fn set_record_dir_field(ui: &mut Ui, path: &str) {
    if !(is_open() && with(|s| s.tab == "files")) {
        return;
    }
    if let Some(frm) = FraFiles::components(ui) {
        frm.edtRecordDir.set_text(ui, path);
    }
}

/// As pastas da biblioteca, cada uma um `fraHiddenFolder`
/// (`forms/settings/parts/hidden_folder.json`): a caixa de esconder e a contagem.
fn hidden_folders(ui: &mut Ui, frm: &g::FraFilesComponents) {
    let folders = with(|s| s.folders.clone());
    let list = frm.lyHiddenFolders.id;
    clear_list(ui, list);
    frm.lblHiddenFoldersEmpty.set_visible(ui, folders.is_empty());
    for (i, f) in folders.into_iter().enumerate() {
        let info = if f.missing {
            lang::tr("(não existe mais)")
        } else {
            tw(if f.count == 1 { "1 gravação" } else { "{{count}} gravações" }, &[("count", &f.count.to_string())])
        };
        let Ok(row) = FraHiddenFolder::create_in(ui, list) else { continue };
        if i > 0 {
            set_top(ui, row.form.id, FIELD_GAP);
        }
        let chk = row.chkHidden;
        chk.set_text(ui, f.name.as_str());
        chk.set_checked(ui, f.hidden);
        let w = text_width(ui, chk.id, &f.name) + 24.0;
        chk.set_width(ui, f64::from(w));
        row.lblCount.set_text(ui, info);
        let name = f.name;
        ui.on(chk.id, "on_change", move |ui, _, _| {
            let on = chk.checked(ui);
            btr::handle_set_folder_hidden(&name, on);
        });
    }
}

/// Os eventos do frame da aba Arquivos (ligados em `forms/files.rs`, que lê os valores).
pub mod files {
    use super::*;

    pub fn browse() {
        settings::handle_pick_record_dir();
    }

    pub fn record_dir(p: &str) {
        if p != recording::record_dir().to_string_lossy() {
            settings::handle_set_record_dir(p);
        }
    }

    /// "Procurar..." da pasta dos prints: a escolhida vai para o campo e para a config.
    pub fn print_browse(ui: &mut Ui) {
        let Some(picked) = settings::handle_pick_print_dir() else { return };
        if settings::handle_set_print_dir(&picked) {
            if let Some(frm) = FraFiles::components(ui) {
                frm.edtPrintDir.set_text(ui, picked.as_str());
            }
        }
    }

    /// A pasta dos prints digitada (ao sair do campo ou no Enter). Não existe: o campo
    /// volta para a que vale.
    pub fn print_dir(ui: &mut Ui, p: &str) {
        let current = config::get_config_str("printDir", "");
        if p == current.trim() || settings::handle_set_print_dir(p) {
            return;
        }
        if let Some(frm) = FraFiles::components(ui) {
            frm.edtPrintDir.set_text(ui, current.as_str());
        }
    }

    pub fn prints_in_library(on: bool) {
        if on != bridge::prints_in_library() {
            settings::handle_set_prints_in_library(on);
        }
    }

    /// A prévia do nome acompanha a digitação.
    pub fn filename_edit(ui: &mut Ui, text: &str) {
        if let Some(frm) = FraFiles::components(ui) {
            frm.lblFilenamePreview.set_text(ui, filename_preview(text));
        }
    }

    pub fn filename(t: &str) {
        if t != config::get_config_str("filenamePattern", settings::DEFAULT_FILENAME_PATTERN) {
            settings::handle_set_filename_pattern(t);
        }
    }

    /// O modo das prévias escolhido (a posição em `LIBRARY_MODES`).
    pub fn library(ui: &mut Ui, i: usize) {
        settings::handle_set_library_thumbs(LIBRARY_MODES.get(i).copied().unwrap_or("auto"));
        ui.set_timeout(0, render_tab);
    }
}

// ---------------------------------------------------------------------------
// Vídeo (`forms/settings/video.json`)
// ---------------------------------------------------------------------------

fn load_video(ui: &mut Ui) {
    let Some(frm) = FraVideo::components(ui) else { return };
    let codec = config::get_config_str("codec", "auto");
    let ci = CODECS.iter().position(|c| c.eq_ignore_ascii_case(&codec)).unwrap_or(0);
    // O logo da placa de vídeo ao lado (o encoder de hardware é dela), dos assets do projeto.
    let logo = match bridge::last_encoder_caps().map(|c| c.vendor) {
        Some(GpuVendor::Amd) => Some("assets/images/gpu/amd.png"),
        Some(GpuVendor::Nvidia) => Some("assets/images/gpu/nvidia.png"),
        Some(GpuVendor::Intel) => Some("assets/images/gpu/intel.png"),
        _ => None,
    };
    frm.imgGpu.set_image(ui, logo.unwrap_or(""));
    frm.imgGpu.set_visible(ui, logo.is_some());
    let options = codec_options();
    let labels: Vec<String> = options.iter().map(|(t, _)| t.clone()).collect();
    frm.cmbCodec.set_groups(ui, codec_groups());
    frm.cmbCodec.set_disabled_items(ui, codec_disabled(&options));
    fitems(ui, Some(frm.cmbCodec), &labels, ci);
    let dim = match codec.as_str() {
        "h264-hw" => 4096,
        "h264-sw" | "hevc-hw" | "av1-hw" | "av1-sw" => 8192,
        _ if bridge::last_encoder_caps().is_some_and(|c| c.h264_hw) => 4096,
        _ => 8192,
    };
    frm.lblCodecMaxRes.set_text(ui, tw("Resolução máxima do codec: {{w}} × {{h}}", &[("w", &dim.to_string()), ("h", &dim.to_string())]));
    frm.lylblCodecSwWarn.set_visible(ui, codec == "av1-sw");
    let (q, fps, kf, presets) = with(|s| (s.quality, s.fps, s.keyframe, s.fps_presets.clone()));
    let quality_ticks = [
        frm.lblQualityTick0,
        frm.lblQualityTick1,
        frm.lblQualityTick2,
        frm.lblQualityTick3,
        frm.lblQualityTick4,
        frm.lblQualityTick5,
        frm.lblQualityTick6,
        frm.lblQualityTick7,
        frm.lblQualityTick8,
        frm.lblQualityTick9,
        frm.lblQualityTick10,
    ];
    let quality = Slider { track: frm.trkQuality, low: frm.lblQualityLow, high: frm.lblQualityHigh, ticks: frm.lyQualityTicks };
    load_slider(ui, Some(quality), 10, q.clamp(0, 10) as usize, Ticks::Fixed(&quality_ticks));
    frm.lblQualityHint.set_text(ui, lang::tr_of("settings.quality.hint", &q.to_string()));
    frm.lylblQualitySvtFloor.set_visible(ui, codec == "av1-sw" && q <= VT_SVT_FLOOR_LEVEL);
    video_test_box(ui, &frm);
    let fi = fps_index(&presets, fps);
    let ticks: Vec<String> = presets.iter().map(|p| p.to_string()).collect();
    let fps_slider = Slider { track: frm.trkFps, low: frm.lblFpsLow, high: frm.lblFpsHigh, ticks: frm.lyFpsTicks };
    load_slider(ui, Some(fps_slider), presets.len().max(1) - 1, fi, Ticks::Made(&ticks));
    frm.lblFpsHint.set_text(ui, fps_hint(presets.get(fi).copied().unwrap_or(fps)));
    let keyframe_ticks = [
        frm.lblKeyframeTick1,
        frm.lblKeyframeTick2,
        frm.lblKeyframeTick3,
        frm.lblKeyframeTick4,
        frm.lblKeyframeTick5,
        frm.lblKeyframeTick6,
        frm.lblKeyframeTick7,
        frm.lblKeyframeTick8,
        frm.lblKeyframeTick9,
        frm.lblKeyframeTick10,
    ];
    let keyframe = Slider { track: frm.trkKeyframe, low: frm.lblKeyframeLow, high: frm.lblKeyframeHigh, ticks: frm.lyKeyframeTicks };
    load_slider(ui, Some(keyframe), 9, (kf.clamp(1, 10) - 1) as usize, Ticks::Fixed(&keyframe_ticks));
    frm.lblKeyframeHint.set_text(ui, keyframe_hint(kf));
}

/// Os eventos do frame da aba Vídeo (ligados em `forms/video.rs`, que lê os valores).
pub mod video {
    use super::*;

    /// O codec escolhido (a posição em `CODECS`).
    pub fn codec(ui: &mut Ui, i: usize) {
        // Um codec que esta máquina não tem volta para o de antes.
        if codec_options().get(i).is_some_and(|(_, ok)| *ok) {
            settings::handle_set_codec(CODECS.get(i).copied().unwrap_or("auto"));
        }
        ui.set_timeout(0, render_tab);
    }

    /// Os trilhos (`step`, a posição) mandam ao core depois de parar (cada envio regrava
    /// a configuração).
    pub fn quality(ui: &mut Ui, step: usize) {
        let v = step as i32;
        with(|s| s.quality = v);
        if let Some(frm) = FraVideo::components(ui) {
            frm.lblQualityHint.set_text(ui, lang::tr_of("settings.quality.hint", &v.to_string()));
        }
        debounce(ui, "quality", 400, move |ui| {
            settings::handle_set_recording_quality(v);
            render_tab(ui);
        });
    }

    pub fn fps(ui: &mut Ui, step: usize) {
        let f = with(|s| s.fps_presets.get(step).copied()).unwrap_or(30);
        with(|s| s.fps = f);
        if let Some(frm) = FraVideo::components(ui) {
            frm.lblFpsHint.set_text(ui, fps_hint(f));
        }
        debounce(ui, "fps", 400, move |ui| {
            settings::handle_set_recording_fps(f);
            render_tab(ui);
        });
    }

    pub fn keyframe(ui: &mut Ui, step: usize) {
        let k = step as i32 + 1;
        with(|s| s.keyframe = k);
        if let Some(frm) = FraVideo::components(ui) {
            frm.lblKeyframeHint.set_text(ui, keyframe_hint(k));
        }
        debounce(ui, "keyframe", 400, move |ui| {
            settings::handle_set_recording_keyframe(k);
            render_tab(ui);
        });
    }

    /// Começa o teste de qualidade (5 s de amostra).
    pub fn test_run(ui: &mut Ui) {
        if with(|s| s.vt.running) {
            return;
        }
        with(|s| {
            s.vt.running = true;
            s.vt.capturing = true;
            s.vt.countdown = 5;
            s.vt.pct = 0;
            s.vt.level = -1;
        });
        render_tab(ui);
        video_test::handle_test_video_quality();
    }

    pub fn test_cancel() {
        video_test::handle_cancel_video_test();
    }

    pub fn test_swap(ui: &mut Ui) {
        with(|s| std::mem::swap(&mut s.vt.a, &mut s.vt.b));
        render_tab(ui);
    }

    /// O zoom da comparação (0: ajustar; 1 a 3: 100% a 300%), do centro.
    pub fn test_zoom(ui: &mut Ui, zoom: i32) {
        with(|s| {
            s.vt.zoom = zoom;
            s.vt.pan = (0.0, 0.0);
        });
        render_tab(ui);
    }

    pub fn test_use(ui: &mut Ui) {
        vt_use(ui);
    }

    /// Perto da divisória (ou sem zoom), arrastar move a divisória; senão, desloca a imagem.
    pub fn stage_down(args: &EventArgs) {
        let EventArgs::Pointer(e) = args else { return };
        with(|s| {
            let Some(geom) = s.vt.geom else { return };
            let cut = 1.0 + geom.w * s.vt.split;
            s.vt.drag = Some((e.position.x, e.position.y, s.vt.pan, (e.position.x - cut).abs() < 16.0 || s.vt.zoom == 0));
        });
    }

    pub fn stage_move(ui: &mut Ui, args: &EventArgs) {
        let EventArgs::Pointer(e) = args else { return };
        let Some(((x0, y0, pan0, splitting), geom)) = with(|s| s.vt.drag.zip(s.vt.geom)) else { return };
        with(|s| {
            if splitting {
                s.vt.split = ((e.position.x - 1.0) / geom.w).clamp(0.0, 1.0);
            } else {
                s.vt.pan = geom.limit((pan0.0 + e.position.x - x0, pan0.1 + e.position.y - y0));
            }
        });
        arrange_compare(ui);
    }

    pub fn stage_up() {
        with(|s| s.vt.drag = None);
    }
}

// ---------------------------------------------------------------------------
// Áudio (`forms/settings/audio.json`)
// ---------------------------------------------------------------------------

fn load_audio(ui: &mut Ui) {
    let ab = encoder::audio_bitrate_kbps();
    let ai = AUDIO_BITRATES.iter().position(|k| *k == ab).unwrap_or(3);
    let labels: Vec<String> = AUDIO_BITRATES
        .iter()
        .map(|k| {
            let key = if *k == AUDIO_BITRATE_DEFAULT { "{{kbps}} kbps (padrão)" } else { "{{kbps}} kbps" };
            tw(key, &[("kbps", &k.to_string())])
        })
        .collect();
    fitems(ui, FraAudio::components(ui).map(|frm| frm.cmbAudioBitrate), &labels, ai);
    if let Some(frm) = FraAudio::components(ui) {
        frm.lblAudioBitrateHint.set_text(ui, audio_hint(AUDIO_BITRATES[ai]));
        frm.chkMuteWhenMuted.set_checked(ui, config::get_config_bool("muteWhenDeviceMuted", true));
    }
}

/// Os eventos do frame da aba Áudio (ligados em `forms/audio.rs`, que lê os valores).
pub mod audio {
    use super::*;

    /// A taxa escolhida (a posição em `AUDIO_BITRATES`).
    pub fn bitrate(ui: &mut Ui, i: usize) {
        settings::handle_set_audio_bitrate(AUDIO_BITRATES.get(i).copied().unwrap_or(AUDIO_BITRATE_DEFAULT));
        ui.set_timeout(0, render_tab);
    }

    pub fn mute_when_muted(ui: &mut Ui, on: bool) {
        check_changed(ui, "muteWhenDeviceMuted", on);
    }
}

// ---------------------------------------------------------------------------
// Buffer (`forms/settings/replay.json`)
// ---------------------------------------------------------------------------

/// Os limites do buffer: os tempos e as memórias oferecidos (com os atuais incluídos) e
/// o escolhido de cada um. A memória vai até o teto que o core calculou pela RAM.
fn replay_limits(r: &ReplayState) -> (Vec<i64>, usize, Vec<i64>, usize, i64) {
    let limit = if r.max_mb_limit > 0 { r.max_mb_limit } else { REPLAY_DEFAULT_MB };
    let (secs, si) = with_current(REPLAY_SEC_OPTIONS.to_vec(), if r.max_sec > 0 { r.max_sec } else { REPLAY_DEFAULT_SEC });
    let mut mbs: Vec<i64> = REPLAY_MB_OPTIONS.iter().copied().filter(|v| *v < limit).collect();
    mbs.push(limit);
    let (mbs, mi) = with_current(mbs, if r.max_mb > 0 { r.max_mb } else { REPLAY_DEFAULT_MB });
    (secs, si, mbs, mi, limit)
}

fn load_replay(ui: &mut Ui) {
    let Some(frm) = FraReplay::components(ui) else { return };
    let r = with(|s| s.replay.clone());
    frm.lyReplayOptions.set_visible(ui, r.is_some());
    let Some(r) = r else { return };
    frm.chkReplayAutoStart.set_checked(ui, r.auto_start);
    frm.chkReplayIntoRecording.set_checked(ui, r.into_recording);
    // Duas colunas: tempo e memória.
    let half = (col_w() - 10.0) / 2.0;
    frm.lblReplayDuration.set_width(ui, f64::from(half + 10.0));
    frm.cmbReplaySeconds.set_width(ui, f64::from(half));
    let (secs, si, mbs, mi, limit) = replay_limits(&r);
    let labels: Vec<String> = secs.iter().map(|v| sec_label(*v)).collect();
    fitems(ui, Some(frm.cmbReplaySeconds), &labels, si);
    let labels: Vec<String> = mbs.iter().map(|v| if *v >= limit { tw("Máximo ({{size}})", &[("size", &mb_label(*v))]) } else { mb_label(*v) }).collect();
    fitems(ui, Some(frm.cmbReplayMemory), &labels, mi);
    frm.edtReplayApps.set_text(ui, r.apps.as_str());
    frm.chkReplayIndicator.set_checked(ui, r.indicator);
    frm.lyRepOptions.set_visible(ui, r.indicator);
    if r.indicator {
        frm.chkRepClickable.set_checked(ui, r.indicator_clickable);
        let op = if r.indicator_opacity > 0 { r.indicator_opacity as i32 } else { 90 };
        load_indicator(ui, replay_indicator(ui), &r.indicator_corner, op);
    }
}

/// Os eventos do frame da aba Buffer (ligados em `forms/replay.rs`, que lê os valores).
pub mod replay_tab {
    use super::*;

    /// Uma chave de liga/desliga: `key` é a da configuração.
    pub fn check(ui: &mut Ui, key: &str, on: bool) {
        check_changed(ui, key, on);
    }

    /// Tempo ou memória (as posições escolhidas nos dois): o core recebe os dois.
    pub fn limits(si: usize, mi: usize) {
        let Some(r) = with(|s| s.replay.clone()) else { return };
        let (secs, _, mbs, _, _) = replay_limits(&r);
        let sec = secs.get(si).copied().unwrap_or(REPLAY_DEFAULT_SEC);
        let mb = mbs.get(mi).copied().unwrap_or(REPLAY_DEFAULT_MB);
        replay::handle_set_replay_limits(sec as i32, mb as i32);
    }

    /// Cada envio reinicia o vigia de programas no core: com atraso enquanto digita.
    pub fn apps_edit(ui: &mut Ui, t: String) {
        debounce(ui, "replay_apps", 600, move |_| replay::handle_set_replay_apps(&t));
    }

    pub fn apps(t: &str) {
        let cur = with(|s| s.replay.as_ref().map(|r| r.apps.clone())).unwrap_or_default();
        if t != cur {
            replay::handle_set_replay_apps(t);
        }
    }

    /// O canto escolhido (a posição em `CORNERS`).
    pub fn corner(i: usize) {
        replay::handle_set_replay_indicator_corner(CORNERS.get(i).copied().unwrap_or("top-right"));
    }

    pub fn opacity(ui: &mut Ui, step: usize) {
        indicator_opacity(ui, replay_indicator(ui), step, replay::handle_set_replay_indicator_opacity);
    }
}

// ---------------------------------------------------------------------------
// Atalhos (`forms/settings/hotkeys.json`)
// ---------------------------------------------------------------------------

fn load_hotkeys(ui: &mut Ui) {
    fit_button(ui, FraHotkeys::components(ui).map(|frm| frm.btnHotkeyAdd));
    if let Some(frm) = FraHotkeys::components(ui) {
        hotkey_cards(ui, &frm);
    }
}

/// Os eventos do frame da aba Atalhos (ligados em `forms/hotkeys.rs`).
pub mod hotkeys_tab {
    use super::*;

    pub fn add() {
        let mut items = with(|s| s.hk_items.clone());
        items.push(Item::new(Mode::Record, ""));
        hk_send(items);
    }
}

// ---------------------------------------------------------------------------
// Dispositivos (`forms/settings/devices.json`)
// ---------------------------------------------------------------------------

fn load_devices(ui: &mut Ui) {
    let Some(frm) = FraDevices::components(ui) else { return };
    device_groups(ui, frm.lyDevices.id, frm.lblDevicesEmpty, |d| Some(!d.hidden), |_, id, on| {
        set_device_flag(id, |d| d.hidden = !on);
        crate::bridge::devices::handle_set_source_hidden(id, !on);
    });
}

// ---------------------------------------------------------------------------
// Transcrição (`forms/settings/transcribe.json`)
// ---------------------------------------------------------------------------

fn load_transcribe(ui: &mut Ui) {
    // O quadro do motor local.
    let (status, detail, btn, danger, progress) = asr_texts();
    let kind = with(|s| s.asr.as_ref().map(|a| a.status.clone())).unwrap_or_default();
    let style = match kind.as_str() {
        "ready" => "s_status_ok",
        "error" => "s_status_fail",
        _ => "s_status",
    };
    if let Some(frm) = FraTranscribe::components(ui) {
        frm.lblTrStatus.set_text(ui, status.as_str());
        frm.lblTrStatus.set_style(ui, style);
        frm.prgTrInstall.set_visible(ui, progress >= 0.0);
        frm.prgTrInstall.set_value(ui, (progress.max(0.0) * 100.0) as f64);
        frm.lblTrDetail.set_text(ui, detail.as_str());
        frm.btnTrEngine.set_text(ui, btn.as_str());
        frm.btnTrEngine.set_style(ui, if danger { "s_danger" } else { "s" });
    }
    fit_button(ui, FraTranscribe::components(ui).map(|frm| frm.btnTrEngine));
    let (_, labels, li) = transcribe_langs();
    fitems(ui, FraTranscribe::components(ui).map(|frm| frm.cmbTrLanguage), &labels, li);
    if let Some(frm) = FraTranscribe::components(ui) {
        frm.chkTrOnStop.set_checked(ui, config::get_config_bool("transcribeOnStop", true));
    }
    // A fila: pendentes, cancelar, o progresso, o erro e as que esperam.
    let (t, pending, cloud, queue, clicked) = with(|s| (s.tr.clone(), s.tr_pending, s.tr_cloud, s.tr_queue.clone(), s.tr_cancel_clicked));
    let canceling = t.canceling || clicked;
    let pending_label = if pending > 0 { tw("Transcrever pendentes ({{count}})", &[("count", &pending.to_string())]) } else { lang::tr("Nada pendente") };
    if let Some(frm) = FraTranscribe::components(ui) {
        frm.btnTrPending.set_text(ui, pending_label);
        frm.btnTrPending.set_enabled(ui, pending > 0);
    }
    fit_button(ui, FraTranscribe::components(ui).map(|frm| frm.btnTrPending));
    if let Some(frm) = FraTranscribe::components(ui) {
        frm.btnTrCancel.set_text(ui, lang::tr(if canceling { "Cancelando…" } else { "Cancelar" }));
        frm.btnTrCancel.set_enabled(ui, !canceling && (t.running || t.queue > 0));
    }
    fit_button(ui, FraTranscribe::components(ui).map(|frm| frm.btnTrCancel));
    let (visible, p, line) = transcribe_line();
    if let Some(frm) = FraTranscribe::components(ui) {
        frm.lyTrProgress.set_visible(ui, visible);
        frm.prgTrQueue.set_value(ui, (p * 100.0) as f64);
        frm.lblTrLine.set_text(ui, line);
    }
    let err = if t.last_error.is_empty() {
        String::new()
    } else if t.last_error_name.is_empty() {
        tw("Motivo da falha: {{error}}", &[("error", &t.last_error)])
    } else {
        tw("Falha em \"{{name}}\": {{error}}", &[("name", &t.last_error_name), ("error", &t.last_error)])
    };
    ftext_or_hide(ui, FraTranscribe::components(ui).map(|frm| frm.lblTrError), &err);
    if let Some(frm) = FraTranscribe::components(ui) {
        frm.lyTrQueue.set_visible(ui, !queue.is_empty());
    }
    if let Some(frm) = FraTranscribe::components(ui) {
        transcribe_queue(ui, frm.pnlTrQueue.id, &queue);
    }
    let cloud_text = if cloud > 0 { tw("{{count}} gravação(ões) ficaram de fora: estão só na nuvem. Transcrever baixaria os arquivos, então o lote pula — abra a gravação para baixá-la, ou transcreva uma a uma pelo menu dela.", &[("count", &cloud.to_string())]) } else { String::new() };
    ftext_or_hide(ui, FraTranscribe::components(ui).map(|frm| frm.lblTrCloud), &cloud_text);
}

/// Os eventos do frame da aba Transcrição (ligados em `forms/transcribe.rs`, que lê os valores).
pub mod transcribe_tab {
    use super::*;

    /// Instalar, cancelar a instalação, tentar de novo ou remover o motor local.
    pub fn engine(ui: &mut Ui) {
        asr_action(ui);
    }

    /// O idioma escolhido (a posição na lista de `transcribe_langs`).
    pub fn language(i: usize) {
        let (vals, _, _) = transcribe_langs();
        btr::handle_set_transcribe_language(vals.get(i).map_or("app", |s| s.as_str()));
    }

    pub fn on_stop(ui: &mut Ui, on: bool) {
        check_changed(ui, "transcribeOnStop", on);
    }

    pub fn pending() {
        btr::handle_transcribe_pending();
    }

    pub fn cancel(ui: &mut Ui) {
        if with(|s| s.tr.canceling || s.tr_cancel_clicked) {
            return;
        }
        with(|s| s.tr_cancel_clicked = true);
        render_tab(ui);
        btr::handle_cancel_transcribe();
    }
}

// ---------------------------------------------------------------------------
// Atualizações (`forms/settings/updates.json`)
// ---------------------------------------------------------------------------

fn load_updates(ui: &mut Ui) {
    if let Some(frm) = FraUpdates::components(ui) {
        frm.lblVersionValue.set_text(ui, crate::version::display_version());
        frm.chkCheckUpdates.set_checked(ui, config::get_config_bool("checkUpdates", true));
        frm.btnCheckNow.set_enabled(ui, !with(|s| s.checking));
    }
    fit_button(ui, FraUpdates::components(ui).map(|frm| frm.btnCheckNow));
    let (status, has_url) = update_status();
    let available = with(|s| s.update.as_ref().is_some_and(|u| u.ok && u.has_update));
    if let Some(frm) = FraUpdates::components(ui) {
        frm.lyUpdateStatus.set_visible(ui, !status.is_empty());
        frm.lblUpdateStatus.set_text(ui, status.as_str());
        frm.lblUpdateStatus.set_style(ui, if available { "s_ok" } else { "s_hint" });
    }
    fit_label(ui, FraUpdates::components(ui).map(|frm| frm.lblUpdateStatus), 0.0);
    if let Some(frm) = FraUpdates::components(ui) {
        frm.lblUpdateOpen.set_visible(ui, has_url);
    }
}

/// Os eventos do frame da aba Atualizações (ligados em `forms/updates.rs`, que lê os valores).
pub mod updates {
    use super::*;

    pub fn auto(ui: &mut Ui, on: bool) {
        check_changed(ui, "checkUpdates", on);
    }

    pub fn check_now(ui: &mut Ui) {
        with(|s| s.checking = true);
        render_tab(ui);
        settings::handle_check_updates();
    }

    pub fn open_page() {
        let url = with(|s| s.update.as_ref().map(|u| u.url.clone())).unwrap_or_default();
        if !url.is_empty() {
            bridge::handle_open_url(&url);
        }
    }
}

// ---------------------------------------------------------------------------
// Sobre (`forms/settings/about.json`)
// ---------------------------------------------------------------------------

fn load_about(ui: &mut Ui) {
    // O endereço do repositório (o mesmo das atualizações).
    if let Some(frm) = FraAbout::components(ui) {
        frm.lblAboutRepositoryValue.set_text(ui, format!("github.com/{}", crate::update::REPOSITORY));
    }
    if let Some(v) = FraAbout::components(ui).map(|frm| frm.imgAbout.id) {
        if let Some((iw, ih, rgba)) = crate::win::main_icon_rgba(96) {
            if let Some(iv) = ui.get_mut::<ImageView>(v) {
                iv.set_picture(Some(Image::rgba(iw, ih, rgba)));
            }
        }
    }
    // Os textos com <b> e <code> do idioma.
    if let Some(frm) = FraAbout::components(ui) {
        for l in [frm.lblAboutIntro1, frm.lblAboutIntro2, frm.lblAboutIntro3] {
            l.set_markup(ui, true);
        }
    }
    fit_label(ui, FraAbout::components(ui).map(|frm| frm.lblAboutCoAuthorValue), 0.0);
    // As listas (recursos e com o que foi feito), do idioma; os títulos estão no JSON.
    if let Some(frm) = FraAbout::components(ui) {
        about_list(ui, frm.lyAboutFeatures.id, &lang_list("about.features"));
        about_list(ui, frm.lyAboutBuiltWith.id, &lang_list("about.builtWith"));
    }
}

/// Os eventos do frame da aba Sobre (ligados em `forms/about.rs`).
pub mod about {
    pub fn repository() {
        crate::bridge::handle_open_url(&crate::update::repository_url());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn app() -> aylis::Application {
        let mut app = aylis::Application::bundled(crate::PROJECT).unwrap();
        crate::generated::register(&mut app.ui);
        app
    }

    /// Um frame de aba criado sozinho, na largura da coluna (o layout dá os tamanhos).
    fn tab_form(ui: &mut Ui, name: &str) {
        let form = ui.create_form(name).unwrap();
        if let Some(cb) = ui.control_mut(form) {
            cb.set_rect(0.0, 0.0, 796.0, 4000.0).set_padding([18.0, 24.0, 18.0, 14.0]);
        }
    }

    /// Os níveis de um teste de qualidade com as imagens em `dir`.
    fn video_result(dir: &std::path::Path) -> VideoTestState {
        std::fs::create_dir_all(dir).unwrap();
        let levels = (1..=2)
            .map(|level| {
                let image = dir.join(format!("q{level}.png"));
                std::fs::write(&image, b"png").unwrap();
                crate::view::VideoTestLevel { level, bytes: 1_000_000 * level as i64, image: image.to_string_lossy().into(), ..Default::default() }
            })
            .collect();
        VideoTestState { width: 1920, height: 1080, duration: 5.0, current_level: 1, levels, ..Default::default() }
    }

    /// O "Ajustar" depois de arrastar a imagem com zoom: o deslocamento não vale e a imagem
    /// fica no quadro (antes saía dele e ficava tudo preto).
    #[test]
    fn fit_ignores_the_pan_of_a_zoom() {
        let mut app = app();
        let ui = &mut app.ui;
        tab_form(ui, "fraVideo");
        let frm = FraVideo::components(ui).unwrap();
        let dir = std::env::temp_dir().join(format!("noobs_compare_{}", std::process::id()));
        let r = video_result(&dir);
        // Arrastou a 300% para longe e voltou ao "Ajustar".
        with(|s| s.vt.pan = (5000.0, 5000.0));
        compare_view(ui, &frm, &r, 1, 2, 0);
        assert!(frm.pnlVtStage.visible(ui) && frm.lyVtB.visible(ui));
        let (x, y) = ui.bounds(frm.imgVtA.id).map(|b| (b.x, b.y)).unwrap();
        let stage_w = col_w();
        assert!(x >= 0.0 && x < stage_w && y >= 0.0 && y < 420.0, "a imagem fora do quadro: {x}, {y}");
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// As listas das abas são frames de `forms/settings/parts/` empilhados no contêiner da
    /// aba, com só o que varia vindo do código; montar de novo troca os frames (não soma).
    #[test]
    fn the_tab_lists_are_frames() {
        let mut app = app();
        let ui = &mut app.ui;
        for name in ["fraDevices", "fraFiles", "fraHotkeys", "fraTranscribe", "fraAbout"] {
            tab_form(ui, name);
        }
        // Dispositivos: um grupo por tipo (só os que têm algum), uma linha por dispositivo.
        with(|s| {
            s.monitors = vec![Device { id: "m1".into(), name: "Monitor 1".into(), info: "1920×1080".into(), ..Default::default() }];
            s.mics = vec![Device { id: "a".into(), name: "Mic 10".into(), ..Default::default() }, Device { id: "b".into(), name: "Mic 2".into(), hidden: true, ..Default::default() }];
        });
        for _ in 0..2 {
            load_devices(ui);
        }
        let devices = FraDevices::components(ui).unwrap();
        let groups: Vec<_> = ui.children(devices.lyDevices.id).iter().map(|&c| g::FraDeviceGroup::components_of(ui, c).unwrap()).collect();
        assert_eq!(groups.len(), 2);
        assert!(!devices.lblDevicesEmpty.visible(ui));
        let rows = |ui: &Ui, group: &g::FraDeviceGroupComponents| -> Vec<g::FraDeviceRowComponents> { ui.children(group.lyRows.id).iter().map(|&c| FraDeviceRow::components_of(ui, c).unwrap()).collect() };
        let monitors = rows(ui, &groups[0]);
        assert_eq!(groups[0].lblGroup.text(ui), lang::tr_of("devices.groups", "monitors"));
        assert_eq!((monitors.len(), monitors[0].lblInfo.text(ui).as_str(), monitors[0].lblInfo.visible(ui)), (1, "1920×1080", true));
        // A ordem natural ("Mic 2" antes de "Mic 10"); o oculto vem desmarcado.
        let mics = rows(ui, &groups[1]);
        assert_eq!((mics[0].lblName.text(ui).as_str(), mics[0].chkOn.checked(ui)), ("Mic 2", false));
        assert_eq!((mics[1].lblName.text(ui).as_str(), mics[1].chkOn.checked(ui), mics[1].lblInfo.visible(ui)), ("Mic 10", true, false));
        // O 1º grupo a 10 px do texto de cima, o 2º a 14.
        let top = |ui: &Ui, id: ComponentId| ui.control(id).unwrap().margins().top;
        assert_eq!((top(ui, groups[0].form.id), top(ui, groups[1].form.id)), (10.0, 14.0));
        with(|s| {
            s.monitors.clear();
            s.mics.clear();
        });
        load_devices(ui);
        assert!(ui.children(devices.lyDevices.id).is_empty() && devices.lblDevicesEmpty.visible(ui));

        // Pastas ocultas: uma linha por pasta, a 6 px uma da outra.
        with(|s| s.folders = vec![RootFolder { name: "NVIDIA".into(), hidden: true, count: 3, ..Default::default() }, RootFolder { name: "Velha".into(), missing: true, ..Default::default() }]);
        let files = FraFiles::components(ui).unwrap();
        hidden_folders(ui, &files);
        hidden_folders(ui, &files);
        let folders: Vec<_> = ui.children(files.lyHiddenFolders.id).iter().map(|&c| g::FraHiddenFolder::components_of(ui, c).unwrap()).collect();
        assert_eq!(folders.len(), 2);
        assert!(folders[0].chkHidden.checked(ui) && !folders[1].chkHidden.checked(ui));
        assert_eq!(folders[0].chkHidden.text(ui), "NVIDIA");
        assert_eq!(folders[0].lblCount.text(ui), tw("{{count}} gravações", &[("count", "3")]));
        assert_eq!((top(ui, folders[0].form.id), top(ui, folders[1].form.id)), (0.0, FIELD_GAP));
        assert!(!files.lblHiddenFoldersEmpty.visible(ui));

        // Atalhos: um cartão por atalho, com as linhas do modo e da altura delas.
        let mut window = Item::new(Mode::Window, "Ctrl+F9");
        window.title = "nova guia".into();
        with(|s| {
            s.hk_items = vec![Item::new(Mode::Record, "Ctrl+Shift+F10"), window, Item::new(Mode::Area, ""), Item::new(Mode::Replay, "Alt+F10")];
            s.hk_status = vec!["", "", "empty", ""];
        });
        let hotkeys = FraHotkeys::components(ui).unwrap();
        hotkey_cards(ui, &hotkeys);
        hotkey_cards(ui, &hotkeys);
        ui.layout();
        let cards: Vec<_> = ui.children(hotkeys.lyHotkeys.id).iter().map(|&c| g::FraHotkeyCard::components_of(ui, c).unwrap()).collect();
        assert_eq!(cards.len(), 4);
        assert!(!hotkeys.lblHotkeysEmpty.visible(ui));
        let rec = cards[0];
        assert!(rec.chkCtrl.checked(ui) && rec.chkShift.checked(ui) && !rec.chkAlt.checked(ui));
        assert_eq!(rec.pnlCtrl.style(ui), "s_mod_on");
        assert_eq!(rec.cmbKey.item_index(ui), key_table().iter().filter(|k| !k.2).position(|k| k.0 == "F10").unwrap() as i64 + 1);
        assert!(!rec.lyWindow.visible(ui) && !rec.lyArea.visible(ui) && rec.lyDelay.visible(ui) && !rec.lyStatus.visible(ui));
        // Gravar as telas: as teclas (32) e o atraso (10 + 32), com o respiro de 10 + 10.
        let height = |ui: &mut Ui, id: ComponentId| ui.bounds(id).unwrap().height;
        assert_eq!(height(ui, rec.form.id), 94.0);
        assert!(cards[1].lyWindow.visible(ui) && cards[1].lyAppAudio.visible(ui));
        assert_eq!(cards[1].edtWindowTitle.text(ui), "nova guia");
        assert_eq!(height(ui, cards[1].form.id), 94.0 + 42.0 + 27.0);
        assert!(cards[2].lyArea.visible(ui) && cards[2].lyTimelapse.visible(ui) && cards[2].lyStatus.visible(ui) && !cards[2].btnAreaClear.visible(ui));
        assert_eq!(cards[2].lblStatus.style(ui), "s_err");
        assert!(!cards[3].lyDelay.visible(ui));
        assert_eq!(height(ui, cards[3].form.id), 52.0);
        assert_eq!((top(ui, cards[0].form.id), top(ui, cards[1].form.id)), (12.0, 10.0));

        // A fila da transcrição: a em curso no alto; as que esperam se arrastam.
        let queue = [QueueItem { id: "1".into(), name: "Reunião".into(), duration: 75, current: true }, QueueItem { id: "2".into(), name: "Aula".into(), duration: 0, current: false }];
        let tr = FraTranscribe::components(ui).unwrap();
        transcribe_queue(ui, tr.pnlTrQueue.id, &queue);
        transcribe_queue(ui, tr.pnlTrQueue.id, &queue);
        ui.layout();
        let items: Vec<_> = ui.children(tr.pnlTrQueue.id).iter().map(|&c| g::FraQueueRow::components_of(ui, c).unwrap()).collect();
        assert_eq!(items.len(), 2);
        assert!(items[0].lblNow.visible(ui) && !items[0].btnRemove.visible(ui) && !items[0].shpLine.visible(ui));
        assert_eq!((items[0].pnlRow.style(ui), items[0].lblDuration.text(ui)), ("s_q_cur".to_owned(), "1:15".to_owned()));
        assert!(!items[1].lblNow.visible(ui) && items[1].btnRemove.visible(ui) && items[1].shpLine.visible(ui) && items[1].pnlRow.draggable(ui));
        assert_eq!(items[1].lblDuration.text(ui), "—");
        // O quadro: as linhas de 30 e a borda de 1.
        assert_eq!(height(ui, tr.pnlTrQueue.id), 62.0);

        // Sobre: as duas listas do idioma, cada item com a altura das linhas dele.
        let about = FraAbout::components(ui).unwrap();
        about_list(ui, about.lyAboutFeatures.id, &lang_list("about.features"));
        about_list(ui, about.lyAboutFeatures.id, &lang_list("about.features"));
        about_list(ui, about.lyAboutBuiltWith.id, &lang_list("about.builtWith"));
        assert_eq!(ui.children(about.lyAboutFeatures.id).len(), lang_list("about.features").len());
        let built: Vec<_> = ui.children(about.lyAboutBuiltWith.id).iter().map(|&c| g::FraAboutItem::components_of(ui, c).unwrap()).collect();
        assert_eq!(built.len(), 4);
        assert_eq!(built[0].lblText.text(ui), lang::tr_of("about.builtWith", "0"));
        assert!(built[0].form.height(ui) > 0.0);
    }

    /// O teste de qualidade: o que é fixo está no JSON da aba Vídeo e aparece conforme o
    /// estado; os níveis são `fraVtLevel`, e as marcas do FPS, `fraSliderTick`.
    #[test]
    fn the_video_test_shows_the_json_parts_by_state() {
        let mut app = app();
        let ui = &mut app.ui;
        tab_form(ui, "fraVideo");
        let frm = FraVideo::components(ui).unwrap();
        // Gravando a amostra: o andamento em vermelho, sem a barra nem o resultado.
        with(|s| {
            s.vt.running = true;
            s.vt.capturing = true;
            s.vt.countdown = 4;
        });
        video_test_box(ui, &frm);
        assert!(frm.lblVtStatus.visible(ui) && !frm.prgVt.visible(ui) && !frm.lyVtResult.visible(ui) && frm.btnVtCancel.visible(ui));
        assert!(!frm.btnVtRun.enabled(ui));
        assert_eq!(frm.lblVtStatus.style(ui), "s_err");
        // Pronto: o resultado com um `fraVtLevel` por nível (o maior em cima).
        let dir = std::env::temp_dir().join(format!("noobs_vt_{}", std::process::id()));
        let r = video_result(&dir);
        with(|s| {
            s.vt.running = false;
            s.vt.capturing = false;
            s.vt.a = 1;
            s.vt.b = 2;
            s.vt.result = Some(r);
        });
        video_test_box(ui, &frm);
        video_test_box(ui, &frm);
        assert!(!frm.lblVtStatus.visible(ui) && !frm.prgVt.visible(ui) && frm.lyVtResult.visible(ui) && !frm.btnVtCancel.visible(ui));
        assert_eq!(frm.btnVtRun.text(ui), lang::tr("Testar de novo"));
        let levels: Vec<_> = ui.children(frm.lyVtLevels.id).iter().map(|&c| g::FraVtLevel::components_of(ui, c).unwrap()).collect();
        assert_eq!(levels.len(), 2);
        assert_eq!(levels[0].lblLevel.text(ui), tw("Nível {{n}}", &[("n", "2")]));
        assert_eq!((levels[0].pnlLevel.style(ui), levels[1].pnlLevel.style(ui)), ("s_vt_b".to_owned(), "s_vt_a".to_owned()));
        assert_eq!((levels[1].lblRelative.text(ui), levels[1].pnlBarFill.style(ui)), (lang::tr("atual"), "s_bar_on".to_owned()));
        // A barra do maior enche a trilha; a do atual, a metade.
        let w = |ui: &Ui, id: ComponentId| ui.control(id).unwrap().width();
        let track = ui.bounds(levels[0].pnlBar.id).unwrap().width;
        assert!(track > 0.0 && (w(ui, levels[0].pnlBarFill.id) - track).abs() < 0.01 && (w(ui, levels[1].pnlBarFill.id) - track / 2.0).abs() < 0.01);
        // O "Usar o nível 1" desligado: já é o atual.
        assert!(!frm.btnVtUse.enabled(ui));
        assert_eq!(frm.btnVtZoomFit.style(ui), "s_on");
        assert_eq!(frm.lblVtTagA.text(ui), tw("Nível {{n}}", &[("n", "1")]));
        let _ = std::fs::remove_dir_all(&dir);
        // As marcas do FPS mudam de número: um `fraSliderTick` para cada, sem acumular.
        let fps = Slider { track: frm.trkFps, low: frm.lblFpsLow, high: frm.lblFpsHigh, ticks: frm.lyFpsTicks };
        let ticks: Vec<String> = ["20", "30", "60"].iter().map(|t| t.to_string()).collect();
        load_slider(ui, Some(fps), 2, 1, Ticks::Made(&ticks));
        load_slider(ui, Some(fps), 2, 1, Ticks::Made(&ticks));
        let marks: Vec<_> = ui.children(frm.lyFpsTicks.id).iter().filter_map(|&c| g::FraSliderTick::components_of(ui, c)).collect();
        assert_eq!(marks.len(), 3);
        assert_eq!(marks[2].lblTick.text(ui), "60");
        assert_eq!(frm.trkFps.value(ui), 1.0);
        // As da qualidade são as do JSON: só mudam de lugar.
        let quality = Slider { track: frm.trkQuality, low: frm.lblQualityLow, high: frm.lblQualityHigh, ticks: frm.lyQualityTicks };
        load_slider(ui, Some(quality), 10, 5, Ticks::Fixed(&[frm.lblQualityTick0, frm.lblQualityTick10]));
        assert_eq!(ui.children(frm.lyQualityTicks.id).len(), 12);
        let x = |ui: &mut Ui, id: ComponentId| ui.bounds(id).unwrap().x;
        assert!(x(ui, frm.lblQualityTick10.id) > x(ui, frm.lblQualityTick0.id));
    }
}
