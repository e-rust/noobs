
/// O form fraReplay: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FraReplay {
    pub form: Form,
    pub lblReplay: Label,
    pub lblReplayHint: Label,
    pub lyReplayOptions: Layout,
    pub lnRule1: Rectangle,
    pub chkReplayAutoStart: CheckBox,
    pub lblReplayAutoStartHint: Label,
    pub chkReplayIntoRecording: CheckBox,
    pub lblReplayIntoRecordingHint: Label,
    pub lyReplayHeads: Layout,
    pub lblReplayDuration: Label,
    pub lblReplayMemory: Label,
    pub lyReplayLimits: Layout,
    pub cmbReplaySeconds: ComboBox,
    pub cmbReplayMemory: ComboBox,
    pub lblReplayMemoryHint: Label,
    pub lblReplayApps: Label,
    pub edtReplayApps: Edit,
    pub lblReplayAppsHint: Label,
    pub chkReplayIndicator: CheckBox,
    pub lblReplayIndicatorHint: Label,
    pub lyRepOptions: Layout,
    pub chkRepClickable: CheckBox,
    pub lblRepClickableHint: Label,
    pub lyRepCorner: Layout,
    pub lblRepCorner: Label,
    pub cmbRepCorner: ComboBox,
    pub lyRepOpacity: Layout,
    pub lblRepOpacity: Label,
    pub lblRepOpacityPct: Label,
    pub pnlRepPreview: Panel,
    pub dotRepPreview: Circle,
    pub lblRepPreview: Label,
    pub lyRepOpacitySlider: Layout,
    pub trkRepOpacity: TrackBar,
    pub data: super::replay::Data,
}

/// Os componentes do fraReplay sem o estado, para o código fora dos handlers
/// (`FraReplay::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FraReplayComponents {
    pub form: Form,
    pub lblReplay: Label,
    pub lblReplayHint: Label,
    pub lyReplayOptions: Layout,
    pub lnRule1: Rectangle,
    pub chkReplayAutoStart: CheckBox,
    pub lblReplayAutoStartHint: Label,
    pub chkReplayIntoRecording: CheckBox,
    pub lblReplayIntoRecordingHint: Label,
    pub lyReplayHeads: Layout,
    pub lblReplayDuration: Label,
    pub lblReplayMemory: Label,
    pub lyReplayLimits: Layout,
    pub cmbReplaySeconds: ComboBox,
    pub cmbReplayMemory: ComboBox,
    pub lblReplayMemoryHint: Label,
    pub lblReplayApps: Label,
    pub edtReplayApps: Edit,
    pub lblReplayAppsHint: Label,
    pub chkReplayIndicator: CheckBox,
    pub lblReplayIndicatorHint: Label,
    pub lyRepOptions: Layout,
    pub chkRepClickable: CheckBox,
    pub lblRepClickableHint: Label,
    pub lyRepCorner: Layout,
    pub lblRepCorner: Label,
    pub cmbRepCorner: ComboBox,
    pub lyRepOpacity: Layout,
    pub lblRepOpacity: Label,
    pub lblRepOpacityPct: Label,
    pub pnlRepPreview: Panel,
    pub dotRepPreview: Circle,
    pub lblRepPreview: Label,
    pub lyRepOpacitySlider: Layout,
    pub trkRepOpacity: TrackBar,
}

