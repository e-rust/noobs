//! Eventos do form frmExport: o código é seu. Os handlers são métodos da
//! struct do form (gerada em `generated/`); a IDE só acrescenta os novos.
//!
//! Cada handler lê o componente tipado (`self.x.valor(ui)`) e chama a função da tela em
//! `app_export`, que guarda o estado e redesenha.

use iron::prelude::*;
use crate::app_export as ex;
use crate::generated::FrmExport;

/// O estado do form: cada janela aberta dele tem o seu (`self.data`).
#[derive(Default)]
pub struct Data {}

/// O índice escolhido num `ComboBox` (o "nenhum" vale o primeiro).
fn index(i: i64) -> usize {
    i.max(0) as usize
}

impl FrmExport {
    pub fn export_close(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        crate::app_export::close(ui);
    }

    pub fn export_resize(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        crate::app_export::resized(ui);
    }

    /// Apertar na prévia: com zoom, começa a deslocar a vista; sem, toca/pausa.
    pub fn export_stage_down(&mut self, ui: &mut Ui, _sender: ComponentId, args: &EventArgs) {
        if let EventArgs::Pointer(e) = args {
            ex::pan_down(ui, e.position.x, e.position.y);
        }
    }

    pub fn export_stage_move(&mut self, ui: &mut Ui, _sender: ComponentId, args: &EventArgs) {
        if let EventArgs::Pointer(e) = args {
            ex::pan_move(ui, e.position.x, e.position.y);
        }
    }

    pub fn export_stage_up(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        ex::pan_up(ui);
    }

    /// A roda sobre a prévia ou a moldura do recorte aproxima no ponto do mouse.
    pub fn export_stage_wheel(&mut self, ui: &mut Ui, sender: ComponentId, args: &EventArgs) {
        if let EventArgs::Wheel(e) = args {
            ex::stage_wheel(ui, sender, e.position, e.delta.y);
        }
    }

    /// A moldura do recorte mudou (arrastando).
    pub fn export_crop_change(&mut self, ui: &mut Ui, sender: ComponentId, _args: &EventArgs) {
        ex::crop_changed(ui, sender, false);
    }

    /// A moldura do recorte foi solta.
    pub fn export_crop_end(&mut self, ui: &mut Ui, sender: ComponentId, _args: &EventArgs) {
        ex::crop_changed(ui, sender, true);
    }

    /// Tocar/pausar: o botão e o toque na moldura do recorte.
    pub fn export_play(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        ex::toggle_play(ui);
    }

    /// "Usar tela inteira": desfaz o recorte.
    pub fn export_crop_reset(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        ex::crop_reset(ui);
    }

    pub fn export_crop_zoom_out(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        ex::crop_zoom_by(ui, 1.0 / 1.4);
    }

    pub fn export_crop_zoom_in(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        ex::crop_zoom_by(ui, 1.4);
    }

    pub fn export_crop_zoom_reset(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        ex::crop_zoom_reset(ui);
    }

    pub fn export_mute(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        ex::toggle_mute(ui);
    }

    pub fn export_volume(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        ex::set_volume(ui, self.trkExVolume.value(ui).round() as i32);
    }

    pub fn export_cc(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        ex::toggle_cc(ui);
    }

    /// Apertar e arrastar na linha do tempo move o cursor.
    pub fn export_tl_down(&mut self, ui: &mut Ui, _sender: ComponentId, args: &EventArgs) {
        if let EventArgs::Pointer(e) = args {
            ex::tl_down(ui, e.position.x);
        }
    }

    pub fn export_tl_move(&mut self, ui: &mut Ui, _sender: ComponentId, args: &EventArgs) {
        if let EventArgs::Pointer(e) = args {
            ex::tl_move(ui, e.position.x);
        }
    }

    pub fn export_tl_up(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let _ = ui;
        ex::tl_up();
    }

    /// A roda sobre a linha do tempo aproxima no ponto do mouse.
    pub fn export_tl_wheel(&mut self, ui: &mut Ui, _sender: ComponentId, args: &EventArgs) {
        if let EventArgs::Wheel(e) = args {
            ex::tl_wheel(ui, e.position.x, e.delta.y);
        }
    }

