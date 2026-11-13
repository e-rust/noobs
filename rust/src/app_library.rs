//! Presenter da biblioteca: recebe a lista do core pelo [`View`](crate::view::View),
//! agrupa por período, filtra pela busca, guarda a seleção, o recorte e a edição
//! de nome, e devolve os gestos ao core por chamada direta. O desenho é a parte
//! `lyLibrary` do `forms/main.json` (com os menus de contexto `pmLib*`); cada grupo
//! e cada cartão é uma instância de frame (`forms/library/*.json`): aqui só entra
//! o que varia por item.
//!
//! O estado que importa (pasta navegada, o que tem nela) NÃO é deduzido aqui:
//! chega inteiro em [`on_library`] a cada mudança. O que é só da tela — seleção,
//! recorte pendente, busca — mora aqui.

use crate::generated as g;
use crate::generated::{FraFolderCard, FraLibGroup, FraRecCard, FrmMain};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

use chrono::{Datelike, Local, NaiveDate, NaiveDateTime};
use aylis::prelude::*;

use crate::app_view::with_ui;
use crate::view::{Library, RecItem};
use crate::{bridge, lang};

/// Abaixo disso o "livres" ganha o aviso.
const LOW_DISK_BYTES: i64 = 5 * 1024 * 1024 * 1024;
/// Espera da busca na transcrição: cada tecla varreria o disco.
const SEARCH_DEBOUNCE_MS: u64 = 300;
/// Miniaturas guardadas; o resto sai pela ordem de uso.
const THUMB_CACHE_MAX: usize = 160;
/// O cartão de gravação, o de pasta e o espaço entre eles.
const CARD_W: f32 = 170.0;
/// A largura da miniatura que acompanha o cursor ao arrastar um cartão (16:9).
const DRAG_THUMB_W: f32 = 160.0;
const CARD_H: f32 = 148.0;
const FOLDER_H: f32 = 54.0;
const GAP: f32 = 12.0;
/// A altura do título de um grupo (com a linha embaixo e o respiro).
/// Título de grupo: 6 + 15,4 + 6 + 1 de linha, e 8 até os cartões.
const GROUP_HEAD: f32 = 28.4 + 8.0;
/// O vão antes de um grupo: 14 depois do cabeçalho, 18 entre grupos.
const FIRST_GAP: f32 = 14.0;
const GROUP_GAP: f32 = 18.0;

#[derive(Default)]
struct State {
    lib: Library,
    /// Lista que chegou durante uma edição de nome: aplica quando ela acabar.
    pending: Option<Library>,
    query: String,
    tr_hits: HashSet<String>,
    search_job: Option<u64>,
    /// Em ordem de seleção: é a ordem da união.
    selection: Vec<String>,
    clipboard: Vec<String>,
    merge_busy: bool,
    /// O cartão de cada gravação/pasta na tela (id → `MediaCard`).
    cards: HashMap<String, g::MediaCard>,
    /// As caixas "selecionar todos" de cada grupo (ids dos itens, a caixa).
    groups: Vec<(Vec<String>, g::CheckBox)>,
    /// Os ids do caminho (na ordem do `Breadcrumb`).
    crumbs: Vec<String>,
    thumbs: HashMap<String, (Image, u64)>,
    thumb_clock: u64,
    /// O menu de contexto aberto: (tipo, id).
    ctx: (String, String),
    /// Quantas colunas cabem (a altura das grades sai daí).
    cols: usize,
    /// O que está sendo arrastado (gravações e/ou pastas).
    drag: Vec<String>,
    /// O histórico de pastas, como no Explorer: os botões voltar e avançar do mouse.
    back: Vec<String>,
    forward: Vec<String>,
    /// A pasta pedida veio do histórico: ao chegar, não entra nele de novo.
    nav_by_history: bool,
}

/// Quantas pastas o voltar guarda.
const HISTORY_MAX: usize = 50;

/// A lista nova entra; se a pasta mudou por um caminho normal (pasta, breadcrumb,
/// Backspace), a de antes vai para o "voltar" e o "avançar" esvazia.
fn set_lib(st: &mut State, lib: Library) {
    let old = std::mem::take(&mut st.lib.current_dir);
    if !old.is_empty() && !old.eq_ignore_ascii_case(&lib.current_dir) {
        if !std::mem::take(&mut st.nav_by_history) {
            st.back.push(old);
            if st.back.len() > HISTORY_MAX {
                st.back.remove(0);
            }
            st.forward.clear();
        }
    }
    st.lib = lib;
}

/// Os botões extras do mouse (voltar e avançar) na galeria. O voltar sem histórico, dentro
/// de uma pasta, sobe um nível (como o Backspace).
pub fn mouse_history(ui: &mut Ui, forward: bool) {
    let target = with(|st| {
        let current = st.lib.current_dir.clone();
        let target = if forward { st.forward.pop() } else { st.back.pop() }?;
        if forward {
            st.back.push(current);
        } else {
            st.forward.push(current);
        }
        st.nav_by_history = true;
        Some(target)
    });
    match target {
        Some(dir) => bridge::handle_open_folder(&dir),
        None if !forward => go_up(ui),
        None => {}
    }
}

thread_local! {
    static ST: RefCell<State> = RefCell::new(State::default());
}

fn with<R>(f: impl FnOnce(&mut State) -> R) -> R {
    ST.with(|s| f(&mut s.borrow_mut()))
}

// ---------------------------------------------------------------------------
// Formatação
// ---------------------------------------------------------------------------

/// `mm:ss` ou `hh:mm:ss`; vazio sem duração.
fn format_duration(sec: i32) -> String {
    if sec <= 0 {
        return String::new();
    }
    let (h, m, s) = (sec / 3600, (sec % 3600) / 60, sec % 60);
    if h > 0 {
        format!("{h:02}:{m:02}:{s:02}")
    } else {
        format!("{m:02}:{s:02}")
    }
}

fn format_bytes(b: i64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;
    let f = b as f64;
    if f >= GB {
        format!("{:.1} GB", f / GB)
    } else if f >= MB {
        format!("{} MB", (f / MB).round())
    } else if f >= KB {
        format!("{} KB", (f / KB).round())
    } else {
        format!("{b} B")
    }
}

fn parse_date(iso: &str) -> Option<NaiveDateTime> {
    NaiveDateTime::parse_from_str(iso, "%Y-%m-%dT%H:%M:%S").ok()
}

fn format_full_date(iso: &str) -> String {
    parse_date(iso).map(|d| d.format("%d/%m/%Y %H:%M:%S").to_string()).unwrap_or_default()
}

fn file_count_text(n: i64) -> String {
    let key = if n == 1 { "{{count}} arquivo" } else { "{{count}} arquivos" };
    lang::tr_with(key, &[("count", &n.to_string())])
}

/// (chave que ordena, rótulo): hoje, ontem, esta semana, e daí por mês.
fn period(iso: &str, today: NaiveDate) -> (String, String) {
    let Some(d) = parse_date(iso) else {
        return ("zzz-unknown".into(), lang::tr("Sem data"));
    };
    let diff = (today - d.date()).num_days();
    if diff <= 0 {
        return ("00-hoje".into(), lang::tr("Hoje"));
    }
    if diff < 2 {
        return ("01-ontem".into(), lang::tr("Ontem"));
    }
    if diff < 7 {
        return ("02-semana".into(), lang::tr("Esta semana"));
    }
    let month = lang::tr_of("months", &d.month0().to_string());
    // Mais novo primeiro: a chave inverte o ano/mês.
    let key = format!("99-{:06}", 999_999 - (d.year() * 100 + d.month0() as i32));
    (key, format!("{month} {}", d.year()))
}

/// O texto em que a busca procura.
fn search_text(it: &RecItem) -> String {
    let mut s = format!("{} {} {} {}", it.name, it.size_text, format_duration(it.duration), it.codec);
    if it.fps > 0 {
        s.push_str(&format!(" {}{}", it.fps, lang::tr("fps")));
    }
    if it.quality >= 0 {
        s.push(' ');
        s.push_str(&lang::tr_with("Q{{level}}", &[("level", &it.quality.to_string())]));
    }
    s.to_lowercase()
}

