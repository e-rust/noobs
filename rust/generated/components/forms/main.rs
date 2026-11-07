
/// O form frmMain: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FrmMain {
    pub form: Form,
    pub pnlSources: Panel,
    pub lnSourcesEdge: Rectangle,
    pub pnlUpdate: Panel,
    pub btnUpdClose: Button,
    pub btnUpdOpen: Button,
    pub lblUpdTitle: Label,
    pub lblUpdBody: Label,
    pub pnlRecordCard: Panel,
    pub lyCardHeader: Layout,
    pub dotStatus: Circle,
    pub btnSettings: Button,
    pub lblTrChip: Label,
    pub lblStatus: Label,
    pub lyTimer: FlowLayout,
    pub lblTimer: Label,
    pub lblTimerMs: Label,
    pub lblSize: Label,
    pub lyRecord: FlowLayout,
    pub btnRecord: RecordButton,
    pub lblRecordAction: Label,
    pub lnCard: Rectangle,
    pub pnlReplay: Panel,
    pub dotReplay: Circle,
    pub btnPause: Button,
    pub btnCapture: Button,
    pub swReplay: Switch,
    pub btnReplaySave: Button,
    pub lyReplayText: Layout,
    pub lblReplay: Label,
    pub lblReplayStatus: Label,
    pub lnSources: Rectangle,
    pub pnlLegend: Panel,
    pub lnLegend: Rectangle,
    pub lyLegendTitle: Layout,
    pub lblLegendTitle: Label,
    pub lyLegend: Layout,
    pub scrSources: ScrollBox,
    pub lySources: Layout,
    pub lyMicsTitle: Layout,
    pub lblMicsCount: Label,
    pub lblMicsTitle: Label,
    pub lyMics: Layout,
    pub lySpeakersTitle: Layout,
    pub lblSpeakersCount: Label,
    pub lblSpeakersTitle: Label,
    pub lySpeakers: Layout,
    pub pnlRecordings: Panel,
    pub scrRecordings: ScrollBox,
    pub lyDisplays: Layout,
    pub lyDisplaysTitle: Layout,
    pub lblScreens: Label,
    pub lblScreensCount: Label,
    pub lblScreensMeta: Label,
    pub scrDisplays: ScreenLayout,
    pub lyLibrary: Layout,
    pub lyLibHeader: Layout,
    pub lblRecordings: Label,
    pub lblLibMeta: Label,
    pub lyLibTools: Layout,
    pub edLibSearch: Edit,
    pub btnLibOpenDir: Button,
    pub btnLibNewFolder: Button,
    pub btnLibMerge: Button,
    pub btnLibExport: Button,
    pub btnLibDelete: Button,
    pub brdLib: Breadcrumb,
    pub lblLibEmpty: Label,
    pub lyLibContent: Layout,
    pub edLibRename: Edit,
    pub cirRippleGlow: Circle,
    pub cirRipple: Circle,
    pub pmLibRecording: PopupMenu,
    pub miRecExport: MenuItem,
    pub miRecCut: MenuItem,
    pub miRecTranscribe: MenuItem,
    pub miRecSep: MenuItem,
    pub miRecDelete: MenuItem,
    pub pmLibFolder: PopupMenu,
    pub miFolderOpen: MenuItem,
    pub miFolderRename: MenuItem,
    pub miFolderCut: MenuItem,
    pub miFolderPaste: MenuItem,
    pub miFolderSep: MenuItem,
    pub miFolderDelete: MenuItem,
    pub pmLibEmpty: PopupMenu,
    pub miEmptyNewFolder: MenuItem,
    pub miEmptyPaste: MenuItem,
    pub miEmptySep: MenuItem,
    pub miEmptyUp: MenuItem,
    pub data: super::main::Data,
}

