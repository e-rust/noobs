//! Eventos do form fraUpdates: o código é seu. Os handlers são métodos da
//! struct do form (gerada em `generated/`); a IDE só acrescenta os novos.

use iron::prelude::*;
use crate::generated::FraUpdates;
use crate::app_settings::updates;

/// O estado do form: cada janela aberta dele tem o seu (`self.data`).
#[derive(Default)]
pub struct Data {}

impl FraUpdates {
    pub fn updates_auto(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        updates::auto(ui, self.chkCheckUpdates.checked(ui));
    }

    pub fn updates_check_now(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        updates::check_now(ui);
    }

    pub fn updates_open_page(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let _ = ui;
        updates::open_page();
    }
}