/// Os selos do cartão: codec (em destaque quando é por hardware), fps, nível
/// de qualidade e o ícone da transcrição.
fn badges(it: &RecItem) -> String {
    let mut out = Vec::new();
    if is_print(&it.id) {
        out.push("PNG".to_owned());
    }
    if !it.codec.is_empty() {
        // Âmbar quando a codificação foi por software (como no original).
        out.push(if it.codec_hw { it.codec.clone() } else { format!("*{}", it.codec) });
    }
    if it.fps > 0 {
        out.push(format!("{}{}", it.fps, lang::tr("fps")));
    }
    if it.quality >= 0 {
        out.push(lang::tr_with("Q{{level}}", &[("level", &it.quality.to_string())]));
    }
    if it.transcript == "ok" {
        out.push("icon:list".into());
    }
    out.join(";")
}

fn folder_meta(count: i64, size_text: &str) -> String {
    let mut s = file_count_text(count);
    if count > 0 && !size_text.is_empty() {
        s.push_str(" · ");
        s.push_str(size_text);
    }
    s
}

// ---------------------------------------------------------------------------
// Tela
// ---------------------------------------------------------------------------

/// Quantos cartões cabem numa linha da largura da biblioteca.
fn columns(ui: &mut Ui) -> usize {
    let w = FrmMain::components(ui).map(|frm| frm.lyLibContent.id).and_then(|id| ui.bounds(id)).map(|b| b.width).filter(|w| *w > 0.0);
    let w = w.or_else(|| FrmMain::components(ui).map(|frm| frm.lyLibrary.id).and_then(|id| ui.bounds(id)).map(|b| b.width - 44.0)).unwrap_or(1100.0);
    (((w + GAP) / (CARD_W + GAP)).floor() as usize).max(1)
}

fn grid_height(n: usize, cols: usize, h: f32) -> f32 {
    let rows = n.div_ceil(cols.max(1));
    if rows == 0 { 0.0 } else { rows as f32 * (h + GAP) - GAP }
}

/// Um grupo (o frame `fraLibGroup`): o título em caixa-alta com a linha e (nas
/// gravações) a caixa de selecionar o grupo todo, e a grade de `n` cartões de
/// altura `h`. Devolve a grade, onde os cartões entram. O frame não recebe clique
/// (os vãos entre os cartões são do fundo: o menu do botão direito).
fn add_group(ui: &mut Ui, parent: ComponentId, label: &str, check: Option<(bool, Vec<String>)>, first: bool, n: usize, cols: usize, h: f32) -> Option<ComponentId> {
    let group = FraLibGroup::create_in(ui, parent).ok()?;
    if first {
        // O vão antes do primeiro grupo é menor (o JSON traz o dos outros); margem
        // não tem setter tipado.
        if let Some(cb) = ui.control_mut(group.lyHead.id) {
            cb.set_margins([0.0, FIRST_GAP, 0.0, 8.0]);
        }
    }
    group.lblTitle.set_text(ui, label);
    match check {
        Some((all, ids)) => {
            // Só o clique do usuário dispara o `on_change` (ver `refresh_marks`).
            ui.without_change_events(|ui| group.chkAll.set_checked(ui, all));
            let ids2 = ids.clone();
            ui.on(group.chkAll.id, "on_change", move |ui, _, _| group_toggle(ui, &ids2));
            with(|st| st.groups.push((ids, group.chkAll)));
        }
        None => group.chkAll.set_visible(ui, false),
    }
    let grid_h = grid_height(n, cols, h);
    group.flowCards.set_height(ui, f64::from(grid_h));
    group.form.set_height(ui, f64::from(group_gap(first) + GROUP_HEAD + grid_h));
    Some(group.flowCards.id)
}

/// O vão antes de um grupo.
fn group_gap(first: bool) -> f32 {
    if first { FIRST_GAP } else { GROUP_GAP }
}

/// Refaz a biblioteca inteira a partir de `lib` + busca + seleção.
fn render(ui: &mut Ui) {
    let today = Local::now().date_naive();
    let cols = columns(ui);
    // Seleção órfã (o arquivo sumiu): sem isso o clique em outro card entraria
    // em "modo seleção" em vez de tocar.
    let (items, groups, folders, meta, low_disk, at_root, crumbs, empty, query) = with(|st| {
        st.cols = cols;
        let ids: HashSet<&str> = st.lib.recordings.iter().map(|r| r.id.as_str()).collect();
        st.selection.retain(|id| ids.contains(id.as_str()));
        let q = st.query.clone();
        let searching = !q.is_empty();
        let mut items: Vec<RecItem> =
            st.lib.recordings.iter().filter(|it| !searching || search_text(it).contains(&q) || st.tr_hits.contains(&it.id)).cloned().collect();
        items.sort_by(|a, b| b.date.cmp(&a.date));
        // Agrupa preservando a ordem (mais novo → mais antigo).
        let mut order: Vec<(String, String)> = Vec::new();
        let mut by_key: HashMap<String, Vec<RecItem>> = HashMap::new();
        for it in &items {
            let (key, label) = period(&it.date, today);
            by_key.entry(key.clone()).or_default().push(it.clone());
            if !order.iter().any(|(k, _)| k == &key) {
                order.push((key, label));
            }
        }
        order.sort_by(|a, b| a.0.cmp(&b.0));
        let groups: Vec<(String, Vec<RecItem>)> = order.into_iter().map(|(k, label)| (label, by_key.remove(&k).unwrap_or_default())).collect();
        // Pastas: o ".." vem primeiro fora da raiz; na busca, só as que casam.
        let mut folders: Vec<(String, String, String, bool)> = Vec::new();
        if !st.lib.at_root && !searching {
            folders.push((st.lib.parent_dir.clone(), "..".into(), lang::tr("Voltar"), true));
        }
        for f in &st.lib.folders {
            let meta = folder_meta(f.count, &f.size_text);
            if searching && !format!("{} {}", f.name, meta).to_lowercase().contains(&q) {
                continue;
            }
            folders.push((f.id.clone(), f.name.clone(), meta, false));
        }
        let crumbs: Vec<(String, String)> = st
            .lib
            .breadcrumb
            .iter()
            .map(|c| (c.id.clone(), if c.root { lang::tr("Gravações") } else { c.name.clone() }))
            .collect();
        let total: i64 = items.iter().map(|it| it.size).sum();
        let mut meta = vec![file_count_text(items.len() as i64)];
        if total > 0 {
            meta.push(lang::tr_with("{{size}} usados", &[("size", &format_bytes(total))]));
        }
        let free = st.lib.free_bytes;
        if free >= 0 {
            meta.push(lang::tr_with("{{size}} livres", &[("size", &format_bytes(free))]));
        }
        let empty = searching && items.is_empty() && folders.is_empty();
        (items, groups, folders, meta, free >= 0 && free < LOW_DISK_BYTES, st.lib.at_root, crumbs, empty, q)
    });
    let _ = items;
    if let Some(frm) = FrmMain::components(ui) {
        frm.lblLibMeta.set_text(ui, meta.join("   |   "));
        frm.lblLibMeta.set_color(ui, if low_disk { "amber" } else { "" });
        frm.lblLibMeta.set_hint(ui, if low_disk { lang::tr("Espaço em disco abaixo de 5 GB na pasta de gravações. Pode não dar pra fazer uma gravação completa sem liberar espaço.") } else { String::new() });
    }
    // O caminho, fora da raiz.
    if let Some(frm) = FrmMain::components(ui) {
        frm.brdLib.set_visible(ui, !at_root);
        frm.brdLib.set_items(ui, crumbs.iter().map(|(_, l)| l.as_str()).collect::<Vec<_>>().join(";"));
    }
    with(|st| st.crumbs = crumbs.iter().map(|(id, _)| id.clone()).collect());
    if let Some(frm) = FrmMain::components(ui) {
        frm.lblLibEmpty.set_visible(ui, empty);
        frm.lblLibEmpty.set_text(ui, if empty { lang::tr_with("Nenhuma gravação encontrada.", &[("query", &query)]) } else { String::new() });
    }

    let Some(content) = FrmMain::components(ui).map(|frm| frm.lyLibContent.id) else { return };
    // Uma passada só pela árvore (o `free` um a um faz uma por grupo).
    let old = ui.children(content).to_vec();
    ui.free_many(&old);
    with(|st| {
        st.cards.clear();
        st.groups.clear();
    });
    let mut height = 0.0;
    let mut first = true;
    if !folders.is_empty() {
        if let Some(grid) = add_group(ui, content, &lang::tr("Pastas"), None, first, folders.len(), cols, FOLDER_H) {
            for (id, name, meta, up) in &folders {
                add_folder_card(ui, grid, id, name, meta, *up);
            }
        }
        height += group_gap(first) + GROUP_HEAD + grid_height(folders.len(), cols, FOLDER_H);
        first = false;
    }
    for (label, list) in &groups {
        let ids: Vec<String> = list.iter().map(|it| it.id.clone()).collect();
        let all = with(|st| ids.iter().all(|id| st.selection.contains(id)));
        if let Some(grid) = add_group(ui, content, label, Some((all, ids)), first, list.len(), cols, CARD_H) {
            for it in list {
                add_rec_card(ui, grid, it);
            }
        }
        height += group_gap(first) + GROUP_HEAD + grid_height(list.len(), cols, CARD_H);
        first = false;
    }
    let crumb_h = if at_root { 0.0 } else { 42.0 };
    let empty_h = if empty { 60.0 } else { 0.0 };
    if let Some(frm) = FrmMain::components(ui) {
        frm.lyLibContent.set_height(ui, f64::from(height));
        // 20 + título 22 + 10 + ações 30, o caminho, o aviso, os grupos e 30 embaixo.
        frm.lyLibrary.set_height(ui, f64::from(20.0 + 22.0 + 10.0 + 30.0 + crumb_h + empty_h + height + 30.0));
    }
    sync_bar(ui);
    ui.layout();
}