/// Os componentes do frmMain sem o estado, para o código fora dos handlers
/// (`FrmMain::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FrmMainComponents {
    pub form: Form,
    pub pnlSources: Panel,
    pub lnSourcesEdge: Rectangle,
    pub pnlUpdate: Panel,
    pub btnUpdClose: Button,
    pub btnUpdOpen: Button,
    pub lblUpdTitle: Label,
    pub lblUpdBody: Label,
    pub pnlRecordCard: Panel,
    pub lyCardHeader: Layout,
    pub dotStatus: Circle,
    pub btnSettings: Button,
    pub lblTrChip: Label,
    pub lblStatus: Label,
    pub lyTimer: FlowLayout,
    pub lblTimer: Label,
    pub lblTimerMs: Label,
    pub lblSize: Label,
    pub lyRecord: FlowLayout,
    pub btnRecord: RecordButton,
    pub lblRecordAction: Label,
    pub lnCard: Rectangle,
    pub pnlReplay: Panel,
    pub dotReplay: Circle,
    pub btnPause: Button,
    pub btnCapture: Button,
    pub swReplay: Switch,
    pub btnReplaySave: Button,
    pub lyReplayText: Layout,
    pub lblReplay: Label,
    pub lblReplayStatus: Label,
    pub lnSources: Rectangle,
    pub pnlLegend: Panel,
    pub lnLegend: Rectangle,
    pub lyLegendTitle: Layout,
    pub lblLegendTitle: Label,
    pub lyLegend: Layout,
    pub scrSources: ScrollBox,
    pub lySources: Layout,
    pub lyMicsTitle: Layout,
    pub lblMicsCount: Label,
    pub lblMicsTitle: Label,
    pub lyMics: Layout,
    pub lySpeakersTitle: Layout,
    pub lblSpeakersCount: Label,
    pub lblSpeakersTitle: Label,
    pub lySpeakers: Layout,
    pub pnlRecordings: Panel,
    pub scrRecordings: ScrollBox,
    pub lyDisplays: Layout,
    pub lyDisplaysTitle: Layout,
    pub lblScreens: Label,
    pub lblScreensCount: Label,
    pub lblScreensMeta: Label,
    pub scrDisplays: ScreenLayout,
    pub lyLibrary: Layout,
    pub lyLibHeader: Layout,
    pub lblRecordings: Label,
    pub lblLibMeta: Label,
    pub lyLibTools: Layout,
    pub edLibSearch: Edit,
    pub btnLibOpenDir: Button,
    pub btnLibNewFolder: Button,
    pub btnLibMerge: Button,
    pub btnLibExport: Button,
    pub btnLibDelete: Button,
    pub brdLib: Breadcrumb,
    pub lblLibEmpty: Label,
    pub lyLibContent: Layout,
    pub edLibRename: Edit,
    pub cirRippleGlow: Circle,
    pub cirRipple: Circle,
    pub pmLibRecording: PopupMenu,
    pub miRecExport: MenuItem,
    pub miRecCut: MenuItem,
    pub miRecTranscribe: MenuItem,
    pub miRecSep: MenuItem,
    pub miRecDelete: MenuItem,
    pub pmLibFolder: PopupMenu,
    pub miFolderOpen: MenuItem,
    pub miFolderRename: MenuItem,
    pub miFolderCut: MenuItem,
    pub miFolderPaste: MenuItem,
    pub miFolderSep: MenuItem,
    pub miFolderDelete: MenuItem,
    pub pmLibEmpty: PopupMenu,
    pub miEmptyNewFolder: MenuItem,
    pub miEmptyPaste: MenuItem,
    pub miEmptySep: MenuItem,
    pub miEmptyUp: MenuItem,
}

