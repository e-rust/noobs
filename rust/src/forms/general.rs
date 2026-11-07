//! Eventos do form fraGeneral: o código é seu. Os handlers são métodos da
//! struct do form (gerada em `generated/`); a IDE só acrescenta os novos.

use iron::prelude::*;
use crate::generated::FraGeneral;
use crate::app_settings::{commits, general};

/// O estado do form: cada janela aberta dele tem o seu (`self.data`).
#[derive(Default)]
pub struct Data {}

impl FraGeneral {
    /// Um dos botões do tema (Sistema, Claro, Escuro): o `tag` é a posição dele.
    pub fn general_theme(&mut self, ui: &mut Ui, sender: ComponentId, _args: &EventArgs) {
        if let Some(b) = self.sender_as_button(ui, sender) {
            general::theme(ui, b.tag(ui).max(0) as usize);
        }
    }

    pub fn general_language(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        general::language(self.cmbLanguage.item_index(ui).max(0) as usize);
    }

    /// O título: ao sair do campo ou no Enter.
    pub fn general_window_title(&mut self, ui: &mut Ui, _sender: ComponentId, args: &EventArgs) {
        if commits(args) {
            general::window_title(self.edtWindowTitle.text(ui).trim());
        }
    }

    pub fn general_autostart(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        general::autostart(self.chkAutostart.checked(ui));
    }

    pub fn general_run_as_admin(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        general::run_as_admin(self.chkRunAsAdmin.checked(ui));
    }

    pub fn general_close_to_tray(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        general::close_to_tray(ui, self.chkCloseToTray.checked(ui));
    }

    pub fn general_reset(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        general::reset(ui);
    }

    pub fn general_hibernate(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        general::hibernate(self.chkHibernate.checked(ui));
    }
}
