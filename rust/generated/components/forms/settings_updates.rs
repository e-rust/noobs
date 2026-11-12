
/// O form fraUpdates: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FraUpdates {
    pub form: Form,
    pub lblVersion: Label,
    pub lblVersionValue: Label,
    pub lnRule1: Rectangle,
    pub chkCheckUpdates: CheckBox,
    pub lblCheckUpdatesHint: Label,
    pub lnRule2: Rectangle,
    pub lyCheckNow: Layout,
    pub btnCheckNow: Button,
    pub lyUpdateStatus: Layout,
    pub lblUpdateStatus: Label,
    pub lblUpdateOpen: Label,
    pub data: super::settings_updates::Data,
}

/// Os componentes do fraUpdates sem o estado, para o código fora dos handlers
/// (`FraUpdates::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FraUpdatesComponents {
    pub form: Form,
    pub lblVersion: Label,
    pub lblVersionValue: Label,
    pub lnRule1: Rectangle,
    pub chkCheckUpdates: CheckBox,
    pub lblCheckUpdatesHint: Label,
    pub lnRule2: Rectangle,
    pub lyCheckNow: Layout,
    pub btnCheckNow: Button,
    pub lyUpdateStatus: Layout,
    pub lblUpdateStatus: Label,
    pub lblUpdateOpen: Label,
}

impl std::ops::Deref for FraUpdates {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FraUpdates {
    /// Monta a instância de um fraUpdates recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            lblVersion: Label { id: handles::resolve(ui, root, "lblVersion", "Label", &mut problems) },
            lblVersionValue: Label { id: handles::resolve(ui, root, "lblVersionValue", "Label", &mut problems) },
            lnRule1: Rectangle { id: handles::resolve(ui, root, "lnRule1", "Rectangle", &mut problems) },
            chkCheckUpdates: CheckBox { id: handles::resolve(ui, root, "chkCheckUpdates", "CheckBox", &mut problems) },
            lblCheckUpdatesHint: Label { id: handles::resolve(ui, root, "lblCheckUpdatesHint", "Label", &mut problems) },
            lnRule2: Rectangle { id: handles::resolve(ui, root, "lnRule2", "Rectangle", &mut problems) },
            lyCheckNow: Layout { id: handles::resolve(ui, root, "lyCheckNow", "Layout", &mut problems) },
            btnCheckNow: Button { id: handles::resolve(ui, root, "btnCheckNow", "Button", &mut problems) },
            lyUpdateStatus: Layout { id: handles::resolve(ui, root, "lyUpdateStatus", "Layout", &mut problems) },
            lblUpdateStatus: Label { id: handles::resolve(ui, root, "lblUpdateStatus", "Label", &mut problems) },
            lblUpdateOpen: Label { id: handles::resolve(ui, root, "lblUpdateOpen", "Label", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form fraUpdates não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FraUpdatesComponents {
            form: instance.form,
            lblVersion: instance.lblVersion,
            lblVersionValue: instance.lblVersionValue,
            lnRule1: instance.lnRule1,
            chkCheckUpdates: instance.chkCheckUpdates,
            lblCheckUpdatesHint: instance.lblCheckUpdatesHint,
            lnRule2: instance.lnRule2,
            lyCheckNow: instance.lyCheckNow,
            btnCheckNow: instance.btnCheckNow,
            lyUpdateStatus: instance.lyUpdateStatus,
            lblUpdateStatus: instance.lblUpdateStatus,
            lblUpdateOpen: instance.lblUpdateOpen,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do fraUpdates aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FraUpdatesComponents> {
        ui.form_components(ui.find_path("fraUpdates")?)
    }

    /// Cria o fraUpdates (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, aylis::prelude::Error> {
        ui.create_form("fraUpdates")
    }

    /// Cria mais um fraUpdates dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`fraUpdates`, `fraUpdates2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FraUpdatesComponents, aylis::prelude::Error> {
        let root = ui.create_form_instance("fraUpdates")?;
        let name = ui.free_name_in(parent, "fraUpdates");
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

    /// Os componentes de uma instância do fraUpdates (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FraUpdatesComponents> {
        ui.form_components(root)
    }

    /// O fraUpdates aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FraUpdates> {
        ui.form_instance(ui.find_path("fraUpdates")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FraUpdates> {
        let form = ui.find_path("fraUpdates")?;
        ui.form_instance_mut(form)
    }

    /// O Button deste form com o nome (`None` se não há um Button com ele).
    pub fn find_button(&self, name: &str) -> Option<Button> {
        match name {
            "btnCheckNow" => Some(self.btnCheckNow),
            _ => None,
        }
    }

    /// O Button que disparou o evento (`None` se o `sender` não é um Button deste form).
    pub fn sender_as_button(&self, ui: &Ui, sender: ComponentId) -> Option<Button> {
        self.find_button(ui.name(sender)?)
    }

    /// O CheckBox deste form com o nome (`None` se não há um CheckBox com ele).
    pub fn find_check_box(&self, name: &str) -> Option<CheckBox> {
        match name {
            "chkCheckUpdates" => Some(self.chkCheckUpdates),
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
            "lblVersion" => Some(self.lblVersion),
            "lblVersionValue" => Some(self.lblVersionValue),
            "lblCheckUpdatesHint" => Some(self.lblCheckUpdatesHint),
            "lblUpdateStatus" => Some(self.lblUpdateStatus),
            "lblUpdateOpen" => Some(self.lblUpdateOpen),
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
            "lyCheckNow" => Some(self.lyCheckNow),
            "lyUpdateStatus" => Some(self.lyUpdateStatus),
            _ => None,
        }
    }

    /// O Layout que disparou o evento (`None` se o `sender` não é um Layout deste form).
    pub fn sender_as_layout(&self, ui: &Ui, sender: ComponentId) -> Option<Layout> {
        self.find_layout(ui.name(sender)?)
    }

    /// O Rectangle deste form com o nome (`None` se não há um Rectangle com ele).
    pub fn find_rectangle(&self, name: &str) -> Option<Rectangle> {
        match name {
            "lnRule1" => Some(self.lnRule1),
            "lnRule2" => Some(self.lnRule2),
            _ => None,
        }
    }

    /// O Rectangle que disparou o evento (`None` se o `sender` não é um Rectangle deste form).
    pub fn sender_as_rectangle(&self, ui: &Ui, sender: ComponentId) -> Option<Rectangle> {
        self.find_rectangle(ui.name(sender)?)
    }
}