/// O cartão de uma gravação (o frame `fraRecCard`). A miniatura vem do cache (o
/// JPG que o core gerou). Frame por cartão custa 2,4× um `MediaCard` criado em código
/// (refazer 500: 10,3 × 4,3 ms em release; criar e liberar a instância, ~8 µs cada),
/// imperceptível e com a interface declarada em JSON (`bench_500_cards`).
fn add_rec_card(ui: &mut Ui, grid: ComponentId, it: &RecItem) {
    let Ok(frame) = FraRecCard::create_in(ui, grid) else { return };
    let card = frame.mcCard;
    let (order, cut, selection_mode) = with(|st| {
        (st.selection.iter().position(|x| x == &it.id).map_or(0, |p| p + 1), st.clipboard.contains(&it.id), !st.selection.is_empty())
    });
    card.set_title(ui, it.name.as_str());
    card.set_subtitle(ui, it.size_text.as_str());
    card.set_duration(ui, format_duration(it.duration));
    card.set_badges(ui, badges(it));
    card.set_hint(ui, format_full_date(&it.date));
    card.set_selected(ui, order > 0);
    card.set_order(ui, order as i64);
    card.set_dimmed(ui, cut);
    card.set_selection_mode(ui, selection_mode);
    let id = it.id.clone();
    ui.on(card.id, "on_drag_start", move |ui, _, _| drag_start(ui, &id));
    ui.on(card.id, "on_drag_end", |ui, _, _| drag_end(ui));
    if let Some(image) = thumb_for(&it.id, &it.thumb) {
        // A miniatura é uma imagem decodificada, não uma propriedade.
        if let Some(mc) = ui.get_mut::<MediaCard>(card.id) {
            mc.set_image(Some(image));
        }
    }
    // O clique no nome abre o campo de renomear do próprio card (Enter ou clicar fora
    // grava, Esc cancela); o resto do card abre o vídeo.
    card.set_editable_title(ui, true);
    let id = it.id.clone();
    ui.on(card.id, "on_rename", move |ui, _, args| {
        if let EventArgs::Text(name) = args {
            rename_item(ui, &id, name, false);
        }
    });
    let id = it.id.clone();
    ui.on(card.id, "on_click", move |ui, _, _| card_clicked(ui, &id));
    let id = it.id.clone();
    ui.on(card.id, "on_select", move |ui, _, _| toggle_select(ui, &id));
    let id = it.id.clone();
    ui.on(card.id, "on_context", move |ui, sender, args| {
        if let EventArgs::Pointer(e) = args {
            open_context(ui, sender, "recording", &id, e.position);
        }
    });
    with(|st| st.cards.insert(it.id.clone(), card));
}

/// O cartão de uma pasta (o frame `fraFolderCard`), ou o ".." (`up`), que leva para
/// a de cima. Pasta recebe o que é solto nela; as outras também se arrastam.
fn add_folder_card(ui: &mut Ui, grid: ComponentId, id: &str, name: &str, meta: &str, up: bool) {
    let Ok(frame) = FraFolderCard::create_in(ui, grid) else { return };
    let handle = frame.mcCard;
    handle.set_title(ui, name);
    handle.set_subtitle(ui, meta);
    if up {
        handle.set_icon(ui, "arrow-up");
        handle.set_hint(ui, lang::tr("Voltar para a pasta acima"));
        handle.set_draggable(ui, false);
    } else {
        handle.set_hint(ui, id);
        handle.set_dimmed(ui, with(|st| st.clipboard.iter().any(|c| c == id)));
    }
    let card = handle.id;
    let target = id.to_owned();
    ui.on(card, "on_drag_over", move |ui, _, _| {
        if !can_drop(&target) {
            ui.reject_drop();
        }
    });
    let target = id.to_owned();
    ui.on(card, "on_drag_drop", move |ui, _, _| drop_on_folder(ui, &target));
    if !up {
        let source = id.to_owned();
        ui.on(card, "on_drag_start", move |ui, _, _| drag_start(ui, &source));
        ui.on(card, "on_drag_end", |ui, _, _| drag_end(ui));
    }
    let target = id.to_owned();
    ui.on(card, "on_click", move |ui, _, _| {
        let _ = ui;
        bridge::handle_open_folder(&target);
    });
    if !up {
        let target = id.to_owned();
        ui.on(card, "on_context", move |ui, sender, args| {
            if let EventArgs::Pointer(e) = args {
                open_context(ui, sender, "folder", &target, e.position);
            }
        });
        // Pasta: o clique (no nome também) entra nela; renomear vem do menu e da pasta
        // nova, pelo `rename()` do card.
        let target = id.to_owned();
        ui.on(card, "on_rename", move |ui, _, args| {
            if let EventArgs::Text(name) = args {
                rename_item(ui, &target, name, true);
            }
        });
        with(|st| st.cards.insert(id.to_owned(), handle));
    }
}

/// A miniatura decodificada (guardada por ordem de uso).
fn thumb_for(id: &str, path: &str) -> Option<Image> {
    let hit = with(|st| {
        st.thumb_clock += 1;
        let clock = st.thumb_clock;
        st.thumbs.get_mut(id).map(|e| {
            e.1 = clock;
            e.0.clone()
        })
    });
    if hit.is_some() || path.is_empty() {
        return hit;
    }
    let bytes = std::fs::read(path).ok()?;
    let image = Image::encoded(bytes);
    with(|st| {
        st.thumb_clock += 1;
        let clock = st.thumb_clock;
        st.thumbs.insert(id.to_owned(), (image.clone(), clock));
        while st.thumbs.len() > THUMB_CACHE_MAX {
            let Some(old) = st.thumbs.iter().min_by_key(|(_, (_, c))| *c).map(|(k, _)| k.clone()) else { break };
            st.thumbs.remove(&old);
        }
    });
    Some(image)
}

/// Botões da barra conforme a seleção.
/// Um print (PNG) na lista: abre fora do player e fica sem as ações de vídeo.
fn is_print(id: &str) -> bool {
    crate::screenshot::is_print_file(id)
}

fn sync_bar(ui: &mut Ui) {
    let (n, busy, prints) = with(|st| (st.selection.len(), st.merge_busy, st.selection.iter().any(|id| is_print(id))));
    let delete_hint = match n {
        0 => lang::tr("Selecione gravações para excluir"),
        1 => lang::tr("Excluir 1 gravação selecionada"),
        _ => lang::tr_with("Excluir {{count}} gravações selecionadas", &[("count", &n.to_string())]),
    };
    let merge_hint = if busy {
        lang::tr("Unindo vídeos...")
    } else if prints {
        lang::tr("Prints não se unem: selecione só vídeos")
    } else if n < 2 {
        lang::tr("Selecione 2 ou mais gravações para unir")
    } else {
        lang::tr_with("Unir {{count}} gravações selecionadas", &[("count", &n.to_string())])
    };
    if let Some(frm) = FrmMain::components(ui) {
        frm.btnLibDelete.set_enabled(ui, n > 0);
        frm.btnLibDelete.set_hint(ui, delete_hint);
        frm.btnLibMerge.set_enabled(ui, n >= 2 && !busy && !prints);
        frm.btnLibMerge.set_hint(ui, merge_hint);
    }
    // Exportar opera sobre UMA gravação, e uma exportação por vez.
    if let Some(frm) = FrmMain::components(ui) {
        frm.btnLibExport.set_enabled(ui, n == 1 && !prints && crate::app_export::running_id().is_empty());
        let hint = if prints {
            lang::tr("Prints não se exportam: selecione um vídeo")
        } else if n == 1 {
            lang::tr("Exportar gravação selecionada")
        } else {
            lang::tr("Selecione uma gravação para exportar")
        };
        frm.btnLibExport.set_hint(ui, hint);
    }
}

