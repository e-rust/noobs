
/// O form fraDeviceRow: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FraDeviceRow {
    pub form: Form,
    pub pnlRow: Panel,
    pub chkOn: CheckBox,
    pub lblInfo: Label,
    pub lblName: Label,
    pub data: super::device_row::Data,
}

/// Os componentes do fraDeviceRow sem o estado, para o código fora dos handlers
/// (`FraDeviceRow::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FraDeviceRowComponents {
    pub form: Form,
    pub pnlRow: Panel,
    pub chkOn: CheckBox,
    pub lblInfo: Label,
    pub lblName: Label,
}

impl std::ops::Deref for FraDeviceRow {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FraDeviceRow {
    /// Monta a instância de um fraDeviceRow recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            pnlRow: Panel { id: handles::resolve(ui, root, "pnlRow", "Panel", &mut problems) },
            chkOn: CheckBox { id: handles::resolve(ui, root, "chkOn", "CheckBox", &mut problems) },
            lblInfo: Label { id: handles::resolve(ui, root, "lblInfo", "Label", &mut problems) },
            lblName: Label { id: handles::resolve(ui, root, "lblName", "Label", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form fraDeviceRow não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FraDeviceRowComponents {
            form: instance.form,
            pnlRow: instance.pnlRow,
            chkOn: instance.chkOn,
            lblInfo: instance.lblInfo,
            lblName: instance.lblName,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do fraDeviceRow aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FraDeviceRowComponents> {
        ui.form_components(ui.find_path("fraDeviceRow")?)
    }

    /// Cria o fraDeviceRow (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, iron::prelude::Error> {
        ui.create_form("fraDeviceRow")
    }

    /// Cria mais um fraDeviceRow dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`fraDeviceRow`, `fraDeviceRow2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FraDeviceRowComponents, iron::prelude::Error> {
        let root = ui.create_form_instance("fraDeviceRow")?;
        let name = ui.free_name_in(parent, "fraDeviceRow");
        if let Err(e) = ui.set_name(root, &name).and_then(|_| ui.set_parent(root, Some(parent))) {
            ui.free(root);
            return Err(e);
        }
        ui.form_components(root).ok_or(iron::prelude::Error::InvalidId)
    }

    /// Os componentes de uma instância do fraDeviceRow (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FraDeviceRowComponents> {
        ui.form_components(root)
    }

    /// O fraDeviceRow aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FraDeviceRow> {
        ui.form_instance(ui.find_path("fraDeviceRow")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FraDeviceRow> {
        let form = ui.find_path("fraDeviceRow")?;
        ui.form_instance_mut(form)
    }

    /// O CheckBox deste form com o nome (`None` se não há um CheckBox com ele).
    pub fn find_check_box(&self, name: &str) -> Option<CheckBox> {
        match name {
            "chkOn" => Some(self.chkOn),
            _ => None,
        }
    }

    /// O CheckBox que disparou o evento (`None` se o `sender` não é um CheckBox deste form).
    pub fn sender_as_check_box(&self, ui: &Ui, sender: ComponentId) -> Option<CheckBox> {
        self.find_check_box(ui.name(sender)?)
    }

    /// O Label deste form com o nome (`None` se não há um Label com ele).
    pub fn find_label(&self, name: &str) -> Option<Label> {
        match name {
            "lblInfo" => Some(self.lblInfo),
            "lblName" => Some(self.lblName),
            _ => None,
        }
    }

    /// O Label que disparou o evento (`None` se o `sender` não é um Label deste form).
    pub fn sender_as_label(&self, ui: &Ui, sender: ComponentId) -> Option<Label> {
        self.find_label(ui.name(sender)?)
    }

    /// O Panel deste form com o nome (`None` se não há um Panel com ele).
    pub fn find_panel(&self, name: &str) -> Option<Panel> {
        match name {
            "pnlRow" => Some(self.pnlRow),
            _ => None,
        }
    }

    /// O Panel que disparou o evento (`None` se o `sender` não é um Panel deste form).
    pub fn sender_as_panel(&self, ui: &Ui, sender: ComponentId) -> Option<Panel> {
        self.find_panel(ui.name(sender)?)
    }
}
