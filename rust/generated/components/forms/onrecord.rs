
/// O form fraOnRecord: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FraOnRecord {
    pub form: Form,
    pub chkRecIndicator: CheckBox,
    pub lblRecIndicatorHint: Label,
    pub lyRecOptions: Layout,
    pub chkRecClickable: CheckBox,
    pub lblRecClickableHint: Label,
    pub lyRecCorner: Layout,
    pub lblRecCorner: Label,
    pub cmbRecCorner: ComboBox,
    pub lyRecOpacity: Layout,
    pub lblRecOpacity: Label,
    pub lblRecOpacityPct: Label,
    pub pnlRecPreview: Panel,
    pub dotRecPreview: Circle,
    pub lblRecPreview: Label,
    pub lyRecOpacitySlider: Layout,
    pub trkRecOpacity: TrackBar,
    pub lnRule1: Rectangle,
    pub chkFullscreen: CheckBox,
    pub lblFullscreenHint: Label,
    pub lnRule2: Rectangle,
    pub chkPlaySound: CheckBox,
    pub lblPlaySoundHint: Label,
    pub lnRule3: Rectangle,
    pub chkMinimize: CheckBox,
    pub lblMinimizeHint: Label,
    pub lnRule4: Rectangle,
    pub chkAutoRecord: CheckBox,
    pub lblAutoRecordHint: Label,
    pub edtAutoApps: Edit,
    pub lblAutoAppsHint: Label,
    pub lyAutoExcept: Layout,
    pub edtAutoExcept: Edit,
    pub lblAutoExceptHint: Label,
    pub lyAutoDevices: Layout,
    pub lnAutoDevices: Rectangle,
    pub lblAutoDevices: Label,
    pub lblAutoDevicesHint: Label,
    pub lyAutoDeviceList: Layout,
    pub lblAutoDevicesEmpty: Label,
    pub lnRule5: Rectangle,
    pub chkStopOnLock: CheckBox,
    pub lblStopOnLockHint: Label,
    pub data: super::onrecord::Data,
}

/// Os componentes do fraOnRecord sem o estado, para o código fora dos handlers
/// (`FraOnRecord::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FraOnRecordComponents {
    pub form: Form,
    pub chkRecIndicator: CheckBox,
    pub lblRecIndicatorHint: Label,
    pub lyRecOptions: Layout,
    pub chkRecClickable: CheckBox,
    pub lblRecClickableHint: Label,
    pub lyRecCorner: Layout,
    pub lblRecCorner: Label,
    pub cmbRecCorner: ComboBox,
    pub lyRecOpacity: Layout,
    pub lblRecOpacity: Label,
    pub lblRecOpacityPct: Label,
    pub pnlRecPreview: Panel,
    pub dotRecPreview: Circle,
    pub lblRecPreview: Label,
    pub lyRecOpacitySlider: Layout,
    pub trkRecOpacity: TrackBar,
    pub lnRule1: Rectangle,
    pub chkFullscreen: CheckBox,
    pub lblFullscreenHint: Label,
    pub lnRule2: Rectangle,
    pub chkPlaySound: CheckBox,
    pub lblPlaySoundHint: Label,
    pub lnRule3: Rectangle,
    pub chkMinimize: CheckBox,
    pub lblMinimizeHint: Label,
    pub lnRule4: Rectangle,
    pub chkAutoRecord: CheckBox,
    pub lblAutoRecordHint: Label,
    pub edtAutoApps: Edit,
    pub lblAutoAppsHint: Label,
    pub lyAutoExcept: Layout,
    pub edtAutoExcept: Edit,
    pub lblAutoExceptHint: Label,
    pub lyAutoDevices: Layout,
    pub lnAutoDevices: Rectangle,
    pub lblAutoDevices: Label,
    pub lblAutoDevicesHint: Label,
    pub lyAutoDeviceList: Layout,
    pub lblAutoDevicesEmpty: Label,
    pub lnRule5: Rectangle,
    pub chkStopOnLock: CheckBox,
    pub lblStopOnLockHint: Label,
}

