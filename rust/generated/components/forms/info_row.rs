
/// O form fraInfoRow: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FraInfoRow {
    pub form: Form,
    pub lblKey: Label,
    pub lblValue: Label,
    pub data: super::info_row::Data,
}

/// Os componentes do fraInfoRow sem o estado, para o código fora dos handlers
/// (`FraInfoRow::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FraInfoRowComponents {
    pub form: Form,
    pub lblKey: Label,
    pub lblValue: Label,
}

impl std::ops::Deref for FraInfoRow {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FraInfoRow {
    /// Monta a instância de um fraInfoRow recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            lblKey: Label { id: handles::resolve(ui, root, "lblKey", "Label", &mut problems) },
            lblValue: Label { id: handles::resolve(ui, root, "lblValue", "Label", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form fraInfoRow não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FraInfoRowComponents {
            form: instance.form,
            lblKey: instance.lblKey,
            lblValue: instance.lblValue,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do fraInfoRow aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FraInfoRowComponents> {
        ui.form_components(ui.find_path("fraInfoRow")?)
    }

    /// Cria o fraInfoRow (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, aylis::prelude::Error> {
        ui.create_form("fraInfoRow")
    }

    /// Cria mais um fraInfoRow dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`fraInfoRow`, `fraInfoRow2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FraInfoRowComponents, aylis::prelude::Error> {
        let root = ui.create_form_instance("fraInfoRow")?;
        let name = ui.free_name_in(parent, "fraInfoRow");
        if let Err(e) = ui.set_name(root, &name).and_then(|_| ui.set_parent(root, Some(parent))) {
            ui.free(root);
            return Err(e);
        }
        ui.form_components(root).ok_or(aylis::prelude::Error::InvalidId)
    }

    /// Os componentes de uma instância do fraInfoRow (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FraInfoRowComponents> {
        ui.form_components(root)
    }

    /// O fraInfoRow aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FraInfoRow> {
        ui.form_instance(ui.find_path("fraInfoRow")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FraInfoRow> {
        let form = ui.find_path("fraInfoRow")?;
        ui.form_instance_mut(form)
    }

    /// O Label deste form com o nome (`None` se não há um Label com ele).
    pub fn find_label(&self, name: &str) -> Option<Label> {
        match name {
            "lblKey" => Some(self.lblKey),
            "lblValue" => Some(self.lblValue),
            _ => None,
        }
    }

    /// O Label que disparou o evento (`None` se o `sender` não é um Label deste form).
    pub fn sender_as_label(&self, ui: &Ui, sender: ComponentId) -> Option<Label> {
        self.find_label(ui.name(sender)?)
    }
}
