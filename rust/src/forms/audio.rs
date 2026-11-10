//! Eventos do form fraAudio: o código é seu. Os handlers são métodos da
//! struct do form (gerada em `generated/`); a IDE só acrescenta os novos.

use aylis::prelude::*;
use crate::generated::FraAudio;
use crate::app_settings::audio;

/// O estado do form: cada janela aberta dele tem o seu (`self.data`).
#[derive(Default)]
pub struct Data {}

impl FraAudio {
    pub fn audio_bitrate(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        audio::bitrate(ui, self.cmbAudioBitrate.item_index(ui).max(0) as usize);
    }

    pub fn audio_mute_when_muted(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        audio::mute_when_muted(ui, self.chkMuteWhenMuted.checked(ui));
    }
}