impl std::ops::Deref for FraOnRecord {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FraOnRecord {
    /// Monta a instância de um fraOnRecord recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            chkRecIndicator: CheckBox { id: handles::resolve(ui, root, "chkRecIndicator", "CheckBox", &mut problems) },
            lblRecIndicatorHint: Label { id: handles::resolve(ui, root, "lblRecIndicatorHint", "Label", &mut problems) },
            lyRecOptions: Layout { id: handles::resolve(ui, root, "lyRecOptions", "Layout", &mut problems) },
            chkRecClickable: CheckBox { id: handles::resolve(ui, root, "chkRecClickable", "CheckBox", &mut problems) },
            lblRecClickableHint: Label { id: handles::resolve(ui, root, "lblRecClickableHint", "Label", &mut problems) },
            lyRecCorner: Layout { id: handles::resolve(ui, root, "lyRecCorner", "Layout", &mut problems) },
            lblRecCorner: Label { id: handles::resolve(ui, root, "lblRecCorner", "Label", &mut problems) },
            cmbRecCorner: ComboBox { id: handles::resolve(ui, root, "cmbRecCorner", "ComboBox", &mut problems) },
            lyRecOpacity: Layout { id: handles::resolve(ui, root, "lyRecOpacity", "Layout", &mut problems) },
            lblRecOpacity: Label { id: handles::resolve(ui, root, "lblRecOpacity", "Label", &mut problems) },
            lblRecOpacityPct: Label { id: handles::resolve(ui, root, "lblRecOpacityPct", "Label", &mut problems) },
            pnlRecPreview: Panel { id: handles::resolve(ui, root, "pnlRecPreview", "Panel", &mut problems) },
            dotRecPreview: Circle { id: handles::resolve(ui, root, "dotRecPreview", "Circle", &mut problems) },
            lblRecPreview: Label { id: handles::resolve(ui, root, "lblRecPreview", "Label", &mut problems) },
            lyRecOpacitySlider: Layout { id: handles::resolve(ui, root, "lyRecOpacitySlider", "Layout", &mut problems) },
            trkRecOpacity: TrackBar { id: handles::resolve(ui, root, "trkRecOpacity", "TrackBar", &mut problems) },
            lnRule1: Rectangle { id: handles::resolve(ui, root, "lnRule1", "Rectangle", &mut problems) },
            chkFullscreen: CheckBox { id: handles::resolve(ui, root, "chkFullscreen", "CheckBox", &mut problems) },
            lblFullscreenHint: Label { id: handles::resolve(ui, root, "lblFullscreenHint", "Label", &mut problems) },
            lnRule2: Rectangle { id: handles::resolve(ui, root, "lnRule2", "Rectangle", &mut problems) },
            chkPlaySound: CheckBox { id: handles::resolve(ui, root, "chkPlaySound", "CheckBox", &mut problems) },
            lblPlaySoundHint: Label { id: handles::resolve(ui, root, "lblPlaySoundHint", "Label", &mut problems) },
            lnRule3: Rectangle { id: handles::resolve(ui, root, "lnRule3", "Rectangle", &mut problems) },
            chkMinimize: CheckBox { id: handles::resolve(ui, root, "chkMinimize", "CheckBox", &mut problems) },
            lblMinimizeHint: Label { id: handles::resolve(ui, root, "lblMinimizeHint", "Label", &mut problems) },
            lnRule4: Rectangle { id: handles::resolve(ui, root, "lnRule4", "Rectangle", &mut problems) },
            chkAutoRecord: CheckBox { id: handles::resolve(ui, root, "chkAutoRecord", "CheckBox", &mut problems) },
            lblAutoRecordHint: Label { id: handles::resolve(ui, root, "lblAutoRecordHint", "Label", &mut problems) },
            edtAutoApps: Edit { id: handles::resolve(ui, root, "edtAutoApps", "Edit", &mut problems) },
            lblAutoAppsHint: Label { id: handles::resolve(ui, root, "lblAutoAppsHint", "Label", &mut problems) },
            lyAutoExcept: Layout { id: handles::resolve(ui, root, "lyAutoExcept", "Layout", &mut problems) },
            edtAutoExcept: Edit { id: handles::resolve(ui, root, "edtAutoExcept", "Edit", &mut problems) },
            lblAutoExceptHint: Label { id: handles::resolve(ui, root, "lblAutoExceptHint", "Label", &mut problems) },
            lyAutoDevices: Layout { id: handles::resolve(ui, root, "lyAutoDevices", "Layout", &mut problems) },
            lnAutoDevices: Rectangle { id: handles::resolve(ui, root, "lnAutoDevices", "Rectangle", &mut problems) },
            lblAutoDevices: Label { id: handles::resolve(ui, root, "lblAutoDevices", "Label", &mut problems) },
            lblAutoDevicesHint: Label { id: handles::resolve(ui, root, "lblAutoDevicesHint", "Label", &mut problems) },
            lyAutoDeviceList: Layout { id: handles::resolve(ui, root, "lyAutoDeviceList", "Layout", &mut problems) },
            lblAutoDevicesEmpty: Label { id: handles::resolve(ui, root, "lblAutoDevicesEmpty", "Label", &mut problems) },
            lnRule5: Rectangle { id: handles::resolve(ui, root, "lnRule5", "Rectangle", &mut problems) },
            chkStopOnLock: CheckBox { id: handles::resolve(ui, root, "chkStopOnLock", "CheckBox", &mut problems) },
            lblStopOnLockHint: Label { id: handles::resolve(ui, root, "lblStopOnLockHint", "Label", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form fraOnRecord não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FraOnRecordComponents {
            form: instance.form,
            chkRecIndicator: instance.chkRecIndicator,
            lblRecIndicatorHint: instance.lblRecIndicatorHint,
            lyRecOptions: instance.lyRecOptions,
            chkRecClickable: instance.chkRecClickable,
            lblRecClickableHint: instance.lblRecClickableHint,
            lyRecCorner: instance.lyRecCorner,
            lblRecCorner: instance.lblRecCorner,
            cmbRecCorner: instance.cmbRecCorner,
            lyRecOpacity: instance.lyRecOpacity,
            lblRecOpacity: instance.lblRecOpacity,
            lblRecOpacityPct: instance.lblRecOpacityPct,
            pnlRecPreview: instance.pnlRecPreview,
            dotRecPreview: instance.dotRecPreview,
            lblRecPreview: instance.lblRecPreview,
            lyRecOpacitySlider: instance.lyRecOpacitySlider,
            trkRecOpacity: instance.trkRecOpacity,
            lnRule1: instance.lnRule1,
            chkFullscreen: instance.chkFullscreen,
            lblFullscreenHint: instance.lblFullscreenHint,
            lnRule2: instance.lnRule2,
            chkPlaySound: instance.chkPlaySound,
            lblPlaySoundHint: instance.lblPlaySoundHint,
            lnRule3: instance.lnRule3,
            chkMinimize: instance.chkMinimize,
            lblMinimizeHint: instance.lblMinimizeHint,
            lnRule4: instance.lnRule4,
            chkAutoRecord: instance.chkAutoRecord,
            lblAutoRecordHint: instance.lblAutoRecordHint,
            edtAutoApps: instance.edtAutoApps,
            lblAutoAppsHint: instance.lblAutoAppsHint,
            lyAutoExcept: instance.lyAutoExcept,
            edtAutoExcept: instance.edtAutoExcept,
            lblAutoExceptHint: instance.lblAutoExceptHint,
            lyAutoDevices: instance.lyAutoDevices,
            lnAutoDevices: instance.lnAutoDevices,
            lblAutoDevices: instance.lblAutoDevices,
            lblAutoDevicesHint: instance.lblAutoDevicesHint,
            lyAutoDeviceList: instance.lyAutoDeviceList,
            lblAutoDevicesEmpty: instance.lblAutoDevicesEmpty,
            lnRule5: instance.lnRule5,
            chkStopOnLock: instance.chkStopOnLock,
            lblStopOnLockHint: instance.lblStopOnLockHint,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do fraOnRecord aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FraOnRecordComponents> {
        ui.form_components(ui.find_path("fraOnRecord")?)
    }

    /// Cria o fraOnRecord (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, iron::prelude::Error> {
        ui.create_form("fraOnRecord")
    }

    /// Cria mais um fraOnRecord dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`fraOnRecord`, `fraOnRecord2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FraOnRecordComponents, iron::prelude::Error> {
        let root = ui.create_form_instance("fraOnRecord")?;
        let name = ui.free_name_in(parent, "fraOnRecord");
        if let Err(e) = ui.set_name(root, &name).and_then(|_| ui.set_parent(root, Some(parent))) {
            ui.free(root);
            return Err(e);
        }
        ui.form_components(root).ok_or(iron::prelude::Error::InvalidId)
    }

    /// Os componentes de uma instância do fraOnRecord (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FraOnRecordComponents> {
        ui.form_components(root)
    }

