
/// O form fraLibGroup: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FraLibGroup {
    pub form: Form,
    pub lyHead: Layout,
    pub shpLine: Rectangle,
    pub chkAll: CheckBox,
    pub lblTitle: Label,
    pub flowCards: FlowLayout,
    pub data: super::group::Data,
}

/// Os componentes do fraLibGroup sem o estado, para o código fora dos handlers
/// (`FraLibGroup::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FraLibGroupComponents {
    pub form: Form,
    pub lyHead: Layout,
    pub shpLine: Rectangle,
    pub chkAll: CheckBox,
    pub lblTitle: Label,
    pub flowCards: FlowLayout,
}

impl std::ops::Deref for FraLibGroup {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FraLibGroup {
    /// Monta a instância de um fraLibGroup recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            lyHead: Layout { id: handles::resolve(ui, root, "lyHead", "Layout", &mut problems) },
            shpLine: Rectangle { id: handles::resolve(ui, root, "shpLine", "Rectangle", &mut problems) },
            chkAll: CheckBox { id: handles::resolve(ui, root, "chkAll", "CheckBox", &mut problems) },
            lblTitle: Label { id: handles::resolve(ui, root, "lblTitle", "Label", &mut problems) },
            flowCards: FlowLayout { id: handles::resolve(ui, root, "flowCards", "FlowLayout", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form fraLibGroup não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FraLibGroupComponents {
            form: instance.form,
            lyHead: instance.lyHead,
            shpLine: instance.shpLine,
            chkAll: instance.chkAll,
            lblTitle: instance.lblTitle,
            flowCards: instance.flowCards,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do fraLibGroup aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FraLibGroupComponents> {
        ui.form_components(ui.find_path("fraLibGroup")?)
    }

    /// Cria o fraLibGroup (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, aylis::prelude::Error> {
        ui.create_form("fraLibGroup")
    }

    /// Cria mais um fraLibGroup dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`fraLibGroup`, `fraLibGroup2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FraLibGroupComponents, aylis::prelude::Error> {
        let root = ui.create_form_instance("fraLibGroup")?;
        let name = ui.free_name_in(parent, "fraLibGroup");
        if let Err(e) = ui.set_name(root, &name).and_then(|_| ui.set_parent(root, Some(parent))) {
            ui.free(root);
            return Err(e);
        }
        ui.form_components(root).ok_or(aylis::prelude::Error::InvalidId)
    }

    /// Os componentes de uma instância do fraLibGroup (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FraLibGroupComponents> {
        ui.form_components(root)
    }

    /// O fraLibGroup aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FraLibGroup> {
        ui.form_instance(ui.find_path("fraLibGroup")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FraLibGroup> {
        let form = ui.find_path("fraLibGroup")?;
        ui.form_instance_mut(form)
    }

    /// O CheckBox deste form com o nome (`None` se não há um CheckBox com ele).
    pub fn find_check_box(&self, name: &str) -> Option<CheckBox> {
        match name {
            "chkAll" => Some(self.chkAll),
            _ => None,
        }
    }

    /// O CheckBox que disparou o evento (`None` se o `sender` não é um CheckBox deste form).
    pub fn sender_as_check_box(&self, ui: &Ui, sender: ComponentId) -> Option<CheckBox> {
        self.find_check_box(ui.name(sender)?)
    }

    /// O FlowLayout deste form com o nome (`None` se não há um FlowLayout com ele).
    pub fn find_flow_layout(&self, name: &str) -> Option<FlowLayout> {
        match name {
            "flowCards" => Some(self.flowCards),
            _ => None,
        }
    }

    /// O FlowLayout que disparou o evento (`None` se o `sender` não é um FlowLayout deste form).
    pub fn sender_as_flow_layout(&self, ui: &Ui, sender: ComponentId) -> Option<FlowLayout> {
        self.find_flow_layout(ui.name(sender)?)
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

    /// O Rectangle deste form com o nome (`None` se não há um Rectangle com ele).
    pub fn find_rectangle(&self, name: &str) -> Option<Rectangle> {
        match name {
            "shpLine" => Some(self.shpLine),
            _ => None,
        }
    }

    /// O Rectangle que disparou o evento (`None` se o `sender` não é um Rectangle deste form).
    pub fn sender_as_rectangle(&self, ui: &Ui, sender: ComponentId) -> Option<Rectangle> {
        self.find_rectangle(ui.name(sender)?)
    }
}