/// Só as marcas de seleção/recorte, sem refazer os cartões.
fn refresh_marks(ui: &mut Ui) {
    let (cards, selection, clipboard, groups) = with(|st| (st.cards.clone(), st.selection.clone(), st.clipboard.clone(), st.groups.clone()));
    let mode = !selection.is_empty();
    for (id, card) in cards {
        let order = selection.iter().position(|x| x == &id).map_or(0, |p| p + 1);
        // Direto nos campos (uma repintura no fim, não uma por propriedade).
        if let Some(mc) = ui.get_mut::<MediaCard>(card.id) {
            mc.selected = order > 0;
            mc.order = order as u32;
            mc.selection_mode = mode && !mc.compact;
            mc.dimmed = clipboard.contains(&id);
        }
    }
    for (ids, b) in groups {
        let all = !ids.is_empty() && ids.iter().all(|id| selection.contains(id));
        // Sem o `on_change`: mudar o valor pelo código também o dispara (como na VCL), e o
        // handler da caixa inverte a seleção do grupo — marcar à mão o último item do grupo
        // virava um laço (marca todos, desmarca todos...) até o limite de recursão.
        ui.without_change_events(|ui| b.set_checked(ui, all));
    }
    sync_bar(ui);
    ui.invalidate();
}

/// A exportação começou/terminou: o botão "Exportar" acompanha.
pub fn sync_export_button() {
    with_ui(sync_bar);
}

/// Exportação concluída: a seleção (a gravação de origem) sai.
pub fn clear_selection() {
    with(|st| st.selection.clear());
    with_ui(refresh_marks);
}

/// A barra de progresso da exportação no cartão da gravação de origem.
pub fn set_export_progress(source: &str, pct: Option<f64>, label: &str) {
    let (source, label) = (source.to_owned(), label.to_owned());
    with_ui(move |ui| {
        let Some(card) = with(|st| st.cards.get(&source).copied()) else { return };
        if let Some(mc) = ui.get_mut::<MediaCard>(card.id) {
            mc.progress = pct.map_or(-1.0, |p| (p / 100.0) as f32);
            mc.progress_text = label;
        }
        ui.invalidate();
    });
}

// ---------------------------------------------------------------------------
// Do core (View)
// ---------------------------------------------------------------------------

pub fn on_library(lib: &Library) {
    let lib = lib.clone();
    with_ui(move |ui| on_library_now(ui, lib));
}

/// Refazer os cartões fecharia o campo de quem está renomeando: a lista nova espera o
/// fim da edição.
fn on_library_now(ui: &mut Ui, lib: Library) {
    if renaming(ui) {
        with(|st| st.pending = Some(lib));
        apply_pending_after_rename(ui);
        return;
    }
    with(|st| set_lib(st, lib));
    render(ui);
}

pub fn on_recording_added(item: &RecItem, free: i64) {
    let item = item.clone();
    with_ui(move |ui| {
        with(|st| {
            if !st.lib.recordings.iter().any(|r| r.id == item.id) {
                st.lib.recordings.push(item);
            }
            st.lib.free_bytes = free;
        });
        render(ui);
    });
}

pub fn on_recording_meta(id: &str, duration: i32, thumb: &str) {
    let (id, thumb) = (id.to_owned(), thumb.to_owned());
    with_ui(move |ui| {
        let found = with(|st| {
            let Some(r) = st.lib.recordings.iter_mut().find(|r| r.id == id) else { return false };
            if duration > 0 {
                r.duration = duration;
            }
            if !thumb.is_empty() {
                r.thumb = thumb.clone();
                st.thumbs.remove(&id);
            }
            true
        });
        if !found {
            return;
        }
        let Some(card) = with(|st| st.cards.get(&id).copied()) else { return };
        let image = if thumb.is_empty() { None } else { thumb_for(&id, &thumb) };
        if let Some(mc) = ui.get_mut::<MediaCard>(card.id) {
            if duration > 0 {
                mc.duration = format_duration(duration);
            }
            if image.is_some() {
                mc.set_image(image);
            }
        }
        ui.invalidate();
    });
}

pub fn on_recording_renamed(old_id: &str, new_id: &str, new_name: &str) {
    let (old_id, new_id, new_name) = (old_id.to_owned(), new_id.to_owned(), new_name.to_owned());
    with_ui(move |ui| {
        with(|st| {
            if let Some(r) = st.lib.recordings.iter_mut().find(|r| r.id == old_id) {
                r.id = new_id.clone();
                r.name = new_name.clone();
            }
            for s in st.selection.iter_mut().chain(st.clipboard.iter_mut()) {
                if *s == old_id {
                    *s = new_id.clone();
                }
            }
            if let Some(t) = st.thumbs.remove(&old_id) {
                st.thumbs.insert(new_id.clone(), t);
            }
        });
        render(ui);
    });
}

pub fn on_recording_removed(id: &str, free: i64) {
    let id = id.to_owned();
    with_ui(move |ui| {
        with(|st| {
            st.lib.recordings.retain(|r| r.id != id);
            st.selection.retain(|s| *s != id);
            st.thumbs.remove(&id);
            if free >= 0 {
                st.lib.free_bytes = free;
            }
        });
        render(ui);
    });
}

pub fn on_folder_created(id: &str) {
    let id = id.to_owned();
    with_ui(move |ui| start_rename(ui, &id));
}

pub fn on_transcript_search(query: &str, ids: &[String]) {
    let (query, ids) = (query.to_owned(), ids.to_vec());
    with_ui(move |ui| {
        let current = with(|st| {
            if query.to_lowercase().trim() != st.query {
                return false;
            }
            st.tr_hits = ids.into_iter().collect();
            true
        });
        if current {
            render(ui);
        }
    });
}

pub fn on_merge_pending() {
    with(|st| st.merge_busy = true);
    with_ui(sync_bar);
    crate::app_view::show_toast(&lang::tr("Unindo vídeos"), &lang::tr("Criando um novo arquivo com os vídeos selecionados."), false, 3000);
}

pub fn on_merge_done(ok: bool) {
    with(|st| {
        st.merge_busy = false;
        if ok {
            st.selection.clear();
        }
    });
    with_ui(refresh_marks);
    if ok {
        crate::app_view::show_toast(&lang::tr("Vídeos unidos"), &lang::tr("Novo vídeo criado. Os originais foram para a lixeira."), false, 5000);
    }
}

// ---------------------------------------------------------------------------
// Gestos
// ---------------------------------------------------------------------------

/// O item sob o clique e, se ele está numa seleção de vários, todos eles.
fn target_ids(id: &str) -> Vec<String> {
    with(|st| if st.selection.len() > 1 && st.selection.iter().any(|s| s == id) { st.selection.clone() } else { vec![id.to_owned()] })
}

fn is_folder(id: &str) -> bool {
    with(|st| st.lib.folders.iter().any(|f| f.id.eq_ignore_ascii_case(id)))
}

// ----- arrastar -----

/// Começa a arrastar o cartão (com a seleção, se ele estiver nela). A lixeira acende.
fn drag_start(ui: &mut Ui, id: &str) {
    if renaming(ui) {
        return;
    }
    let ids = target_ids(id);
    let label = with(|st| {
        let name_of = |id: &str| {
            st.lib
                .recordings
                .iter()
                .find(|r| r.id == id)
                .map(|r| r.name.clone())
                .or_else(|| st.lib.folders.iter().find(|f| f.id == id).map(|f| f.name.clone()))
                .unwrap_or_default()
        };
        let first = name_of(&ids[0]);
        if ids.len() > 1 { format!("{first} +{}", ids.len() - 1) } else { first }
    });
    with(|st| st.drag = ids.clone());
    ui.begin_drag(ids.join("\n"), label);
    // A miniatura do primeiro vai junto do cursor (no Windows, por cima de tudo); a pasta,
    // sem miniatura, leva o desenho do próprio cartão.
    let (thumb, card) = with(|st| (st.thumbs.get(&ids[0]).map(|(image, _)| image.clone()), st.cards.get(&ids[0]).map(|c| c.id)));
    let visual = match thumb {
        Some(thumb) => Some(DragVisual::new(thumb, SizeF::new(DRAG_THUMB_W, DRAG_THUMB_W * 9.0 / 16.0))),
        None => card.and_then(|card| {
            let size = ui.control(card).map(|cb| SizeF::new(cb.width(), cb.height()))?;
            Some(DragVisual::new(ui.snapshot(card, 2.0)?, size))
        }),
    };
    ui.set_drag_image(visual);
    if let Some(frm) = FrmMain::components(ui) {
        frm.btnLibDelete.set_enabled(ui, true);
        frm.btnLibDelete.set_text_color(ui, "red");
    }
}

