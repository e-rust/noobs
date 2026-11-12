
/// O form fraHiddenFolder: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FraHiddenFolder {
    pub form: Form,
    pub chkHidden: CheckBox,
    pub lblCount: Label,
    pub data: super::settings_parts_hidden_folder::Data,
}

/// Os componentes do fraHiddenFolder sem o estado, para o código fora dos handlers
/// (`FraHiddenFolder::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FraHiddenFolderComponents {
    pub form: Form,
    pub chkHidden: CheckBox,
    pub lblCount: Label,
}

impl std::ops::Deref for FraHiddenFolder {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FraHiddenFolder {
    /// Monta a instância de um fraHiddenFolder recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            chkHidden: CheckBox { id: handles::resolve(ui, root, "chkHidden", "CheckBox", &mut problems) },
            lblCount: Label { id: handles::resolve(ui, root, "lblCount", "Label", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form fraHiddenFolder não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FraHiddenFolderComponents {
            form: instance.form,
            chkHidden: instance.chkHidden,
            lblCount: instance.lblCount,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do fraHiddenFolder aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FraHiddenFolderComponents> {
        ui.form_components(ui.find_path("fraHiddenFolder")?)
    }

    /// Cria o fraHiddenFolder (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, aylis::prelude::Error> {
        ui.create_form("fraHiddenFolder")
    }

    /// Cria mais um fraHiddenFolder dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`fraHiddenFolder`, `fraHiddenFolder2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FraHiddenFolderComponents, aylis::prelude::Error> {
        let root = ui.create_form_instance("fraHiddenFolder")?;
        let name = ui.free_name_in(parent, "fraHiddenFolder");
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

    /// Os componentes de uma instância do fraHiddenFolder (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FraHiddenFolderComponents> {
        ui.form_components(root)
    }

    /// O fraHiddenFolder aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FraHiddenFolder> {
        ui.form_instance(ui.find_path("fraHiddenFolder")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FraHiddenFolder> {
        let form = ui.find_path("fraHiddenFolder")?;
        ui.form_instance_mut(form)
    }

    /// O CheckBox deste form com o nome (`None` se não há um CheckBox com ele).
    pub fn find_check_box(&self, name: &str) -> Option<CheckBox> {
        match name {
            "chkHidden" => Some(self.chkHidden),
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
            "lblCount" => Some(self.lblCount),
            _ => None,
        }
    }

    /// O Label que disparou o evento (`None` se o `sender` não é um Label deste form).
    pub fn sender_as_label(&self, ui: &Ui, sender: ComponentId) -> Option<Label> {
        self.find_label(ui.name(sender)?)
    }
}
