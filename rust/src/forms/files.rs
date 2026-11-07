//! Eventos do form fraFiles: o código é seu. Os handlers são métodos da
//! struct do form (gerada em `generated/`); a IDE só acrescenta os novos.

use iron::prelude::*;
use crate::generated::FraFiles;
use crate::app_settings::{commits, files};

/// O estado do form: cada janela aberta dele tem o seu (`self.data`).
#[derive(Default)]
pub struct Data {}

impl FraFiles {
    pub fn files_browse(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let _ = ui;
        files::browse();
    }

    /// A pasta: ao sair do campo ou no Enter.
    pub fn files_record_dir(&mut self, ui: &mut Ui, _sender: ComponentId, args: &EventArgs) {
        if commits(args) {
            files::record_dir(self.edtRecordDir.text(ui).trim());
        }
    }

    /// A prévia do nome a cada tecla.
    pub fn files_filename_edit(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        let text = self.edtFilename.text(ui);
        files::filename_edit(ui, &text);
    }

    pub fn files_filename(&mut self, ui: &mut Ui, _sender: ComponentId, args: &EventArgs) {
        if commits(args) {
            files::filename(self.edtFilename.text(ui).trim());
        }
    }

    pub fn files_library(&mut self, ui: &mut Ui, _sender: ComponentId, _args: &EventArgs) {
        files::library(ui, self.cmbLibrary.item_index(ui).max(0) as usize);
    }
}