/// Acabou (solto ou cancelado): a barra volta ao normal.
fn drag_end(ui: &mut Ui) {
    with(|st| st.drag.clear());
    if let Some(frm) = FrmMain::components(ui) {
        frm.btnLibDelete.set_text_color(ui, "");
    }
    sync_bar(ui);
}

/// Nem a própria pasta, nem pasta dentro dela mesma, nem onde já está.
fn can_drop(target: &str) -> bool {
    let t = target.to_lowercase();
    if t.is_empty() {
        return false;
    }
    with(|st| {
        !st.drag.is_empty()
            && st.drag.iter().all(|s| {
                let s = s.to_lowercase();
                if s == t || t.starts_with(&format!("{s}\\")) {
                    return false;
                }
                !matches!(s.rfind('\\'), Some(cut) if cut > 0 && s[..cut] == t)
            })
    })
}

fn drop_on_folder(ui: &mut Ui, target: &str) {
    let ids = with(|st| st.drag.clone());
    if ids.is_empty() || !can_drop(target) || export_blocked(&ids) {
        return;
    }
    // Solto: a seleção não descreve mais o que está na tela.
    with(|st| st.selection.clear());
    refresh_marks(ui);
    bridge::handle_move_items(&ids, target);
}

/// Solto na lixeira: pasta tem confirmação própria (avisa quantas gravações vão
/// junto); arrastar uma pasta leva só ela.
pub fn drop_on_trash(ui: &mut Ui) {
    let ids = with(|st| st.drag.clone());
    if ids.is_empty() {
        return;
    }
    if ids.len() == 1 && is_folder(&ids[0]) {
        delete_folder(ui, &ids[0]);
    } else {
        delete_recordings(ui, ids.into_iter().filter(|x| !is_folder(x)).collect());
    }
}

fn export_blocked(ids: &[String]) -> bool {
    if ids.iter().any(|id| bridge::export_in_use(id)) {
        crate::app_view::show_toast(&lang::tr("Erro"), &lang::tr("Esta gravação está sendo exportada agora — espere terminar ou cancele a exportação."), true, 6000);
        return true;
    }
    false
}

fn toggle_select(ui: &mut Ui, id: &str) {
    with(|st| {
        if let Some(p) = st.selection.iter().position(|s| s == id) {
            st.selection.remove(p);
        } else {
            st.selection.push(id.to_owned());
        }
    });
    refresh_marks(ui);
}

fn group_toggle(ui: &mut Ui, ids: &[String]) {
    with(|st| {
        let all = ids.iter().all(|id| st.selection.contains(id));
        for id in ids {
            let has = st.selection.contains(id);
            if all && has {
                st.selection.retain(|s| s != id);
            } else if !all && !has {
                st.selection.push(id.clone());
            }
        }
    });
    refresh_marks(ui);
}

/// Com algo selecionado, o clique seleciona; senão abre (o vídeo no player, o print no
/// visualizador). O clique no nome não chega aqui: o card abre o campo de renomear.
fn card_clicked(ui: &mut Ui, id: &str) {
    if with(|st| !st.selection.is_empty()) {
        toggle_select(ui, id);
        return;
    }
    if is_print(id) {
        crate::app_viewer::open(ui, id);
        return;
    }
    crate::app_player::open(ui, id);
}

/// Os prints da lista como ela está na tela (a busca e a ordem, do mais novo ao mais
/// antigo): o ← e o → do visualizador andam por eles.
pub fn print_ids() -> Vec<String> {
    with(|st| {
        let q = st.query.clone();
        let searching = !q.is_empty();
        let mut items: Vec<&RecItem> = st
            .lib
            .recordings
            .iter()
            .filter(|it| is_print(&it.id))
            .filter(|it| !searching || search_text(it).contains(&q) || st.tr_hits.contains(&it.id))
            .collect();
        items.sort_by(|a, b| b.date.cmp(&a.date));
        items.into_iter().map(|it| it.id.clone()).collect()
    })
}

pub fn search(ui: &mut Ui, text: &str) {
    let q = text.to_lowercase().trim().to_owned();
    let (changed, job) = with(|st| {
        if st.query == q {
            return (false, None);
        }
        st.query = q.clone();
        st.tr_hits.clear();
        (true, st.search_job.take())
    });
    if !changed {
        return;
    }
    if let Some(job) = job {
        ui.clear_timeout(job);
    }
    if q.is_empty() {
        bridge::handle_search_transcripts("");
    } else {
        let q2 = q.clone();
        let job = ui.set_timeout(SEARCH_DEBOUNCE_MS, move |_| bridge::handle_search_transcripts(&q2));
        with(|st| st.search_job = Some(job));
    }
    render(ui);
}

fn cut(ui: &mut Ui, ids: Vec<String>) {
    if ids.is_empty() {
        return;
    }
    let n = ids.len();
    with(|st| st.clipboard = ids);
    refresh_marks(ui);
    // As pastas recortadas também apagam (a marca está nos cartões de pasta).
    let msg = if n == 1 { lang::tr("1 item pronto pra colar em outra pasta.") } else { lang::tr_with("{{count}} itens prontos pra colar em outra pasta.", &[("count", &n.to_string())]) };
    crate::app_view::show_toast(&lang::tr("Recortado"), &msg, false, 3500);
}

fn paste(ui: &mut Ui, target: &str) {
    let ids = with(|st| st.clipboard.clone());
    if ids.is_empty() || export_blocked(&ids) {
        return;
    }
    with(|st| st.clipboard.clear());
    refresh_marks(ui);
    bridge::handle_move_items(&ids, target);
}

/// O diálogo de confirmação (o `confirm` do aylis); `on_ok` roda no OK.
fn confirm(ui: &mut Ui, title: String, message: String, ok: String, danger: bool, on_ok: impl FnOnce(&mut Ui) + 'static) {
    let icon = if danger { crate::app_view::ConfirmIcon::Delete } else { crate::app_view::ConfirmIcon::Merge };
    crate::app_view::confirm(ui, &title, &message, &ok, danger, icon, on_ok);
}

fn do_delete(ui: &mut Ui, ids: Vec<String>) {
    with(|st| {
        st.lib.recordings.retain(|r| !ids.contains(&r.id));
        st.selection.retain(|s| !ids.contains(s));
    });
    render(ui);
    for id in &ids {
        bridge::handle_delete_recording(id);
    }
}

fn delete_recordings(ui: &mut Ui, ids: Vec<String>) {
    let ids: Vec<String> = ids.into_iter().filter(|s| !s.is_empty()).collect();
    if ids.is_empty() || export_blocked(&ids) {
        return;
    }
    let msg = if ids.len() == 1 {
        lang::tr("Mover esta gravação para a Lixeira do Windows?")
    } else {
        lang::tr_with("Mover {{count}} gravações para a Lixeira do Windows?", &[("count", &ids.len().to_string())])
    };
    confirm(ui, lang::tr("Excluir gravações"), msg, lang::tr("Excluir"), true, move |ui| do_delete(ui, ids));
}

fn delete_one(ui: &mut Ui, id: &str) {
    if export_blocked(&[id.to_owned()]) {
        return;
    }
    let name = with(|st| st.lib.recordings.iter().find(|r| r.id == id).map(|r| r.name.clone())).unwrap_or_else(|| id.to_owned());
    let ids = vec![id.to_owned()];
    confirm(
        ui,
        lang::tr("Excluir gravação"),
        lang::tr_with("Mover \"{{name}}\" para a Lixeira do Windows?", &[("name", &name)]),
        lang::tr("Excluir"),
        true,
        move |ui| do_delete(ui, ids),
    );
}

