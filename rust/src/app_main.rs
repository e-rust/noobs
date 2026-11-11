//! Tela principal (coluna de fontes + bloco "Telas"): o cartão de gravar, o
//! buffer em memória, microfones e saídas com medidor, a legenda das faixas e
//! as telas com prévia. O desenho é o form `frmMain` (`forms/main.json`); aqui
//! entra o que o core conta pelo [`crate::view::View`] e os cliques vão direto
//! pros handlers do core.
//!
//! Só a main thread toca aqui, sempre com o `Ui` (pelo `app_view::with_ui`).

use crate::generated::{Circle, FraLegendRow, FraSourceRow, FrmMain};
use std::cell::RefCell;
use std::collections::HashMap;
use std::time::{Duration, Instant};

use aylis::prelude::*;

use crate::app_view::{main_form, with_ui};
use crate::view::{AudioMeter, Device, MonitorThumb, PendingKind, RecState, ReplayState};
use crate::{bridge, config, lang};

/// Cor de cada faixa isolada (2, 3, …), em ciclo. Sem verde/vermelho (a bolinha
/// de padrão, gravar/parar). As dez ficam na paleta do projeto (`track_1`…`track_10`).
const TRACK_COLORS: usize = 10;

/// Uma cor da paleta do projeto (`NoOBS.aylisproj`) ou do tema, pelo nome.
fn pal(ui: &Ui, name: &str) -> Color {
    ui.theme().color(name)
}

fn track_color(ui: &Ui, t: i32) -> Color {
    pal(ui, &format!("track_{}", (t - 2).max(0) as usize % TRACK_COLORS + 1))
}

/// Amplitude linear 0..1 → largura 0..1 em dB (−60 dB = 0, 0 dB = 1).
fn amp_to_bar(v: f32) -> f32 {
    if v <= 0.0 {
        return 0.0;
    }
    ((20.0 * v.log10() + 60.0) / 60.0).clamp(0.0, 1.0)
}

/// Altura de uma linha de dispositivo e de uma da legenda.
const SOURCE_ROW: f32 = 45.0;
const LEGEND_ROW: f32 = 22.0;

#[derive(Default)]
struct State {
    monitors: Vec<Device>,
    webcams: Vec<Device>,
    mics: Vec<Device>,
    speakers: Vec<Device>,
    /// O medidor de cada dispositivo de áudio (id do dispositivo → componente).
    meters: HashMap<String, ComponentId>,
    rec: RecState,
    /// `None` antes do 1º estado (o som não toca no arranque).
    last_active: Option<bool>,
    /// Quando o cronômetro marcaria zero; o número do `set_timeout` dele.
    timer_start: Option<Instant>,
    timer_job: Option<u64>,
    finalizing: bool,
    pause_locked: bool,
    last_stop_sound: Option<Instant>,
    replay: ReplayState,
    replay_received: Option<Instant>,
    replay_job: Option<u64>,
    countdown_sec: u32,
    countdown_end: Option<Instant>,
    countdown_total: f32,
    countdown_job: Option<u64>,
    /// A onda vermelha: se o anel e o brilho já subiram para cima de tudo, e o tique.
    ripple_raised: bool,
    ripple_job: Option<u64>,
}

thread_local! {
    static ST: RefCell<State> = RefCell::new(State::default());
}

fn with<R>(f: impl FnOnce(&mut State) -> R) -> R {
    ST.with(|s| f(&mut s.borrow_mut()))
}

/// Os textos fixos estão no form (o Aylis traduz); aqui só o que depende deles.
fn apply_texts(ui: &mut Ui) {
    if let Some(id) = FrmMain::components(ui).map(|frm| frm.lblScreens.id) {
        let w = ui.measure_text(&lang::tr("Telas"), &TextStyle::default()).width;
        if let Some(cb) = ui.control_mut(id) {
            cb.set_width(w * 16.0 / 14.0 + 4.0);
        }
    }
}

/// Liga a tela principal: textos, estado inicial. Uma vez, depois do form criado.
pub fn setup(ui: &mut Ui) {
    apply_texts(ui);
    render_recording(ui);
    render_replay(ui);
}

// ---------------------------------------------------------------------------
// Dispositivos
// ---------------------------------------------------------------------------

fn visible(list: &[Device]) -> impl Iterator<Item = &Device> {
    list.iter().filter(|d| !d.hidden)
}

fn count_text(list: &[Device]) -> String {
    let total = visible(list).count();
    let sel = visible(list).filter(|d| d.enabled).count();
    format!("{sel} / {total}")
}

/// Uma linha de dispositivo de áudio: a faixa colorida na borda, a bolinha do
/// padrão, o nome, o medidor e a chave. A linha inteira liga/desliga.
///
/// A linha é o frame `fraSourceRow` (forms/main/source_row.json), com o estilo,
/// as medidas e os avisos fixos; aqui só o que muda por dispositivo.
fn add_source_row(ui: &mut Ui, parent: ComponentId, d: &Device, locked: bool, meters: &mut HashMap<String, ComponentId>) {
    let Ok(row) = FraSourceRow::create_in(ui, parent) else { return };
    if locked {
        // Gravando, a linha trava: meio apagada e sem a mãozinha.
        row.pnlRow.set_cursor(ui, "default");
        row.pnlRow.set_opacity(ui, 0.5);
        row.swOn.set_enabled(ui, false);
    }
    // Faixa de 3 px colada na borda da janela (12 px pra fora da linha).
    let color = if d.enabled && d.track > 1 {
        Some(track_color(ui, d.track))
    } else if d.enabled {
        None
    } else {
        Some(ui.theme().colors.get("border_2").copied().unwrap_or(ui.theme().color("border")))
    };
    match color {
        Some(color) => row.shpStripe.set_fill(ui, color.with_opacity(if d.enabled { 1.0 } else { 0.7 }).to_hex()),
        None => row.shpStripe.set_visible(ui, false),
    }
    row.swOn.set_checked(ui, d.enabled);
    let id = d.id.clone();
    let sw = row.swOn;
    ui.on(sw.id, "on_change", move |ui, _, _| {
        let on = sw.checked(ui);
        toggle_source(ui, &id, on);
    });
    row.dotDefault.set_visible(ui, d.is_default);
    // O aviso do Bluetooth (o texto de cada um está no frame) muda pra microfone/saída.
    row.icoBtMic.set_visible(ui, d.is_bluetooth && meters_is_mic(d));
    row.icoBtSpeaker.set_visible(ui, d.is_bluetooth && !meters_is_mic(d));
    row.lblName.set_text(ui, d.name.as_str());
    // Os níveis chegam a cada 100 ms: a barra desliza até o novo (`smoothing` no frame).
    row.mtrLevel.set_dim(ui, !d.enabled);
    meters.insert(d.id.clone(), row.mtrLevel.id);
    let id = d.id.clone();
    let on = !d.enabled;
    if !locked {
        ui.on(row.pnlRow.id, "on_click", move |ui, _, _| toggle_source(ui, &id, on));
    }
}