impl std::ops::Deref for FrmMain {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FrmMain {
    /// Monta a instância de um frmMain recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            pnlSources: Panel { id: handles::resolve(ui, root, "pnlSources", "Panel", &mut problems) },
            lnSourcesEdge: Rectangle { id: handles::resolve(ui, root, "lnSourcesEdge", "Rectangle", &mut problems) },
            pnlUpdate: Panel { id: handles::resolve(ui, root, "pnlUpdate", "Panel", &mut problems) },
            btnUpdClose: Button { id: handles::resolve(ui, root, "btnUpdClose", "Button", &mut problems) },
            btnUpdOpen: Button { id: handles::resolve(ui, root, "btnUpdOpen", "Button", &mut problems) },
            lblUpdTitle: Label { id: handles::resolve(ui, root, "lblUpdTitle", "Label", &mut problems) },
            lblUpdBody: Label { id: handles::resolve(ui, root, "lblUpdBody", "Label", &mut problems) },
            pnlRecordCard: Panel { id: handles::resolve(ui, root, "pnlRecordCard", "Panel", &mut problems) },
            lyCardHeader: Layout { id: handles::resolve(ui, root, "lyCardHeader", "Layout", &mut problems) },
            dotStatus: Circle { id: handles::resolve(ui, root, "dotStatus", "Circle", &mut problems) },
            btnSettings: Button { id: handles::resolve(ui, root, "btnSettings", "Button", &mut problems) },
            lblTrChip: Label { id: handles::resolve(ui, root, "lblTrChip", "Label", &mut problems) },
            lblStatus: Label { id: handles::resolve(ui, root, "lblStatus", "Label", &mut problems) },
            lyTimer: FlowLayout { id: handles::resolve(ui, root, "lyTimer", "FlowLayout", &mut problems) },
            lblTimer: Label { id: handles::resolve(ui, root, "lblTimer", "Label", &mut problems) },
            lblTimerMs: Label { id: handles::resolve(ui, root, "lblTimerMs", "Label", &mut problems) },
            lblSize: Label { id: handles::resolve(ui, root, "lblSize", "Label", &mut problems) },
            lyRecord: FlowLayout { id: handles::resolve(ui, root, "lyRecord", "FlowLayout", &mut problems) },
            btnRecord: RecordButton { id: handles::resolve(ui, root, "btnRecord", "RecordButton", &mut problems) },
            lblRecordAction: Label { id: handles::resolve(ui, root, "lblRecordAction", "Label", &mut problems) },
            lnCard: Rectangle { id: handles::resolve(ui, root, "lnCard", "Rectangle", &mut problems) },
            pnlReplay: Panel { id: handles::resolve(ui, root, "pnlReplay", "Panel", &mut problems) },
            dotReplay: Circle { id: handles::resolve(ui, root, "dotReplay", "Circle", &mut problems) },
            btnPause: Button { id: handles::resolve(ui, root, "btnPause", "Button", &mut problems) },
            btnCapture: Button { id: handles::resolve(ui, root, "btnCapture", "Button", &mut problems) },
            swReplay: Switch { id: handles::resolve(ui, root, "swReplay", "Switch", &mut problems) },
            btnReplaySave: Button { id: handles::resolve(ui, root, "btnReplaySave", "Button", &mut problems) },
            lyReplayText: Layout { id: handles::resolve(ui, root, "lyReplayText", "Layout", &mut problems) },
            lblReplay: Label { id: handles::resolve(ui, root, "lblReplay", "Label", &mut problems) },
            lblReplayStatus: Label { id: handles::resolve(ui, root, "lblReplayStatus", "Label", &mut problems) },
            lnSources: Rectangle { id: handles::resolve(ui, root, "lnSources", "Rectangle", &mut problems) },
            pnlLegend: Panel { id: handles::resolve(ui, root, "pnlLegend", "Panel", &mut problems) },
            lnLegend: Rectangle { id: handles::resolve(ui, root, "lnLegend", "Rectangle", &mut problems) },
            lyLegendTitle: Layout { id: handles::resolve(ui, root, "lyLegendTitle", "Layout", &mut problems) },
            lblLegendTitle: Label { id: handles::resolve(ui, root, "lblLegendTitle", "Label", &mut problems) },
            lyLegend: Layout { id: handles::resolve(ui, root, "lyLegend", "Layout", &mut problems) },
            scrSources: ScrollBox { id: handles::resolve(ui, root, "scrSources", "ScrollBox", &mut problems) },
            lySources: Layout { id: handles::resolve(ui, root, "lySources", "Layout", &mut problems) },
            lyMicsTitle: Layout { id: handles::resolve(ui, root, "lyMicsTitle", "Layout", &mut problems) },
            lblMicsCount: Label { id: handles::resolve(ui, root, "lblMicsCount", "Label", &mut problems) },
            lblMicsTitle: Label { id: handles::resolve(ui, root, "lblMicsTitle", "Label", &mut problems) },
            lyMics: Layout { id: handles::resolve(ui, root, "lyMics", "Layout", &mut problems) },
            lySpeakersTitle: Layout { id: handles::resolve(ui, root, "lySpeakersTitle", "Layout", &mut problems) },
            lblSpeakersCount: Label { id: handles::resolve(ui, root, "lblSpeakersCount", "Label", &mut problems) },
            lblSpeakersTitle: Label { id: handles::resolve(ui, root, "lblSpeakersTitle", "Label", &mut problems) },
            lySpeakers: Layout { id: handles::resolve(ui, root, "lySpeakers", "Layout", &mut problems) },
            pnlRecordings: Panel { id: handles::resolve(ui, root, "pnlRecordings", "Panel", &mut problems) },
            scrRecordings: ScrollBox { id: handles::resolve(ui, root, "scrRecordings", "ScrollBox", &mut problems) },
            lyDisplays: Layout { id: handles::resolve(ui, root, "lyDisplays", "Layout", &mut problems) },
            lyDisplaysTitle: Layout { id: handles::resolve(ui, root, "lyDisplaysTitle", "Layout", &mut problems) },
            lblScreens: Label { id: handles::resolve(ui, root, "lblScreens", "Label", &mut problems) },
            lblScreensCount: Label { id: handles::resolve(ui, root, "lblScreensCount", "Label", &mut problems) },
            lblScreensMeta: Label { id: handles::resolve(ui, root, "lblScreensMeta", "Label", &mut problems) },
            scrDisplays: ScreenLayout { id: handles::resolve(ui, root, "scrDisplays", "ScreenLayout", &mut problems) },
            lyLibrary: Layout { id: handles::resolve(ui, root, "lyLibrary", "Layout", &mut problems) },
            lyLibHeader: Layout { id: handles::resolve(ui, root, "lyLibHeader", "Layout", &mut problems) },
            lblRecordings: Label { id: handles::resolve(ui, root, "lblRecordings", "Label", &mut problems) },
            lblLibMeta: Label { id: handles::resolve(ui, root, "lblLibMeta", "Label", &mut problems) },
            lyLibTools: Layout { id: handles::resolve(ui, root, "lyLibTools", "Layout", &mut problems) },
            edLibSearch: Edit { id: handles::resolve(ui, root, "edLibSearch", "Edit", &mut problems) },
            btnLibOpenDir: Button { id: handles::resolve(ui, root, "btnLibOpenDir", "Button", &mut problems) },
            btnLibNewFolder: Button { id: handles::resolve(ui, root, "btnLibNewFolder", "Button", &mut problems) },
            btnLibMerge: Button { id: handles::resolve(ui, root, "btnLibMerge", "Button", &mut problems) },
            btnLibExport: Button { id: handles::resolve(ui, root, "btnLibExport", "Button", &mut problems) },
            btnLibDelete: Button { id: handles::resolve(ui, root, "btnLibDelete", "Button", &mut problems) },
            brdLib: Breadcrumb { id: handles::resolve(ui, root, "brdLib", "Breadcrumb", &mut problems) },
            lblLibEmpty: Label { id: handles::resolve(ui, root, "lblLibEmpty", "Label", &mut problems) },
            lyLibContent: Layout { id: handles::resolve(ui, root, "lyLibContent", "Layout", &mut problems) },
            edLibRename: Edit { id: handles::resolve(ui, root, "edLibRename", "Edit", &mut problems) },
            cirRippleGlow: Circle { id: handles::resolve(ui, root, "cirRippleGlow", "Circle", &mut problems) },
            cirRipple: Circle { id: handles::resolve(ui, root, "cirRipple", "Circle", &mut problems) },
            pmLibRecording: PopupMenu { id: handles::resolve(ui, root, "pmLibRecording", "PopupMenu", &mut problems) },
            miRecExport: MenuItem { id: handles::resolve(ui, root, "miRecExport", "MenuItem", &mut problems) },
            miRecCut: MenuItem { id: handles::resolve(ui, root, "miRecCut", "MenuItem", &mut problems) },
            miRecTranscribe: MenuItem { id: handles::resolve(ui, root, "miRecTranscribe", "MenuItem", &mut problems) },
            miRecSep: MenuItem { id: handles::resolve(ui, root, "miRecSep", "MenuItem", &mut problems) },
            miRecDelete: MenuItem { id: handles::resolve(ui, root, "miRecDelete", "MenuItem", &mut problems) },
            pmLibFolder: PopupMenu { id: handles::resolve(ui, root, "pmLibFolder", "PopupMenu", &mut problems) },
            miFolderOpen: MenuItem { id: handles::resolve(ui, root, "miFolderOpen", "MenuItem", &mut problems) },
            miFolderRename: MenuItem { id: handles::resolve(ui, root, "miFolderRename", "MenuItem", &mut problems) },
            miFolderCut: MenuItem { id: handles::resolve(ui, root, "miFolderCut", "MenuItem", &mut problems) },
            miFolderPaste: MenuItem { id: handles::resolve(ui, root, "miFolderPaste", "MenuItem", &mut problems) },
            miFolderSep: MenuItem { id: handles::resolve(ui, root, "miFolderSep", "MenuItem", &mut problems) },
            miFolderDelete: MenuItem { id: handles::resolve(ui, root, "miFolderDelete", "MenuItem", &mut problems) },
            pmLibEmpty: PopupMenu { id: handles::resolve(ui, root, "pmLibEmpty", "PopupMenu", &mut problems) },
            miEmptyNewFolder: MenuItem { id: handles::resolve(ui, root, "miEmptyNewFolder", "MenuItem", &mut problems) },
            miEmptyPaste: MenuItem { id: handles::resolve(ui, root, "miEmptyPaste", "MenuItem", &mut problems) },
            miEmptySep: MenuItem { id: handles::resolve(ui, root, "miEmptySep", "MenuItem", &mut problems) },
            miEmptyUp: MenuItem { id: handles::resolve(ui, root, "miEmptyUp", "MenuItem", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form frmMain não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FrmMainComponents {
            form: instance.form,
            pnlSources: instance.pnlSources,
            lnSourcesEdge: instance.lnSourcesEdge,
            pnlUpdate: instance.pnlUpdate,
            btnUpdClose: instance.btnUpdClose,
            btnUpdOpen: instance.btnUpdOpen,
            lblUpdTitle: instance.lblUpdTitle,
            lblUpdBody: instance.lblUpdBody,
            pnlRecordCard: instance.pnlRecordCard,
            lyCardHeader: instance.lyCardHeader,
            dotStatus: instance.dotStatus,
            btnSettings: instance.btnSettings,
            lblTrChip: instance.lblTrChip,
            lblStatus: instance.lblStatus,
            lyTimer: instance.lyTimer,
            lblTimer: instance.lblTimer,
            lblTimerMs: instance.lblTimerMs,
            lblSize: instance.lblSize,
            lyRecord: instance.lyRecord,
            btnRecord: instance.btnRecord,
            lblRecordAction: instance.lblRecordAction,
            lnCard: instance.lnCard,
            pnlReplay: instance.pnlReplay,
            dotReplay: instance.dotReplay,
            btnPause: instance.btnPause,
            btnCapture: instance.btnCapture,
            swReplay: instance.swReplay,
            btnReplaySave: instance.btnReplaySave,
            lyReplayText: instance.lyReplayText,
            lblReplay: instance.lblReplay,
            lblReplayStatus: instance.lblReplayStatus,
            lnSources: instance.lnSources,
            pnlLegend: instance.pnlLegend,
            lnLegend: instance.lnLegend,
            lyLegendTitle: instance.lyLegendTitle,
            lblLegendTitle: instance.lblLegendTitle,
            lyLegend: instance.lyLegend,
            scrSources: instance.scrSources,
            lySources: instance.lySources,
            lyMicsTitle: instance.lyMicsTitle,
            lblMicsCount: instance.lblMicsCount,
            lblMicsTitle: instance.lblMicsTitle,
            lyMics: instance.lyMics,
            lySpeakersTitle: instance.lySpeakersTitle,
            lblSpeakersCount: instance.lblSpeakersCount,
            lblSpeakersTitle: instance.lblSpeakersTitle,
            lySpeakers: instance.lySpeakers,
            pnlRecordings: instance.pnlRecordings,
            scrRecordings: instance.scrRecordings,
            lyDisplays: instance.lyDisplays,
            lyDisplaysTitle: instance.lyDisplaysTitle,
            lblScreens: instance.lblScreens,
            lblScreensCount: instance.lblScreensCount,
            lblScreensMeta: instance.lblScreensMeta,
            scrDisplays: instance.scrDisplays,
            lyLibrary: instance.lyLibrary,
            lyLibHeader: instance.lyLibHeader,
            lblRecordings: instance.lblRecordings,
            lblLibMeta: instance.lblLibMeta,
            lyLibTools: instance.lyLibTools,
            edLibSearch: instance.edLibSearch,
            btnLibOpenDir: instance.btnLibOpenDir,
            btnLibNewFolder: instance.btnLibNewFolder,
            btnLibMerge: instance.btnLibMerge,
            btnLibExport: instance.btnLibExport,
            btnLibDelete: instance.btnLibDelete,
            brdLib: instance.brdLib,
            lblLibEmpty: instance.lblLibEmpty,
            lyLibContent: instance.lyLibContent,
            edLibRename: instance.edLibRename,
            cirRippleGlow: instance.cirRippleGlow,
            cirRipple: instance.cirRipple,
            pmLibRecording: instance.pmLibRecording,
            miRecExport: instance.miRecExport,
            miRecCut: instance.miRecCut,
            miRecTranscribe: instance.miRecTranscribe,
            miRecSep: instance.miRecSep,
            miRecDelete: instance.miRecDelete,
            pmLibFolder: instance.pmLibFolder,
            miFolderOpen: instance.miFolderOpen,
            miFolderRename: instance.miFolderRename,
            miFolderCut: instance.miFolderCut,
            miFolderPaste: instance.miFolderPaste,
            miFolderSep: instance.miFolderSep,
            miFolderDelete: instance.miFolderDelete,
            pmLibEmpty: instance.pmLibEmpty,
            miEmptyNewFolder: instance.miEmptyNewFolder,
            miEmptyPaste: instance.miEmptyPaste,
            miEmptySep: instance.miEmptySep,
            miEmptyUp: instance.miEmptyUp,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do frmMain aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FrmMainComponents> {
        ui.form_components(ui.find_path("frmMain")?)
    }

    /// Cria o frmMain (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, iron::prelude::Error> {
        ui.create_form("frmMain")
    }

    /// Cria mais um frmMain dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`frmMain`, `frmMain2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FrmMainComponents, iron::prelude::Error> {
        let root = ui.create_form_instance("frmMain")?;
        let name = ui.free_name_in(parent, "frmMain");
        if let Err(e) = ui.set_name(root, &name).and_then(|_| ui.set_parent(root, Some(parent))) {
            ui.free(root);
            return Err(e);
        }
        ui.form_components(root).ok_or(iron::prelude::Error::InvalidId)
    }

    /// Os componentes de uma instância do frmMain (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FrmMainComponents> {
        ui.form_components(root)
    }

