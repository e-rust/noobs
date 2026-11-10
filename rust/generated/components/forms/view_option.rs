
/// O form fraViewOption: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FraViewOption {
    pub form: Form,
    pub pnlOption: Panel,
    pub pnlMark: Panel,
    pub cirDot: Circle,
    pub btnCheck: Button,
    pub btnIcon: Button,
    pub lblLabel: Label,
    pub data: super::view_option::Data,
}

/// Os componentes do fraViewOption sem o estado, para o código fora dos handlers
/// (`FraViewOption::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FraViewOptionComponents {
    pub form: Form,
    pub pnlOption: Panel,
    pub pnlMark: Panel,
    pub cirDot: Circle,
    pub btnCheck: Button,
    pub btnIcon: Button,
    pub lblLabel: Label,
}

impl std::ops::Deref for FraViewOption {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FraViewOption {
    /// Monta a instância de um fraViewOption recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            pnlOption: Panel { id: handles::resolve(ui, root, "pnlOption", "Panel", &mut problems) },
            pnlMark: Panel { id: handles::resolve(ui, root, "pnlMark", "Panel", &mut problems) },
            cirDot: Circle { id: handles::resolve(ui, root, "cirDot", "Circle", &mut problems) },
            btnCheck: Button { id: handles::resolve(ui, root, "btnCheck", "Button", &mut problems) },
            btnIcon: Button { id: handles::resolve(ui, root, "btnIcon", "Button", &mut problems) },
            lblLabel: Label { id: handles::resolve(ui, root, "lblLabel", "Label", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form fraViewOption não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FraViewOptionComponents {
            form: instance.form,
            pnlOption: instance.pnlOption,
            pnlMark: instance.pnlMark,
            cirDot: instance.cirDot,
            btnCheck: instance.btnCheck,
            btnIcon: instance.btnIcon,
            lblLabel: instance.lblLabel,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do fraViewOption aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FraViewOptionComponents> {
        ui.form_components(ui.find_path("fraViewOption")?)
    }

    /// Cria o fraViewOption (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, aylis::prelude::Error> {
        ui.create_form("fraViewOption")
    }

    /// Cria mais um fraViewOption dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`fraViewOption`, `fraViewOption2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FraViewOptionComponents, aylis::prelude::Error> {
        let root = ui.create_form_instance("fraViewOption")?;
        let name = ui.free_name_in(parent, "fraViewOption");
        if let Err(e) = ui.set_name(root, &name).and_then(|_| ui.set_parent(root, Some(parent))) {
            ui.free(root);
            return Err(e);
        }
        ui.form_components(root).ok_or(aylis::prelude::Error::InvalidId)
    }

    /// Os componentes de uma instância do fraViewOption (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FraViewOptionComponents> {
        ui.form_components(root)
    }

    /// O fraViewOption aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FraViewOption> {
        ui.form_instance(ui.find_path("fraViewOption")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FraViewOption> {
        let form = ui.find_path("fraViewOption")?;
        ui.form_instance_mut(form)
    }

    /// O Button deste form com o nome (`None` se não há um Button com ele).
    pub fn find_button(&self, name: &str) -> Option<Button> {
        match name {
            "btnCheck" => Some(self.btnCheck),
            "btnIcon" => Some(self.btnIcon),
            _ => None,
        }
    }

    /// O Button que disparou o evento (`None` se o `sender` não é um Button deste form).
    pub fn sender_as_button(&self, ui: &Ui, sender: ComponentId) -> Option<Button> {
        self.find_button(ui.name(sender)?)
    }

    /// O Circle deste form com o nome (`None` se não há um Circle com ele).
    pub fn find_circle(&self, name: &str) -> Option<Circle> {
        match name {
            "cirDot" => Some(self.cirDot),
            _ => None,
        }
    }

    /// O Circle que disparou o evento (`None` se o `sender` não é um Circle deste form).
    pub fn sender_as_circle(&self, ui: &Ui, sender: ComponentId) -> Option<Circle> {
        self.find_circle(ui.name(sender)?)
    }

    /// O Label deste form com o nome (`None` se não há um Label com ele).
    pub fn find_label(&self, name: &str) -> Option<Label> {
        match name {
            "lblLabel" => Some(self.lblLabel),
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
            "pnlOption" => Some(self.pnlOption),
            "pnlMark" => Some(self.pnlMark),
            _ => None,
        }
    }

    /// O Panel que disparou o evento (`None` se o `sender` não é um Panel deste form).
    pub fn sender_as_panel(&self, ui: &Ui, sender: ComponentId) -> Option<Panel> {
        self.find_panel(ui.name(sender)?)
    }
}
