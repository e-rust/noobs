
/// O form fraTranscribe: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FraTranscribe {
    pub form: Form,
    pub lblTrLocal: Label,
    pub lblTrLocalHint: Label,
    pub pnlTrEngine: Panel,
    pub lblTrStatus: Label,
    pub prgTrInstall: ProgressBar,
    pub lblTrDetail: Label,
    pub lyTrEngineButtons: Layout,
    pub btnTrEngine: Button,
    pub lnRule1: Rectangle,
    pub lblTrLanguage: Label,
    pub cmbTrLanguage: ComboBox,
    pub lblTrLanguageHint: Label,
    pub lnRule2: Rectangle,
    pub chkTrOnStop: CheckBox,
    pub lblTrOnStopHint: Label,
    pub lnRule3: Rectangle,
    pub lblTrQueue: Label,
    pub lyTrQueueButtons: Layout,
    pub btnTrPending: Button,
    pub btnTrCancel: Button,
    pub lyTrProgress: Layout,
    pub prgTrQueue: ProgressBar,
    pub lblTrLine: Label,
    pub lblTrError: Label,
    pub lyTrQueue: Layout,
    pub pnlTrQueue: Panel,
    pub lblTrCloud: Label,
    pub lblTrQueueHint: Label,
    pub data: super::settings_transcribe::Data,
}

/// Os componentes do fraTranscribe sem o estado, para o código fora dos handlers
/// (`FraTranscribe::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FraTranscribeComponents {
    pub form: Form,
    pub lblTrLocal: Label,
    pub lblTrLocalHint: Label,
    pub pnlTrEngine: Panel,
    pub lblTrStatus: Label,
    pub prgTrInstall: ProgressBar,
    pub lblTrDetail: Label,
    pub lyTrEngineButtons: Layout,
    pub btnTrEngine: Button,
    pub lnRule1: Rectangle,
    pub lblTrLanguage: Label,
    pub cmbTrLanguage: ComboBox,
    pub lblTrLanguageHint: Label,
    pub lnRule2: Rectangle,
    pub chkTrOnStop: CheckBox,
    pub lblTrOnStopHint: Label,
    pub lnRule3: Rectangle,
    pub lblTrQueue: Label,
    pub lyTrQueueButtons: Layout,
    pub btnTrPending: Button,
    pub btnTrCancel: Button,
    pub lyTrProgress: Layout,
    pub prgTrQueue: ProgressBar,
    pub lblTrLine: Label,
    pub lblTrError: Label,
    pub lyTrQueue: Layout,
    pub pnlTrQueue: Panel,
    pub lblTrCloud: Label,
    pub lblTrQueueHint: Label,
}