/// O aviso do Bluetooth muda pra microfone/saída: o id do WASAPI de captura
/// começa com `{0.0.1`.
fn meters_is_mic(d: &Device) -> bool {
    d.id.starts_with("{0.0.1")
}

fn clear(ui: &mut Ui, parent: ComponentId) {
    for child in ui.children(parent).to_vec() {
        ui.free(child);
    }
}

fn refresh_sources(ui: &mut Ui) {
    let locked = with(|s| s.rec.active);
    let (mics, speakers) = with(|s| (visible(&s.mics).cloned().collect::<Vec<_>>(), visible(&s.speakers).cloned().collect::<Vec<_>>()));
    let mut meters = HashMap::new();
    if let Some(frm) = FrmMain::components(ui) {
        for (list, parent) in [(&mics, frm.lyMics), (&speakers, frm.lySpeakers)] {
            clear(ui, parent.id);
            for d in list.iter() {
                add_source_row(ui, parent.id, d, locked, &mut meters);
            }
            parent.set_height(ui, f64::from(list.len() as f32 * SOURCE_ROW));
        }
    }
    with(|s| s.meters = meters);
    let (mc, sc) = with(|s| (count_text(&s.mics), count_text(&s.speakers)));
    if let Some(frm) = FrmMain::components(ui) {
        frm.lblMicsCount.set_text(ui, mc);
        frm.lblSpeakersCount.set_text(ui, sc);
    }
    // A coluna rola: a altura do conteúdo é a soma das seções.
    // 14 + título 15 + 6, as linhas, 18 + título 15 + 6, as linhas e 14 embaixo.
    let content = 14.0 + 15.0 + 6.0 + 18.0 + 15.0 + 6.0 + (mics.len() + speakers.len()) as f32 * SOURCE_ROW + 14.0;
    if let Some(frm) = FrmMain::components(ui) {
        frm.lySources.set_height(ui, f64::from(content));
    }
    render_legend(ui);
    ui.layout();
}

/// Uma linha da legenda das faixas.
struct LegendItem {
    kind: &'static str,
    label: String,
    name: String,
    hint: String,
    color: Color,
}

/// Legenda das faixas: Padrão, Faixa 1 (mix), faixas com o dispositivo
/// padrão, de microfone, de saída, e "sem faixa".
fn legend_items(ui: &Ui, s: &State) -> Vec<LegendItem> {
    struct Info {
        names: Vec<String>,
        has_default: bool,
        mic: bool,
        spk: bool,
    }
    let mut tracks: std::collections::BTreeMap<i32, Info> = Default::default();
    let mut disabled: Vec<String> = Vec::new();
    let mut any_default = false;
    for (list, is_mic) in [(&s.mics, true), (&s.speakers, false)] {
        for d in visible(list) {
            any_default |= d.is_default;
            if d.track >= 2 {
                let e = tracks.entry(d.track).or_insert(Info { names: vec![], has_default: false, mic: false, spk: false });
                e.names.push(d.name.clone());
                if is_mic {
                    e.mic = true;
                } else {
                    e.spk = true;
                }
                e.has_default |= d.is_default && d.enabled;
            } else if !d.enabled {
                disabled.push(d.name.clone());
            }
        }
    }
    let mut out = Vec::new();
    if tracks.is_empty() && disabled.is_empty() && !any_default {
        return out;
    }
    let item = |kind: &'static str, label: String, name: String, hint: String, color: Color| LegendItem { kind, label, name, hint, color };
    if any_default {
        out.push(item("default", lang::tr("Padrão"), lang::tr("Dispositivo padrão do sistema"), lang::tr("Dispositivo definido como padrão no Windows. Indicado por um ponto colorido ao lado do nome do dispositivo na lista."), Color::TRANSPARENT));
    }
    if !tracks.is_empty() {
        out.push(item("mix", lang::tr("Faixa 1"), lang::tr("Mix (todos)"), lang::tr("Mistura de todos os dispositivos habilitados. Gravada como a primeira faixa de áudio do arquivo final."), Color::TRANSPARENT));
    }
    let render = |t: i32, info: &Info| {
        let label = match info.names.len() {
            1 => info.names[0].clone(),
            2 => info.names.join(", "),
            n => lang::tr_with("{{count}} dispositivos agrupados", &[("count", &n.to_string())]),
        };
        item("track", lang::tr_with("Faixa {{n}}", &[("n", &t.to_string())]), label, info.names.join(", "), track_color(ui, t))
    };
    for (t, info) in tracks.iter().filter(|(_, i)| i.has_default) {
        out.push(render(*t, info));
    }
    for (t, info) in tracks.iter().filter(|(_, i)| !i.has_default && i.mic && !i.spk) {
        out.push(render(*t, info));
    }
    for (t, info) in tracks.iter().filter(|(_, i)| !i.has_default && i.spk) {
        out.push(render(*t, info));
    }
    if !disabled.is_empty() {
        let label = if disabled.len() == 1 {
            disabled[0].clone()
        } else {
            lang::tr_with("{{count}} dispositivos desabilitados", &[("count", &disabled.len().to_string())])
        };
        out.push(item("off", lang::tr("Sem faixa"), label, disabled.join(", "), Color::TRANSPARENT));
    }
    out
}