impl std::ops::Deref for FraReplay {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FraReplay {
    /// Monta a instância de um fraReplay recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            lblReplay: Label { id: handles::resolve(ui, root, "lblReplay", "Label", &mut problems) },
            lblReplayHint: Label { id: handles::resolve(ui, root, "lblReplayHint", "Label", &mut problems) },
            lyReplayOptions: Layout { id: handles::resolve(ui, root, "lyReplayOptions", "Layout", &mut problems) },
            lnRule1: Rectangle { id: handles::resolve(ui, root, "lnRule1", "Rectangle", &mut problems) },
            chkReplayAutoStart: CheckBox { id: handles::resolve(ui, root, "chkReplayAutoStart", "CheckBox", &mut problems) },
            lblReplayAutoStartHint: Label { id: handles::resolve(ui, root, "lblReplayAutoStartHint", "Label", &mut problems) },
            chkReplayIntoRecording: CheckBox { id: handles::resolve(ui, root, "chkReplayIntoRecording", "CheckBox", &mut problems) },
            lblReplayIntoRecordingHint: Label { id: handles::resolve(ui, root, "lblReplayIntoRecordingHint", "Label", &mut problems) },
            lyReplayHeads: Layout { id: handles::resolve(ui, root, "lyReplayHeads", "Layout", &mut problems) },
            lblReplayDuration: Label { id: handles::resolve(ui, root, "lblReplayDuration", "Label", &mut problems) },
            lblReplayMemory: Label { id: handles::resolve(ui, root, "lblReplayMemory", "Label", &mut problems) },
            lyReplayLimits: Layout { id: handles::resolve(ui, root, "lyReplayLimits", "Layout", &mut problems) },
            cmbReplaySeconds: ComboBox { id: handles::resolve(ui, root, "cmbReplaySeconds", "ComboBox", &mut problems) },
            cmbReplayMemory: ComboBox { id: handles::resolve(ui, root, "cmbReplayMemory", "ComboBox", &mut problems) },
            lblReplayMemoryHint: Label { id: handles::resolve(ui, root, "lblReplayMemoryHint", "Label", &mut problems) },
            lblReplayApps: Label { id: handles::resolve(ui, root, "lblReplayApps", "Label", &mut problems) },
            edtReplayApps: Edit { id: handles::resolve(ui, root, "edtReplayApps", "Edit", &mut problems) },
            lblReplayAppsHint: Label { id: handles::resolve(ui, root, "lblReplayAppsHint", "Label", &mut problems) },
            chkReplayIndicator: CheckBox { id: handles::resolve(ui, root, "chkReplayIndicator", "CheckBox", &mut problems) },
            lblReplayIndicatorHint: Label { id: handles::resolve(ui, root, "lblReplayIndicatorHint", "Label", &mut problems) },
            lyRepOptions: Layout { id: handles::resolve(ui, root, "lyRepOptions", "Layout", &mut problems) },
            chkRepClickable: CheckBox { id: handles::resolve(ui, root, "chkRepClickable", "CheckBox", &mut problems) },
            lblRepClickableHint: Label { id: handles::resolve(ui, root, "lblRepClickableHint", "Label", &mut problems) },
            lyRepCorner: Layout { id: handles::resolve(ui, root, "lyRepCorner", "Layout", &mut problems) },
            lblRepCorner: Label { id: handles::resolve(ui, root, "lblRepCorner", "Label", &mut problems) },
            cmbRepCorner: ComboBox { id: handles::resolve(ui, root, "cmbRepCorner", "ComboBox", &mut problems) },
            lyRepOpacity: Layout { id: handles::resolve(ui, root, "lyRepOpacity", "Layout", &mut problems) },
            lblRepOpacity: Label { id: handles::resolve(ui, root, "lblRepOpacity", "Label", &mut problems) },
            lblRepOpacityPct: Label { id: handles::resolve(ui, root, "lblRepOpacityPct", "Label", &mut problems) },
            pnlRepPreview: Panel { id: handles::resolve(ui, root, "pnlRepPreview", "Panel", &mut problems) },
            dotRepPreview: Circle { id: handles::resolve(ui, root, "dotRepPreview", "Circle", &mut problems) },
            lblRepPreview: Label { id: handles::resolve(ui, root, "lblRepPreview", "Label", &mut problems) },
            lyRepOpacitySlider: Layout { id: handles::resolve(ui, root, "lyRepOpacitySlider", "Layout", &mut problems) },
            trkRepOpacity: TrackBar { id: handles::resolve(ui, root, "trkRepOpacity", "TrackBar", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form fraReplay não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FraReplayComponents {
            form: instance.form,
            lblReplay: instance.lblReplay,
            lblReplayHint: instance.lblReplayHint,
            lyReplayOptions: instance.lyReplayOptions,
            lnRule1: instance.lnRule1,
            chkReplayAutoStart: instance.chkReplayAutoStart,
            lblReplayAutoStartHint: instance.lblReplayAutoStartHint,
            chkReplayIntoRecording: instance.chkReplayIntoRecording,
            lblReplayIntoRecordingHint: instance.lblReplayIntoRecordingHint,
            lyReplayHeads: instance.lyReplayHeads,
            lblReplayDuration: instance.lblReplayDuration,
            lblReplayMemory: instance.lblReplayMemory,
            lyReplayLimits: instance.lyReplayLimits,
            cmbReplaySeconds: instance.cmbReplaySeconds,
            cmbReplayMemory: instance.cmbReplayMemory,
            lblReplayMemoryHint: instance.lblReplayMemoryHint,
            lblReplayApps: instance.lblReplayApps,
            edtReplayApps: instance.edtReplayApps,
            lblReplayAppsHint: instance.lblReplayAppsHint,
            chkReplayIndicator: instance.chkReplayIndicator,
            lblReplayIndicatorHint: instance.lblReplayIndicatorHint,
            lyRepOptions: instance.lyRepOptions,
            chkRepClickable: instance.chkRepClickable,
            lblRepClickableHint: instance.lblRepClickableHint,
            lyRepCorner: instance.lyRepCorner,
            lblRepCorner: instance.lblRepCorner,
            cmbRepCorner: instance.cmbRepCorner,
            lyRepOpacity: instance.lyRepOpacity,
            lblRepOpacity: instance.lblRepOpacity,
            lblRepOpacityPct: instance.lblRepOpacityPct,
            pnlRepPreview: instance.pnlRepPreview,
            dotRepPreview: instance.dotRepPreview,
            lblRepPreview: instance.lblRepPreview,
            lyRepOpacitySlider: instance.lyRepOpacitySlider,
            trkRepOpacity: instance.trkRepOpacity,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do fraReplay aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FraReplayComponents> {
        ui.form_components(ui.find_path("fraReplay")?)
    }

    /// Cria o fraReplay (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, iron::prelude::Error> {
        ui.create_form("fraReplay")
    }

    /// Cria mais um fraReplay dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`fraReplay`, `fraReplay2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FraReplayComponents, iron::prelude::Error> {
        let root = ui.create_form_instance("fraReplay")?;
        let name = ui.free_name_in(parent, "fraReplay");
        if let Err(e) = ui.set_name(root, &name).and_then(|_| ui.set_parent(root, Some(parent))) {
            ui.free(root);
            return Err(e);
        }
        ui.form_components(root).ok_or(iron::prelude::Error::InvalidId)
    }

    /// Os componentes de uma instância do fraReplay (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FraReplayComponents> {
        ui.form_components(root)
    }

