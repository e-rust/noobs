//! Os eventos do form fraInfoSection: o código é seu. A IDE só acrescenta, no fim do `impl`,
//! os handlers novos (os componentes e o registro ficam em `generated/`).

#[allow(unused_imports)]
use iron::prelude::*;

use crate::generated::FraInfoSection;

/// O estado do form: cada janela aberta dele tem o seu (`self.data`).
#[derive(Default)]
pub struct Data {}

impl FraInfoSection {
}