    /// O frmMain aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FrmMain> {
        ui.form_instance(ui.find_path("frmMain")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FrmMain> {
        let form = ui.find_path("frmMain")?;
        ui.form_instance_mut(form)
    }

    /// O Breadcrumb deste form com o nome (`None` se não há um Breadcrumb com ele).
    pub fn find_breadcrumb(&self, name: &str) -> Option<Breadcrumb> {
        match name {
            "brdLib" => Some(self.brdLib),
            _ => None,
        }
    }

    /// O Breadcrumb que disparou o evento (`None` se o `sender` não é um Breadcrumb deste form).
    pub fn sender_as_breadcrumb(&self, ui: &Ui, sender: ComponentId) -> Option<Breadcrumb> {
        self.find_breadcrumb(ui.name(sender)?)
    }

    /// O Button deste form com o nome (`None` se não há um Button com ele).
    pub fn find_button(&self, name: &str) -> Option<Button> {
        match name {
            "btnUpdClose" => Some(self.btnUpdClose),
            "btnUpdOpen" => Some(self.btnUpdOpen),
            "btnSettings" => Some(self.btnSettings),
            "btnPause" => Some(self.btnPause),
            "btnCapture" => Some(self.btnCapture),
            "btnReplaySave" => Some(self.btnReplaySave),
            "btnLibOpenDir" => Some(self.btnLibOpenDir),
            "btnLibNewFolder" => Some(self.btnLibNewFolder),
            "btnLibMerge" => Some(self.btnLibMerge),
            "btnLibExport" => Some(self.btnLibExport),
            "btnLibDelete" => Some(self.btnLibDelete),
            _ => None,
        }
    }

    /// O Button que disparou o evento (`None` se o `sender` não é um Button deste form).
    pub fn sender_as_button(&self, ui: &Ui, sender: ComponentId) -> Option<Button> {
        self.find_button(ui.name(sender)?)
    }

    /// O Circle deste form com o nome (`None` se não há um Circle com ele).
    pub fn find_circle(&self, name: &str) -> Option<Circle> {
        match name {
            "dotStatus" => Some(self.dotStatus),
            "dotReplay" => Some(self.dotReplay),
            "cirRippleGlow" => Some(self.cirRippleGlow),
            "cirRipple" => Some(self.cirRipple),
            _ => None,
        }
    }

    /// O Circle que disparou o evento (`None` se o `sender` não é um Circle deste form).
    pub fn sender_as_circle(&self, ui: &Ui, sender: ComponentId) -> Option<Circle> {
        self.find_circle(ui.name(sender)?)
    }

    /// O Edit deste form com o nome (`None` se não há um Edit com ele).
    pub fn find_edit(&self, name: &str) -> Option<Edit> {
        match name {
            "edLibSearch" => Some(self.edLibSearch),
            "edLibRename" => Some(self.edLibRename),
            _ => None,
        }
    }

    /// O Edit que disparou o evento (`None` se o `sender` não é um Edit deste form).
    pub fn sender_as_edit(&self, ui: &Ui, sender: ComponentId) -> Option<Edit> {
        self.find_edit(ui.name(sender)?)
    }

    /// O FlowLayout deste form com o nome (`None` se não há um FlowLayout com ele).
    pub fn find_flow_layout(&self, name: &str) -> Option<FlowLayout> {
        match name {
            "lyTimer" => Some(self.lyTimer),
            "lyRecord" => Some(self.lyRecord),
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
            "lblUpdTitle" => Some(self.lblUpdTitle),
            "lblUpdBody" => Some(self.lblUpdBody),
            "lblTrChip" => Some(self.lblTrChip),
            "lblStatus" => Some(self.lblStatus),
            "lblTimer" => Some(self.lblTimer),
            "lblTimerMs" => Some(self.lblTimerMs),
            "lblSize" => Some(self.lblSize),
            "lblRecordAction" => Some(self.lblRecordAction),
            "lblReplay" => Some(self.lblReplay),
            "lblReplayStatus" => Some(self.lblReplayStatus),
            "lblLegendTitle" => Some(self.lblLegendTitle),
            "lblMicsCount" => Some(self.lblMicsCount),
            "lblMicsTitle" => Some(self.lblMicsTitle),
            "lblSpeakersCount" => Some(self.lblSpeakersCount),
            "lblSpeakersTitle" => Some(self.lblSpeakersTitle),
            "lblScreens" => Some(self.lblScreens),
            "lblScreensCount" => Some(self.lblScreensCount),
            "lblScreensMeta" => Some(self.lblScreensMeta),
            "lblRecordings" => Some(self.lblRecordings),
            "lblLibMeta" => Some(self.lblLibMeta),
            "lblLibEmpty" => Some(self.lblLibEmpty),
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
            "lyCardHeader" => Some(self.lyCardHeader),
            "lyReplayText" => Some(self.lyReplayText),
            "lyLegendTitle" => Some(self.lyLegendTitle),
            "lyLegend" => Some(self.lyLegend),
            "lySources" => Some(self.lySources),
            "lyMicsTitle" => Some(self.lyMicsTitle),
            "lyMics" => Some(self.lyMics),
            "lySpeakersTitle" => Some(self.lySpeakersTitle),
            "lySpeakers" => Some(self.lySpeakers),
            "lyDisplays" => Some(self.lyDisplays),
            "lyDisplaysTitle" => Some(self.lyDisplaysTitle),
            "lyLibrary" => Some(self.lyLibrary),
            "lyLibHeader" => Some(self.lyLibHeader),
            "lyLibTools" => Some(self.lyLibTools),
            "lyLibContent" => Some(self.lyLibContent),
            _ => None,
        }
    }

    /// O Layout que disparou o evento (`None` se o `sender` não é um Layout deste form).
    pub fn sender_as_layout(&self, ui: &Ui, sender: ComponentId) -> Option<Layout> {
        self.find_layout(ui.name(sender)?)
    }

    /// O MenuItem deste form com o nome (`None` se não há um MenuItem com ele).
    pub fn find_menu_item(&self, name: &str) -> Option<MenuItem> {
        match name {
            "miRecExport" => Some(self.miRecExport),
            "miRecCut" => Some(self.miRecCut),
            "miRecTranscribe" => Some(self.miRecTranscribe),
            "miRecSep" => Some(self.miRecSep),
            "miRecDelete" => Some(self.miRecDelete),
            "miFolderOpen" => Some(self.miFolderOpen),
            "miFolderRename" => Some(self.miFolderRename),
            "miFolderCut" => Some(self.miFolderCut),
            "miFolderPaste" => Some(self.miFolderPaste),
            "miFolderSep" => Some(self.miFolderSep),
            "miFolderDelete" => Some(self.miFolderDelete),
            "miEmptyNewFolder" => Some(self.miEmptyNewFolder),
            "miEmptyPaste" => Some(self.miEmptyPaste),
            "miEmptySep" => Some(self.miEmptySep),
            "miEmptyUp" => Some(self.miEmptyUp),
            _ => None,
        }
    }

    /// O MenuItem que disparou o evento (`None` se o `sender` não é um MenuItem deste form).
    pub fn sender_as_menu_item(&self, ui: &Ui, sender: ComponentId) -> Option<MenuItem> {
        self.find_menu_item(ui.name(sender)?)
    }

    /// O Panel deste form com o nome (`None` se não há um Panel com ele).
    pub fn find_panel(&self, name: &str) -> Option<Panel> {
        match name {
            "pnlSources" => Some(self.pnlSources),
            "pnlUpdate" => Some(self.pnlUpdate),
            "pnlRecordCard" => Some(self.pnlRecordCard),
            "pnlReplay" => Some(self.pnlReplay),
            "pnlLegend" => Some(self.pnlLegend),
            "pnlRecordings" => Some(self.pnlRecordings),
            _ => None,
        }
    }

    /// O Panel que disparou o evento (`None` se o `sender` não é um Panel deste form).
    pub fn sender_as_panel(&self, ui: &Ui, sender: ComponentId) -> Option<Panel> {
        self.find_panel(ui.name(sender)?)
    }

    /// O PopupMenu deste form com o nome (`None` se não há um PopupMenu com ele).
    pub fn find_popup_menu(&self, name: &str) -> Option<PopupMenu> {
        match name {
            "pmLibRecording" => Some(self.pmLibRecording),
            "pmLibFolder" => Some(self.pmLibFolder),
            "pmLibEmpty" => Some(self.pmLibEmpty),
            _ => None,
        }
    }

    /// O PopupMenu que disparou o evento (`None` se o `sender` não é um PopupMenu deste form).
    pub fn sender_as_popup_menu(&self, ui: &Ui, sender: ComponentId) -> Option<PopupMenu> {
        self.find_popup_menu(ui.name(sender)?)
    }

    /// O RecordButton deste form com o nome (`None` se não há um RecordButton com ele).
    pub fn find_record_button(&self, name: &str) -> Option<RecordButton> {
        match name {
            "btnRecord" => Some(self.btnRecord),
            _ => None,
        }
    }

    /// O RecordButton que disparou o evento (`None` se o `sender` não é um RecordButton deste form).
    pub fn sender_as_record_button(&self, ui: &Ui, sender: ComponentId) -> Option<RecordButton> {
        self.find_record_button(ui.name(sender)?)
    }

    /// O Rectangle deste form com o nome (`None` se não há um Rectangle com ele).
    pub fn find_rectangle(&self, name: &str) -> Option<Rectangle> {
        match name {
            "lnSourcesEdge" => Some(self.lnSourcesEdge),
            "lnCard" => Some(self.lnCard),
            "lnSources" => Some(self.lnSources),
            "lnLegend" => Some(self.lnLegend),
            _ => None,
        }
    }

    /// O Rectangle que disparou o evento (`None` se o `sender` não é um Rectangle deste form).
    pub fn sender_as_rectangle(&self, ui: &Ui, sender: ComponentId) -> Option<Rectangle> {
        self.find_rectangle(ui.name(sender)?)
    }

    /// O ScreenLayout deste form com o nome (`None` se não há um ScreenLayout com ele).
    pub fn find_screen_layout(&self, name: &str) -> Option<ScreenLayout> {
        match name {
            "scrDisplays" => Some(self.scrDisplays),
            _ => None,
        }
    }

    /// O ScreenLayout que disparou o evento (`None` se o `sender` não é um ScreenLayout deste form).
    pub fn sender_as_screen_layout(&self, ui: &Ui, sender: ComponentId) -> Option<ScreenLayout> {
        self.find_screen_layout(ui.name(sender)?)
    }

    /// O ScrollBox deste form com o nome (`None` se não há um ScrollBox com ele).
    pub fn find_scroll_box(&self, name: &str) -> Option<ScrollBox> {
        match name {
            "scrSources" => Some(self.scrSources),
            "scrRecordings" => Some(self.scrRecordings),
            _ => None,
        }
    }

    /// O ScrollBox que disparou o evento (`None` se o `sender` não é um ScrollBox deste form).
    pub fn sender_as_scroll_box(&self, ui: &Ui, sender: ComponentId) -> Option<ScrollBox> {
        self.find_scroll_box(ui.name(sender)?)
    }

    /// O Switch deste form com o nome (`None` se não há um Switch com ele).
    pub fn find_switch(&self, name: &str) -> Option<Switch> {
        match name {
            "swReplay" => Some(self.swReplay),
            _ => None,
        }
    }

    /// O Switch que disparou o evento (`None` se o `sender` não é um Switch deste form).
    pub fn sender_as_switch(&self, ui: &Ui, sender: ComponentId) -> Option<Switch> {
        self.find_switch(ui.name(sender)?)
    }
}
