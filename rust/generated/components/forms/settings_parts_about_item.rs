
/// O form fraAboutItem: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FraAboutItem {
    pub form: Form,
    pub lblText: Label,
    pub cirDot: Circle,
    pub data: super::settings_parts_about_item::Data,
}

/// Os componentes do fraAboutItem sem o estado, para o código fora dos handlers
/// (`FraAboutItem::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FraAboutItemComponents {
    pub form: Form,
    pub lblText: Label,
    pub cirDot: Circle,
}

impl std::ops::Deref for FraAboutItem {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FraAboutItem {
    /// Monta a instância de um fraAboutItem recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            lblText: Label { id: handles::resolve(ui, root, "lblText", "Label", &mut problems) },
            cirDot: Circle { id: handles::resolve(ui, root, "cirDot", "Circle", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form fraAboutItem não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FraAboutItemComponents {
            form: instance.form,
            lblText: instance.lblText,
            cirDot: instance.cirDot,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do fraAboutItem aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FraAboutItemComponents> {
        ui.form_components(ui.find_path("fraAboutItem")?)
    }

    /// Cria o fraAboutItem (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, aylis::prelude::Error> {
        ui.create_form("fraAboutItem")
    }

    /// Cria mais um fraAboutItem dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`fraAboutItem`, `fraAboutItem2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FraAboutItemComponents, aylis::prelude::Error> {
        let root = ui.create_form_instance("fraAboutItem")?;
        let name = ui.free_name_in(parent, "fraAboutItem");
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

    /// Os componentes de uma instância do fraAboutItem (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FraAboutItemComponents> {
        ui.form_components(root)
    }

    /// O fraAboutItem aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FraAboutItem> {
        ui.form_instance(ui.find_path("fraAboutItem")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FraAboutItem> {
        let form = ui.find_path("fraAboutItem")?;
        ui.form_instance_mut(form)
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
            "lblText" => Some(self.lblText),
            _ => None,
        }
    }

    /// O Label que disparou o evento (`None` se o `sender` não é um Label deste form).
    pub fn sender_as_label(&self, ui: &Ui, sender: ComponentId) -> Option<Label> {
        self.find_label(ui.name(sender)?)
    }
}
