
/// O form frmPlayer: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FrmPlayer {
    pub form: Form,
    pub pnlPlayer: Panel,
    pub pnlPlayerSide: Panel,
    pub recSideEdge: Rectangle,
    pub lySideBody: Layout,
    pub lyPlayerSideHead: Layout,
    pub recSideHeadLine: Rectangle,
    pub btnSideClose: Button,
    pub lblSideTitle: Label,
    pub lyTrSearch: Layout,
    pub recTrSearchLine: Rectangle,
    pub edTrSearch: Edit,
    pub scrPlayerSide: ScrollBox,
    pub lySide: Layout,
    pub lblSideLoading: Label,
    pub lblSideEmpty: Label,
    pub lyStage: Layout,
    pub frmVideo: FrameView,
    pub lblCaption: Label,
    pub lblPlayerZoom: Label,
    pub pnlPauseFlash: Panel,
    pub cirPauseFlash: Circle,
    pub recPauseFlash1: Rectangle,
    pub recPauseFlash2: Rectangle,
    pub pnlPlayerLoading: Panel,
    pub lyPlayerLoading: Layout,
    pub aniPlayerLoading: ActivityIndicator,
    pub lblPlayerLoading: Label,
    pub lyChrome: Layout,
    pub pnlPlayerHead: Panel,
    pub btnPlayerClose: Button,
    pub pnlPlayerPaused: Panel,
    pub recPaused1: Rectangle,
    pub recPaused2: Rectangle,
    pub lblPlayerPaused: Label,
    pub lblPlayerTitle: Label,
    pub pnlPlayerBar: Panel,
    pub btnPlay: Button,
    pub lblPlayerTime: Label,
    pub lyPlayerClock: Layout,
    pub btnClockIcon: Button,
    pub lblPlayerClock: Label,
    pub btnFullscreen: Button,
    pub btnInfo: Button,
    pub btnCaptions: Button,
    pub btnSpeed: Button,
    pub btnMute: Button,
    pub wavPlayer: Waveform,
    pub lyVolPop: Layout,
    pub pnlVolPop: Panel,
    pub lblVolPct: Label,
    pub trkVolume: TrackBar,
    pub lyCcPop: Layout,
    pub pnlCcPop: Panel,
    pub pnlCcCaptions: Panel,
    pub lblCcCheck1: Label,
    pub lblCcKey1: Label,
    pub lblCcCaptions: Label,
    pub pnlCcTranscript: Panel,
    pub lblCcCheck2: Label,
    pub lblCcTranscript: Label,
    pub pnlSpeedPop: Panel,
    pub lySpeedItems: Layout,
    pub btnSpeed025: Button,
    pub btnSpeed05: Button,
    pub btnSpeed1: Button,
    pub btnSpeed15: Button,
    pub btnSpeed2: Button,
    pub btnSpeed3: Button,
    pub btnSpeed8: Button,
    pub btnSpeed16: Button,
    pub btnSpeed32: Button,
    pub data: super::player::Data,
}

