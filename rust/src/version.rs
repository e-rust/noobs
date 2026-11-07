//! Versão do app e comparação semântica.
//!
//! A versão vem do `git describe --tags --always --dirty`, lido pelo
//! `build.rs`:
//! * `v0.15.0-beta` — release: exatamente na tag, limpo;
//! * `v0.15.0-beta-1-gd7e4f73` — commits depois da tag;
//! * `...-dirty` — alterações não commitadas;
//! * `d7e4f73` — repositório sem tags: não é versão.
//!
//! Os três últimos são build de desenvolvimento: a checagem de atualização
//! NÃO roda neles (estão à frente da tag por definição; avisaria sempre).

/// Versão crua do `git describe`; `dev` sem git.
pub const APP_VERSION: &str = match option_env!("NOOBS_APP_VERSION") {
    Some(v) => v,
    None => "dev",
};

#[derive(Debug, Default, PartialEq, Eq)]
struct SemVer {
    major: u32,
    minor: u32,
    patch: u32,
    /// Vazio = release final.
    pre: String,
}

/// Segmento `<n>-g<hash>` (commits depois da tag) nas posições `i`, `i+1`.
fn is_describe_pair(parts: &[&str], i: usize) -> bool {
    i + 1 < parts.len()
        && parts[i].parse::<u32>().is_ok()
        && parts[i + 1].len() >= 2
        && parts[i + 1].starts_with('g')
}

/// Sufixo do describe: `-<n>-g<hash>` e/ou `-dirty`.
fn has_describe_suffix(s: &str) -> bool {
    if s.ends_with("-dirty") {
        return true;
    }
    let parts: Vec<&str> = s.split('-').collect();
    (0..parts.len()).any(|i| i >= 1 && is_describe_pair(&parts, i))
}

/// Aceita `v1.2.3`, `1.2.3`, `1.2.3-beta`; ignora o sufixo do describe.
fn parse_semver(s: &str) -> Option<SemVer> {
    let t = s.trim();
    let t = t.strip_prefix('v').or_else(|| t.strip_prefix('V')).unwrap_or(t);
    let t = t.strip_suffix("-dirty").unwrap_or(t);
    if t.is_empty() {
        return None;
    }
    let parts: Vec<&str> = t.split('-').collect();
    let mut pre: Vec<&str> = Vec::new();
    let mut i = 1;
    while i < parts.len() {
        // Daqui pra frente é sufixo do describe.
        if is_describe_pair(&parts, i) {
            break;
        }
        pre.push(parts[i]);
        i += 1;
    }
    let core: Vec<&str> = parts[0].split('.').collect();
    // Exige ao menos MAJOR.MINOR.
    if core.len() < 2 {
        return None;
    }
    Some(SemVer {
        major: core[0].parse().ok()?,
        minor: core[1].parse().ok()?,
        patch: if core.len() >= 3 { core[2].parse().ok()? } else { 0 },
        pre: pre.join("-"),
    })
}

/// O build NÃO está exatamente numa tag limpa (ou não há versão).
pub fn is_dev_build() -> bool {
    parse_semver(APP_VERSION).is_none() || has_describe_suffix(APP_VERSION)
}

/// Para a UI: sem o `v`; build de dev mantém o sufixo de propósito.
pub fn display_version() -> String {
    APP_VERSION.strip_prefix('v').unwrap_or(APP_VERSION).to_owned()
}

/// -1/0/1. MAJOR.MINOR.PATCH numérico e pré-lançamento pela regra semver
/// (`1.0.0-beta < 1.0.0`); versão inválida dos dois lados = 0.
pub fn compare_versions(a: &str, b: &str) -> i32 {
    let (Some(va), Some(vb)) = (parse_semver(a), parse_semver(b)) else { return 0 };
    let ord = (va.major, va.minor, va.patch).cmp(&(vb.major, vb.minor, vb.patch)).then_with(|| {
        match (va.pre.is_empty(), vb.pre.is_empty()) {
            (true, true) => std::cmp::Ordering::Equal,
            (true, false) => std::cmp::Ordering::Greater,
            (false, true) => std::cmp::Ordering::Less,
            (false, false) => va.pre.to_lowercase().cmp(&vb.pre.to_lowercase()),
        }
    });
    ord as i32
}

/// A versão é um pré-lançamento (`0.0.1-beta`)? Uma inválida não é.
pub fn is_prerelease(v: &str) -> bool {
    parse_semver(v).is_some_and(|s| !s.pre.is_empty())
}

/// Uma release publicada no GitHub: a tag e se foi marcada como pré-release.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Published {
    pub tag: String,
    pub prerelease: bool,
}

