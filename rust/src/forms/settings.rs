//! Eventos do form frmSettings: o código é seu. Os handlers são métodos da
//! struct do form (gerada em `generated/`); a IDE só acrescenta os novos.

use aylis::prelude::*;
use crate::generated::FrmSettings;

/// O estado do form: cada janela aberta dele tem o seu (`self.data`).
#[derive(Default)]
pub struct Data {}

impl FrmSettings {
    pub fn settings_close(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        crate::app_settings::close(ui);
    }

    /// Uma aba da barra lateral: o `tag` da linha é a posição dela.
    pub fn settings_tab(&mut self, ui: &mut Ui, sender: ComponentId, _args: &EventArgs) {
        let index = self.sender_as_panel(ui, sender).map_or(0, |row| row.tag(ui));
        crate::app_settings::show_tab_at(ui, index.max(0) as usize);
    }

    pub fn settings_resize(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        crate::app_settings::resized(ui);
    }
}

