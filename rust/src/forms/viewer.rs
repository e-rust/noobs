//! Os eventos do form frmViewer: o código é seu. A IDE só acrescenta, no fim do `impl`,
//! os handlers novos (os componentes e o registro ficam em `generated/`).

#[allow(unused_imports)]
use aylis::prelude::*;

use crate::generated::FrmViewer;

/// O estado do form: cada janela aberta dele tem o seu (`self.data`).
#[derive(Default)]
pub struct Data {}

// Os handlers têm o nome do componente (`btnSalvar_click`).
#[allow(non_snake_case)]
impl FrmViewer {
}