/// O canal de atualização de quem está em `current`:
/// - numa release, só releases mais novas (as pré-releases nunca aparecem);
/// - numa pré-release, a release mais nova, se saiu alguma depois dela (e daí em diante a
///   pessoa está no canal de releases); senão, a pré-release mais nova.
///
/// Escolhe pela versão, não pela data de publicação. `None`: nada mais novo.
pub fn pick_update<'a>(current: &str, published: &'a [Published]) -> Option<&'a Published> {
    let newer = |p: &&Published| parse_semver(&p.tag).is_some() && compare_versions(current, &p.tag) < 0;
    let best = |list: Vec<&'a Published>| list.into_iter().reduce(|a, b| if compare_versions(&a.tag, &b.tag) >= 0 { a } else { b });
    // Release de verdade: a marcada no GitHub e sem sufixo de pré-lançamento na versão.
    let releases: Vec<&Published> = published.iter().filter(|p| !p.prerelease && !is_prerelease(&p.tag)).filter(newer).collect();
    if !releases.is_empty() || !is_prerelease(current) {
        return best(releases);
    }
    best(published.iter().filter(newer).collect())
}

/// `remote` é mais novo que a versão atual? Qualquer dúvida = `false`:
/// avisar à toa destrói a confiança no aviso.
pub fn is_remote_newer(remote: &str) -> bool {
    if remote.trim().is_empty() || is_dev_build() || parse_semver(remote).is_none() {
        return false;
    }
    compare_versions(APP_VERSION, remote) < 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sufixo_do_describe() {
        assert!(!has_describe_suffix("v0.15.0-beta"));
        assert!(has_describe_suffix("v0.15.0-beta-1-gd7e4f73"));
        assert!(has_describe_suffix("v0.15.0-beta-dirty"));
        assert!(parse_semver("d7e4f73").is_none());
    }

    fn published(list: &[(&str, bool)]) -> Vec<Published> {
        list.iter().map(|(tag, prerelease)| Published { tag: tag.to_string(), prerelease: *prerelease }).collect()
    }

    fn picked(current: &str, list: &[(&str, bool)]) -> Option<String> {
        pick_update(current, &published(list)).map(|p| p.tag.clone())
    }

    /// Na pré-release: segue as pré-releases; saiu uma release depois dela, vai para a release.
    #[test]
    fn a_pre_release_follows_the_pre_releases_until_a_release_comes_out() {
        assert_eq!(picked("v0.0.1-beta", &[("v0.0.1-beta", true)]), None, "nada mais novo");
        assert_eq!(picked("v0.0.1-beta", &[("v0.0.1-beta", true), ("v0.0.2-beta", true)]).as_deref(), Some("v0.0.2-beta"));
        // A ordem da lista (a data) não importa: vale a versão.
        assert_eq!(picked("v0.0.1-beta", &[("v0.0.3-beta", true), ("v0.0.2-beta", true)]).as_deref(), Some("v0.0.3-beta"));
        // Saiu a release: vai para ela, mesmo com uma pré-release mais nova publicada depois.
        assert_eq!(picked("v0.0.1-beta", &[("v0.0.2-beta", true), ("v0.0.1", false), ("v0.0.3-beta", true)]).as_deref(), Some("v0.0.1"));
        assert_eq!(picked("v0.0.2-beta", &[("v0.0.1", false), ("v0.0.3-beta", true)]).as_deref(), Some("v0.0.3-beta"), "a release velha não conta");
    }

    /// Na release: só releases, nunca uma pré-release (nem a mais nova de todas).
    #[test]
    fn a_release_only_moves_between_releases() {
        assert_eq!(picked("v0.0.1", &[("v0.0.2-beta", true), ("v0.0.3-beta", true)]), None);
        assert_eq!(picked("v0.0.1", &[("v0.0.2-beta", true), ("v0.0.2", false), ("v0.1.0-beta", true)]).as_deref(), Some("v0.0.2"));
        // Marcada como release no GitHub, mas com sufixo de pré-lançamento: não é release.
        assert_eq!(picked("v0.0.1", &[("v0.0.2-beta", false)]), None);
        assert_eq!(picked("v0.0.2", &[("v0.0.1", false)]), None, "mais velha não é atualização");
    }

    #[test]
    fn pre_lancamento_e_menor() {
        assert_eq!(compare_versions("v1.0.0-beta", "1.0.0"), -1);
        assert_eq!(compare_versions("v0.16.0-beta", "v0.15.2-beta"), 1);
        assert_eq!(compare_versions("v0.15.0-beta-3-gabc1234", "v0.15.0-beta"), 0);
        assert_eq!(compare_versions("1.2", "1.2.0"), 0);
    }
}