fn render_legend(ui: &mut Ui) {
    let items = with(|s| legend_items(ui, s));
    let Some(parent) = FrmMain::components(ui).map(|frm| frm.lyLegend.id) else { return };
    clear(ui, parent);
    // "Sem faixa" na cor da borda (a mais fraca, se o tema tiver), senão a do texto apagado.
    let off = if ui.theme().colors.contains_key("border_2") { "border_2" } else { "text_muted" };
    for it in &items {
        // A linha é o frame `fraLegendRow` (forms/main/legend_row.json): a bolinha do
        // padrão e a barrinha das faixas já estão lá, escondidas; aqui aparece uma delas.
        let Ok(row) = FraLegendRow::create_in(ui, parent) else { continue };
        row.lyRow.set_hint(ui, it.hint.as_str());
        if it.kind == "off" {
            row.lyRow.set_opacity(ui, 0.7);
        }
        if it.kind == "default" {
            row.dotDefault.set_visible(ui, true);
        } else {
            let fill = match it.kind {
                "mix" => "text_muted".to_owned(),
                "off" => off.to_owned(),
                _ => it.color.to_hex(),
            };
            row.shpTrack.set_fill(ui, fill);
            row.shpTrack.set_visible(ui, true);
        }
        row.lblTrack.set_text(ui, it.label.as_str());
        row.lblName.set_text(ui, it.name.as_str());
    }
    // Linha, 14, o título (13), 8, as linhas, 4 + os 14 do fim da lista.
    let h = if items.is_empty() { 0.0 } else { 1.0 + 14.0 + 13.0 + 8.0 + items.len() as f32 * LEGEND_ROW + 18.0 };
    if let Some(frm) = FrmMain::components(ui) {
        frm.pnlLegend.set_height(ui, f64::from(h));
        frm.pnlLegend.set_visible(ui, !items.is_empty());
    }
}

/// Bloco "Telas": monitores na posição real e webcams à direita.
fn rebuild_displays(ui: &mut Ui) {
    let (mons, cams, locked) = with(|s| (visible(&s.monitors).cloned().collect::<Vec<_>>(), visible(&s.webcams).cloned().collect::<Vec<_>>(), s.rec.active));
    let max_x = mons.iter().map(|d| d.x + d.width).max().unwrap_or(0);
    let mut screens: Vec<ScreenItem> = mons
        .iter()
        .map(|d| ScreenItem {
            id: d.id.clone(),
            name: if d.name.is_empty() { lang::tr("Monitor") } else { d.name.clone() },
            x: d.x,
            y: d.y,
            width: d.width,
            height: d.height,
            enabled: d.enabled,
            camera: false,
            image: None,
        })
        .collect();
    // Webcams à direita do bloco dos monitores, em y = 0.
    let mut cam_x = max_x;
    for d in &cams {
        let (w, h) = (if d.width > 0 { d.width } else { 1280 }, if d.height > 0 { d.height } else { 720 });
        screens.push(ScreenItem {
            id: d.id.clone(),
            name: if d.name.is_empty() { lang::tr("Webcam") } else { d.name.clone() },
            x: cam_x,
            y: 0,
            width: w,
            height: h,
            enabled: d.enabled,
            camera: true,
            image: None,
        });
        cam_x += w;
    }
    if let Some(id) = FrmMain::components(ui).map(|frm| frm.scrDisplays.id) {
        if let Some(layout) = ui.get_mut::<ScreenLayout>(id) {
            layout.set_screens(screens);
            layout.locked = locked;
        }
        ui.invalidate();
    }
    // Contagem e "3840×2160" da composição marcada, com o aviso se passar do
    // limite do encoder.
    let total = mons.len() + cams.len();
    let sel = mons.iter().chain(cams.iter()).filter(|d| d.enabled).count();
    if let Some(frm) = FrmMain::components(ui) {
        frm.lblScreensCount.set_text(ui, format!("{sel} / {total}"));
    }
    let (mut cw, mut ch) = (0, 0);
    for d in mons.iter().chain(cams.iter()).filter(|d| d.enabled) {
        cw += d.width;
        ch = ch.max(d.height);
    }
    if cw > 0 && ch > 0 {
        let max = codec_max_dim();
        let over = cw > max || ch > max;
        let text = if over { format!("{cw}×{ch}  ⚠") } else { format!("{cw}×{ch}") };
        if let Some(frm) = FrmMain::components(ui) {
            frm.lblScreensMeta.set_text(ui, text);
        }
        let hint = if over { lang::tr_with("Resolução combinada das telas excede o limite do codec ({{max}} × {{max}}). A gravação será reduzida proporcionalmente. Mude pra AV1 ou HEVC em Configurações pra gravar até 8192 × 8192.", &[("max", &max.to_string())]) } else { String::new() };
        if let Some(frm) = FrmMain::components(ui) {
            frm.lblScreensMeta.set_hint(ui, hint);
        }
    } else {
        if let Some(frm) = FrmMain::components(ui) {
            frm.lblScreensMeta.set_text(ui, "");
        }
    }
}

/// Maior dimensão do encoder escolhido: H.264 por hardware para em 4096; no
/// automático, vale o H.264 por hardware se a máquina tiver.
fn codec_max_dim() -> i32 {
    match config::get_config_str("codec", "auto").as_str() {
        "h264-hw" => 4096,
        "h264-sw" | "hevc-hw" | "av1-hw" | "av1-sw" => 8192,
        _ => {
            if bridge::last_encoder_caps().is_some_and(|c| c.h264_hw) {
                4096
            } else {
                8192
            }
        }
    }
}

