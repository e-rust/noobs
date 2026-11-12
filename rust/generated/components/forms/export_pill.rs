
/// O form fraExPill: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FraExPill {
    pub form: Form,
    pub pnlPill: Panel,
    pub chkPill: CheckBox,
    pub lblPill: Label,
    pub data: super::export_pill::Data,
}

/// Os componentes do fraExPill sem o estado, para o código fora dos handlers
/// (`FraExPill::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FraExPillComponents {
    pub form: Form,
    pub pnlPill: Panel,
    pub chkPill: CheckBox,
    pub lblPill: Label,
}

impl std::ops::Deref for FraExPill {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FraExPill {
    /// Monta a instância de um fraExPill recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            pnlPill: Panel { id: handles::resolve(ui, root, "pnlPill", "Panel", &mut problems) },
            chkPill: CheckBox { id: handles::resolve(ui, root, "chkPill", "CheckBox", &mut problems) },
            lblPill: Label { id: handles::resolve(ui, root, "lblPill", "Label", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form fraExPill não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FraExPillComponents {
            form: instance.form,
            pnlPill: instance.pnlPill,
            chkPill: instance.chkPill,
            lblPill: instance.lblPill,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do fraExPill aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FraExPillComponents> {
        ui.form_components(ui.find_path("fraExPill")?)
    }

    /// Cria o fraExPill (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, aylis::prelude::Error> {
        ui.create_form("fraExPill")
    }

    /// Cria mais um fraExPill dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`fraExPill`, `fraExPill2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FraExPillComponents, aylis::prelude::Error> {
        let root = ui.create_form_instance("fraExPill")?;
        let name = ui.free_name_in(parent, "fraExPill");
        if let Err(e) = ui.set_name(root, &name).and_then(|_| ui.set_parent(root, Some(parent))) {
            ui.free(root);
            return Err(e);
        }
        // Como o componente Frame: sem `fill` próprio, transparente (G05).
        if matches!(ui.get_prop(root, "fill"), Ok(aylis::prelude::PropValue::Null)) {
            let _ = ui.set_prop(root, "fill", aylis::prelude::Color::TRANSPARENT);
        }
        ui.form_components(root).ok_or(aylis::prelude::Error::InvalidId)
    }

    /// Os componentes de uma instância do fraExPill (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FraExPillComponents> {
        ui.form_components(root)
    }

    /// O fraExPill aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FraExPill> {
        ui.form_instance(ui.find_path("fraExPill")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FraExPill> {
        let form = ui.find_path("fraExPill")?;
        ui.form_instance_mut(form)
    }

    /// O CheckBox deste form com o nome (`None` se não há um CheckBox com ele).
    pub fn find_check_box(&self, name: &str) -> Option<CheckBox> {
        match name {
            "chkPill" => Some(self.chkPill),
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
            "lblPill" => Some(self.lblPill),
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
            "pnlPill" => Some(self.pnlPill),
            _ => None,
        }
    }

    /// O Panel que disparou o evento (`None` se o `sender` não é um Panel deste form).
    pub fn sender_as_panel(&self, ui: &Ui, sender: ComponentId) -> Option<Panel> {
        self.find_panel(ui.name(sender)?)
    }
}
