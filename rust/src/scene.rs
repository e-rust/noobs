//! Tipo puro de monitor e o filtro dos monitores gravados. O canvas compacto
//! (lado a lado) é calculado no próprio motor de gravação.

use crate::config;

#[derive(Debug, Clone, Default)]
pub struct CaptureMonitor {
    pub index: i32,
    pub name: String,
    pub width: i32,
    pub height: i32,
    pub position_x: i32,
    pub position_y: i32,
}

/// Só entram os monitores marcados e não ocultos.
/// `auto_profile`: gravação disparada pelo watcher de mic (perfil auto).
pub fn filter_enabled_monitors(mons: &[CaptureMonitor], auto_profile: bool) -> Vec<CaptureMonitor> {
    mons.iter()
        .filter(|m| {
            let id = m.index.to_string();
            if auto_profile {
                config::get_source_active_for_auto("monitors", &id, true)
            } else {
                config::get_source_active("monitors", &id, true)
            }
        })
        .cloned()
        .collect()
}