fn update_can_record(ui: &mut Ui) {
    let (any, recording) = with(|s| ([&s.monitors, &s.webcams, &s.mics, &s.speakers].iter().any(|l| visible(l).any(|d| d.enabled)), s.rec.active));
    if let Some(frm) = FrmMain::components(ui) {
        frm.btnRecord.set_enabled(ui, any || recording);
        frm.btnRecord.set_hint(ui, if any || recording { String::new() } else { lang::tr("Habilite ao menos um dispositivo (monitor, webcam, microfone ou alto-falante) pra iniciar a gravação.") });
    }
}

/// "Microfone adicionado: X · Saída removida: Y · e mais N".
fn format_changes(changes: &[String]) -> String {
    const MAX: usize = 3;
    if changes.len() <= MAX {
        return changes.join(" · ");
    }
    format!("{} · {}", changes[..MAX].join(" · "), lang::tr_with("…e mais {{count}}", &[("count", &(changes.len() - MAX).to_string())]))
}

pub fn on_monitors(list: &[Device], changes: &[String]) {
    let list = list.to_vec();
    let changes = changes.to_vec();
    with_ui(move |ui| {
        with(|s| s.monitors = list);
        rebuild_displays(ui);
        update_can_record(ui);
        if !changes.is_empty() {
            crate::app_view::show_toast(&lang::tr("Monitores atualizados"), &format_changes(&changes), true, 4500);
        }
    });
}

pub fn on_webcams(list: &[Device], changes: &[String]) {
    let list = list.to_vec();
    let changes = changes.to_vec();
    with_ui(move |ui| {
        with(|s| s.webcams = list);
        rebuild_displays(ui);
        update_can_record(ui);
        // Mesmo título do áudio: um hot-plug USB que mexe nos dois vira um aviso só.
        if !changes.is_empty() {
            crate::app_view::show_toast(&lang::tr("Dispositivos atualizados"), &format_changes(&changes), true, 4500);
        }
    });
}

pub fn on_audio_sources(mics: &[Device], speakers: &[Device], silent: bool, changes: &[String]) {
    let (mics, speakers, changes) = (mics.to_vec(), speakers.to_vec(), changes.to_vec());
    with_ui(move |ui| {
        with(|s| {
            s.mics = mics;
            s.speakers = speakers;
        });
        refresh_sources(ui);
        update_can_record(ui);
        if !silent {
            crate::app_view::show_toast(&lang::tr("Dispositivos atualizados"), &format_changes(&changes), true, 4500);
        }
    });
}

pub fn on_audio_meters(items: &[AudioMeter]) {
    let items = items.to_vec();
    with_ui(move |ui| {
        // Coberta por outra tela (Configurações, player, exportação), a tela principal não
        // mostra os medidores: atualizar fazia cada um deslizar (e pedir quadros) por baixo, e a
        // janela parada desenhava na taxa do monitor.
        if crate::app_view::main_covered(ui) {
            return;
        }
        let meters = with(|s| s.meters.clone());
        for m in &items {
            let Some(&id) = meters.get(&m.id) else { continue };
            let Some(meter) = ui.get_mut::<LevelMeter>(id) else { continue };
            meter.stereo = m.channels >= 2;
            meter.muted = m.muted;
            meter.value = amp_to_bar(m.left);
            meter.value_right = amp_to_bar(m.right);
            // Só o medidor repinta (o `invalidate` refazia o layout e a janela inteira).
            ui.damage(id);
        }
    });
}

pub fn on_monitor_thumbs(items: &[MonitorThumb]) {
    let items: Vec<(String, Option<Image>)> = items
        .iter()
        .map(|t| {
            let image = (t.rgb.len() == (t.width * t.height * 3) as usize && !t.rgb.is_empty()).then(|| {
                let mut rgba = Vec::with_capacity(t.rgb.len() / 3 * 4);
                for px in t.rgb.chunks_exact(3) {
                    rgba.extend_from_slice(&[px[0], px[1], px[2], 255]);
                }
                Image::rgba(t.width, t.height, rgba)
            });
            (t.id.clone(), image)
        })
        .collect();
    with_ui(move |ui| {
        let Some(id) = FrmMain::components(ui).map(|frm| frm.scrDisplays.id) else { return };
        if let Some(layout) = ui.get_mut::<ScreenLayout>(id) {
            for (screen, image) in items {
                layout.set_image(&screen, image);
            }
        }
        // As imagens ficam guardadas; coberta a tela principal, não repinta. Só a prévia das
        // telas repinta (não a janela inteira).
        if !crate::app_view::main_covered(ui) {
            ui.damage(id);
        }
    });
}

pub fn on_device_change_pending(_kind: PendingKind, _pending: bool) {}

pub fn on_refresh_busy(busy: bool, what: &str) {
    // Começar a gravar não cobre a coluna: só desativa o resto.
    if what == "starting" {
        with_ui(move |ui| {
            if let Some(frm) = FrmMain::components(ui) {
                frm.scrSources.set_enabled(ui, !busy);
                frm.scrDisplays.set_enabled(ui, !busy);
            }
        });
    }
}

/// Liga/desliga um dispositivo: a tela muda na hora; o core grava a
/// preferência e, no áudio, recalcula as faixas e manda a lista de volta.
fn toggle_source(ui: &mut Ui, id: &str, enabled: bool) {
    with(|s| {
        for list in [&mut s.monitors, &mut s.webcams, &mut s.mics, &mut s.speakers] {
            if let Some(d) = list.iter_mut().find(|d| d.id == id) {
                d.enabled = enabled;
            }
        }
    });
    rebuild_displays(ui);
    let id_owned = id.to_owned();
    // A lista é refeita fora deste evento (a linha clicada sai da árvore).
    ui.set_timeout(0, |ui| refresh_sources(ui));
    update_can_record(ui);
    bridge::handle_toggle_source(&id_owned, enabled);
}

/// O clique numa tela do bloco "Telas" (o `ScreenLayout` já marcou).
pub fn display_change(ui: &mut Ui, sender: ComponentId, args: &EventArgs) {
    let EventArgs::Text(id) = args else { return };
    let on = ui.get::<ScreenLayout>(sender).is_some_and(|l| l.is_enabled(id));
    let id = id.clone();
    toggle_source(ui, &id, on);
}

