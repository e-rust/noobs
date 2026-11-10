
/// O form fraTurn: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FraTurn {
    pub form: Form,
    pub pnlTurn: Panel,
    pub recMark: Rectangle,
    pub lyHead: Layout,
    pub lblTime: Label,
    pub lblSpeaker: Label,
    pub lblText: Label,
    pub data: super::turn::Data,
}

/// Os componentes do fraTurn sem o estado, para o código fora dos handlers
/// (`FraTurn::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FraTurnComponents {
    pub form: Form,
    pub pnlTurn: Panel,
    pub recMark: Rectangle,
    pub lyHead: Layout,
    pub lblTime: Label,
    pub lblSpeaker: Label,
    pub lblText: Label,
}

impl std::ops::Deref for FraTurn {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FraTurn {
    /// Monta a instância de um fraTurn recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            pnlTurn: Panel { id: handles::resolve(ui, root, "pnlTurn", "Panel", &mut problems) },
            recMark: Rectangle { id: handles::resolve(ui, root, "recMark", "Rectangle", &mut problems) },
            lyHead: Layout { id: handles::resolve(ui, root, "lyHead", "Layout", &mut problems) },
            lblTime: Label { id: handles::resolve(ui, root, "lblTime", "Label", &mut problems) },
            lblSpeaker: Label { id: handles::resolve(ui, root, "lblSpeaker", "Label", &mut problems) },
            lblText: Label { id: handles::resolve(ui, root, "lblText", "Label", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form fraTurn não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FraTurnComponents {
            form: instance.form,
            pnlTurn: instance.pnlTurn,
            recMark: instance.recMark,
            lyHead: instance.lyHead,
            lblTime: instance.lblTime,
            lblSpeaker: instance.lblSpeaker,
            lblText: instance.lblText,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do fraTurn aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FraTurnComponents> {
        ui.form_components(ui.find_path("fraTurn")?)
    }

    /// Cria o fraTurn (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, aylis::prelude::Error> {
        ui.create_form("fraTurn")
    }

    /// Cria mais um fraTurn dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`fraTurn`, `fraTurn2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FraTurnComponents, aylis::prelude::Error> {
        let root = ui.create_form_instance("fraTurn")?;
        let name = ui.free_name_in(parent, "fraTurn");
        if let Err(e) = ui.set_name(root, &name).and_then(|_| ui.set_parent(root, Some(parent))) {
            ui.free(root);
            return Err(e);
        }
        ui.form_components(root).ok_or(aylis::prelude::Error::InvalidId)
    }

    /// Os componentes de uma instância do fraTurn (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FraTurnComponents> {
        ui.form_components(root)
    }

    /// O fraTurn aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FraTurn> {
        ui.form_instance(ui.find_path("fraTurn")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FraTurn> {
        let form = ui.find_path("fraTurn")?;
        ui.form_instance_mut(form)
    }

    /// O Label deste form com o nome (`None` se não há um Label com ele).
    pub fn find_label(&self, name: &str) -> Option<Label> {
        match name {
            "lblTime" => Some(self.lblTime),
            "lblSpeaker" => Some(self.lblSpeaker),
            "lblText" => Some(self.lblText),
            _ => None,
        }
    }

    /// O Label que disparou o evento (`None` se o `sender` não é um Label deste form).
    pub fn sender_as_label(&self, ui: &Ui, sender: ComponentId) -> Option<Label> {
        self.find_label(ui.name(sender)?)
    }

    /// O Layout deste form com o nome (`None` se não há um Layout com ele).
    pub fn find_layout(&self, name: &str) -> Option<Layout> {
        match name {
            "lyHead" => Some(self.lyHead),
            _ => None,
        }
    }

    /// O Layout que disparou o evento (`None` se o `sender` não é um Layout deste form).
    pub fn sender_as_layout(&self, ui: &Ui, sender: ComponentId) -> Option<Layout> {
        self.find_layout(ui.name(sender)?)
    }

    /// O Panel deste form com o nome (`None` se não há um Panel com ele).
    pub fn find_panel(&self, name: &str) -> Option<Panel> {
        match name {
            "pnlTurn" => Some(self.pnlTurn),
            _ => None,
        }
    }

    /// O Panel que disparou o evento (`None` se o `sender` não é um Panel deste form).
    pub fn sender_as_panel(&self, ui: &Ui, sender: ComponentId) -> Option<Panel> {
        self.find_panel(ui.name(sender)?)
    }

    /// O Rectangle deste form com o nome (`None` se não há um Rectangle com ele).
    pub fn find_rectangle(&self, name: &str) -> Option<Rectangle> {
        match name {
            "recMark" => Some(self.recMark),
            _ => None,
        }
    }

    /// O Rectangle que disparou o evento (`None` se o `sender` não é um Rectangle deste form).
    pub fn sender_as_rectangle(&self, ui: &Ui, sender: ComponentId) -> Option<Rectangle> {
        self.find_rectangle(ui.name(sender)?)
    }
}
