//! Eventos do form fraOnRecord: o código é seu. Os handlers são métodos da
//! struct do form (gerada em `generated/`); a IDE só acrescenta os novos.

use iron::prelude::*;
use crate::generated::FraOnRecord;
use crate::app_settings::{commits, onrecord, step_of};

/// O estado do form: cada janela aberta dele tem o seu (`self.data`).
#[derive(Default)]
pub struct Data {}

impl FraOnRecord {
    pub fn onrecord_rec_indicator(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        onrecord::check(ui, "recIndicator", self.chkRecIndicator.checked(ui));
    }

    pub fn onrecord_rec_clickable(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        onrecord::check(ui, "recIndicatorClickable", self.chkRecClickable.checked(ui));
    }

    pub fn onrecord_rec_corner(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        onrecord::rec_corner(self.cmbRecCorner.item_index(ui).max(0) as usize);
    }

    pub fn onrecord_rec_opacity(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        onrecord::rec_opacity(ui, step_of(self.trkRecOpacity.value(ui)));
    }

    pub fn onrecord_fullscreen(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        onrecord::check(ui, "fullscreenDetect", self.chkFullscreen.checked(ui));
    }

    pub fn onrecord_play_sound(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        onrecord::check(ui, "playSoundOnRecord", self.chkPlaySound.checked(ui));
    }

    pub fn onrecord_minimize(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        onrecord::check(ui, "minimizeOnRecord", self.chkMinimize.checked(ui));
    }

    pub fn onrecord_auto_record(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        onrecord::check(ui, "autoRecordOnMic", self.chkAutoRecord.checked(ui));
    }

    /// A lista de apps: ao sair do campo ou no Enter.
    pub fn onrecord_auto_apps(&mut self, ui: &mut Ui, _sender: ComponentId, args: &EventArgs) {
        if commits(args) {
            let text = self.edtAutoApps.text(ui);
            onrecord::auto_apps(ui, text.trim());
        }
    }

    pub fn onrecord_auto_except(&mut self, ui: &mut Ui, _sender: ComponentId, args: &EventArgs) {
        if commits(args) {
            onrecord::auto_except(self.edtAutoExcept.text(ui).trim());
        }
    }

    pub fn onrecord_stop_on_lock(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        onrecord::check(ui, "stopOnLock", self.chkStopOnLock.checked(ui));
    }
}
