
/// O form fraHotkeyCard: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FraHotkeyCard {
    pub form: Form,
    pub pnlCard: Panel,
    pub lyKeys: Layout,
    pub cmbMode: ComboBox,
    pub pnlCtrl: Panel,
    pub chkCtrl: CheckBox,
    pub pnlShift: Panel,
    pub chkShift: CheckBox,
    pub pnlAlt: Panel,
    pub chkAlt: CheckBox,
    pub pnlWin: Panel,
    pub chkWin: CheckBox,
    pub btnRemove: Button,
    pub lblPlus: Label,
    pub cmbKey: ComboBox,
    pub lyWindow: Layout,
    pub lblWindowTitle: Label,
    pub edtWindowTitle: Edit,
    pub lyAppAudio: Layout,
    pub chkAppAudio: CheckBox,
    pub lyShot: Layout,
    pub lblShotTarget: Label,
    pub cmbShotTarget: ComboBox,
    pub lyArea: Layout,
    pub btnAreaClear: Button,
    pub btnAreaDefine: Button,
    pub lblArea: Label,
    pub lyTimelapse: Layout,
    pub chkTimelapse: CheckBox,
    pub lyShotInfo: Layout,
    pub lblShotInfo: Label,
    pub lyDelay: Layout,
    pub lblDelay: Label,
    pub cmbDelay: ComboBox,
    pub lyStatus: Layout,
    pub lblStatus: Label,
    pub data: super::settings_parts_hotkey_card::Data,
}

/// Os componentes do fraHotkeyCard sem o estado, para o código fora dos handlers
/// (`FraHotkeyCard::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FraHotkeyCardComponents {
    pub form: Form,
    pub pnlCard: Panel,
    pub lyKeys: Layout,
    pub cmbMode: ComboBox,
    pub pnlCtrl: Panel,
    pub chkCtrl: CheckBox,
    pub pnlShift: Panel,
    pub chkShift: CheckBox,
    pub pnlAlt: Panel,
    pub chkAlt: CheckBox,
    pub pnlWin: Panel,
    pub chkWin: CheckBox,
    pub btnRemove: Button,
    pub lblPlus: Label,
    pub cmbKey: ComboBox,
    pub lyWindow: Layout,
    pub lblWindowTitle: Label,
    pub edtWindowTitle: Edit,
    pub lyAppAudio: Layout,
    pub chkAppAudio: CheckBox,
    pub lyShot: Layout,
    pub lblShotTarget: Label,
    pub cmbShotTarget: ComboBox,
    pub lyArea: Layout,
    pub btnAreaClear: Button,
    pub btnAreaDefine: Button,
    pub lblArea: Label,
    pub lyTimelapse: Layout,
    pub chkTimelapse: CheckBox,
    pub lyShotInfo: Layout,
    pub lblShotInfo: Label,
    pub lyDelay: Layout,
    pub lblDelay: Label,
    pub cmbDelay: ComboBox,
    pub lyStatus: Layout,
    pub lblStatus: Label,
}

