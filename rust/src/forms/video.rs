//! Eventos do form fraVideo: o código é seu. Os handlers são métodos da
//! struct do form (gerada em `generated/`); a IDE só acrescenta os novos.

use iron::prelude::*;
use crate::generated::FraVideo;
use crate::app_settings::{step_of, video};

/// O estado do form: cada janela aberta dele tem o seu (`self.data`).
#[derive(Default)]
pub struct Data {}

impl FraVideo {
    pub fn video_codec(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        video::codec(ui, self.cmbCodec.item_index(ui).max(0) as usize);
    }

    pub fn video_quality(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        video::quality(ui, step_of(self.trkQuality.value(ui)));
    }

    pub fn video_fps(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        video::fps(ui, step_of(self.trkFps.value(ui)));
    }

    pub fn video_keyframe(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        video::keyframe(ui, step_of(self.trkKeyframe.value(ui)));
    }

    /// "Testar os níveis" / "Testar de novo".
    pub fn video_test_run(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        video::test_run(ui);
    }

    pub fn video_test_cancel(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let _ = ui;
        video::test_cancel();
    }

    /// "Trocar A ↔ B".
    pub fn video_test_swap(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        video::test_swap(ui);
    }

    /// Os botões do zoom da comparação: o `tag` é o zoom (0 é o "Ajustar").
    pub fn video_test_zoom(&mut self, ui: &mut Ui, sender: ComponentId, _args: &EventArgs) {
        if let Some(b) = self.sender_as_button(ui, sender) {
            video::test_zoom(ui, b.tag(ui) as i32);
        }
    }

    /// Arrastar na comparação move a divisória; com zoom, desloca a imagem.
    pub fn video_stage_down(&mut self, ui: &mut Ui, _sender: ComponentId, args: &EventArgs) {
        let _ = ui;
        video::stage_down(args);
    }

    pub fn video_stage_move(&mut self, ui: &mut Ui, _sender: ComponentId, args: &EventArgs) {
        video::stage_move(ui, args);
    }

    pub fn video_stage_up(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let _ = ui;
        video::stage_up();
    }

    /// "Usar o nível N nas gravações".
    pub fn video_test_use(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        video::test_use(ui);
    }
}
