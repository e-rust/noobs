// 1. O ícone do app vem do projeto (`publish.icon` no `NoOBS.aylisproj`): o
//    `aylis_build::project()` o embute como recurso de ícone 1 (do
//    `.aylis\app-icon.ico`, com todos os tamanhos), que a janela, a bandeja e a
//    hibernação carregam (`win::main_icon`).
// 2. Versão a partir da tag do git: criar a tag
//    É o ato de lançar, então a versão nunca fica dessincronizada. Build fora
//    de uma tag limpa carrega o sufixo do describe — e o app trata como
//    desenvolvimento (não checa atualização).
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
    let version = Command::new("git")
        .args(["describe", "--tags", "--always", "--dirty"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_owned())
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| "dev".to_owned());
    println!("cargo:rustc-env=NOOBS_APP_VERSION={version}");
}