impl std::ops::Deref for FraTranscribe {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FraTranscribe {
    /// Monta a instância de um fraTranscribe recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            lblTrLocal: Label { id: handles::resolve(ui, root, "lblTrLocal", "Label", &mut problems) },
            lblTrLocalHint: Label { id: handles::resolve(ui, root, "lblTrLocalHint", "Label", &mut problems) },
            pnlTrEngine: Panel { id: handles::resolve(ui, root, "pnlTrEngine", "Panel", &mut problems) },
            lblTrStatus: Label { id: handles::resolve(ui, root, "lblTrStatus", "Label", &mut problems) },
            prgTrInstall: ProgressBar { id: handles::resolve(ui, root, "prgTrInstall", "ProgressBar", &mut problems) },
            lblTrDetail: Label { id: handles::resolve(ui, root, "lblTrDetail", "Label", &mut problems) },
            lyTrEngineButtons: Layout { id: handles::resolve(ui, root, "lyTrEngineButtons", "Layout", &mut problems) },
            btnTrEngine: Button { id: handles::resolve(ui, root, "btnTrEngine", "Button", &mut problems) },
            lnRule1: Rectangle { id: handles::resolve(ui, root, "lnRule1", "Rectangle", &mut problems) },
            lblTrLanguage: Label { id: handles::resolve(ui, root, "lblTrLanguage", "Label", &mut problems) },
            cmbTrLanguage: ComboBox { id: handles::resolve(ui, root, "cmbTrLanguage", "ComboBox", &mut problems) },
            lblTrLanguageHint: Label { id: handles::resolve(ui, root, "lblTrLanguageHint", "Label", &mut problems) },
            lnRule2: Rectangle { id: handles::resolve(ui, root, "lnRule2", "Rectangle", &mut problems) },
            chkTrOnStop: CheckBox { id: handles::resolve(ui, root, "chkTrOnStop", "CheckBox", &mut problems) },
            lblTrOnStopHint: Label { id: handles::resolve(ui, root, "lblTrOnStopHint", "Label", &mut problems) },
            lnRule3: Rectangle { id: handles::resolve(ui, root, "lnRule3", "Rectangle", &mut problems) },
            lblTrQueue: Label { id: handles::resolve(ui, root, "lblTrQueue", "Label", &mut problems) },
            lyTrQueueButtons: Layout { id: handles::resolve(ui, root, "lyTrQueueButtons", "Layout", &mut problems) },
            btnTrPending: Button { id: handles::resolve(ui, root, "btnTrPending", "Button", &mut problems) },
            btnTrCancel: Button { id: handles::resolve(ui, root, "btnTrCancel", "Button", &mut problems) },
            lyTrProgress: Layout { id: handles::resolve(ui, root, "lyTrProgress", "Layout", &mut problems) },
            prgTrQueue: ProgressBar { id: handles::resolve(ui, root, "prgTrQueue", "ProgressBar", &mut problems) },
            lblTrLine: Label { id: handles::resolve(ui, root, "lblTrLine", "Label", &mut problems) },
            lblTrError: Label { id: handles::resolve(ui, root, "lblTrError", "Label", &mut problems) },
            lyTrQueue: Layout { id: handles::resolve(ui, root, "lyTrQueue", "Layout", &mut problems) },
            pnlTrQueue: Panel { id: handles::resolve(ui, root, "pnlTrQueue", "Panel", &mut problems) },
            lblTrCloud: Label { id: handles::resolve(ui, root, "lblTrCloud", "Label", &mut problems) },
            lblTrQueueHint: Label { id: handles::resolve(ui, root, "lblTrQueueHint", "Label", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form fraTranscribe não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FraTranscribeComponents {
            form: instance.form,
            lblTrLocal: instance.lblTrLocal,
            lblTrLocalHint: instance.lblTrLocalHint,
            pnlTrEngine: instance.pnlTrEngine,
            lblTrStatus: instance.lblTrStatus,
            prgTrInstall: instance.prgTrInstall,
            lblTrDetail: instance.lblTrDetail,
            lyTrEngineButtons: instance.lyTrEngineButtons,
            btnTrEngine: instance.btnTrEngine,
            lnRule1: instance.lnRule1,
            lblTrLanguage: instance.lblTrLanguage,
            cmbTrLanguage: instance.cmbTrLanguage,
            lblTrLanguageHint: instance.lblTrLanguageHint,
            lnRule2: instance.lnRule2,
            chkTrOnStop: instance.chkTrOnStop,
            lblTrOnStopHint: instance.lblTrOnStopHint,
            lnRule3: instance.lnRule3,
            lblTrQueue: instance.lblTrQueue,
            lyTrQueueButtons: instance.lyTrQueueButtons,
            btnTrPending: instance.btnTrPending,
            btnTrCancel: instance.btnTrCancel,
            lyTrProgress: instance.lyTrProgress,
            prgTrQueue: instance.prgTrQueue,
            lblTrLine: instance.lblTrLine,
            lblTrError: instance.lblTrError,
            lyTrQueue: instance.lyTrQueue,
            pnlTrQueue: instance.pnlTrQueue,
            lblTrCloud: instance.lblTrCloud,
            lblTrQueueHint: instance.lblTrQueueHint,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do fraTranscribe aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FraTranscribeComponents> {
        ui.form_components(ui.find_path("fraTranscribe")?)
    }

    /// Cria o fraTranscribe (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, aylis::prelude::Error> {
        ui.create_form("fraTranscribe")
    }

    /// Cria mais um fraTranscribe dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`fraTranscribe`, `fraTranscribe2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FraTranscribeComponents, aylis::prelude::Error> {
        let root = ui.create_form_instance("fraTranscribe")?;
        let name = ui.free_name_in(parent, "fraTranscribe");
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

    /// Os componentes de uma instância do fraTranscribe (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FraTranscribeComponents> {
        ui.form_components(root)
    }

    /// O fraTranscribe aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FraTranscribe> {
        ui.form_instance(ui.find_path("fraTranscribe")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FraTranscribe> {
        let form = ui.find_path("fraTranscribe")?;
        ui.form_instance_mut(form)
    }

    /// O Button deste form com o nome (`None` se não há um Button com ele).
    pub fn find_button(&self, name: &str) -> Option<Button> {
        match name {
            "btnTrEngine" => Some(self.btnTrEngine),
            "btnTrPending" => Some(self.btnTrPending),
            "btnTrCancel" => Some(self.btnTrCancel),
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
            "chkTrOnStop" => Some(self.chkTrOnStop),
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
            "cmbTrLanguage" => Some(self.cmbTrLanguage),
            _ => None,
        }
    }

    /// O ComboBox que disparou o evento (`None` se o `sender` não é um ComboBox deste form).
    pub fn sender_as_combo_box(&self, ui: &Ui, sender: ComponentId) -> Option<ComboBox> {
        self.find_combo_box(ui.name(sender)?)
    }

    /// O Label deste form com o nome (`None` se não há um Label com ele).
    pub fn find_label(&self, name: &str) -> Option<Label> {
        match name {
            "lblTrLocal" => Some(self.lblTrLocal),
            "lblTrLocalHint" => Some(self.lblTrLocalHint),
            "lblTrStatus" => Some(self.lblTrStatus),
            "lblTrDetail" => Some(self.lblTrDetail),
            "lblTrLanguage" => Some(self.lblTrLanguage),
            "lblTrLanguageHint" => Some(self.lblTrLanguageHint),
            "lblTrOnStopHint" => Some(self.lblTrOnStopHint),
            "lblTrQueue" => Some(self.lblTrQueue),
            "lblTrLine" => Some(self.lblTrLine),
            "lblTrError" => Some(self.lblTrError),
            "lblTrCloud" => Some(self.lblTrCloud),
            "lblTrQueueHint" => Some(self.lblTrQueueHint),
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
            "lyTrEngineButtons" => Some(self.lyTrEngineButtons),
            "lyTrQueueButtons" => Some(self.lyTrQueueButtons),
            "lyTrProgress" => Some(self.lyTrProgress),
            "lyTrQueue" => Some(self.lyTrQueue),
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
            "pnlTrEngine" => Some(self.pnlTrEngine),
            "pnlTrQueue" => Some(self.pnlTrQueue),
            _ => None,
        }
    }

    /// O Panel que disparou o evento (`None` se o `sender` não é um Panel deste form).
    pub fn sender_as_panel(&self, ui: &Ui, sender: ComponentId) -> Option<Panel> {
        self.find_panel(ui.name(sender)?)
    }

    /// O ProgressBar deste form com o nome (`None` se não há um ProgressBar com ele).
    pub fn find_progress_bar(&self, name: &str) -> Option<ProgressBar> {
        match name {
            "prgTrInstall" => Some(self.prgTrInstall),
            "prgTrQueue" => Some(self.prgTrQueue),
            _ => None,
        }
    }

    /// O ProgressBar que disparou o evento (`None` se o `sender` não é um ProgressBar deste form).
    pub fn sender_as_progress_bar(&self, ui: &Ui, sender: ComponentId) -> Option<ProgressBar> {
        self.find_progress_bar(ui.name(sender)?)
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
