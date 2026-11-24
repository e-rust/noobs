// 1. O ícone do app vem do projeto (`publish.icon` no `NoOBS.aylisproj`): o
//    `aylis_build::project()` o embute como recurso de ícone 1 (do
//    `.aylis\app-icon.ico`, com todos os tamanhos), que a janela, a bandeja e a
//    hibernação carregam (`win::main_icon`).
// 2. A versão: na release, a do `NOOBS_RELEASE_VERSION` (o `scripts\release.bat` define
//    com a versão que publica; o instalador é compilado antes do commit e da tag, e o
//    `git describe` daria a tag anterior com `-dirty`). Fora dela, o `git describe`.
// 3. Embute o projeto Aylis da pasta de cima (`NoOBS.aylisproj`, forms, temas, assets e
//    as traduções de `locales\`).
use std::process::Command;

fn main() {
    aylis_build::project();

    // Recalcula a versão quando o HEAD ou o índice mudam (commit, checkout,
    // arquivo alterado → `-dirty`) e quando uma TAG é criada: criar a tag no
    // commit atual não mexe em nenhum dos dois, e o exe sairia com a versão
    // de antes da tag.
    println!("cargo:rerun-if-changed=../.git/HEAD");
    println!("cargo:rerun-if-changed=../.git/index");
    println!("cargo:rerun-if-changed=../.git/refs/tags");
    println!("cargo:rerun-if-changed=../.git/packed-refs");
    println!("cargo:rerun-if-env-changed=NOOBS_RELEASE_VERSION");
    let release = std::env::var("NOOBS_RELEASE_VERSION").ok().map(|v| v.trim().trim_start_matches('v').to_owned()).filter(|v| !v.is_empty());
    let version = match release {
        Some(v) => {
            assert!(is_release_version(&v), "NOOBS_RELEASE_VERSION inválida: {v:?} (use 0.0.16-beta ou 1.0.0)");
            format!("v{v}")
        }
        None => Command::new("git")
            .args(["describe", "--tags", "--always", "--dirty"])
            .output()
            .ok()
            .filter(|o| o.status.success())
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_owned())
            .filter(|v| !v.is_empty())
            .unwrap_or_else(|| "dev".to_owned()),
    };
    println!("cargo:rustc-env=NOOBS_APP_VERSION={version}");
}

/// `X.Y.Z` ou `X.Y.Z-sufixo` (letras, números e pontos no sufixo), como o `release.bat` aceita.
fn is_release_version(v: &str) -> bool {
    let (core, pre) = match v.split_once('-') {
        Some((c, p)) => (c, Some(p)),
        None => (v, None),
    };
    let nums: Vec<&str> = core.split('.').collect();
    nums.len() == 3
        && nums.iter().all(|n| !n.is_empty() && n.chars().all(|c| c.is_ascii_digit()))
        && pre.is_none_or(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_alphanumeric() || c == '.'))
}
