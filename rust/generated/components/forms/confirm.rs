
/// O form fraConfirm: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FraConfirm {
    pub form: Form,
    pub pnlScrim: Panel,
    pub pnlCard: Panel,
    pub cirIcon: Circle,
    pub pthTrash: Path,
    pub pthMerge: Path,
    pub lblTitle: Label,
    pub lblMessage: Label,
    pub btnCancel: Button,
    pub btnOk: Button,
    pub data: super::confirm::Data,
}

/// Os componentes do fraConfirm sem o estado, para o código fora dos handlers
/// (`FraConfirm::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FraConfirmComponents {
    pub form: Form,
    pub pnlScrim: Panel,
    pub pnlCard: Panel,
    pub cirIcon: Circle,
    pub pthTrash: Path,
    pub pthMerge: Path,
    pub lblTitle: Label,
    pub lblMessage: Label,
    pub btnCancel: Button,
    pub btnOk: Button,
}

impl std::ops::Deref for FraConfirm {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FraConfirm {
    /// Monta a instância de um fraConfirm recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            pnlScrim: Panel { id: handles::resolve(ui, root, "pnlScrim", "Panel", &mut problems) },
            pnlCard: Panel { id: handles::resolve(ui, root, "pnlCard", "Panel", &mut problems) },
            cirIcon: Circle { id: handles::resolve(ui, root, "cirIcon", "Circle", &mut problems) },
            pthTrash: Path { id: handles::resolve(ui, root, "pthTrash", "Path", &mut problems) },
            pthMerge: Path { id: handles::resolve(ui, root, "pthMerge", "Path", &mut problems) },
            lblTitle: Label { id: handles::resolve(ui, root, "lblTitle", "Label", &mut problems) },
            lblMessage: Label { id: handles::resolve(ui, root, "lblMessage", "Label", &mut problems) },
            btnCancel: Button { id: handles::resolve(ui, root, "btnCancel", "Button", &mut problems) },
            btnOk: Button { id: handles::resolve(ui, root, "btnOk", "Button", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form fraConfirm não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FraConfirmComponents {
            form: instance.form,
            pnlScrim: instance.pnlScrim,
            pnlCard: instance.pnlCard,
            cirIcon: instance.cirIcon,
            pthTrash: instance.pthTrash,
            pthMerge: instance.pthMerge,
            lblTitle: instance.lblTitle,
            lblMessage: instance.lblMessage,
            btnCancel: instance.btnCancel,
            btnOk: instance.btnOk,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do fraConfirm aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FraConfirmComponents> {
        ui.form_components(ui.find_path("fraConfirm")?)
    }

    /// Cria o fraConfirm (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, aylis::prelude::Error> {
        ui.create_form("fraConfirm")
    }

    /// Cria mais um fraConfirm dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`fraConfirm`, `fraConfirm2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FraConfirmComponents, aylis::prelude::Error> {
        let root = ui.create_form_instance("fraConfirm")?;
        let name = ui.free_name_in(parent, "fraConfirm");
        if let Err(e) = ui.set_name(root, &name).and_then(|_| ui.set_parent(root, Some(parent))) {
            ui.free(root);
            return Err(e);
        }
        ui.form_components(root).ok_or(aylis::prelude::Error::InvalidId)
    }

    /// Os componentes de uma instância do fraConfirm (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FraConfirmComponents> {
        ui.form_components(root)
    }

    /// O fraConfirm aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FraConfirm> {
        ui.form_instance(ui.find_path("fraConfirm")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FraConfirm> {
        let form = ui.find_path("fraConfirm")?;
        ui.form_instance_mut(form)
    }

    /// O Button deste form com o nome (`None` se não há um Button com ele).
    pub fn find_button(&self, name: &str) -> Option<Button> {
        match name {
            "btnCancel" => Some(self.btnCancel),
            "btnOk" => Some(self.btnOk),
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
            "cirIcon" => Some(self.cirIcon),
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
            "lblTitle" => Some(self.lblTitle),
            "lblMessage" => Some(self.lblMessage),
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
            "pnlScrim" => Some(self.pnlScrim),
            "pnlCard" => Some(self.pnlCard),
            _ => None,
        }
    }

    /// O Panel que disparou o evento (`None` se o `sender` não é um Panel deste form).
    pub fn sender_as_panel(&self, ui: &Ui, sender: ComponentId) -> Option<Panel> {
        self.find_panel(ui.name(sender)?)
    }

    /// O Path deste form com o nome (`None` se não há um Path com ele).
    pub fn find_path(&self, name: &str) -> Option<Path> {
        match name {
            "pthTrash" => Some(self.pthTrash),
            "pthMerge" => Some(self.pthMerge),
            _ => None,
        }
    }

    /// O Path que disparou o evento (`None` se o `sender` não é um Path deste form).
    pub fn sender_as_path(&self, ui: &Ui, sender: ComponentId) -> Option<Path> {
        self.find_path(ui.name(sender)?)
    }
}