/// Os componentes do frmPlayer sem o estado, para o código fora dos handlers
/// (`FrmPlayer::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FrmPlayerComponents {
    pub form: Form,
    pub pnlPlayer: Panel,
    pub pnlPlayerSide: Panel,
    pub recSideEdge: Rectangle,
    pub lySideBody: Layout,
    pub lyPlayerSideHead: Layout,
    pub recSideHeadLine: Rectangle,
    pub btnSideClose: Button,
    pub lblSideTitle: Label,
    pub lyTrSearch: Layout,
    pub recTrSearchLine: Rectangle,
    pub edTrSearch: Edit,
    pub scrPlayerSide: ScrollBox,
    pub lySide: Layout,
    pub lblSideLoading: Label,
    pub lblSideEmpty: Label,
    pub lyStage: Layout,
    pub frmVideo: FrameView,
    pub lblCaption: Label,
    pub lblPlayerZoom: Label,
    pub pnlPauseFlash: Panel,
    pub cirPauseFlash: Circle,
    pub recPauseFlash1: Rectangle,
    pub recPauseFlash2: Rectangle,
    pub pnlPlayerLoading: Panel,
    pub lyPlayerLoading: Layout,
    pub aniPlayerLoading: ActivityIndicator,
    pub lblPlayerLoading: Label,
    pub lyChrome: Layout,
    pub pnlPlayerHead: Panel,
    pub btnPlayerClose: Button,
    pub pnlPlayerPaused: Panel,
    pub recPaused1: Rectangle,
    pub recPaused2: Rectangle,
    pub lblPlayerPaused: Label,
    pub lblPlayerTitle: Label,
    pub pnlPlayerBar: Panel,
    pub btnPlay: Button,
    pub lblPlayerTime: Label,
    pub lyPlayerClock: Layout,
    pub btnClockIcon: Button,
    pub lblPlayerClock: Label,
    pub btnFullscreen: Button,
    pub btnInfo: Button,
    pub btnCaptions: Button,
    pub btnSpeed: Button,
    pub btnMute: Button,
    pub wavPlayer: Waveform,
    pub lyVolPop: Layout,
    pub pnlVolPop: Panel,
    pub lblVolPct: Label,
    pub trkVolume: TrackBar,
    pub lyCcPop: Layout,
    pub pnlCcPop: Panel,
    pub pnlCcCaptions: Panel,
    pub lblCcCheck1: Label,
    pub lblCcKey1: Label,
    pub lblCcCaptions: Label,
    pub pnlCcTranscript: Panel,
    pub lblCcCheck2: Label,
    pub lblCcTranscript: Label,
    pub pnlSpeedPop: Panel,
    pub lySpeedItems: Layout,
    pub btnSpeed025: Button,
    pub btnSpeed05: Button,
    pub btnSpeed1: Button,
    pub btnSpeed15: Button,
    pub btnSpeed2: Button,
    pub btnSpeed3: Button,
    pub btnSpeed8: Button,
    pub btnSpeed16: Button,
    pub btnSpeed32: Button,
}

