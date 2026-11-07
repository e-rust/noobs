//! Há versão nova no GitHub Releases?
//!
//! `GET https://api.github.com/repos/<REPOSITORY>/releases?per_page=30`, sem
//! autenticação (60 pedidos/hora por IP — folgado para 1 por dia), pelo
//! WinHTTP do sistema (TLS do Windows, sem DLL extra).
//!
//! Sempre em worker; o resultado volta na main. Falha é silenciosa por
//! desenho: sem internet, rate limit ou API fora só vão pro log. Pior que não
//! avisar de uma atualização é avisar errado.

use std::sync::atomic::{AtomicBool, Ordering};

use crate::win_http::{self, Request, Timeouts};
use crate::{log, mainthread, version};

/// O repositório da versão Rust (dono/nome): as atualizações e o link do Sobre saem daqui.
pub const REPOSITORY: &str = "e-rust/noobs";

/// A página do repositório (o link do Sobre).
pub fn repository_url() -> String {
    format!("https://github.com/{REPOSITORY}")
}

/// As releases mais recentes (pré-releases incluídas: o `releases/latest` do GitHub as ignora);
/// quem escolhe o que oferecer é o canal da versão atual (`version::pick_update`). Rascunhos
/// não aparecem sem autenticação.
fn api_url() -> String {
    format!("https://api.github.com/repos/{REPOSITORY}/releases?per_page=30")
}

#[derive(Debug, Clone, Default)]
pub struct UpdateResult {
    /// A consulta funcionou (200 + JSON válido).
    pub ok: bool,
    pub has_update: bool,
    pub latest_tag: String,
    pub release_url: String,
    /// Motivo quando `ok = false` (só pro log).
    pub error: String,
    /// Código HTTP. 0 = NÃO houve resposta (rede, DNS, timeout). A diferença
    /// importa: se o GitHub respondeu qualquer coisa (até 403/500), a checagem
    /// "aconteceu"; sem resposta, tem que ser tentada de novo em vez de
    /// queimar o intervalo de 24 h.
    pub status_code: u32,
}

static CHECKING: AtomicBool = AtomicBool::new(false);

pub fn is_checking() -> bool {
    CHECKING.load(Ordering::SeqCst)
}

fn fetch_latest() -> UpdateResult {
    let mut r = UpdateResult::default();
    // Timeouts curtos: roda em segundo plano e não vale segurar recurso.
    let url = api_url();
    let req = Request::new("GET", &url)
        // A API RECUSA (403) pedido sem User-Agent.
        .header("User-Agent", "NoOBS-UpdateCheck")
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .timeouts(Timeouts { connect_ms: 5_000, send_ms: 5_000, receive_ms: 8_000 });
    let resp = match win_http::request(&req) {
        Ok(resp) => resp,
        Err(e) => {
            r.error = e.to_string();
            return r;
        }
    };
    // O servidor respondeu: registra o código ANTES de qualquer saída.
    r.status_code = resp.status;
    if resp.status != 200 {
        // 404 = ainda sem release; 403 = rate limit.
        r.error = format!("HTTP {}", resp.status);
        return r;
    }
    let Ok(list) = serde_json::from_slice::<serde_json::Value>(&resp.body) else {
        r.error = "JSON inválido".into();
        return r;
    };
    // A lista vem da mais nova para a mais antiga (pela data); vazia = ainda sem release.
    let Some(list) = list.as_array() else {
        r.error = "resposta sem lista".into();
        return r;
    };
    // Rascunhos não contam (sem autenticação nem aparecem, mas por garantia).
    let releases: Vec<(version::Published, String)> = list
        .iter()
        .filter(|j| !j.get("draft").and_then(|v| v.as_bool()).unwrap_or(false))
        .filter_map(|j| {
            let tag = j.get("tag_name").and_then(|v| v.as_str())?.trim().to_owned();
            let url = j.get("html_url").and_then(|v| v.as_str()).unwrap_or("").to_owned();
            let prerelease = j.get("prerelease").and_then(|v| v.as_bool()).unwrap_or(false);
            (!tag.is_empty()).then_some((version::Published { tag, prerelease }, url))
        })
        .collect();
    if releases.is_empty() {
        r.error = "sem release".into();
        return r;
    }
    r.ok = true;
    // O canal de quem está nesta versão (veja `version::pick_update`). Build de
    // desenvolvimento nunca é avisado.
    let published: Vec<version::Published> = releases.iter().map(|(p, _)| p.clone()).collect();
    let picked = if version::is_dev_build() { None } else { version::pick_update(version::APP_VERSION, &published) };
    match picked.and_then(|p| releases.iter().find(|(q, _)| q.tag == p.tag)) {
        Some((p, url)) => {
            r.latest_tag = p.tag.clone();
            r.release_url = url.clone();
            r.has_update = version::is_remote_newer(&p.tag);
        }
        None => r.latest_tag = version::APP_VERSION.to_owned(),
    }
    r
}

/// Dispara em worker; `callback` roda na MAIN. No-op com uma já em curso
/// (clique repetido no botão).
pub fn check_for_updates(callback: fn(UpdateResult)) {
    // Teste: sem rede (o GitHub).
    #[cfg(test)]
    {
        let _ = callback;
        crate::testkit::record_effect("check_updates");
        return;
    }
    #[allow(unreachable_code)]
    if CHECKING.swap(true, Ordering::SeqCst) {
        log::log("Update: checagem já em andamento — ignorando.");
        return;
    }
    std::thread::spawn(move || {
        let r = fetch_latest();
        if r.ok {
            log!("Update: atual={} remoto={} novidade={}", version::APP_VERSION, r.latest_tag, r.has_update);
        } else {
            log!("Update: falha na checagem ({}).", r.error);
        }
        CHECKING.store(false, Ordering::SeqCst);
        mainthread::queue(move || callback(r));
    });
}
