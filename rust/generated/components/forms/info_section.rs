
/// O form fraInfoSection: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FraInfoSection {
    pub form: Form,
    pub lblTitle: Label,
    pub recLine: Rectangle,
    pub data: super::info_section::Data,
}

/// Os componentes do fraInfoSection sem o estado, para o código fora dos handlers
/// (`FraInfoSection::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FraInfoSectionComponents {
    pub form: Form,
    pub lblTitle: Label,
    pub recLine: Rectangle,
}

impl std::ops::Deref for FraInfoSection {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FraInfoSection {
    /// Monta a instância de um fraInfoSection recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            lblTitle: Label { id: handles::resolve(ui, root, "lblTitle", "Label", &mut problems) },
            recLine: Rectangle { id: handles::resolve(ui, root, "recLine", "Rectangle", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form fraInfoSection não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FraInfoSectionComponents {
            form: instance.form,
            lblTitle: instance.lblTitle,
            recLine: instance.recLine,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do fraInfoSection aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FraInfoSectionComponents> {
        ui.form_components(ui.find_path("fraInfoSection")?)
    }

    /// Cria o fraInfoSection (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, iron::prelude::Error> {
        ui.create_form("fraInfoSection")
    }

    /// Cria mais um fraInfoSection dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`fraInfoSection`, `fraInfoSection2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FraInfoSectionComponents, iron::prelude::Error> {
        let root = ui.create_form_instance("fraInfoSection")?;
        let name = ui.free_name_in(parent, "fraInfoSection");
        if let Err(e) = ui.set_name(root, &name).and_then(|_| ui.set_parent(root, Some(parent))) {
            ui.free(root);
            return Err(e);
        }
        ui.form_components(root).ok_or(iron::prelude::Error::InvalidId)
    }

    /// Os componentes de uma instância do fraInfoSection (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FraInfoSectionComponents> {
        ui.form_components(root)
    }

    /// O fraInfoSection aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FraInfoSection> {
        ui.form_instance(ui.find_path("fraInfoSection")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FraInfoSection> {
        let form = ui.find_path("fraInfoSection")?;
        ui.form_instance_mut(form)
    }

    /// O Label deste form com o nome (`None` se não há um Label com ele).
    pub fn find_label(&self, name: &str) -> Option<Label> {
        match name {
            "lblTitle" => Some(self.lblTitle),
            _ => None,
        }
    }

    /// O Label que disparou o evento (`None` se o `sender` não é um Label deste form).
    pub fn sender_as_label(&self, ui: &Ui, sender: ComponentId) -> Option<Label> {
        self.find_label(ui.name(sender)?)
    }

    /// O Rectangle deste form com o nome (`None` se não há um Rectangle com ele).
    pub fn find_rectangle(&self, name: &str) -> Option<Rectangle> {
        match name {
            "recLine" => Some(self.recLine),
            _ => None,
        }
    }

    /// O Rectangle que disparou o evento (`None` se o `sender` não é um Rectangle deste form).
    pub fn sender_as_rectangle(&self, ui: &Ui, sender: ComponentId) -> Option<Rectangle> {
        self.find_rectangle(ui.name(sender)?)
    }
}
