
/// O form fraVtLevel: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FraVtLevel {
    pub form: Form,
    pub pnlLevel: Panel,
    pub lblLevel: Label,
    pub lblRelative: Label,
    pub lblSize: Label,
    pub pnlBar: Panel,
    pub pnlBarFill: Panel,
    pub data: super::vt_level::Data,
}

/// Os componentes do fraVtLevel sem o estado, para o código fora dos handlers
/// (`FraVtLevel::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FraVtLevelComponents {
    pub form: Form,
    pub pnlLevel: Panel,
    pub lblLevel: Label,
    pub lblRelative: Label,
    pub lblSize: Label,
    pub pnlBar: Panel,
    pub pnlBarFill: Panel,
}

impl std::ops::Deref for FraVtLevel {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FraVtLevel {
    /// Monta a instância de um fraVtLevel recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            pnlLevel: Panel { id: handles::resolve(ui, root, "pnlLevel", "Panel", &mut problems) },
            lblLevel: Label { id: handles::resolve(ui, root, "lblLevel", "Label", &mut problems) },
            lblRelative: Label { id: handles::resolve(ui, root, "lblRelative", "Label", &mut problems) },
            lblSize: Label { id: handles::resolve(ui, root, "lblSize", "Label", &mut problems) },
            pnlBar: Panel { id: handles::resolve(ui, root, "pnlBar", "Panel", &mut problems) },
            pnlBarFill: Panel { id: handles::resolve(ui, root, "pnlBarFill", "Panel", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form fraVtLevel não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FraVtLevelComponents {
            form: instance.form,
            pnlLevel: instance.pnlLevel,
            lblLevel: instance.lblLevel,
            lblRelative: instance.lblRelative,
            lblSize: instance.lblSize,
            pnlBar: instance.pnlBar,
            pnlBarFill: instance.pnlBarFill,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do fraVtLevel aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FraVtLevelComponents> {
        ui.form_components(ui.find_path("fraVtLevel")?)
    }

    /// Cria o fraVtLevel (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, aylis::prelude::Error> {
        ui.create_form("fraVtLevel")
    }

    /// Cria mais um fraVtLevel dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`fraVtLevel`, `fraVtLevel2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FraVtLevelComponents, aylis::prelude::Error> {
        let root = ui.create_form_instance("fraVtLevel")?;
        let name = ui.free_name_in(parent, "fraVtLevel");
        if let Err(e) = ui.set_name(root, &name).and_then(|_| ui.set_parent(root, Some(parent))) {
            ui.free(root);
            return Err(e);
        }
        ui.form_components(root).ok_or(aylis::prelude::Error::InvalidId)
    }

    /// Os componentes de uma instância do fraVtLevel (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FraVtLevelComponents> {
        ui.form_components(root)
    }

    /// O fraVtLevel aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FraVtLevel> {
        ui.form_instance(ui.find_path("fraVtLevel")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FraVtLevel> {
        let form = ui.find_path("fraVtLevel")?;
        ui.form_instance_mut(form)
    }

    /// O Label deste form com o nome (`None` se não há um Label com ele).
    pub fn find_label(&self, name: &str) -> Option<Label> {
        match name {
            "lblLevel" => Some(self.lblLevel),
            "lblRelative" => Some(self.lblRelative),
            "lblSize" => Some(self.lblSize),
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
            "pnlLevel" => Some(self.pnlLevel),
            "pnlBar" => Some(self.pnlBar),
            "pnlBarFill" => Some(self.pnlBarFill),
            _ => None,
        }
    }

    /// O Panel que disparou o evento (`None` se o `sender` não é um Panel deste form).
    pub fn sender_as_panel(&self, ui: &Ui, sender: ComponentId) -> Option<Panel> {
        self.find_panel(ui.name(sender)?)
    }
}
