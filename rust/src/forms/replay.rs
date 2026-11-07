//! Eventos do form fraReplay: o código é seu. Os handlers são métodos da
//! struct do form (gerada em `generated/`); a IDE só acrescenta os novos.

use iron::prelude::*;
use crate::generated::FraReplay;
use crate::app_settings::{commits, replay_tab, step_of};

/// O estado do form: cada janela aberta dele tem o seu (`self.data`).
#[derive(Default)]
pub struct Data {}

impl FraReplay {
    pub fn replay_auto_start(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        replay_tab::check(ui, "replayAutoStart", self.chkReplayAutoStart.checked(ui));
    }

    pub fn replay_into_recording(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        replay_tab::check(ui, "replayIntoRecording", self.chkReplayIntoRecording.checked(ui));
    }

    /// Tempo ou memória: o core recebe os dois (o outro, o que está escolhido).
    pub fn replay_seconds(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        replay_tab::limits(self.cmbReplaySeconds.item_index(ui).max(0) as usize, self.cmbReplayMemory.item_index(ui).max(0) as usize);
    }

    pub fn replay_memory(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        replay_tab::limits(self.cmbReplaySeconds.item_index(ui).max(0) as usize, self.cmbReplayMemory.item_index(ui).max(0) as usize);
    }

    /// Os programas: com atraso enquanto digita; ao sair do campo ou no Enter, na hora.
    pub fn replay_apps_edit(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let text = self.edtReplayApps.text(ui);
        replay_tab::apps_edit(ui, text);
    }

    pub fn replay_apps(&mut self, ui: &mut Ui, _sender: ComponentId, args: &EventArgs) {
        if commits(args) {
            replay_tab::apps(self.edtReplayApps.text(ui).trim());
        }
    }

    pub fn replay_indicator(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        replay_tab::check(ui, "replayIndicator", self.chkReplayIndicator.checked(ui));
    }

    pub fn replay_clickable(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        replay_tab::check(ui, "replayIndicatorClickable", self.chkRepClickable.checked(ui));
    }

    pub fn replay_corner(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        replay_tab::corner(self.cmbRepCorner.item_index(ui).max(0) as usize);
    }

    pub fn replay_opacity(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        replay_tab::opacity(ui, step_of(self.trkRepOpacity.value(ui)));
    }
}
