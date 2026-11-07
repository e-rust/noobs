//! Restos de versões anteriores no perfil do usuário. As traduções são as do Iron
//! (`locales/`, embutidas com o projeto) e a interface vai no exe pelo `build.rs`.

use std::path::Path;

use crate::log;

/// Resto de versões anteriores no perfil: a interface extraída em
/// `%LOCALAPPDATA%\NoOBS\ui\`. Apagado no arranque.
///
/// Os dados do WebView2 (`%LOCALAPPDATA%\TNoOBS\`) NÃO saem daqui: enquanto a
/// versão web estiver instalada na mesma máquina ela depende deles, e apagar
/// a pasta deixou o WebView2 dela sem subir. Quem remove é
/// o instalador.
pub fn remove_old_ui_folders() {
    let base = std::env::var("LOCALAPPDATA").unwrap_or_default();
    if base.is_empty() {
        return;
    }
    for dir in [Path::new(&base).join("NoOBS").join("ui")] {
        if dir.is_dir() {
            match std::fs::remove_dir_all(&dir) {
                Ok(()) => log!("UI: pasta antiga apagada ({}).", dir.display()),
                Err(e) => log!("UI: nao consegui apagar a pasta antiga \"{}\": {e}", dir.display()),
            }
        }
    }
}
