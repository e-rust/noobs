//! Eventos do form frmPlayer: o código é seu. Os handlers são métodos da
//! struct do form (gerada em `generated/`); a IDE só acrescenta os novos.

use aylis::prelude::*;
use crate::generated::FrmPlayer;

/// O estado do form: cada janela aberta dele tem o seu (`self.data`).
#[derive(Default)]
pub struct Data {}

impl FrmPlayer {
    pub fn player_side_close(&mut self, ui: &mut Ui, sender: ComponentId, args: &EventArgs) {
        let _ = (&sender, &args);
        crate::app_player::close_side(ui);
    }

    pub fn player_tr_search(&mut self, ui: &mut Ui, sender: ComponentId, args: &EventArgs) {
        let _ = (&sender, &args);
        let text = self.edTrSearch.text(ui);
        crate::app_player::tr_search(ui, &text);
    }

    pub fn player_stage_resize(&mut self, ui: &mut Ui, sender: ComponentId, args: &EventArgs) {
        let _ = (&sender, &args);
        crate::app_player::stage_resized(ui);
    }

    pub fn player_close(&mut self, ui: &mut Ui, sender: ComponentId, args: &EventArgs) {
        let _ = (&sender, &args);
        crate::app_player::close(ui);
    }

    pub fn player_play(&mut self, ui: &mut Ui, sender: ComponentId, args: &EventArgs) {
        let _ = (&sender, &args);
        crate::app_player::toggle_play(ui);
    }

    pub fn player_fullscreen(&mut self, ui: &mut Ui, sender: ComponentId, args: &EventArgs) {
        let _ = (&sender, &args);
        crate::app_player::toggle_fullscreen(ui);
    }

    pub fn player_info(&mut self, ui: &mut Ui, sender: ComponentId, args: &EventArgs) {
        let _ = (&sender, &args);
        crate::app_player::toggle_info(ui);
    }

    pub fn player_captions(&mut self, ui: &mut Ui, sender: ComponentId, args: &EventArgs) {
        let _ = (&sender, &args);
        crate::app_player::toggle_captions(ui);
    }

    pub fn player_speed(&mut self, ui: &mut Ui, sender: ComponentId, args: &EventArgs) {
        let _ = (&sender, &args);
        crate::app_player::speed_menu(ui, sender);
    }

    pub fn player_mute(&mut self, ui: &mut Ui, sender: ComponentId, args: &EventArgs) {
        let _ = (&sender, &args);
        crate::app_player::toggle_mute(ui);
    }

    pub fn player_seek(&mut self, ui: &mut Ui, sender: ComponentId, args: &EventArgs) {
        let _ = (&sender, &args);
        if let EventArgs::Value(v) = args {
            crate::app_player::seek_to(ui, v.to_f64().unwrap_or(0.0));
        }
    }

    pub fn player_volume(&mut self, ui: &mut Ui, sender: ComponentId, args: &EventArgs) {
        let _ = (&sender, &args);
        let v = self.trkVolume.value(ui);
        crate::app_player::set_volume(ui, v);
    }

    pub fn player_transcript(&mut self, ui: &mut Ui, sender: ComponentId, args: &EventArgs) {
        let _ = (&sender, &args);
        crate::app_player::toggle_transcript(ui);
    }

    /// O usuário rolou a lateral: a transcrição para de acompanhar se a fala atual saiu da vista.
    pub fn player_side_scroll(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        crate::app_player::side_scrolled(ui);
    }

    /// O pulso do indicador central na pausa, a cada quadro do monitor.
    pub fn player_flash_frame(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        crate::app_player::flash_frame(ui);
    }
}