// ---------------------------------------------------------------------------
// Gravação
// ---------------------------------------------------------------------------

/// "HH:MM:SS" e ".cc" (centésimos) do cronômetro.
fn timer_text(ms: u64) -> (String, String) {
    let s = ms / 1000;
    (format!("{:02}:{:02}:{:02}", s / 3600, s / 60 % 60, s % 60), format!(".{:02}", ms % 1000 / 10))
}

fn render_timer(ui: &mut Ui, ms: u64) {
    let (a, b) = timer_text(ms);
    if let Some(frm) = FrmMain::components(ui) {
        frm.lblTimer.set_text(ui, a);
        frm.lblTimerMs.set_text(ui, b);
    }
}

/// Cronômetro a 50 ms, da "partida" pra frente.
fn start_timer(ui: &mut Ui, elapsed_ms: u64) {
    stop_timer(ui);
    with(|s| s.timer_start = Some(Instant::now() - Duration::from_millis(elapsed_ms)));
    render_timer(ui, elapsed_ms);
    tick_timer(ui);
}

fn tick_timer(ui: &mut Ui) {
    let job = ui.set_timeout(50, |ui| {
        let Some(start) = with(|s| s.timer_start) else { return };
        render_timer(ui, start.elapsed().as_millis() as u64);
        tick_timer(ui);
    });
    with(|s| s.timer_job = Some(job));
}

fn stop_timer(ui: &mut Ui) {
    if let Some(job) = with(|s| {
        s.timer_start = None;
        s.timer_job.take()
    }) {
        ui.clear_timeout(job);
    }
}

/// O cartão no estado da gravação: botão, status, cores, texto do botão.
fn render_recording(ui: &mut Ui) {
    let (active, paused, window, finalizing, countdown) =
        with(|s| (s.rec.active, s.rec.paused, s.rec.window.clone(), s.finalizing, s.countdown_sec));
    if let Some(frm) = FrmMain::components(ui) {
        frm.btnRecord.set_recording(ui, active);
        frm.btnRecord.set_paused(ui, paused);
        frm.btnRecord.set_finalizing(ui, finalizing);
    }
    let status = if countdown > 0 {
        lang::tr_with("Gravação em {{sec}} s", &[("sec", &countdown.to_string())])
    } else if active && paused {
        lang::tr("Pausado")
    } else if active && !window.is_empty() {
        lang::tr_with("Gravando: {{title}}", &[("title", &window)])
    } else if active {
        lang::tr("Gravando")
    } else if finalizing {
        lang::tr("Finalizando gravação...")
    } else {
        lang::tr("Pronto para gravar")
    };
    if let Some(frm) = FrmMain::components(ui) {
        frm.lblStatus.set_text(ui, status);
    }
    // As cores pelo nome do tema: acompanham a troca de tema sem repintar à mão.
    let (dot, status_color) = if active && paused {
        ("amber", "amber")
    } else if active {
        ("red", "red")
    } else {
        ("green", "")
    };
    if let Some(frm) = FrmMain::components(ui) {
        frm.dotStatus.set_fill(ui, dot);
        frm.lblStatus.set_color(ui, status_color);
        frm.lblStatus.set_font_weight(ui, if active { "semibold" } else { "default" });
        frm.lblTimer.set_color(ui, if active && paused { "amber" } else { "" });
        frm.lblRecordAction.set_text(ui, lang::tr(if active { "Parar gravação" } else { "Iniciar gravação" }));
        frm.pnlRecordCard.set_border_color(ui, if active { "red" } else { "" });
        frm.btnPause.set_enabled(ui, active);
        frm.btnPause.set_icon(ui, if paused { "play" } else { "pause" });
        frm.btnPause.set_hint(ui, lang::tr(if paused { "Retomar gravação" } else { "Pausar gravação" }));
        frm.btnCapture.set_enabled(ui, !active);
    }
    let size = with(|s| s.rec.size_text.clone()).filter(|_| active).unwrap_or_default();
    if let Some(frm) = FrmMain::components(ui) {
        frm.lblSize.set_text(ui, size);
    }
}

pub fn on_recording_state(st: &RecState) {
    let st = st.clone();
    with_ui(move |ui| {
        // Som de início: só na virada pra gravando (não no 1º estado do arranque).
        let prev = with(|s| s.last_active.replace(st.active));
        if prev == Some(false) && st.active && config::get_config_bool("playSoundOnRecord", true) {
            crate::sounds::play_start();
        }
        let changed = prev != Some(st.active);
        with(|s| {
            if let Some(t) = &st.size_text {
                s.rec.size_text = Some(t.clone());
            }
            if !st.active {
                s.rec.size_text = None;
            }
            s.rec.active = st.active;
            s.rec.paused = st.active && st.paused;
            s.rec.window = if st.active { st.window.clone() } else { String::new() };
            s.pause_locked = false;
        });
        render_recording(ui);
        let (active, paused, running) = with(|s| (s.rec.active, s.rec.paused, s.timer_job.is_some()));
        if active {
            // Pausada: o cronômetro para no tempo gravado (o core manda o tempo já
            // congelado). Gravando: anda a partir do tempo que o core mandou.
            let ms = st.elapsed_ms.unwrap_or(0);
            if paused {
                stop_timer(ui);
                render_timer(ui, ms);
            } else if st.elapsed_ms.is_some() || !running {
                start_timer(ui, ms);
            }
        } else {
            stop_timer(ui);
            render_timer(ui, 0);
        }
        // Gravando, as fontes travam (e destravam no fim).
        if changed {
            refresh_sources(ui);
            rebuild_displays(ui);
            update_can_record(ui);
            render_replay(ui);
        }
    });
}

pub fn on_recording_finalizing(active: bool) {
    with_ui(move |ui| {
        with(|s| s.finalizing = active);
        render_recording(ui);
    });
}

