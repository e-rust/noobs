
/// O form fraFiles: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FraFiles {
    pub form: Form,
    pub lblRecordDir: Label,
    pub lyRecordDir: Layout,
    pub btnRecordDir: Button,
    pub edtRecordDir: Edit,
    pub lnRule1: Rectangle,
    pub lblFilename: Label,
    pub edtFilename: Edit,
    pub lblFilenameHint: Label,
    pub lblFilenamePreview: Label,
    pub lnRule2: Rectangle,
    pub lblLibrary: Label,
    pub cmbLibrary: ComboBox,
    pub lblLibraryHint: Label,
    pub lnRule3: Rectangle,
    pub lblHiddenFolders: Label,
    pub lblHiddenFoldersHint: Label,
    pub lyHiddenFolders: Layout,
    pub lblHiddenFoldersEmpty: Label,
    pub data: super::files::Data,
}

/// Os componentes do fraFiles sem o estado, para o código fora dos handlers
/// (`FraFiles::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FraFilesComponents {
    pub form: Form,
    pub lblRecordDir: Label,
    pub lyRecordDir: Layout,
    pub btnRecordDir: Button,
    pub edtRecordDir: Edit,
    pub lnRule1: Rectangle,
    pub lblFilename: Label,
    pub edtFilename: Edit,
    pub lblFilenameHint: Label,
    pub lblFilenamePreview: Label,
    pub lnRule2: Rectangle,
    pub lblLibrary: Label,
    pub cmbLibrary: ComboBox,
    pub lblLibraryHint: Label,
    pub lnRule3: Rectangle,
    pub lblHiddenFolders: Label,
    pub lblHiddenFoldersHint: Label,
    pub lyHiddenFolders: Layout,
    pub lblHiddenFoldersEmpty: Label,
}

impl std::ops::Deref for FraFiles {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FraFiles {
    /// Monta a instância de um fraFiles recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            lblRecordDir: Label { id: handles::resolve(ui, root, "lblRecordDir", "Label", &mut problems) },
            lyRecordDir: Layout { id: handles::resolve(ui, root, "lyRecordDir", "Layout", &mut problems) },
            btnRecordDir: Button { id: handles::resolve(ui, root, "btnRecordDir", "Button", &mut problems) },
            edtRecordDir: Edit { id: handles::resolve(ui, root, "edtRecordDir", "Edit", &mut problems) },
            lnRule1: Rectangle { id: handles::resolve(ui, root, "lnRule1", "Rectangle", &mut problems) },
            lblFilename: Label { id: handles::resolve(ui, root, "lblFilename", "Label", &mut problems) },
            edtFilename: Edit { id: handles::resolve(ui, root, "edtFilename", "Edit", &mut problems) },
            lblFilenameHint: Label { id: handles::resolve(ui, root, "lblFilenameHint", "Label", &mut problems) },
            lblFilenamePreview: Label { id: handles::resolve(ui, root, "lblFilenamePreview", "Label", &mut problems) },
            lnRule2: Rectangle { id: handles::resolve(ui, root, "lnRule2", "Rectangle", &mut problems) },
            lblLibrary: Label { id: handles::resolve(ui, root, "lblLibrary", "Label", &mut problems) },
            cmbLibrary: ComboBox { id: handles::resolve(ui, root, "cmbLibrary", "ComboBox", &mut problems) },
            lblLibraryHint: Label { id: handles::resolve(ui, root, "lblLibraryHint", "Label", &mut problems) },
            lnRule3: Rectangle { id: handles::resolve(ui, root, "lnRule3", "Rectangle", &mut problems) },
            lblHiddenFolders: Label { id: handles::resolve(ui, root, "lblHiddenFolders", "Label", &mut problems) },
            lblHiddenFoldersHint: Label { id: handles::resolve(ui, root, "lblHiddenFoldersHint", "Label", &mut problems) },
            lyHiddenFolders: Layout { id: handles::resolve(ui, root, "lyHiddenFolders", "Layout", &mut problems) },
            lblHiddenFoldersEmpty: Label { id: handles::resolve(ui, root, "lblHiddenFoldersEmpty", "Label", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form fraFiles não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FraFilesComponents {
            form: instance.form,
            lblRecordDir: instance.lblRecordDir,
            lyRecordDir: instance.lyRecordDir,
            btnRecordDir: instance.btnRecordDir,
            edtRecordDir: instance.edtRecordDir,
            lnRule1: instance.lnRule1,
            lblFilename: instance.lblFilename,
            edtFilename: instance.edtFilename,
            lblFilenameHint: instance.lblFilenameHint,
            lblFilenamePreview: instance.lblFilenamePreview,
            lnRule2: instance.lnRule2,
            lblLibrary: instance.lblLibrary,
            cmbLibrary: instance.cmbLibrary,
            lblLibraryHint: instance.lblLibraryHint,
            lnRule3: instance.lnRule3,
            lblHiddenFolders: instance.lblHiddenFolders,
            lblHiddenFoldersHint: instance.lblHiddenFoldersHint,
            lyHiddenFolders: instance.lyHiddenFolders,
            lblHiddenFoldersEmpty: instance.lblHiddenFoldersEmpty,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do fraFiles aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FraFilesComponents> {
        ui.form_components(ui.find_path("fraFiles")?)
    }

    /// Cria o fraFiles (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, iron::prelude::Error> {
        ui.create_form("fraFiles")
    }

    /// Cria mais um fraFiles dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`fraFiles`, `fraFiles2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FraFilesComponents, iron::prelude::Error> {
        let root = ui.create_form_instance("fraFiles")?;
        let name = ui.free_name_in(parent, "fraFiles");
        if let Err(e) = ui.set_name(root, &name).and_then(|_| ui.set_parent(root, Some(parent))) {
            ui.free(root);
            return Err(e);
        }
        ui.form_components(root).ok_or(iron::prelude::Error::InvalidId)
    }

    /// Os componentes de uma instância do fraFiles (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FraFilesComponents> {
        ui.form_components(root)
    }