fn delete_folder(ui: &mut Ui, id: &str) {
    let (name, count) = with(|st| st.lib.folders.iter().find(|f| f.id == id).map(|f| (f.name.clone(), f.count)).unwrap_or((id.to_owned(), 0)));
    let msg = match count {
        0 => lang::tr_with("Mover a pasta \"{{name}}\" para a Lixeira do Windows?", &[("name", &name)]),
        1 => lang::tr_with("A pasta \"{{name}}\" tem 1 gravação dentro. Mover a pasta e tudo que está nela para a Lixeira do Windows?", &[("name", &name), ("count", "1")]),
        n => lang::tr_with("A pasta \"{{name}}\" tem {{count}} gravações dentro. Mover a pasta e tudo que está nela para a Lixeira do Windows?", &[("name", &name), ("count", &n.to_string())]),
    };
    let id = id.to_owned();
    confirm(ui, lang::tr("Excluir pasta"), msg, lang::tr("Excluir"), true, move |_| {
        with(|st| st.clipboard.retain(|c| *c != id));
        bridge::handle_delete_folder(&id);
    });
}

pub fn merge_selected(ui: &mut Ui) {
    let ids = with(|st| st.selection.clone());
    if ids.len() < 2 || export_blocked(&ids) {
        return;
    }
    let msg = lang::tr_with("Criar um novo vídeo unindo {{count}} gravações na ordem selecionada? Os originais irão para a lixeira (recuperáveis).", &[("count", &ids.len().to_string())]);
    confirm(ui, lang::tr("Unir gravações"), msg, lang::tr("Unir"), false, move |ui| {
        with(|st| st.merge_busy = true);
        sync_bar(ui);
        bridge::handle_merge_recordings(&ids);
    });
}

pub fn delete_selected(ui: &mut Ui) {
    let ids = with(|st| st.selection.clone());
    delete_recordings(ui, ids);
}

pub fn export_selected(ui: &mut Ui) {
    let ids = with(|st| st.selection.clone());
    if ids.len() == 1 {
        crate::app_export::open_for(ui, &ids[0]);
    }
}

// ----- renomear -----

fn clean_name(s: &str) -> String {
    let s: String = s.replace(['\r', '\n'], " ").chars().filter(|c| !"\\/:*?\"<>|".contains(*c)).collect();
    s.trim().chars().take(150).collect()
}

/// Algum cartão está com o campo de renomear aberto (o do próprio `MediaCard`)?
fn renaming(ui: &Ui) -> bool {
    let cards: Vec<g::MediaCard> = with(|st| st.cards.values().copied().collect());
    cards.iter().any(|c| ui.get::<MediaCard>(c.id).is_some_and(MediaCard::is_renaming))
}

/// Abre o campo de renomear do cartão `id` (pasta nova, "Renomear" do menu).
fn start_rename(ui: &mut Ui, id: &str) {
    if let Some(card) = with(|st| st.cards.get(id).copied()) {
        let _ = card.rename(ui);
    }
}

/// A lista que chegou durante uma edição de nome: aplica quando o campo fechar (gravou,
/// cancelou com Esc ou o clique fora).
fn apply_pending_after_rename(ui: &mut Ui) {
    ui.set_timeout(150, |ui| {
        if with(|st| st.pending.is_none()) {
            return;
        }
        if renaming(ui) {
            apply_pending_after_rename(ui);
            return;
        }
        if let Some(lib) = with(|st| st.pending.take()) {
            with(|st| set_lib(st, lib));
            render(ui);
        }
    });
}

/// O `on_rename` do cartão: o nome novo (o card já o mostra). Grava no disco.
fn rename_item(ui: &mut Ui, id: &str, name: &str, folder: bool) {
    let new_name = clean_name(name);
    let original = with(|st| {
        if folder {
            st.lib.folders.iter().find(|f| f.id == id).map(|f| f.name.clone())
        } else {
            st.lib.recordings.iter().find(|r| r.id == id).map(|r| r.name.clone())
        }
    })
    .unwrap_or_default();
    let card = with(|st| st.cards.get(id).copied());
    if new_name.is_empty() || new_name == original || (!folder && export_blocked(&[id.to_owned()])) {
        // Recusado (vazio, igual, em exportação): o card volta ao nome de antes.
        if let Some(c) = card {
            c.set_title(ui, original.as_str());
        }
        return;
    }
    if folder {
        bridge::handle_rename_folder(id, &new_name);
    } else {
        with(|st| {
            if let Some(r) = st.lib.recordings.iter_mut().find(|r| r.id == id) {
                r.name = new_name.clone();
            }
        });
        if let Some(c) = card {
            c.set_title(ui, new_name.as_str());
        }
        bridge::handle_rename_recording(id, &new_name);
    }
}

// ----- menu de contexto -----

/// Abre o menu do tipo (`pmLibRecording`, `pmLibFolder` ou `pmLibEmpty`, no
/// `forms/main.json`): os itens são fixos, aqui só se ajusta o que depende do alvo
/// (quantos selecionados, se há o que colar, se está na raiz).
fn open_context(ui: &mut Ui, card: ComponentId, kind: &str, id: &str, local: PointF) {
    let Some(frm) = FrmMain::components(ui) else { return };
    let can_paste = with(|st| !st.clipboard.is_empty());
    let menu = match kind {
        "recording" => {
            let ids = target_ids(id);
            let bulk = ids.len() > 1;
            let busy = !crate::app_export::running_id().is_empty();
            let count = ids.len().to_string();
            // Print: sem exportar nem transcrever (não é vídeo).
            let prints = ids.iter().any(|i| is_print(i));
            frm.miRecExport.set_enabled(ui, !bulk && !busy && !prints);
            frm.miRecTranscribe.set_enabled(ui, !prints);
            frm.miRecCut.set_text(ui, if bulk { lang::tr_with("Recortar {{count}} selecionadas", &[("count", &count)]) } else { lang::tr("Recortar") });
            frm.miRecDelete.set_text(ui, if bulk { lang::tr_with("Excluir {{count}} selecionadas", &[("count", &count)]) } else { lang::tr("Excluir") });
            frm.pmLibRecording.id
        }
        "folder" => {
            frm.miFolderPaste.set_enabled(ui, can_paste);
            frm.pmLibFolder.id
        }
        _ => {
            frm.miEmptyPaste.set_enabled(ui, can_paste);
            let below_root = !with(|st| st.lib.at_root);
            frm.miEmptySep.set_visible(ui, below_root);
            frm.miEmptyUp.set_visible(ui, below_root);
            frm.pmLibEmpty.id
        }
    };
    with(|st| st.ctx = (kind.to_owned(), id.to_owned()));
    if let Some(cb) = ui.control_mut(card) {
        cb.set_popup_menu(Some(menu));
    }
    let origin = ui.absolute_rect(card).map_or(PointF::ZERO, |r| PointF::new(r.x, r.y));
    ui.open_popup_menu(card, PointF::new(origin.x + local.x, origin.y + local.y));
}

/// Um item dos menus `pmLib*` (o handler `library_menu` do `frmMain`): a ação de cada um.
pub fn menu_click(ui: &mut Ui, item: ComponentId) {
    let Some(frm) = FrmMain::components(ui) else { return };
    let actions = [
        (frm.miRecExport, "export"),
        (frm.miRecCut, "cut"),
        (frm.miRecTranscribe, "transcribe"),
        (frm.miRecDelete, "delete"),
        (frm.miFolderOpen, "open"),
        (frm.miFolderRename, "rename"),
        (frm.miFolderCut, "cut-folder"),
        (frm.miFolderPaste, "paste-into"),
        (frm.miFolderDelete, "delete-folder"),
        (frm.miEmptyNewFolder, "new-folder"),
        (frm.miEmptyPaste, "paste"),
        (frm.miEmptyUp, "up"),
    ];
    if let Some((_, action)) = actions.into_iter().find(|(mi, _)| mi.id == item) {
        ctx_run(ui, action);
    }
}

fn ctx_run(ui: &mut Ui, action: &str) {
    let (_, id) = with(|st| st.ctx.clone());
    match action {
        "export" => crate::app_export::open_for(ui, &id),
        "cut" => cut(ui, target_ids(&id)),
        "transcribe" => bridge::handle_transcribe_recording(&id),
        "delete" => {
            let ids = target_ids(&id);
            if ids.len() > 1 {
                delete_recordings(ui, ids);
            } else {
                delete_one(ui, &id);
            }
        }
        "open" => bridge::handle_open_folder(&id),
        "rename" => start_rename(ui, &id),
        "cut-folder" => cut(ui, vec![id]),
        "paste-into" => paste(ui, &id),
        "delete-folder" => delete_folder(ui, &id),
        "new-folder" => new_folder(ui),
        "paste" => paste(ui, ""),
        "up" => go_up(ui),
        _ => {}
    }
}

/// O fundo da biblioteca: o menu de pasta nova, colar e voltar.
pub fn background_context(ui: &mut Ui, sender: ComponentId, args: &EventArgs) {
    if let EventArgs::Pointer(e) = args {
        if e.button == PointerButton::Secondary {
            open_context(ui, sender, "empty", "", e.position);
        }
    }
}