impl std::ops::Deref for FraHotkeyCard {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FraHotkeyCard {
    /// Monta a instância de um fraHotkeyCard recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            pnlCard: Panel { id: handles::resolve(ui, root, "pnlCard", "Panel", &mut problems) },
            lyKeys: Layout { id: handles::resolve(ui, root, "lyKeys", "Layout", &mut problems) },
            cmbMode: ComboBox { id: handles::resolve(ui, root, "cmbMode", "ComboBox", &mut problems) },
            pnlCtrl: Panel { id: handles::resolve(ui, root, "pnlCtrl", "Panel", &mut problems) },
            chkCtrl: CheckBox { id: handles::resolve(ui, root, "chkCtrl", "CheckBox", &mut problems) },
            pnlShift: Panel { id: handles::resolve(ui, root, "pnlShift", "Panel", &mut problems) },
            chkShift: CheckBox { id: handles::resolve(ui, root, "chkShift", "CheckBox", &mut problems) },
            pnlAlt: Panel { id: handles::resolve(ui, root, "pnlAlt", "Panel", &mut problems) },
            chkAlt: CheckBox { id: handles::resolve(ui, root, "chkAlt", "CheckBox", &mut problems) },
            pnlWin: Panel { id: handles::resolve(ui, root, "pnlWin", "Panel", &mut problems) },
            chkWin: CheckBox { id: handles::resolve(ui, root, "chkWin", "CheckBox", &mut problems) },
            btnRemove: Button { id: handles::resolve(ui, root, "btnRemove", "Button", &mut problems) },
            lblPlus: Label { id: handles::resolve(ui, root, "lblPlus", "Label", &mut problems) },
            cmbKey: ComboBox { id: handles::resolve(ui, root, "cmbKey", "ComboBox", &mut problems) },
            lyWindow: Layout { id: handles::resolve(ui, root, "lyWindow", "Layout", &mut problems) },
            lblWindowTitle: Label { id: handles::resolve(ui, root, "lblWindowTitle", "Label", &mut problems) },
            edtWindowTitle: Edit { id: handles::resolve(ui, root, "edtWindowTitle", "Edit", &mut problems) },
            lyAppAudio: Layout { id: handles::resolve(ui, root, "lyAppAudio", "Layout", &mut problems) },
            chkAppAudio: CheckBox { id: handles::resolve(ui, root, "chkAppAudio", "CheckBox", &mut problems) },
            lyShot: Layout { id: handles::resolve(ui, root, "lyShot", "Layout", &mut problems) },
            lblShotTarget: Label { id: handles::resolve(ui, root, "lblShotTarget", "Label", &mut problems) },
            cmbShotTarget: ComboBox { id: handles::resolve(ui, root, "cmbShotTarget", "ComboBox", &mut problems) },
            lyArea: Layout { id: handles::resolve(ui, root, "lyArea", "Layout", &mut problems) },
            btnAreaClear: Button { id: handles::resolve(ui, root, "btnAreaClear", "Button", &mut problems) },
            btnAreaDefine: Button { id: handles::resolve(ui, root, "btnAreaDefine", "Button", &mut problems) },
            lblArea: Label { id: handles::resolve(ui, root, "lblArea", "Label", &mut problems) },
            lyTimelapse: Layout { id: handles::resolve(ui, root, "lyTimelapse", "Layout", &mut problems) },
            chkTimelapse: CheckBox { id: handles::resolve(ui, root, "chkTimelapse", "CheckBox", &mut problems) },
            lyShotInfo: Layout { id: handles::resolve(ui, root, "lyShotInfo", "Layout", &mut problems) },
            lblShotInfo: Label { id: handles::resolve(ui, root, "lblShotInfo", "Label", &mut problems) },
            lyDelay: Layout { id: handles::resolve(ui, root, "lyDelay", "Layout", &mut problems) },
            lblDelay: Label { id: handles::resolve(ui, root, "lblDelay", "Label", &mut problems) },
            cmbDelay: ComboBox { id: handles::resolve(ui, root, "cmbDelay", "ComboBox", &mut problems) },
            lyStatus: Layout { id: handles::resolve(ui, root, "lyStatus", "Layout", &mut problems) },
            lblStatus: Label { id: handles::resolve(ui, root, "lblStatus", "Label", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form fraHotkeyCard não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FraHotkeyCardComponents {
            form: instance.form,
            pnlCard: instance.pnlCard,
            lyKeys: instance.lyKeys,
            cmbMode: instance.cmbMode,
            pnlCtrl: instance.pnlCtrl,
            chkCtrl: instance.chkCtrl,
            pnlShift: instance.pnlShift,
            chkShift: instance.chkShift,
            pnlAlt: instance.pnlAlt,
            chkAlt: instance.chkAlt,
            pnlWin: instance.pnlWin,
            chkWin: instance.chkWin,
            btnRemove: instance.btnRemove,
            lblPlus: instance.lblPlus,
            cmbKey: instance.cmbKey,
            lyWindow: instance.lyWindow,
            lblWindowTitle: instance.lblWindowTitle,
            edtWindowTitle: instance.edtWindowTitle,
            lyAppAudio: instance.lyAppAudio,
            chkAppAudio: instance.chkAppAudio,
            lyShot: instance.lyShot,
            lblShotTarget: instance.lblShotTarget,
            cmbShotTarget: instance.cmbShotTarget,
            lyArea: instance.lyArea,
            btnAreaClear: instance.btnAreaClear,
            btnAreaDefine: instance.btnAreaDefine,
            lblArea: instance.lblArea,
            lyTimelapse: instance.lyTimelapse,
            chkTimelapse: instance.chkTimelapse,
            lyShotInfo: instance.lyShotInfo,
            lblShotInfo: instance.lblShotInfo,
            lyDelay: instance.lyDelay,
            lblDelay: instance.lblDelay,
            cmbDelay: instance.cmbDelay,
            lyStatus: instance.lyStatus,
            lblStatus: instance.lblStatus,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do fraHotkeyCard aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FraHotkeyCardComponents> {
        ui.form_components(ui.find_path("fraHotkeyCard")?)
    }

    /// Cria o fraHotkeyCard (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, aylis::prelude::Error> {
        ui.create_form("fraHotkeyCard")
    }

    /// Cria mais um fraHotkeyCard dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`fraHotkeyCard`, `fraHotkeyCard2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FraHotkeyCardComponents, aylis::prelude::Error> {
        let root = ui.create_form_instance("fraHotkeyCard")?;
        let name = ui.free_name_in(parent, "fraHotkeyCard");
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

    /// Os componentes de uma instância do fraHotkeyCard (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FraHotkeyCardComponents> {
        ui.form_components(root)
    }

    /// O fraHotkeyCard aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FraHotkeyCard> {
        ui.form_instance(ui.find_path("fraHotkeyCard")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FraHotkeyCard> {
        let form = ui.find_path("fraHotkeyCard")?;
        ui.form_instance_mut(form)
    }

    /// O Button deste form com o nome (`None` se não há um Button com ele).
    pub fn find_button(&self, name: &str) -> Option<Button> {
        match name {
            "btnRemove" => Some(self.btnRemove),
            "btnAreaClear" => Some(self.btnAreaClear),
            "btnAreaDefine" => Some(self.btnAreaDefine),
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
            "chkCtrl" => Some(self.chkCtrl),
            "chkShift" => Some(self.chkShift),
            "chkAlt" => Some(self.chkAlt),
            "chkWin" => Some(self.chkWin),
            "chkAppAudio" => Some(self.chkAppAudio),
            "chkTimelapse" => Some(self.chkTimelapse),
            _ => None,
        }
    }

    /// O CheckBox que disparou o evento (`None` se o `sender` não é um CheckBox deste form).
    pub fn sender_as_check_box(&self, ui: &Ui, sender: ComponentId) -> Option<CheckBox> {
        self.find_check_box(ui.name(sender)?)
    }

    /// O ComboBox deste form com o nome (`None` se não há um ComboBox com ele).
    pub fn find_combo_box(&self, name: &str) -> Option<ComboBox> {
        match name {
            "cmbMode" => Some(self.cmbMode),
            "cmbKey" => Some(self.cmbKey),
            "cmbShotTarget" => Some(self.cmbShotTarget),
            "cmbDelay" => Some(self.cmbDelay),
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
            "edtWindowTitle" => Some(self.edtWindowTitle),
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
            "lblPlus" => Some(self.lblPlus),
            "lblWindowTitle" => Some(self.lblWindowTitle),
            "lblShotTarget" => Some(self.lblShotTarget),
            "lblArea" => Some(self.lblArea),
            "lblShotInfo" => Some(self.lblShotInfo),
            "lblDelay" => Some(self.lblDelay),
            "lblStatus" => Some(self.lblStatus),
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
            "lyKeys" => Some(self.lyKeys),
            "lyWindow" => Some(self.lyWindow),
            "lyAppAudio" => Some(self.lyAppAudio),
            "lyShot" => Some(self.lyShot),
            "lyArea" => Some(self.lyArea),
            "lyTimelapse" => Some(self.lyTimelapse),
            "lyShotInfo" => Some(self.lyShotInfo),
            "lyDelay" => Some(self.lyDelay),
            "lyStatus" => Some(self.lyStatus),
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
            "pnlCard" => Some(self.pnlCard),
            "pnlCtrl" => Some(self.pnlCtrl),
            "pnlShift" => Some(self.pnlShift),
            "pnlAlt" => Some(self.pnlAlt),
            "pnlWin" => Some(self.pnlWin),
            _ => None,
        }
    }

    /// O Panel que disparou o evento (`None` se o `sender` não é um Panel deste form).
    pub fn sender_as_panel(&self, ui: &Ui, sender: ComponentId) -> Option<Panel> {
        self.find_panel(ui.name(sender)?)
    }
}