impl std::ops::Deref for FrmPlayer {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FrmPlayer {
    /// Monta a instância de um frmPlayer recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            pnlPlayer: Panel { id: handles::resolve(ui, root, "pnlPlayer", "Panel", &mut problems) },
            pnlPlayerSide: Panel { id: handles::resolve(ui, root, "pnlPlayerSide", "Panel", &mut problems) },
            recSideEdge: Rectangle { id: handles::resolve(ui, root, "recSideEdge", "Rectangle", &mut problems) },
            lySideBody: Layout { id: handles::resolve(ui, root, "lySideBody", "Layout", &mut problems) },
            lyPlayerSideHead: Layout { id: handles::resolve(ui, root, "lyPlayerSideHead", "Layout", &mut problems) },
            recSideHeadLine: Rectangle { id: handles::resolve(ui, root, "recSideHeadLine", "Rectangle", &mut problems) },
            btnSideClose: Button { id: handles::resolve(ui, root, "btnSideClose", "Button", &mut problems) },
            lblSideTitle: Label { id: handles::resolve(ui, root, "lblSideTitle", "Label", &mut problems) },
            lyTrSearch: Layout { id: handles::resolve(ui, root, "lyTrSearch", "Layout", &mut problems) },
            recTrSearchLine: Rectangle { id: handles::resolve(ui, root, "recTrSearchLine", "Rectangle", &mut problems) },
            edTrSearch: Edit { id: handles::resolve(ui, root, "edTrSearch", "Edit", &mut problems) },
            scrPlayerSide: ScrollBox { id: handles::resolve(ui, root, "scrPlayerSide", "ScrollBox", &mut problems) },
            lySide: Layout { id: handles::resolve(ui, root, "lySide", "Layout", &mut problems) },
            lblSideLoading: Label { id: handles::resolve(ui, root, "lblSideLoading", "Label", &mut problems) },
            lblSideEmpty: Label { id: handles::resolve(ui, root, "lblSideEmpty", "Label", &mut problems) },
            lyStage: Layout { id: handles::resolve(ui, root, "lyStage", "Layout", &mut problems) },
            frmVideo: FrameView { id: handles::resolve(ui, root, "frmVideo", "FrameView", &mut problems) },
            lblCaption: Label { id: handles::resolve(ui, root, "lblCaption", "Label", &mut problems) },
            lblPlayerZoom: Label { id: handles::resolve(ui, root, "lblPlayerZoom", "Label", &mut problems) },
            pnlPauseFlash: Panel { id: handles::resolve(ui, root, "pnlPauseFlash", "Panel", &mut problems) },
            cirPauseFlash: Circle { id: handles::resolve(ui, root, "cirPauseFlash", "Circle", &mut problems) },
            recPauseFlash1: Rectangle { id: handles::resolve(ui, root, "recPauseFlash1", "Rectangle", &mut problems) },
            recPauseFlash2: Rectangle { id: handles::resolve(ui, root, "recPauseFlash2", "Rectangle", &mut problems) },
            pnlPlayerLoading: Panel { id: handles::resolve(ui, root, "pnlPlayerLoading", "Panel", &mut problems) },
            lyPlayerLoading: Layout { id: handles::resolve(ui, root, "lyPlayerLoading", "Layout", &mut problems) },
            aniPlayerLoading: ActivityIndicator { id: handles::resolve(ui, root, "aniPlayerLoading", "ActivityIndicator", &mut problems) },
            lblPlayerLoading: Label { id: handles::resolve(ui, root, "lblPlayerLoading", "Label", &mut problems) },
            lyChrome: Layout { id: handles::resolve(ui, root, "lyChrome", "Layout", &mut problems) },
            pnlPlayerHead: Panel { id: handles::resolve(ui, root, "pnlPlayerHead", "Panel", &mut problems) },
            btnPlayerClose: Button { id: handles::resolve(ui, root, "btnPlayerClose", "Button", &mut problems) },
            pnlPlayerPaused: Panel { id: handles::resolve(ui, root, "pnlPlayerPaused", "Panel", &mut problems) },
            recPaused1: Rectangle { id: handles::resolve(ui, root, "recPaused1", "Rectangle", &mut problems) },
            recPaused2: Rectangle { id: handles::resolve(ui, root, "recPaused2", "Rectangle", &mut problems) },
            lblPlayerPaused: Label { id: handles::resolve(ui, root, "lblPlayerPaused", "Label", &mut problems) },
            lblPlayerTitle: Label { id: handles::resolve(ui, root, "lblPlayerTitle", "Label", &mut problems) },
            pnlPlayerBar: Panel { id: handles::resolve(ui, root, "pnlPlayerBar", "Panel", &mut problems) },
            btnPlay: Button { id: handles::resolve(ui, root, "btnPlay", "Button", &mut problems) },
            lblPlayerTime: Label { id: handles::resolve(ui, root, "lblPlayerTime", "Label", &mut problems) },
            lyPlayerClock: Layout { id: handles::resolve(ui, root, "lyPlayerClock", "Layout", &mut problems) },
            btnClockIcon: Button { id: handles::resolve(ui, root, "btnClockIcon", "Button", &mut problems) },
            lblPlayerClock: Label { id: handles::resolve(ui, root, "lblPlayerClock", "Label", &mut problems) },
            btnFullscreen: Button { id: handles::resolve(ui, root, "btnFullscreen", "Button", &mut problems) },
            btnInfo: Button { id: handles::resolve(ui, root, "btnInfo", "Button", &mut problems) },
            btnCaptions: Button { id: handles::resolve(ui, root, "btnCaptions", "Button", &mut problems) },
            btnSpeed: Button { id: handles::resolve(ui, root, "btnSpeed", "Button", &mut problems) },
            btnMute: Button { id: handles::resolve(ui, root, "btnMute", "Button", &mut problems) },
            wavPlayer: Waveform { id: handles::resolve(ui, root, "wavPlayer", "Waveform", &mut problems) },
            lyVolPop: Layout { id: handles::resolve(ui, root, "lyVolPop", "Layout", &mut problems) },
            pnlVolPop: Panel { id: handles::resolve(ui, root, "pnlVolPop", "Panel", &mut problems) },
            lblVolPct: Label { id: handles::resolve(ui, root, "lblVolPct", "Label", &mut problems) },
            trkVolume: TrackBar { id: handles::resolve(ui, root, "trkVolume", "TrackBar", &mut problems) },
            lyCcPop: Layout { id: handles::resolve(ui, root, "lyCcPop", "Layout", &mut problems) },
            pnlCcPop: Panel { id: handles::resolve(ui, root, "pnlCcPop", "Panel", &mut problems) },
            pnlCcCaptions: Panel { id: handles::resolve(ui, root, "pnlCcCaptions", "Panel", &mut problems) },
            lblCcCheck1: Label { id: handles::resolve(ui, root, "lblCcCheck1", "Label", &mut problems) },
            lblCcKey1: Label { id: handles::resolve(ui, root, "lblCcKey1", "Label", &mut problems) },
            lblCcCaptions: Label { id: handles::resolve(ui, root, "lblCcCaptions", "Label", &mut problems) },
            pnlCcTranscript: Panel { id: handles::resolve(ui, root, "pnlCcTranscript", "Panel", &mut problems) },
            lblCcCheck2: Label { id: handles::resolve(ui, root, "lblCcCheck2", "Label", &mut problems) },
            lblCcTranscript: Label { id: handles::resolve(ui, root, "lblCcTranscript", "Label", &mut problems) },
            pnlSpeedPop: Panel { id: handles::resolve(ui, root, "pnlSpeedPop", "Panel", &mut problems) },
            lySpeedItems: Layout { id: handles::resolve(ui, root, "lySpeedItems", "Layout", &mut problems) },
            btnSpeed025: Button { id: handles::resolve(ui, root, "btnSpeed025", "Button", &mut problems) },
            btnSpeed05: Button { id: handles::resolve(ui, root, "btnSpeed05", "Button", &mut problems) },
            btnSpeed1: Button { id: handles::resolve(ui, root, "btnSpeed1", "Button", &mut problems) },
            btnSpeed15: Button { id: handles::resolve(ui, root, "btnSpeed15", "Button", &mut problems) },
            btnSpeed2: Button { id: handles::resolve(ui, root, "btnSpeed2", "Button", &mut problems) },
            btnSpeed3: Button { id: handles::resolve(ui, root, "btnSpeed3", "Button", &mut problems) },
            btnSpeed8: Button { id: handles::resolve(ui, root, "btnSpeed8", "Button", &mut problems) },
            btnSpeed16: Button { id: handles::resolve(ui, root, "btnSpeed16", "Button", &mut problems) },
            btnSpeed32: Button { id: handles::resolve(ui, root, "btnSpeed32", "Button", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form frmPlayer não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FrmPlayerComponents {
            form: instance.form,
            pnlPlayer: instance.pnlPlayer,
            pnlPlayerSide: instance.pnlPlayerSide,
            recSideEdge: instance.recSideEdge,
            lySideBody: instance.lySideBody,
            lyPlayerSideHead: instance.lyPlayerSideHead,
            recSideHeadLine: instance.recSideHeadLine,
            btnSideClose: instance.btnSideClose,
            lblSideTitle: instance.lblSideTitle,
            lyTrSearch: instance.lyTrSearch,
            recTrSearchLine: instance.recTrSearchLine,
            edTrSearch: instance.edTrSearch,
            scrPlayerSide: instance.scrPlayerSide,
            lySide: instance.lySide,
            lblSideLoading: instance.lblSideLoading,
            lblSideEmpty: instance.lblSideEmpty,
            lyStage: instance.lyStage,
            frmVideo: instance.frmVideo,
            lblCaption: instance.lblCaption,
            lblPlayerZoom: instance.lblPlayerZoom,
            pnlPauseFlash: instance.pnlPauseFlash,
            cirPauseFlash: instance.cirPauseFlash,
            recPauseFlash1: instance.recPauseFlash1,
            recPauseFlash2: instance.recPauseFlash2,
            pnlPlayerLoading: instance.pnlPlayerLoading,
            lyPlayerLoading: instance.lyPlayerLoading,
            aniPlayerLoading: instance.aniPlayerLoading,
            lblPlayerLoading: instance.lblPlayerLoading,
            lyChrome: instance.lyChrome,
            pnlPlayerHead: instance.pnlPlayerHead,
            btnPlayerClose: instance.btnPlayerClose,
            pnlPlayerPaused: instance.pnlPlayerPaused,
            recPaused1: instance.recPaused1,
            recPaused2: instance.recPaused2,
            lblPlayerPaused: instance.lblPlayerPaused,
            lblPlayerTitle: instance.lblPlayerTitle,
            pnlPlayerBar: instance.pnlPlayerBar,
            btnPlay: instance.btnPlay,
            lblPlayerTime: instance.lblPlayerTime,
            lyPlayerClock: instance.lyPlayerClock,
            btnClockIcon: instance.btnClockIcon,
            lblPlayerClock: instance.lblPlayerClock,
            btnFullscreen: instance.btnFullscreen,
            btnInfo: instance.btnInfo,
            btnCaptions: instance.btnCaptions,
            btnSpeed: instance.btnSpeed,
            btnMute: instance.btnMute,
            wavPlayer: instance.wavPlayer,
            lyVolPop: instance.lyVolPop,
            pnlVolPop: instance.pnlVolPop,
            lblVolPct: instance.lblVolPct,
            trkVolume: instance.trkVolume,
            lyCcPop: instance.lyCcPop,
            pnlCcPop: instance.pnlCcPop,
            pnlCcCaptions: instance.pnlCcCaptions,
            lblCcCheck1: instance.lblCcCheck1,
            lblCcKey1: instance.lblCcKey1,
            lblCcCaptions: instance.lblCcCaptions,
            pnlCcTranscript: instance.pnlCcTranscript,
            lblCcCheck2: instance.lblCcCheck2,
            lblCcTranscript: instance.lblCcTranscript,
            pnlSpeedPop: instance.pnlSpeedPop,
            lySpeedItems: instance.lySpeedItems,
            btnSpeed025: instance.btnSpeed025,
            btnSpeed05: instance.btnSpeed05,
            btnSpeed1: instance.btnSpeed1,
            btnSpeed15: instance.btnSpeed15,
            btnSpeed2: instance.btnSpeed2,
            btnSpeed3: instance.btnSpeed3,
            btnSpeed8: instance.btnSpeed8,
            btnSpeed16: instance.btnSpeed16,
            btnSpeed32: instance.btnSpeed32,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do frmPlayer aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FrmPlayerComponents> {
        ui.form_components(ui.find_path("frmPlayer")?)
    }

    /// Cria o frmPlayer (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, aylis::prelude::Error> {
        ui.create_form("frmPlayer")
    }

    /// Cria mais um frmPlayer dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`frmPlayer`, `frmPlayer2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FrmPlayerComponents, aylis::prelude::Error> {
        let root = ui.create_form_instance("frmPlayer")?;
        let name = ui.free_name_in(parent, "frmPlayer");
        if let Err(e) = ui.set_name(root, &name).and_then(|_| ui.set_parent(root, Some(parent))) {
            ui.free(root);
            return Err(e);
        }
        ui.form_components(root).ok_or(aylis::prelude::Error::InvalidId)
    }

    /// Os componentes de uma instância do frmPlayer (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FrmPlayerComponents> {
        ui.form_components(root)
    }

