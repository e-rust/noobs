
/// O form fraHotkeys: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FraHotkeys {
    pub form: Form,
    pub lblHotkeysHint: Label,
    pub lyHotkeys: Layout,
    pub lblHotkeysEmpty: Label,
    pub lyHotkeysAdd: Layout,
    pub btnHotkeyAdd: Button,
    pub data: super::hotkeys::Data,
}

/// Os componentes do fraHotkeys sem o estado, para o código fora dos handlers
/// (`FraHotkeys::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FraHotkeysComponents {
    pub form: Form,
    pub lblHotkeysHint: Label,
    pub lyHotkeys: Layout,
    pub lblHotkeysEmpty: Label,
    pub lyHotkeysAdd: Layout,
    pub btnHotkeyAdd: Button,
}

impl std::ops::Deref for FraHotkeys {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FraHotkeys {
    /// Monta a instância de um fraHotkeys recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            lblHotkeysHint: Label { id: handles::resolve(ui, root, "lblHotkeysHint", "Label", &mut problems) },
            lyHotkeys: Layout { id: handles::resolve(ui, root, "lyHotkeys", "Layout", &mut problems) },
            lblHotkeysEmpty: Label { id: handles::resolve(ui, root, "lblHotkeysEmpty", "Label", &mut problems) },
            lyHotkeysAdd: Layout { id: handles::resolve(ui, root, "lyHotkeysAdd", "Layout", &mut problems) },
            btnHotkeyAdd: Button { id: handles::resolve(ui, root, "btnHotkeyAdd", "Button", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form fraHotkeys não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FraHotkeysComponents {
            form: instance.form,
            lblHotkeysHint: instance.lblHotkeysHint,
            lyHotkeys: instance.lyHotkeys,
            lblHotkeysEmpty: instance.lblHotkeysEmpty,
            lyHotkeysAdd: instance.lyHotkeysAdd,
            btnHotkeyAdd: instance.btnHotkeyAdd,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do fraHotkeys aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FraHotkeysComponents> {
        ui.form_components(ui.find_path("fraHotkeys")?)
    }

    /// Cria o fraHotkeys (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, aylis::prelude::Error> {
        ui.create_form("fraHotkeys")
    }

    /// Cria mais um fraHotkeys dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`fraHotkeys`, `fraHotkeys2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FraHotkeysComponents, aylis::prelude::Error> {
        let root = ui.create_form_instance("fraHotkeys")?;
        let name = ui.free_name_in(parent, "fraHotkeys");
        if let Err(e) = ui.set_name(root, &name).and_then(|_| ui.set_parent(root, Some(parent))) {
            ui.free(root);
            return Err(e);
        }
        ui.form_components(root).ok_or(aylis::prelude::Error::InvalidId)
    }

    /// Os componentes de uma instância do fraHotkeys (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FraHotkeysComponents> {
        ui.form_components(root)
    }

    /// O fraHotkeys aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FraHotkeys> {
        ui.form_instance(ui.find_path("fraHotkeys")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FraHotkeys> {
        let form = ui.find_path("fraHotkeys")?;
        ui.form_instance_mut(form)
    }

    /// O Button deste form com o nome (`None` se não há um Button com ele).
    pub fn find_button(&self, name: &str) -> Option<Button> {
        match name {
            "btnHotkeyAdd" => Some(self.btnHotkeyAdd),
            _ => None,
        }
    }

    /// O Button que disparou o evento (`None` se o `sender` não é um Button deste form).
    pub fn sender_as_button(&self, ui: &Ui, sender: ComponentId) -> Option<Button> {
        self.find_button(ui.name(sender)?)
    }

    /// O Label deste form com o nome (`None` se não há um Label com ele).
    pub fn find_label(&self, name: &str) -> Option<Label> {
        match name {
            "lblHotkeysHint" => Some(self.lblHotkeysHint),
            "lblHotkeysEmpty" => Some(self.lblHotkeysEmpty),
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
            "lyHotkeys" => Some(self.lyHotkeys),
            "lyHotkeysAdd" => Some(self.lyHotkeysAdd),
            _ => None,
        }
    }

    /// O Layout que disparou o evento (`None` se o `sender` não é um Layout deste form).
    pub fn sender_as_layout(&self, ui: &Ui, sender: ComponentId) -> Option<Layout> {
        self.find_layout(ui.name(sender)?)
    }
}