pub fn new_folder(_ui: &mut Ui) {
    bridge::handle_create_folder(&lang::tr("Nova pasta"));
}

fn go_up(_ui: &mut Ui) {
    let (at_root, parent) = with(|st| (st.lib.at_root, st.lib.parent_dir.clone()));
    if !at_root {
        bridge::handle_open_folder(&parent);
    }
}

pub fn crumb(_ui: &mut Ui, args: &EventArgs) {
    let EventArgs::Value(v) = args else { return };
    let Ok(i) = v.to_i64() else { return };
    let id = with(|st| st.crumbs.get(i as usize).cloned());
    if let Some(id) = id {
        bridge::handle_open_folder(&id);
    }
}

/// A largura mudou: as grades refazem as linhas.
pub fn resized(ui: &mut Ui) {
    let cols = columns(ui);
    if with(|st| st.cols) != cols {
        render(ui);
    }
}

/// Ctrl+X / Ctrl+V / Backspace na biblioteca.
pub fn key(ui: &mut Ui, k: &KeyEvent) -> bool {
    let ctrl = k.modifiers.ctrl;
    match k.key {
        Key::Char('x') | Key::Char('X') if ctrl => {
            let ids = with(|st| st.selection.clone());
            cut(ui, ids);
            true
        }
        Key::Char('v') | Key::Char('V') if ctrl => {
            paste(ui, "");
            true
        }
        Key::Backspace => {
            go_up(ui);
            true
        }
        _ => false,
    }
}

/// Os textos fixos (troca de idioma).
pub fn apply_texts(ui: &mut Ui) {
    if let Some(frm) = FrmMain::components(ui) {
        frm.lblRecordings.set_text(ui, lang::tr("Gravações"));
        frm.edLibSearch.set_placeholder(ui, lang::tr("Buscar..."));
        frm.btnLibOpenDir.set_hint(ui, lang::tr("Abrir pasta de gravações"));
        frm.btnLibNewFolder.set_hint(ui, lang::tr("Nova pasta"));
    }
    let w = ui.measure_text(&lang::tr("Gravações"), &TextStyle::default()).width * 16.0 / 14.0 + 8.0;
    if let Some(frm) = FrmMain::components(ui) {
        frm.lblRecordings.set_width(ui, f64::from(w));
    }
}