    /// O fraReplay aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FraReplay> {
        ui.form_instance(ui.find_path("fraReplay")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FraReplay> {
        let form = ui.find_path("fraReplay")?;
        ui.form_instance_mut(form)
    }

    /// O CheckBox deste form com o nome (`None` se não há um CheckBox com ele).
    pub fn find_check_box(&self, name: &str) -> Option<CheckBox> {
        match name {
            "chkReplayAutoStart" => Some(self.chkReplayAutoStart),
            "chkReplayIntoRecording" => Some(self.chkReplayIntoRecording),
            "chkReplayIndicator" => Some(self.chkReplayIndicator),
            "chkRepClickable" => Some(self.chkRepClickable),
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
            "dotRepPreview" => Some(self.dotRepPreview),
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
            "cmbReplaySeconds" => Some(self.cmbReplaySeconds),
            "cmbReplayMemory" => Some(self.cmbReplayMemory),
            "cmbRepCorner" => Some(self.cmbRepCorner),
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
            "edtReplayApps" => Some(self.edtReplayApps),
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
            "lblReplay" => Some(self.lblReplay),
            "lblReplayHint" => Some(self.lblReplayHint),
            "lblReplayAutoStartHint" => Some(self.lblReplayAutoStartHint),
            "lblReplayIntoRecordingHint" => Some(self.lblReplayIntoRecordingHint),
            "lblReplayDuration" => Some(self.lblReplayDuration),
            "lblReplayMemory" => Some(self.lblReplayMemory),
            "lblReplayMemoryHint" => Some(self.lblReplayMemoryHint),
            "lblReplayApps" => Some(self.lblReplayApps),
            "lblReplayAppsHint" => Some(self.lblReplayAppsHint),
            "lblReplayIndicatorHint" => Some(self.lblReplayIndicatorHint),
            "lblRepClickableHint" => Some(self.lblRepClickableHint),
            "lblRepCorner" => Some(self.lblRepCorner),
            "lblRepOpacity" => Some(self.lblRepOpacity),
            "lblRepOpacityPct" => Some(self.lblRepOpacityPct),
            "lblRepPreview" => Some(self.lblRepPreview),
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
            "lyReplayOptions" => Some(self.lyReplayOptions),
            "lyReplayHeads" => Some(self.lyReplayHeads),
            "lyReplayLimits" => Some(self.lyReplayLimits),
            "lyRepOptions" => Some(self.lyRepOptions),
            "lyRepCorner" => Some(self.lyRepCorner),
            "lyRepOpacity" => Some(self.lyRepOpacity),
            "lyRepOpacitySlider" => Some(self.lyRepOpacitySlider),
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
            "pnlRepPreview" => Some(self.pnlRepPreview),
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
            "trkRepOpacity" => Some(self.trkRepOpacity),
            _ => None,
        }
    }

    /// O TrackBar que disparou o evento (`None` se o `sender` não é um TrackBar deste form).
    pub fn sender_as_track_bar(&self, ui: &Ui, sender: ComponentId) -> Option<TrackBar> {
        self.find_track_bar(ui.name(sender)?)
    }
}