    /// Arrastar o polegar da barra de rolagem da linha do tempo.
    pub fn export_sb_down(&mut self, ui: &mut Ui, _sender: ComponentId, args: &EventArgs) {
        if let EventArgs::Pointer(e) = args {
            ex::sb_down(ui, e.position.x);
        }
    }

    pub fn export_sb_move(&mut self, ui: &mut Ui, _sender: ComponentId, args: &EventArgs) {
        if let EventArgs::Pointer(e) = args {
            ex::sb_move(ui, e.position.x);
        }
    }

    pub fn export_sb_up(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let _ = ui;
        ex::sb_up();
    }

    pub fn export_cut(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        ex::cut_here(ui);
    }

    pub fn export_uncut(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        ex::remove_cut(ui);
    }

    pub fn export_toggle(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        ex::toggle_part(ui);
    }

    pub fn export_tl_zoom_out(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        ex::tl_zoom(ui, false);
    }

    pub fn export_tl_zoom_in(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        ex::tl_zoom(ui, true);
    }

    /// O campo do tempo: vale ao sair dele ou com Enter.
    pub fn export_time_exit(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let text = self.edtExTime.text(ui);
        ex::time_commit(ui, &text);
    }

    pub fn export_time_key(&mut self, ui: &mut Ui, _sender: ComponentId, args: &EventArgs) {
        if let EventArgs::Key(k) = args {
            if k.key == Key::Enter {
                let text = self.edtExTime.text(ui);
                ex::time_commit(ui, &text);
            }
        }
    }

    /// "Manter o áudio original": o clique passa o novo estado (desligada, não faz nada).
    pub fn export_orig(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        if self.pnlExOrig.enabled(ui) {
            ex::orig_clicked(ui, !self.chkExOrig.checked(ui));
        }
    }

    /// "Juntar as faixas em uma só".
    pub fn export_mix(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        if self.pnlExMix.enabled(ui) {
            ex::mix_clicked(ui, !self.chkExMix.checked(ui));
        }
    }

    pub fn export_res(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        ex::res_changed(ui, index(self.cmbExRes.item_index(ui)));
    }

    pub fn export_enc(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        ex::enc_changed(ui, index(self.cmbExEnc.item_index(ui)));
    }

    pub fn export_scale(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        ex::scale_changed(ui, index(self.cmbExScale.item_index(ui)));
    }

    /// A velocidade digitada (cada tecla).
    pub fn export_speed_change(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let text = self.edtExSpeed.text(ui);
        ex::speed_edit(ui, &text);
    }

    pub fn export_speed_exit(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        ex::speed_commit(ui);
    }

    /// As setas sobem e descem a velocidade nos passos da web; Enter confirma.
    pub fn export_speed_key(&mut self, ui: &mut Ui, _sender: ComponentId, args: &EventArgs) {
        if let EventArgs::Key(k) = args {
            match k.key {
                Key::Up => ex::speed_step(ui, 1),
                Key::Down => ex::speed_step(ui, -1),
                Key::Enter => ex::speed_commit(ui),
                _ => {}
            }
        }
    }

    pub fn export_fps(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        ex::fps_changed(ui, self.trkExFps.value(ui).round());
    }

    pub fn export_quality(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        ex::quality_changed(ui, self.trkExQuality.value(ui).round());
    }

    /// "Gravar a legenda no vídeo".
    pub fn export_burn(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        if self.pnlExBurn.enabled(ui) {
            ex::burn_clicked(ui, !self.chkExBurn.checked(ui));
        }
    }

    pub fn export_cont(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        ex::cont_changed(ui, index(self.cmbExCont.item_index(ui)));
    }

    /// "Exportar" / "Cancelar".
    pub fn export_run(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        ex::run_or_cancel(ui);
    }

    /// "Gerar prévia" (ou cancelar a que está sendo gerada).
    pub fn export_preview(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        ex::preview_action(ui);
    }

    pub fn export_prevsec(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        ex::prevsec_changed(index(self.cmbExPrevSec.item_index(ui)));
    }
}