pub fn on_language_changed(ui: &mut Ui) {
    apply_texts(ui);
    render(ui);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Entrar em pastas enche o "voltar" e esvazia o "avançar"; a pasta que chega pelo
    /// histórico não entra nele de novo; a mesma pasta (uma atualização) não conta.
    #[test]
    fn historico_de_pastas_como_no_explorer() {
        let dir = |d: &str| Library { current_dir: d.into(), ..Default::default() };
        let mut st = State::default();
        set_lib(&mut st, dir("D:\\V"));
        set_lib(&mut st, dir("D:\\V\\A"));
        set_lib(&mut st, dir("D:\\V\\A\\B"));
        set_lib(&mut st, dir("D:\\V\\A\\B"));
        assert_eq!(st.back, vec!["D:\\V", "D:\\V\\A"]);
        // Voltar (o que o `mouse_history` faz antes de pedir a pasta) e a pasta chega.
        st.forward.push(st.lib.current_dir.clone());
        st.back.pop();
        st.nav_by_history = true;
        set_lib(&mut st, dir("D:\\V\\A"));
        assert_eq!((st.back.clone(), st.forward.clone()), (vec!["D:\\V".to_string()], vec!["D:\\V\\A\\B".to_string()]));
        assert!(!st.nav_by_history);
        // Uma pasta nova pelo caminho normal: o "avançar" esvazia.
        set_lib(&mut st, dir("D:\\V\\C"));
        assert_eq!(st.back, vec!["D:\\V", "D:\\V\\A"]);
        assert!(st.forward.is_empty());
    }

    #[test]
    fn duracao_como_a_web() {
        assert_eq!(format_duration(0), "");
        assert_eq!(format_duration(65), "01:05");
        assert_eq!(format_duration(3725), "01:02:05");
    }

    #[test]
    fn bytes_como_a_web() {
        assert_eq!(format_bytes(512), "512 B");
        assert_eq!(format_bytes(1536), "2 KB");
        assert_eq!(format_bytes(5 * 1024 * 1024), "5 MB");
        assert_eq!(format_bytes(1536 * 1024 * 1024), "1.5 GB");
    }

    #[test]
    fn nome_limpo() {
        assert_eq!(clean_name("  a/b:c*?\"<>|\nd  "), "abc d");
        assert_eq!(clean_name(&"x".repeat(200)).len(), 150);
    }

    #[test]
    fn periodos() {
        let today = NaiveDate::from_ymd_opt(2026, 10, 23).unwrap();
        assert_eq!(period("2026-10-23T08:00:00", today).0, "00-hoje");
        assert_eq!(period("2026-10-22T23:59:59", today).0, "01-ontem");
        assert_eq!(period("2026-10-18T10:00:00", today).0, "02-semana");
        let (k1, _) = period("2026-09-01T10:00:00", today);
        let (k2, _) = period("2025-12-01T10:00:00", today);
        assert!(k1 < k2, "mês mais novo antes");
        assert_eq!(period("lixo", today).0, "zzz-unknown");
    }

    fn app() -> aylis::Application {
        let mut app = aylis::Application::bundled(crate::PROJECT).unwrap();
        crate::generated::register(&mut app.ui);
        app
    }

    /// Uma biblioteca de `n` gravações espalhadas por 10 meses e 3 pastas, fora da raiz.
    fn sample_library(n: usize) -> Library {
        let recordings = (0..n)
            .map(|i| RecItem {
                id: format!("C:\\rec\\v{i}.mp4"),
                name: format!("Vídeo {i}"),
                date: format!("2025-{:02}-10T10:00:00", 1 + i % 10),
                size: 1024 * 1024,
                size_text: "1 MB".into(),
                duration: 65,
                codec: "H.264".into(),
                fps: 60,
                quality: 2,
                transcript: if i % 2 == 0 { "ok".into() } else { String::new() },
                ..Default::default()
            })
            .collect();
        let folders = (0..3).map(|i| crate::view::FolderItem { id: format!("C:\\rec\\p{i}"), name: format!("Pasta {i}"), count: 2, size: 0, size_text: "2 MB".into() }).collect();
        Library { recordings, folders, at_root: false, parent_dir: "C:\\".into(), free_bytes: -1, ..Default::default() }
    }


    /// Renomear é o do próprio `MediaCard`: o cartão de gravação tem `editable_title`, o
    /// `rename()` abre o campo e o `on_rename` grava o nome novo na lista e no cartão.
    #[test]
    fn renomear_usa_o_campo_do_card() {
        let mut app = app();
        let ui = &mut app.ui;
        ui.create_form("frmMain").unwrap();
        with(|st| {
            *st = State::default();
            st.lib = sample_library(2);
        });
        render(ui);
        let first = r"C:\rec\v0.mp4".to_string();
        let card = with(|st| st.cards[&first]);
        assert!(card.editable_title(ui), "o clique no nome do cartão de gravação renomeia");
        assert!(!renaming(ui));
        start_rename(ui, &first);
        assert!(renaming(ui), "o rename() abre o campo do card");
        // Uma lista que chega no meio espera o fim da edição (refazer fecharia o campo).
        let mut lib = sample_library(2);
        lib.recordings[1].name = "Chegou durante".into();
        on_library_now(ui, lib);
        assert!(with(|st| st.pending.is_some()), "a lista nova espera");
        assert!(renaming(ui), "o campo continua aberto");
        // O nome novo, pelo evento do card.
        ui.fire(card.id, "on_rename", &EventArgs::Text("Novo nome".into()));
        assert_eq!(with(|st| st.lib.recordings.iter().find(|r| r.id == first).map(|r| r.name.clone())), Some("Novo nome".into()));
        assert_eq!(card.title(ui), "Novo nome");
        // Vazio volta ao nome de antes.
        ui.fire(card.id, "on_rename", &EventArgs::Text("  ".into()));
        assert_eq!(card.title(ui), "Novo nome");
        with(|st| *st = State::default());
    }

    /// Marcar à mão todos os itens de um grupo marca a caixa "selecionar todos" SEM
    /// disparar o `on_change` dela (que inverteria a seleção: o laço "evento recursivo
    /// demais"); o clique do usuário na caixa continua selecionando e desmarcando o grupo.
    #[test]
    fn caixa_do_grupo_nao_entra_em_laco() {
        use std::cell::Cell;
        use std::rc::Rc;
        let mut app = app();
        let ui = &mut app.ui;
        ui.create_form("frmMain").unwrap();
        with(|st| {
            *st = State::default();
            st.lib = sample_library(4);
        });
        render(ui);
        let (ids, chk) = with(|st| st.groups[0].clone());
        // Conta as vezes que o handler da caixa roda (e faz o que o de verdade faz).
        let runs = Rc::new(Cell::new(0));
        let (r, ids2) = (runs.clone(), ids.clone());
        ui.on(chk.id, "on_change", move |ui, _, _| {
            r.set(r.get() + 1);
            group_toggle(ui, &ids2);
        });
        for id in &ids {
            toggle_select(ui, id);
        }
        assert_eq!(runs.get(), 0, "o código marcou a caixa: o on_change não roda");
        assert!(chk.checked(ui), "todos marcados à mão: a caixa marca");
        assert!(ids.iter().all(|id| with(|st| st.selection.contains(id))), "a seleção fica como o usuário deixou");
        // O clique de verdade na caixa (desmarca o grupo todo).
        ui.click(chk.id);
        assert_eq!(runs.get(), 1, "o clique roda o handler uma vez");
        assert!(ids.iter().all(|id| with(|st| !st.selection.contains(id))), "o clique desmarca o grupo");
        assert!(!chk.checked(ui));
        with(|st| *st = State::default());
    }

    /// Pasta: o clique entra nela (sem `editable_title`); renomear vem do menu.
    #[test]
    fn pasta_renomeia_pelo_menu() {
        let mut app = app();
        let ui = &mut app.ui;
        ui.create_form("frmMain").unwrap();
        with(|st| {
            *st = State::default();
            st.lib = sample_library(1);
        });
        render(ui);
        let pasta = with(|st| st.lib.folders[0].id.clone());
        let card = with(|st| st.cards[&pasta]);
        assert!(!card.editable_title(ui));
        start_rename(ui, &pasta);
        assert!(renaming(ui));
        with(|st| *st = State::default());
    }

    /// Os grupos são instâncias do frame `fraLibGroup` (só o que varia sai do código), e
    /// os cartões, dos frames `fraRecCard`/`fraFolderCard`.
    #[test]
    fn the_groups_are_frames_and_the_cards_follow_the_items() {
        let mut app = app();
        let ui = &mut app.ui;
        ui.create_form("frmMain").unwrap();
        let first = r"C:\rec\v0.mp4".to_string();
        with(|st| {
            *st = State::default();
            st.lib = sample_library(4);
            st.selection = vec![first.clone()];
            st.clipboard = vec![r"C:\rec\p1".into()];
        });
        render(ui);
        let main = FrmMain::components(ui).unwrap();
        let groups: Vec<_> = ui.children(main.lyLibContent.id).iter().map(|&c| FraLibGroup::components_of(ui, c).unwrap()).collect();
        // Pastas + 4 meses.
        assert_eq!(groups.len(), 5);
        assert!(!groups[0].chkAll.visible(ui), "pasta não tem 'selecionar todos'");
        assert!(groups[1].chkAll.visible(ui));
        assert_eq!(groups[0].lblTitle.text(ui), lang::tr("Pastas"));
        // O primeiro vão é menor que o dos outros grupos.
        assert_eq!(ui.control(groups[0].lyHead.id).unwrap().margins().top, FIRST_GAP);
        assert_eq!(ui.control(groups[1].lyHead.id).unwrap().margins().top, GROUP_GAP);
        let cols = with(|st| st.cols);
        assert_eq!(groups[0].form.height(ui), f64::from(FIRST_GAP + GROUP_HEAD + grid_height(4, cols, FOLDER_H)));
        assert_eq!(groups[0].flowCards.height(ui), f64::from(grid_height(4, cols, FOLDER_H)));

        // ".." e as 3 pastas; o ".." não arrasta nem entra em `cards`.
        let folders: Vec<_> = ui.children(groups[0].flowCards.id).iter().map(|&c| FraFolderCard::components_of(ui, c).unwrap().mcCard).collect();
        assert_eq!(folders.len(), 4);
        assert_eq!(folders[0].icon(ui), "arrow-up");
        assert!(!folders[0].draggable(ui));
        assert_eq!(folders[1].icon(ui), "folder");
        assert!(folders[1].compact(ui) && folders[1].accepts_drops(ui) && folders[1].draggable(ui));
        assert!(folders[2].dimmed(ui), "a pasta recortada apaga");
        assert_eq!(with(|st| st.cards.len()), 4 + 3);

        let card = with(|st| st.cards[&first]);
        assert_eq!(card.title(ui), "Vídeo 0");
        assert_eq!(card.duration(ui), "01:05");
        assert!(card.selected(ui) && card.order(ui) == 1);
        assert!(card.draggable(ui));

        // O clique é do cartão; ao lado dele (o grupo não recebe clique), do fundo. A janela
        // alta, para a biblioteca inteira caber sem rolar.
        let root = ui.root_of(card.id);
        if let Some(cb) = ui.control_mut(root) {
            cb.set_rect(0.0, 0.0, 1400.0, 6000.0);
        }
        ui.layout();
        render(ui); // a largura nova muda as colunas: os cartões são outros
        let card = with(|st| st.cards[&first]);
        let r = ui.absolute_rect(card.id).unwrap();
        assert_eq!(ui.hit_test(root, PointF::new(r.x + 20.0, r.y + 20.0)), Some(card.id));
        assert_eq!(ui.hit_test(root, PointF::new(r.x + CARD_W + GAP / 2.0, r.y + 20.0)), Some(main.lyLibContent.id));

        with(|st| *st = State::default());
    }

    /// Os menus de contexto são os do form: só o que depende do alvo muda.
    #[test]
    fn the_context_menus_come_from_the_form() {
        let mut app = app();
        let ui = &mut app.ui;
        ui.create_form("frmMain").unwrap();
        with(|st| {
            *st = State::default();
            st.lib = sample_library(2);
            st.selection = vec![r"C:\rec\v0.mp4".into(), r"C:\rec\v1.mp4".into()];
        });
        render(ui);
        let main = FrmMain::components(ui).unwrap();
        let card = with(|st| st.cards[r"C:\rec\v0.mp4"]).id;
        open_context(ui, card, "recording", r"C:\rec\v0.mp4", PointF::ZERO);
        assert!(!main.miRecExport.enabled(ui), "exportar é de uma só");
        assert_eq!(main.miRecCut.text(ui), lang::tr_with("Recortar {{count}} selecionadas", &[("count", "2")]));
        assert!(main.miRecDelete.danger(ui));
        open_context(ui, main.lyLibContent.id, "empty", "", PointF::ZERO);
        assert!(!main.miEmptyPaste.enabled(ui), "nada para colar");
        assert!(main.miEmptyUp.visible(ui), "fora da raiz, tem o Voltar");
        with(|st| *st = State::default());
    }

    /// Quanto custa montar a biblioteca com 500 cartões (rode em release, com `--ignored
    /// --nocapture`): a primeira montagem, o refazer e, para comparar, 500 instâncias de
    /// um frame de cartão contra 500 `MediaCard` soltos.
    #[test]
    #[ignore]
    fn bench_500_cards() {
        let mut app = app();
        let ui = &mut app.ui;
        ui.create_form("frmMain").unwrap();
        with(|st| st.lib = sample_library(500));
        let mut times = Vec::new();
        for _ in 0..6 {
            let t = std::time::Instant::now();
            render(ui);
            times.push(t.elapsed().as_secs_f64() * 1000.0);
        }
        assert_eq!(with(|st| st.cards.len()), 503);
        println!("500 cartões: primeira {:.1} ms; refazer {:?} ms", times[0], times[1..].iter().map(|t| format!("{t:.1}")).collect::<Vec<_>>());
        let grid = ui.create("FlowLayout", None).unwrap();
        let t = std::time::Instant::now();
        for _ in 0..500 {
            ui.create("MediaCard", Some(grid)).unwrap();
        }
        let plain = t.elapsed().as_secs_f64() * 1000.0;
        let t = std::time::Instant::now();
        for _ in 0..500 {
            FraRecCard::create_in(ui, grid).unwrap();
        }
        let frames = t.elapsed().as_secs_f64() * 1000.0;
        println!("500 MediaCard: {plain:.1} ms; 500 fraRecCard::create_in: {frames:.1} ms");
        with(|st| *st = State::default());
    }
}