/// Som de parada com debounce de 800 ms: o clique toca na hora e o aviso do
/// core chega logo depois.
pub fn play_stop_sound() {
    let now = Instant::now();
    let recent = with(|s| s.last_stop_sound.replace(now)).is_some_and(|t| now.duration_since(t) < Duration::from_millis(800));
    if !recent {
        crate::sounds::play_stop();
    }
}

pub fn record_click(ui: &mut Ui) {
    let recording = with(|s| s.rec.active);
    if recording {
        // Som de parada AGORA (o arquivo pode levar centenas de ms pra fechar).
        if config::get_config_bool("playSoundOnRecord", true) {
            play_stop_sound();
        }
        bridge::handle_record_stop();
    } else {
        // A onda só no começo: parar já se vê pelo fim do pulso vermelho.
        if with(|s| s.countdown_sec) == 0 {
            start_ripple(ui);
        }
        bridge::handle_record_start_button();
    }
}

/// `cubic-bezier(x1, y1, x2, y2)` em `t` (0..1): acha o parâmetro da curva
/// cujo x é `t` (bissecção) e devolve o y.
fn cubic_bezier(x1: f32, y1: f32, x2: f32, y2: f32, t: f32) -> f32 {
    let b = |a: f32, c: f32, u: f32| 3.0 * a * u * (1.0 - u).powi(2) + 3.0 * c * u * u * (1.0 - u) + u.powi(3);
    let (mut lo, mut hi) = (0.0f32, 1.0f32);
    for _ in 0..30 {
        let mid = (lo + hi) / 2.0;
        if b(x1, x2, mid) < t {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    b(y1, y2, (lo + hi) / 2.0)
}

/// Onda vermelha que sai do botão de gravar: um anel de 4 px no raio 98
/// crescendo até cobrir a janela e sumindo, 1,8 s com
/// `cubic-bezier(0.16, 1, 0.3, 1)`, e um brilho largo e fraco junto.
///
/// O relógio só anda o que a tela pôde mostrar (no máximo 50 ms por tique):
/// com a main travada montando a captura, um relógio de parede pularia o
/// começo da curva — onde está quase toda a expansão.
pub fn start_ripple(ui: &mut Ui) {
    let Some(form) = main_form() else { return };
    let Some(frm) = FrmMain::components(ui) else { return };
    let Some(b) = ui.absolute_rect(frm.btnRecord.id) else { return };
    let Some(f) = ui.bounds(form) else { return };
    let (cx, cy) = (b.x + b.width / 2.0, b.y + b.height / 2.0);
    let max_r = (cx.max(f.width - cx).powi(2) + cy.max(f.height - cy).powi(2)).sqrt();
    if let Some(job) = with(|s| s.ripple_job.take()) {
        ui.clear_timeout(job);
    }
    // O anel e o brilho: círculos só de contorno e sem clique, no form (`cirRipple`,
    // `cirRippleGlow`). Na 1ª onda vão para o fim da lista do form, por cima de tudo,
    // inclusive das telas que entram nele depois (Configurações, seletor...).
    let (ring, glow) = (frm.cirRipple, frm.cirRippleGlow);
    if !with(|s| std::mem::replace(&mut s.ripple_raised, true)) {
        let _ = ui.set_child_index(glow.id, usize::MAX);
        let _ = ui.set_child_index(ring.id, usize::MAX);
    }
    let (last, played) = (Instant::now(), 0.0f32);
    fn step(ui: &mut Ui, ring: Circle, glow: Circle, (cx, cy, max_r): (f32, f32, f32), e: f32) {
        let s = e * (max_r / 94.0 + 1.0);
        for (c, w) in [(ring, 4.0 * s), (glow, 24.0 * s)] {
            let r = 98.0 * s;
            c.set_stroke_width(ui, w.max(0.5) as f64);
            c.set_opacity(ui, (1.0 - e) as f64);
            if let Some(cb) = ui.control_mut(c.id) {
                cb.set_rect(cx - r, cy - r, 2.0 * r, 2.0 * r).set_visible(s > 0.0);
            }
        }
    }
    let geo = (cx, cy, max_r);
    step(ui, ring, glow, geo, 0.0);
    fn tick(ui: &mut Ui, ring: Circle, glow: Circle, geo: (f32, f32, f32), mut last: Instant, mut played: f32) {
        let now = Instant::now();
        played += now.duration_since(last).as_secs_f32().min(0.05);
        last = now;
        let k = (played / 1.8).min(1.0);
        if k >= 1.0 {
            for c in [ring, glow] {
                c.set_visible(ui, false);
            }
            with(|s| s.ripple_job = None);
            return;
        }
        step(ui, ring, glow, geo, cubic_bezier(0.16, 1.0, 0.3, 1.0, k));
        let job = ui.set_timeout(16, move |ui| tick(ui, ring, glow, geo, last, played));
        with(|s| s.ripple_job = Some(job));
    }
    let job = ui.set_timeout(16, move |ui| tick(ui, ring, glow, geo, last, played));
    with(|s| s.ripple_job = Some(job));
}

/// "Capturar...": o seletor (gravando, não abre).
pub fn capture_click(ui: &mut Ui) {
    let recording = with(|s| s.rec.active);
    crate::app_picker::open(ui, recording);
}

/// Trava até o próximo estado: dois cliques rápidos cruzariam pausa e retomada.
pub fn pause_click(_ui: &mut Ui) {
    let (active, paused, locked) = with(|s| (s.rec.active, s.rec.paused, s.pause_locked));
    if !active || locked {
        return;
    }
    with(|s| s.pause_locked = true);
    bridge::handle_record_pause(!paused);
}

// ---------------------------------------------------------------------------
// Contagem do atraso (atalho com atraso)
// ---------------------------------------------------------------------------

pub fn on_record_countdown(sec: u32, total: u32) {
    with_ui(move |ui| {
        let prev = with(|s| std::mem::replace(&mut s.countdown_sec, sec));
        if sec == 0 {
            if let Some(job) = with(|s| {
                s.countdown_end = None;
                s.countdown_job.take()
            }) {
                ui.clear_timeout(job);
            }
            if let Some(frm) = FrmMain::components(ui) {
                frm.btnRecord.set_countdown(ui, 0i64);
            }
            render_recording(ui);
            return;
        }
        // Contagem nova (ou que voltou a subir): o anel recomeça do ponto certo.
        if prev == 0 || sec > prev || with(|s| s.countdown_end.is_none()) {
            let tot = total.max(sec) as f32;
            with(|s| {
                s.countdown_total = tot;
                s.countdown_end = Some(Instant::now() + Duration::from_secs(sec as u64));
            });
        }
        if let Some(frm) = FrmMain::components(ui) {
            frm.btnRecord.set_countdown(ui, sec as i64);
        }
        render_recording(ui);
        if with(|s| s.countdown_job.is_none()) {
            tick_countdown(ui);
        }
    });
}

/// O anel da contagem esvazia a cada quadro.
fn tick_countdown(ui: &mut Ui) {
    let (end, total) = with(|s| (s.countdown_end, s.countdown_total));
    let Some(end) = end else {
        with(|s| s.countdown_job = None);
        return;
    };
    let left = end.saturating_duration_since(Instant::now()).as_secs_f32();
    if let Some(frm) = FrmMain::components(ui) {
        frm.btnRecord.set_countdown_fraction(ui, if total > 0.0 { f64::from(left / total) } else { 0.0 });
    }
    let job = ui.set_timeout(16, tick_countdown);
    with(|s| s.countdown_job = Some(job));
}

// ---------------------------------------------------------------------------
// Buffer em memória
// ---------------------------------------------------------------------------

/// "m:ss".
fn fmt_min_sec(sec: i64) -> String {
    format!("{}:{:02}", sec / 60, sec % 60)
}

/// Texto e botões da linha do buffer. O "guardado" anda sozinho entre um
/// estado e outro (o core só manda quando algo muda), limitado ao teto.
fn render_replay(ui: &mut Ui) {
    let (st, received, recording) = with(|s| (s.replay.clone(), s.replay_received, s.rec.active));
    let max = if st.max_sec > 0 { st.max_sec } else { 300 };
    let text = if st.active {
        if st.saving {
            lang::tr("Salvando trecho…")
        } else {
            let ms = st.since_ms + received.map_or(0, |r| r.elapsed().as_millis() as u64);
            let held = ((ms / 1000) as i64).min(st.max_sec.max(0));
            lang::tr_with("Guardando {{held}} de {{max}}", &[("held", &fmt_min_sec(held)), ("max", &fmt_min_sec(st.max_sec.max(0)))])
        }
    } else if st.enabled && recording {
        lang::tr("Pausado durante a gravação")
    } else {
        lang::tr_with("Desligado · guarda os últimos {{max}}", &[("max", &fmt_min_sec(max))])
    };
    if let Some(frm) = FrmMain::components(ui) {
        frm.lblReplayStatus.set_text(ui, text);
        frm.swReplay.set_checked(ui, st.enabled);
        frm.btnReplaySave.set_visible(ui, st.active);
        frm.btnReplaySave.set_enabled(ui, !st.saving);
    }
    let hint = if st.hotkey.is_empty() { lang::tr("Salva o que está no buffer como gravação") } else { lang::tr_with("Salva o que está no buffer como gravação ({{spec}})", &[("spec", &st.hotkey)]) };
    if let Some(frm) = FrmMain::components(ui) {
        frm.btnReplaySave.set_hint(ui, hint);
    }
    let border2 = if ui.theme().colors.contains_key("border_2") { "border_2" } else { "border" };
    if let Some(frm) = FrmMain::components(ui) {
        frm.dotReplay.set_fill(ui, if st.active { "green" } else { border2 });
    }
}

pub fn on_replay_state(st: &ReplayState) {
    let st = st.clone();
    with_ui(move |ui| {
        with(|s| {
            s.replay = st.clone();
            s.replay_received = Some(Instant::now());
        });
        render_replay(ui);
        let ticking = with(|s| s.replay_job.is_some());
        if st.active && !ticking {
            tick_replay(ui);
        } else if !st.active {
            if let Some(job) = with(|s| s.replay_job.take()) {
                ui.clear_timeout(job);
            }
        }
    });
}

fn tick_replay(ui: &mut Ui) {
    let job = ui.set_timeout(1000, |ui| {
        render_replay(ui);
        tick_replay(ui);
    });
    with(|s| s.replay_job = Some(job));
}

pub fn on_replay_saved(name: &str, duration_sec: i32) {
    crate::app_view::show_toast(
        &lang::tr("Trecho salvo"),
        &lang::tr_with("{{name}} — {{dur}}", &[("name", name), ("dur", &fmt_min_sec(duration_sec.max(0) as i64))]),
        false,
        6000,
    );
}

pub fn on_replay_restarted() {
    crate::app_view::show_toast(&lang::tr("Buffer reiniciado"), &lang::tr("O buffer recomeçou com as novas configurações de gravação. O que estava guardado foi descartado."), false, 6000);
}

/// A linha inteira do buffer (ou a chave) liga/desliga.
pub fn replay_toggle(_ui: &mut Ui) {
    let on = with(|s| s.replay.enabled);
    bridge::handle_set_replay_enabled(!on);
}

/// A chave já mudou de estado: segue o que ela diz (sem inverter de novo).
pub fn replay_switch(on: bool) {
    if on != with(|s| s.replay.enabled) {
        bridge::handle_set_replay_enabled(on);
    }
}

pub fn replay_save(_ui: &mut Ui) {
    let (active, saving) = with(|s| (s.replay.active, s.replay.saving));
    if active && !saving {
        bridge::handle_save_replay();
    }
}

/// Troca de idioma: os textos montados aqui são refeitos.
pub fn on_language_changed(ui: &mut Ui) {
    apply_texts(ui);
    render_recording(ui);
    refresh_sources(ui);
    rebuild_displays(ui);
    render_replay(ui);
}

thread_local! {
    /// Página da versão nova (o "Abrir página" do aviso).
    static UPDATE_URL: RefCell<String> = const { RefCell::new(String::new()) };
}

/// Versão nova: o aviso no topo da coluna (dispensar vale só pra sessão).
pub fn show_update_banner(ui: &mut Ui, latest: &str, url: &str) {
    UPDATE_URL.with(|u| *u.borrow_mut() = url.to_owned());
    if let Some(frm) = FrmMain::components(ui) {
        frm.lblUpdTitle.set_text(ui, lang::tr("Nova versão disponível"));
        frm.lblUpdBody.set_text(ui, lang::tr_with("A versão {{version}} já está disponível para download.", &[("version", latest)]));
        // Sem página, a linha do botão some inteira (não fica um vão no aviso).
        frm.lyUpdActions.set_visible(ui, !url.is_empty());
        frm.pnlUpdate.set_visible(ui, true);
    }
}

pub fn hide_update_banner(ui: &mut Ui) {
    if let Some(frm) = FrmMain::components(ui) {
        frm.pnlUpdate.set_visible(ui, false);
    }
}

pub fn open_update_page(ui: &mut Ui) {
    let url = UPDATE_URL.with(|u| u.borrow().clone());
    if !url.is_empty() {
        bridge::handle_open_url(&url);
    }
    hide_update_banner(ui);
}

/// Selo da transcrição no cabeçalho do cartão: (mostrar, rodando, número, dica).
pub fn on_transcribe_chip(chip: (bool, bool, i32, String)) {
    with_ui(move |ui| {
        let text = format!("{} {}", if chip.1 { "◌" } else { "≡" }, chip.2);
        if let Some(frm) = FrmMain::components(ui) {
            frm.lblTrChip.set_text(ui, text);
            frm.lblTrChip.set_hint(ui, chip.3.as_str());
            frm.lblTrChip.set_visible(ui, chip.0);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn app() -> aylis::Application {
        let mut app = aylis::Application::bundled(crate::PROJECT).unwrap();
        crate::generated::register(&mut app.ui);
        app
    }

    /// As linhas de dispositivo são instâncias do frame `fraSourceRow`, empilhadas no
    /// pai, e só o que varia por dispositivo sai do código.
    #[test]
    fn the_source_rows_are_frames() {
        let mut app = app();
        let ui = &mut app.ui;
        let list = ui.create("Layout", None).unwrap();
        if let Some(cb) = ui.control_mut(list) {
            cb.set_rect(0.0, 0.0, 376.0, 200.0);
        }
        let mic = Device { id: "{0.0.1}.a".into(), name: "Mic".into(), enabled: true, is_default: true, is_bluetooth: true, ..Default::default() };
        let spk = Device { id: "{0.0.0}.b".into(), name: "Fone".into(), enabled: false, ..Default::default() };
        let mut meters = HashMap::new();
        add_source_row(ui, list, &mic, false, &mut meters);
        add_source_row(ui, list, &spk, true, &mut meters);
        ui.layout();
        let rows: Vec<_> = ui.children(list).iter().map(|&c| FraSourceRow::components_of(ui, c).unwrap()).collect();
        assert_eq!(rows.len(), 2);
        assert_eq!(meters.get("{0.0.1}.a"), Some(&rows[0].mtrLevel.id));
        assert_eq!(rows[0].lblName.text(ui), "Mic");
        assert!(rows[0].swOn.checked(ui) && rows[0].swOn.enabled(ui));
        assert!(rows[0].dotDefault.visible(ui) && rows[0].icoBtMic.visible(ui) && !rows[0].icoBtSpeaker.visible(ui));
        // Ligado na faixa 1 (o mix): sem a faixa colorida.
        assert!(!rows[0].shpStripe.visible(ui));
        assert!(!rows[1].swOn.checked(ui) && !rows[1].swOn.enabled(ui));
        assert!(!rows[1].dotDefault.visible(ui) && !rows[1].icoBtMic.visible(ui));
        assert!(rows[1].shpStripe.visible(ui) && rows[1].mtrLevel.dim(ui));
        assert_eq!(rows[1].pnlRow.opacity(ui), 0.5);
        let (a, b) = (ui.bounds(rows[0].form.id).unwrap(), ui.bounds(rows[1].form.id).unwrap());
        assert_eq!((a.y, a.width, a.height, b.y), (0.0, 376.0, SOURCE_ROW, SOURCE_ROW));
        let stripe = ui.bounds(rows[1].shpStripe.id).unwrap();
        assert_eq!((stripe.x, stripe.width, stripe.height), (-12.0, 3.0, SOURCE_ROW));
    }

    /// A legenda: cada linha é um `fraLegendRow`, com a bolinha (padrão) ou a barrinha.
    #[test]
    fn the_legend_rows_are_frames() {
        let mut app = app();
        let ui = &mut app.ui;
        ui.create_form("frmMain").unwrap();
        // Padrão, mix, a faixa 2 e "sem faixa".
        with(|s| {
            s.mics = vec![Device { id: "a".into(), name: "Mic".into(), enabled: true, is_default: true, track: 2, ..Default::default() }];
            s.speakers = vec![Device { id: "b".into(), name: "Fone".into(), ..Default::default() }];
        });
        render_legend(ui);
        let main = FrmMain::components(ui).unwrap();
        let rows: Vec<_> = ui.children(main.lyLegend.id).iter().map(|&c| FraLegendRow::components_of(ui, c).unwrap()).collect();
        assert_eq!(rows.len(), 4);
        assert!(rows[0].dotDefault.visible(ui) && !rows[0].shpTrack.visible(ui));
        assert!(!rows[2].dotDefault.visible(ui) && rows[2].shpTrack.visible(ui));
        assert_eq!(rows[2].lblName.text(ui), "Mic");
        assert!((rows[3].lyRow.opacity(ui) - 0.7).abs() < 1e-6);
        assert!(main.pnlLegend.visible(ui));
        // A onda: os dois círculos do form, escondidos até o clique.
        assert!(!main.cirRipple.visible(ui) && !main.cirRippleGlow.visible(ui));
    }
}