    /// O fraFiles aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FraFiles> {
        ui.form_instance(ui.find_path("fraFiles")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FraFiles> {
        let form = ui.find_path("fraFiles")?;
        ui.form_instance_mut(form)
    }

    /// O Button deste form com o nome (`None` se não há um Button com ele).
    pub fn find_button(&self, name: &str) -> Option<Button> {
        match name {
            "btnRecordDir" => Some(self.btnRecordDir),
            _ => None,
        }
    }

    /// O Button que disparou o evento (`None` se o `sender` não é um Button deste form).
    pub fn sender_as_button(&self, ui: &Ui, sender: ComponentId) -> Option<Button> {
        self.find_button(ui.name(sender)?)
    }

    /// O ComboBox deste form com o nome (`None` se não há um ComboBox com ele).
    pub fn find_combo_box(&self, name: &str) -> Option<ComboBox> {
        match name {
            "cmbLibrary" => Some(self.cmbLibrary),
            _ => None,
        }
    }

    /// O ComboBox que disparou o evento (`None` se o `sender` não é um ComboBox deste form).
    pub fn sender_as_combo_box(&self, ui: &Ui, sender: ComponentId) -> Option<ComboBox> {
        self.find_combo_box(ui.name(sender)?)
    }

    /// O Edit deste form com o nome (`None` se não há um Edit com ele).
    pub fn find_edit(&self, name: &str) -> Option<Edit> {
        match name {
            "edtRecordDir" => Some(self.edtRecordDir),
            "edtFilename" => Some(self.edtFilename),
            _ => None,
        }
    }

    /// O Edit que disparou o evento (`None` se o `sender` não é um Edit deste form).
    pub fn sender_as_edit(&self, ui: &Ui, sender: ComponentId) -> Option<Edit> {
        self.find_edit(ui.name(sender)?)
    }

    /// O Label deste form com o nome (`None` se não há um Label com ele).
    pub fn find_label(&self, name: &str) -> Option<Label> {
        match name {
            "lblRecordDir" => Some(self.lblRecordDir),
            "lblFilename" => Some(self.lblFilename),
            "lblFilenameHint" => Some(self.lblFilenameHint),
            "lblFilenamePreview" => Some(self.lblFilenamePreview),
            "lblLibrary" => Some(self.lblLibrary),
            "lblLibraryHint" => Some(self.lblLibraryHint),
            "lblHiddenFolders" => Some(self.lblHiddenFolders),
            "lblHiddenFoldersHint" => Some(self.lblHiddenFoldersHint),
            "lblHiddenFoldersEmpty" => Some(self.lblHiddenFoldersEmpty),
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
            "lyRecordDir" => Some(self.lyRecordDir),
            "lyHiddenFolders" => Some(self.lyHiddenFolders),
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
            "lnRule3" => Some(self.lnRule3),
            _ => None,
        }
    }

    /// O Rectangle que disparou o evento (`None` se o `sender` não é um Rectangle deste form).
    pub fn sender_as_rectangle(&self, ui: &Ui, sender: ComponentId) -> Option<Rectangle> {
        self.find_rectangle(ui.name(sender)?)
    }
}
