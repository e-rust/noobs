
/// O form fraWindowCard: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FraWindowCard {
    pub form: Form,
    pub pnlCard: Panel,
    pub imgThumb: Image,
    pub lblTitle: Label,
    pub lblInfo: Label,
    pub data: super::window_card::Data,
}

/// Os componentes do fraWindowCard sem o estado, para o código fora dos handlers
/// (`FraWindowCard::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FraWindowCardComponents {
    pub form: Form,
    pub pnlCard: Panel,
    pub imgThumb: Image,
    pub lblTitle: Label,
    pub lblInfo: Label,
}

impl std::ops::Deref for FraWindowCard {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FraWindowCard {
    /// Monta a instância de um fraWindowCard recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            pnlCard: Panel { id: handles::resolve(ui, root, "pnlCard", "Panel", &mut problems) },
            imgThumb: Image { id: handles::resolve(ui, root, "imgThumb", "Image", &mut problems) },
            lblTitle: Label { id: handles::resolve(ui, root, "lblTitle", "Label", &mut problems) },
            lblInfo: Label { id: handles::resolve(ui, root, "lblInfo", "Label", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form fraWindowCard não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FraWindowCardComponents {
            form: instance.form,
            pnlCard: instance.pnlCard,
            imgThumb: instance.imgThumb,
            lblTitle: instance.lblTitle,
            lblInfo: instance.lblInfo,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do fraWindowCard aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FraWindowCardComponents> {
        ui.form_components(ui.find_path("fraWindowCard")?)
    }

    /// Cria o fraWindowCard (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, aylis::prelude::Error> {
        ui.create_form("fraWindowCard")
    }

    /// Cria mais um fraWindowCard dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`fraWindowCard`, `fraWindowCard2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FraWindowCardComponents, aylis::prelude::Error> {
        let root = ui.create_form_instance("fraWindowCard")?;
        let name = ui.free_name_in(parent, "fraWindowCard");
        if let Err(e) = ui.set_name(root, &name).and_then(|_| ui.set_parent(root, Some(parent))) {
            ui.free(root);
            return Err(e);
        }
        ui.form_components(root).ok_or(aylis::prelude::Error::InvalidId)
    }

    /// Os componentes de uma instância do fraWindowCard (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FraWindowCardComponents> {
        ui.form_components(root)
    }

    /// O fraWindowCard aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FraWindowCard> {
        ui.form_instance(ui.find_path("fraWindowCard")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FraWindowCard> {
        let form = ui.find_path("fraWindowCard")?;
        ui.form_instance_mut(form)
    }

    /// O Image deste form com o nome (`None` se não há um Image com ele).
    pub fn find_image(&self, name: &str) -> Option<Image> {
        match name {
            "imgThumb" => Some(self.imgThumb),
            _ => None,
        }
    }

    /// O Image que disparou o evento (`None` se o `sender` não é um Image deste form).
    pub fn sender_as_image(&self, ui: &Ui, sender: ComponentId) -> Option<Image> {
        self.find_image(ui.name(sender)?)
    }

    /// O Label deste form com o nome (`None` se não há um Label com ele).
    pub fn find_label(&self, name: &str) -> Option<Label> {
        match name {
            "lblTitle" => Some(self.lblTitle),
            "lblInfo" => Some(self.lblInfo),
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
            "pnlCard" => Some(self.pnlCard),
            _ => None,
        }
    }

    /// O Panel que disparou o evento (`None` se o `sender` não é um Panel deste form).
    pub fn sender_as_panel(&self, ui: &Ui, sender: ComponentId) -> Option<Panel> {
        self.find_panel(ui.name(sender)?)
    }
}