    /// O fraOnRecord aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FraOnRecord> {
        ui.form_instance(ui.find_path("fraOnRecord")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FraOnRecord> {
        let form = ui.find_path("fraOnRecord")?;
        ui.form_instance_mut(form)
    }

    /// O CheckBox deste form com o nome (`None` se não há um CheckBox com ele).
    pub fn find_check_box(&self, name: &str) -> Option<CheckBox> {
        match name {
            "chkRecIndicator" => Some(self.chkRecIndicator),
            "chkRecClickable" => Some(self.chkRecClickable),
            "chkFullscreen" => Some(self.chkFullscreen),
            "chkPlaySound" => Some(self.chkPlaySound),
            "chkMinimize" => Some(self.chkMinimize),
            "chkAutoRecord" => Some(self.chkAutoRecord),
            "chkStopOnLock" => Some(self.chkStopOnLock),
            _ => None,
        }
    }

    /// O CheckBox que disparou o evento (`None` se o `sender` não é um CheckBox deste form).
    pub fn sender_as_check_box(&self, ui: &Ui, sender: ComponentId) -> Option<CheckBox> {
        self.find_check_box(ui.name(sender)?)
    }

    /// O Circle deste form com o nome (`None` se não há um Circle com ele).
    pub fn find_circle(&self, name: &str) -> Option<Circle> {
        match name {
            "dotRecPreview" => Some(self.dotRecPreview),
            _ => None,
        }
    }

    /// O Circle que disparou o evento (`None` se o `sender` não é um Circle deste form).
    pub fn sender_as_circle(&self, ui: &Ui, sender: ComponentId) -> Option<Circle> {
        self.find_circle(ui.name(sender)?)
    }

    /// O ComboBox deste form com o nome (`None` se não há um ComboBox com ele).
    pub fn find_combo_box(&self, name: &str) -> Option<ComboBox> {
        match name {
            "cmbRecCorner" => Some(self.cmbRecCorner),
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
            "edtAutoApps" => Some(self.edtAutoApps),
            "edtAutoExcept" => Some(self.edtAutoExcept),
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
            "lblRecIndicatorHint" => Some(self.lblRecIndicatorHint),
            "lblRecClickableHint" => Some(self.lblRecClickableHint),
            "lblRecCorner" => Some(self.lblRecCorner),
            "lblRecOpacity" => Some(self.lblRecOpacity),
            "lblRecOpacityPct" => Some(self.lblRecOpacityPct),
            "lblRecPreview" => Some(self.lblRecPreview),
            "lblFullscreenHint" => Some(self.lblFullscreenHint),
            "lblPlaySoundHint" => Some(self.lblPlaySoundHint),
            "lblMinimizeHint" => Some(self.lblMinimizeHint),
            "lblAutoRecordHint" => Some(self.lblAutoRecordHint),
            "lblAutoAppsHint" => Some(self.lblAutoAppsHint),
            "lblAutoExceptHint" => Some(self.lblAutoExceptHint),
            "lblAutoDevices" => Some(self.lblAutoDevices),
            "lblAutoDevicesHint" => Some(self.lblAutoDevicesHint),
            "lblAutoDevicesEmpty" => Some(self.lblAutoDevicesEmpty),
            "lblStopOnLockHint" => Some(self.lblStopOnLockHint),
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
            "lyRecOptions" => Some(self.lyRecOptions),
            "lyRecCorner" => Some(self.lyRecCorner),
            "lyRecOpacity" => Some(self.lyRecOpacity),
            "lyRecOpacitySlider" => Some(self.lyRecOpacitySlider),
            "lyAutoExcept" => Some(self.lyAutoExcept),
            "lyAutoDevices" => Some(self.lyAutoDevices),
            "lyAutoDeviceList" => Some(self.lyAutoDeviceList),
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
            "pnlRecPreview" => Some(self.pnlRecPreview),
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
            "lnRule1" => Some(self.lnRule1),
            "lnRule2" => Some(self.lnRule2),
            "lnRule3" => Some(self.lnRule3),
            "lnRule4" => Some(self.lnRule4),
            "lnAutoDevices" => Some(self.lnAutoDevices),
            "lnRule5" => Some(self.lnRule5),
            _ => None,
        }
    }

    /// O Rectangle que disparou o evento (`None` se o `sender` não é um Rectangle deste form).
    pub fn sender_as_rectangle(&self, ui: &Ui, sender: ComponentId) -> Option<Rectangle> {
        self.find_rectangle(ui.name(sender)?)
    }

    /// O TrackBar deste form com o nome (`None` se não há um TrackBar com ele).
    pub fn find_track_bar(&self, name: &str) -> Option<TrackBar> {
        match name {
            "trkRecOpacity" => Some(self.trkRecOpacity),
            _ => None,
        }
    }

    /// O TrackBar que disparou o evento (`None` se o `sender` não é um TrackBar deste form).
    pub fn sender_as_track_bar(&self, ui: &Ui, sender: ComponentId) -> Option<TrackBar> {
        self.find_track_bar(ui.name(sender)?)
    }
}
