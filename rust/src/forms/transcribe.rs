//! Eventos do form fraTranscribe: o código é seu. Os handlers são métodos da
//! struct do form (gerada em `generated/`); a IDE só acrescenta os novos.

use iron::prelude::*;
use crate::generated::FraTranscribe;
use crate::app_settings::transcribe_tab;

/// O estado do form: cada janela aberta dele tem o seu (`self.data`).
#[derive(Default)]
pub struct Data {}

impl FraTranscribe {
    pub fn transcribe_engine(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        transcribe_tab::engine(ui);
    }

    pub fn transcribe_language(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        transcribe_tab::language(self.cmbTrLanguage.item_index(ui).max(0) as usize);
    }

    pub fn transcribe_on_stop(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        transcribe_tab::on_stop(ui, self.chkTrOnStop.checked(ui));
    }

    pub fn transcribe_pending(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let _ = ui;
        transcribe_tab::pending();
    }

    pub fn transcribe_cancel(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        transcribe_tab::cancel(ui);
    }
}