    /// O frmPlayer aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FrmPlayer> {
        ui.form_instance(ui.find_path("frmPlayer")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FrmPlayer> {
        let form = ui.find_path("frmPlayer")?;
        ui.form_instance_mut(form)
    }

    /// O ActivityIndicator deste form com o nome (`None` se não há um ActivityIndicator com ele).
    pub fn find_activity_indicator(&self, name: &str) -> Option<ActivityIndicator> {
        match name {
            "aniPlayerLoading" => Some(self.aniPlayerLoading),
            _ => None,
        }
    }

    /// O ActivityIndicator que disparou o evento (`None` se o `sender` não é um ActivityIndicator deste form).
    pub fn sender_as_activity_indicator(&self, ui: &Ui, sender: ComponentId) -> Option<ActivityIndicator> {
        self.find_activity_indicator(ui.name(sender)?)
    }

    /// O Button deste form com o nome (`None` se não há um Button com ele).
    pub fn find_button(&self, name: &str) -> Option<Button> {
        match name {
            "btnSideClose" => Some(self.btnSideClose),
            "btnPlayerClose" => Some(self.btnPlayerClose),
            "btnPlay" => Some(self.btnPlay),
            "btnClockIcon" => Some(self.btnClockIcon),
            "btnFullscreen" => Some(self.btnFullscreen),
            "btnInfo" => Some(self.btnInfo),
            "btnCaptions" => Some(self.btnCaptions),
            "btnSpeed" => Some(self.btnSpeed),
            "btnMute" => Some(self.btnMute),
            "btnSpeed025" => Some(self.btnSpeed025),
            "btnSpeed05" => Some(self.btnSpeed05),
            "btnSpeed1" => Some(self.btnSpeed1),
            "btnSpeed15" => Some(self.btnSpeed15),
            "btnSpeed2" => Some(self.btnSpeed2),
            "btnSpeed3" => Some(self.btnSpeed3),
            "btnSpeed8" => Some(self.btnSpeed8),
            "btnSpeed16" => Some(self.btnSpeed16),
            "btnSpeed32" => Some(self.btnSpeed32),
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
            "cirPauseFlash" => Some(self.cirPauseFlash),
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
            "edTrSearch" => Some(self.edTrSearch),
            _ => None,
        }
    }

    /// O Edit que disparou o evento (`None` se o `sender` não é um Edit deste form).
    pub fn sender_as_edit(&self, ui: &Ui, sender: ComponentId) -> Option<Edit> {
        self.find_edit(ui.name(sender)?)
    }

    /// O FrameView deste form com o nome (`None` se não há um FrameView com ele).
    pub fn find_frame_view(&self, name: &str) -> Option<FrameView> {
        match name {
            "frmVideo" => Some(self.frmVideo),
            _ => None,
        }
    }

    /// O FrameView que disparou o evento (`None` se o `sender` não é um FrameView deste form).
    pub fn sender_as_frame_view(&self, ui: &Ui, sender: ComponentId) -> Option<FrameView> {
        self.find_frame_view(ui.name(sender)?)
    }

    /// O Label deste form com o nome (`None` se não há um Label com ele).
    pub fn find_label(&self, name: &str) -> Option<Label> {
        match name {
            "lblSideTitle" => Some(self.lblSideTitle),
            "lblSideLoading" => Some(self.lblSideLoading),
            "lblSideEmpty" => Some(self.lblSideEmpty),
            "lblCaption" => Some(self.lblCaption),
            "lblPlayerZoom" => Some(self.lblPlayerZoom),
            "lblPlayerLoading" => Some(self.lblPlayerLoading),
            "lblPlayerPaused" => Some(self.lblPlayerPaused),
            "lblPlayerTitle" => Some(self.lblPlayerTitle),
            "lblPlayerTime" => Some(self.lblPlayerTime),
            "lblPlayerClock" => Some(self.lblPlayerClock),
            "lblVolPct" => Some(self.lblVolPct),
            "lblCcCheck1" => Some(self.lblCcCheck1),
            "lblCcKey1" => Some(self.lblCcKey1),
            "lblCcCaptions" => Some(self.lblCcCaptions),
            "lblCcCheck2" => Some(self.lblCcCheck2),
            "lblCcTranscript" => Some(self.lblCcTranscript),
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
            "lySideBody" => Some(self.lySideBody),
            "lyPlayerSideHead" => Some(self.lyPlayerSideHead),
            "lyTrSearch" => Some(self.lyTrSearch),
            "lySide" => Some(self.lySide),
            "lyStage" => Some(self.lyStage),
            "lyPlayerLoading" => Some(self.lyPlayerLoading),
            "lyChrome" => Some(self.lyChrome),
            "lyPlayerClock" => Some(self.lyPlayerClock),
            "lyVolPop" => Some(self.lyVolPop),
            "lyCcPop" => Some(self.lyCcPop),
            "lySpeedItems" => Some(self.lySpeedItems),
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
            "pnlPlayer" => Some(self.pnlPlayer),
            "pnlPlayerSide" => Some(self.pnlPlayerSide),
            "pnlPauseFlash" => Some(self.pnlPauseFlash),
            "pnlPlayerLoading" => Some(self.pnlPlayerLoading),
            "pnlPlayerHead" => Some(self.pnlPlayerHead),
            "pnlPlayerPaused" => Some(self.pnlPlayerPaused),
            "pnlPlayerBar" => Some(self.pnlPlayerBar),
            "pnlVolPop" => Some(self.pnlVolPop),
            "pnlCcPop" => Some(self.pnlCcPop),
            "pnlCcCaptions" => Some(self.pnlCcCaptions),
            "pnlCcTranscript" => Some(self.pnlCcTranscript),
            "pnlSpeedPop" => Some(self.pnlSpeedPop),
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
            "recSideEdge" => Some(self.recSideEdge),
            "recSideHeadLine" => Some(self.recSideHeadLine),
            "recTrSearchLine" => Some(self.recTrSearchLine),
            "recPauseFlash1" => Some(self.recPauseFlash1),
            "recPauseFlash2" => Some(self.recPauseFlash2),
            "recPaused1" => Some(self.recPaused1),
            "recPaused2" => Some(self.recPaused2),
            _ => None,
        }
    }

    /// O Rectangle que disparou o evento (`None` se o `sender` não é um Rectangle deste form).
    pub fn sender_as_rectangle(&self, ui: &Ui, sender: ComponentId) -> Option<Rectangle> {
        self.find_rectangle(ui.name(sender)?)
    }

    /// O ScrollBox deste form com o nome (`None` se não há um ScrollBox com ele).
    pub fn find_scroll_box(&self, name: &str) -> Option<ScrollBox> {
        match name {
            "scrPlayerSide" => Some(self.scrPlayerSide),
            _ => None,
        }
    }

    /// O ScrollBox que disparou o evento (`None` se o `sender` não é um ScrollBox deste form).
    pub fn sender_as_scroll_box(&self, ui: &Ui, sender: ComponentId) -> Option<ScrollBox> {
        self.find_scroll_box(ui.name(sender)?)
    }

    /// O TrackBar deste form com o nome (`None` se não há um TrackBar com ele).
    pub fn find_track_bar(&self, name: &str) -> Option<TrackBar> {
        match name {
            "trkVolume" => Some(self.trkVolume),
            _ => None,
        }
    }

    /// O TrackBar que disparou o evento (`None` se o `sender` não é um TrackBar deste form).
    pub fn sender_as_track_bar(&self, ui: &Ui, sender: ComponentId) -> Option<TrackBar> {
        self.find_track_bar(ui.name(sender)?)
    }

    /// O Waveform deste form com o nome (`None` se não há um Waveform com ele).
    pub fn find_waveform(&self, name: &str) -> Option<Waveform> {
        match name {
            "wavPlayer" => Some(self.wavPlayer),
            _ => None,
        }
    }

    /// O Waveform que disparou o evento (`None` se o `sender` não é um Waveform deste form).
    pub fn sender_as_waveform(&self, ui: &Ui, sender: ComponentId) -> Option<Waveform> {
        self.find_waveform(ui.name(sender)?)
    }
}
