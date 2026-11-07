
/// O form fraVideo: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FraVideo {
    pub form: Form,
    pub lblCodec: Label,
    pub lyCodec: Layout,
    pub imgGpu: Image,
    pub cmbCodec: ComboBox,
    pub lblCodecMaxRes: Label,
    pub lylblCodecSwWarn: Layout,
    pub barlblCodecSwWarn: Rectangle,
    pub lblCodecSwWarn: Label,
    pub lblCodecHint: Label,
    pub lnRule1: Rectangle,
    pub lblQuality: Label,
    pub lyQuality: Layout,
    pub lblQualityLow: Label,
    pub lblQualityHigh: Label,
    pub lyQualityTicks: Layout,
    pub trkQuality: TrackBar,
    pub lblQualityTick0: Label,
    pub lblQualityTick1: Label,
    pub lblQualityTick2: Label,
    pub lblQualityTick3: Label,
    pub lblQualityTick4: Label,
    pub lblQualityTick5: Label,
    pub lblQualityTick6: Label,
    pub lblQualityTick7: Label,
    pub lblQualityTick8: Label,
    pub lblQualityTick9: Label,
    pub lblQualityTick10: Label,
    pub lblQualityHint: Label,
    pub lylblQualitySvtFloor: Layout,
    pub barlblQualitySvtFloor: Rectangle,
    pub lblQualitySvtFloor: Label,
    pub lnRule2: Rectangle,
    pub lblVideoTest: Label,
    pub lblVideoTestHint: Label,
    pub lyVideoTest: Layout,
    pub lyVtButtons: Layout,
    pub btnVtRun: Button,
    pub btnVtCancel: Button,
    pub lblVtStatus: Label,
    pub prgVt: ProgressBar,
    pub lyVtResult: Layout,
    pub lblVtSample: Label,
    pub lyVtLevels: Layout,
    pub lblVtPickHint: Label,
    pub lyVtTools: Layout,
    pub btnVtSwap: Button,
    pub btnVtZoom300: Button,
    pub btnVtZoom200: Button,
    pub btnVtZoom100: Button,
    pub btnVtZoomFit: Button,
    pub pnlVtStage: Panel,
    pub imgVtA: Image,
    pub lyVtB: Layout,
    pub imgVtB: Image,
    pub shpVtSplit: Rectangle,
    pub cirVtKnob: Circle,
    pub lblVtKnob: Label,
    pub pnlVtTagA: Panel,
    pub lblVtTagA: Label,
    pub shpVtTagA: Rectangle,
    pub pnlVtTagB: Panel,
    pub lblVtTagB: Label,
    pub shpVtTagB: Rectangle,
    pub lblVtDragHint: Label,
    pub lyVtUse: Layout,
    pub btnVtUse: Button,
    pub lnRule3: Rectangle,
    pub lblFps: Label,
    pub lyFps: Layout,
    pub lblFpsLow: Label,
    pub lblFpsHigh: Label,
    pub lyFpsTicks: Layout,
    pub trkFps: TrackBar,
    pub lblFpsHint: Label,
    pub lnRule4: Rectangle,
    pub lblKeyframe: Label,
    pub lyKeyframe: Layout,
    pub lblKeyframeLow: Label,
    pub lblKeyframeHigh: Label,
    pub lyKeyframeTicks: Layout,
    pub trkKeyframe: TrackBar,
    pub lblKeyframeTick1: Label,
    pub lblKeyframeTick2: Label,
    pub lblKeyframeTick3: Label,
    pub lblKeyframeTick4: Label,
    pub lblKeyframeTick5: Label,
    pub lblKeyframeTick6: Label,
    pub lblKeyframeTick7: Label,
    pub lblKeyframeTick8: Label,
    pub lblKeyframeTick9: Label,
    pub lblKeyframeTick10: Label,
    pub lblKeyframeHint: Label,
    pub data: super::video::Data,
}

/// Os componentes do fraVideo sem o estado, para o código fora dos handlers
/// (`FraVideo::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FraVideoComponents {
    pub form: Form,
    pub lblCodec: Label,
    pub lyCodec: Layout,
    pub imgGpu: Image,
    pub cmbCodec: ComboBox,
    pub lblCodecMaxRes: Label,
    pub lylblCodecSwWarn: Layout,
    pub barlblCodecSwWarn: Rectangle,
    pub lblCodecSwWarn: Label,
    pub lblCodecHint: Label,
    pub lnRule1: Rectangle,
    pub lblQuality: Label,
    pub lyQuality: Layout,
    pub lblQualityLow: Label,
    pub lblQualityHigh: Label,
    pub lyQualityTicks: Layout,
    pub trkQuality: TrackBar,
    pub lblQualityTick0: Label,
    pub lblQualityTick1: Label,
    pub lblQualityTick2: Label,
    pub lblQualityTick3: Label,
    pub lblQualityTick4: Label,
    pub lblQualityTick5: Label,
    pub lblQualityTick6: Label,
    pub lblQualityTick7: Label,
    pub lblQualityTick8: Label,
    pub lblQualityTick9: Label,
    pub lblQualityTick10: Label,
    pub lblQualityHint: Label,
    pub lylblQualitySvtFloor: Layout,
    pub barlblQualitySvtFloor: Rectangle,
    pub lblQualitySvtFloor: Label,
    pub lnRule2: Rectangle,
    pub lblVideoTest: Label,
    pub lblVideoTestHint: Label,
    pub lyVideoTest: Layout,
    pub lyVtButtons: Layout,
    pub btnVtRun: Button,
    pub btnVtCancel: Button,
    pub lblVtStatus: Label,
    pub prgVt: ProgressBar,
    pub lyVtResult: Layout,
    pub lblVtSample: Label,
    pub lyVtLevels: Layout,
    pub lblVtPickHint: Label,
    pub lyVtTools: Layout,
    pub btnVtSwap: Button,
    pub btnVtZoom300: Button,
    pub btnVtZoom200: Button,
    pub btnVtZoom100: Button,
    pub btnVtZoomFit: Button,
    pub pnlVtStage: Panel,
    pub imgVtA: Image,
    pub lyVtB: Layout,
    pub imgVtB: Image,
    pub shpVtSplit: Rectangle,
    pub cirVtKnob: Circle,
    pub lblVtKnob: Label,
    pub pnlVtTagA: Panel,
    pub lblVtTagA: Label,
    pub shpVtTagA: Rectangle,
    pub pnlVtTagB: Panel,
    pub lblVtTagB: Label,
    pub shpVtTagB: Rectangle,
    pub lblVtDragHint: Label,
    pub lyVtUse: Layout,
    pub btnVtUse: Button,
    pub lnRule3: Rectangle,
    pub lblFps: Label,
    pub lyFps: Layout,
    pub lblFpsLow: Label,
    pub lblFpsHigh: Label,
    pub lyFpsTicks: Layout,
    pub trkFps: TrackBar,
    pub lblFpsHint: Label,
    pub lnRule4: Rectangle,
    pub lblKeyframe: Label,
    pub lyKeyframe: Layout,
    pub lblKeyframeLow: Label,
    pub lblKeyframeHigh: Label,
    pub lyKeyframeTicks: Layout,
    pub trkKeyframe: TrackBar,
    pub lblKeyframeTick1: Label,
    pub lblKeyframeTick2: Label,
    pub lblKeyframeTick3: Label,
    pub lblKeyframeTick4: Label,
    pub lblKeyframeTick5: Label,
    pub lblKeyframeTick6: Label,
    pub lblKeyframeTick7: Label,
    pub lblKeyframeTick8: Label,
    pub lblKeyframeTick9: Label,
    pub lblKeyframeTick10: Label,
    pub lblKeyframeHint: Label,
}

impl std::ops::Deref for FraVideo {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FraVideo {
    /// Monta a instância de um fraVideo recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            lblCodec: Label { id: handles::resolve(ui, root, "lblCodec", "Label", &mut problems) },
            lyCodec: Layout { id: handles::resolve(ui, root, "lyCodec", "Layout", &mut problems) },
            imgGpu: Image { id: handles::resolve(ui, root, "imgGpu", "Image", &mut problems) },
            cmbCodec: ComboBox { id: handles::resolve(ui, root, "cmbCodec", "ComboBox", &mut problems) },
            lblCodecMaxRes: Label { id: handles::resolve(ui, root, "lblCodecMaxRes", "Label", &mut problems) },
            lylblCodecSwWarn: Layout { id: handles::resolve(ui, root, "lylblCodecSwWarn", "Layout", &mut problems) },
            barlblCodecSwWarn: Rectangle { id: handles::resolve(ui, root, "barlblCodecSwWarn", "Rectangle", &mut problems) },
            lblCodecSwWarn: Label { id: handles::resolve(ui, root, "lblCodecSwWarn", "Label", &mut problems) },
            lblCodecHint: Label { id: handles::resolve(ui, root, "lblCodecHint", "Label", &mut problems) },
            lnRule1: Rectangle { id: handles::resolve(ui, root, "lnRule1", "Rectangle", &mut problems) },
            lblQuality: Label { id: handles::resolve(ui, root, "lblQuality", "Label", &mut problems) },
            lyQuality: Layout { id: handles::resolve(ui, root, "lyQuality", "Layout", &mut problems) },
            lblQualityLow: Label { id: handles::resolve(ui, root, "lblQualityLow", "Label", &mut problems) },
            lblQualityHigh: Label { id: handles::resolve(ui, root, "lblQualityHigh", "Label", &mut problems) },
            lyQualityTicks: Layout { id: handles::resolve(ui, root, "lyQualityTicks", "Layout", &mut problems) },
            trkQuality: TrackBar { id: handles::resolve(ui, root, "trkQuality", "TrackBar", &mut problems) },
            lblQualityTick0: Label { id: handles::resolve(ui, root, "lblQualityTick0", "Label", &mut problems) },
            lblQualityTick1: Label { id: handles::resolve(ui, root, "lblQualityTick1", "Label", &mut problems) },
            lblQualityTick2: Label { id: handles::resolve(ui, root, "lblQualityTick2", "Label", &mut problems) },
            lblQualityTick3: Label { id: handles::resolve(ui, root, "lblQualityTick3", "Label", &mut problems) },
            lblQualityTick4: Label { id: handles::resolve(ui, root, "lblQualityTick4", "Label", &mut problems) },
            lblQualityTick5: Label { id: handles::resolve(ui, root, "lblQualityTick5", "Label", &mut problems) },
            lblQualityTick6: Label { id: handles::resolve(ui, root, "lblQualityTick6", "Label", &mut problems) },
            lblQualityTick7: Label { id: handles::resolve(ui, root, "lblQualityTick7", "Label", &mut problems) },
            lblQualityTick8: Label { id: handles::resolve(ui, root, "lblQualityTick8", "Label", &mut problems) },
            lblQualityTick9: Label { id: handles::resolve(ui, root, "lblQualityTick9", "Label", &mut problems) },
            lblQualityTick10: Label { id: handles::resolve(ui, root, "lblQualityTick10", "Label", &mut problems) },
            lblQualityHint: Label { id: handles::resolve(ui, root, "lblQualityHint", "Label", &mut problems) },
            lylblQualitySvtFloor: Layout { id: handles::resolve(ui, root, "lylblQualitySvtFloor", "Layout", &mut problems) },
            barlblQualitySvtFloor: Rectangle { id: handles::resolve(ui, root, "barlblQualitySvtFloor", "Rectangle", &mut problems) },
            lblQualitySvtFloor: Label { id: handles::resolve(ui, root, "lblQualitySvtFloor", "Label", &mut problems) },
            lnRule2: Rectangle { id: handles::resolve(ui, root, "lnRule2", "Rectangle", &mut problems) },
            lblVideoTest: Label { id: handles::resolve(ui, root, "lblVideoTest", "Label", &mut problems) },
            lblVideoTestHint: Label { id: handles::resolve(ui, root, "lblVideoTestHint", "Label", &mut problems) },
            lyVideoTest: Layout { id: handles::resolve(ui, root, "lyVideoTest", "Layout", &mut problems) },
            lyVtButtons: Layout { id: handles::resolve(ui, root, "lyVtButtons", "Layout", &mut problems) },
            btnVtRun: Button { id: handles::resolve(ui, root, "btnVtRun", "Button", &mut problems) },
            btnVtCancel: Button { id: handles::resolve(ui, root, "btnVtCancel", "Button", &mut problems) },
            lblVtStatus: Label { id: handles::resolve(ui, root, "lblVtStatus", "Label", &mut problems) },
            prgVt: ProgressBar { id: handles::resolve(ui, root, "prgVt", "ProgressBar", &mut problems) },
            lyVtResult: Layout { id: handles::resolve(ui, root, "lyVtResult", "Layout", &mut problems) },
            lblVtSample: Label { id: handles::resolve(ui, root, "lblVtSample", "Label", &mut problems) },
            lyVtLevels: Layout { id: handles::resolve(ui, root, "lyVtLevels", "Layout", &mut problems) },
            lblVtPickHint: Label { id: handles::resolve(ui, root, "lblVtPickHint", "Label", &mut problems) },
            lyVtTools: Layout { id: handles::resolve(ui, root, "lyVtTools", "Layout", &mut problems) },
            btnVtSwap: Button { id: handles::resolve(ui, root, "btnVtSwap", "Button", &mut problems) },
            btnVtZoom300: Button { id: handles::resolve(ui, root, "btnVtZoom300", "Button", &mut problems) },
            btnVtZoom200: Button { id: handles::resolve(ui, root, "btnVtZoom200", "Button", &mut problems) },
            btnVtZoom100: Button { id: handles::resolve(ui, root, "btnVtZoom100", "Button", &mut problems) },
            btnVtZoomFit: Button { id: handles::resolve(ui, root, "btnVtZoomFit", "Button", &mut problems) },
            pnlVtStage: Panel { id: handles::resolve(ui, root, "pnlVtStage", "Panel", &mut problems) },
            imgVtA: Image { id: handles::resolve(ui, root, "imgVtA", "Image", &mut problems) },
            lyVtB: Layout { id: handles::resolve(ui, root, "lyVtB", "Layout", &mut problems) },
            imgVtB: Image { id: handles::resolve(ui, root, "imgVtB", "Image", &mut problems) },
            shpVtSplit: Rectangle { id: handles::resolve(ui, root, "shpVtSplit", "Rectangle", &mut problems) },
            cirVtKnob: Circle { id: handles::resolve(ui, root, "cirVtKnob", "Circle", &mut problems) },
            lblVtKnob: Label { id: handles::resolve(ui, root, "lblVtKnob", "Label", &mut problems) },
            pnlVtTagA: Panel { id: handles::resolve(ui, root, "pnlVtTagA", "Panel", &mut problems) },
            lblVtTagA: Label { id: handles::resolve(ui, root, "lblVtTagA", "Label", &mut problems) },
            shpVtTagA: Rectangle { id: handles::resolve(ui, root, "shpVtTagA", "Rectangle", &mut problems) },
            pnlVtTagB: Panel { id: handles::resolve(ui, root, "pnlVtTagB", "Panel", &mut problems) },
            lblVtTagB: Label { id: handles::resolve(ui, root, "lblVtTagB", "Label", &mut problems) },
            shpVtTagB: Rectangle { id: handles::resolve(ui, root, "shpVtTagB", "Rectangle", &mut problems) },
            lblVtDragHint: Label { id: handles::resolve(ui, root, "lblVtDragHint", "Label", &mut problems) },
            lyVtUse: Layout { id: handles::resolve(ui, root, "lyVtUse", "Layout", &mut problems) },
            btnVtUse: Button { id: handles::resolve(ui, root, "btnVtUse", "Button", &mut problems) },
            lnRule3: Rectangle { id: handles::resolve(ui, root, "lnRule3", "Rectangle", &mut problems) },
            lblFps: Label { id: handles::resolve(ui, root, "lblFps", "Label", &mut problems) },
            lyFps: Layout { id: handles::resolve(ui, root, "lyFps", "Layout", &mut problems) },
            lblFpsLow: Label { id: handles::resolve(ui, root, "lblFpsLow", "Label", &mut problems) },
            lblFpsHigh: Label { id: handles::resolve(ui, root, "lblFpsHigh", "Label", &mut problems) },
            lyFpsTicks: Layout { id: handles::resolve(ui, root, "lyFpsTicks", "Layout", &mut problems) },
            trkFps: TrackBar { id: handles::resolve(ui, root, "trkFps", "TrackBar", &mut problems) },
            lblFpsHint: Label { id: handles::resolve(ui, root, "lblFpsHint", "Label", &mut problems) },
            lnRule4: Rectangle { id: handles::resolve(ui, root, "lnRule4", "Rectangle", &mut problems) },
            lblKeyframe: Label { id: handles::resolve(ui, root, "lblKeyframe", "Label", &mut problems) },
            lyKeyframe: Layout { id: handles::resolve(ui, root, "lyKeyframe", "Layout", &mut problems) },
            lblKeyframeLow: Label { id: handles::resolve(ui, root, "lblKeyframeLow", "Label", &mut problems) },
            lblKeyframeHigh: Label { id: handles::resolve(ui, root, "lblKeyframeHigh", "Label", &mut problems) },
            lyKeyframeTicks: Layout { id: handles::resolve(ui, root, "lyKeyframeTicks", "Layout", &mut problems) },
            trkKeyframe: TrackBar { id: handles::resolve(ui, root, "trkKeyframe", "TrackBar", &mut problems) },
            lblKeyframeTick1: Label { id: handles::resolve(ui, root, "lblKeyframeTick1", "Label", &mut problems) },
            lblKeyframeTick2: Label { id: handles::resolve(ui, root, "lblKeyframeTick2", "Label", &mut problems) },
            lblKeyframeTick3: Label { id: handles::resolve(ui, root, "lblKeyframeTick3", "Label", &mut problems) },
            lblKeyframeTick4: Label { id: handles::resolve(ui, root, "lblKeyframeTick4", "Label", &mut problems) },
            lblKeyframeTick5: Label { id: handles::resolve(ui, root, "lblKeyframeTick5", "Label", &mut problems) },
            lblKeyframeTick6: Label { id: handles::resolve(ui, root, "lblKeyframeTick6", "Label", &mut problems) },
            lblKeyframeTick7: Label { id: handles::resolve(ui, root, "lblKeyframeTick7", "Label", &mut problems) },
            lblKeyframeTick8: Label { id: handles::resolve(ui, root, "lblKeyframeTick8", "Label", &mut problems) },
            lblKeyframeTick9: Label { id: handles::resolve(ui, root, "lblKeyframeTick9", "Label", &mut problems) },
            lblKeyframeTick10: Label { id: handles::resolve(ui, root, "lblKeyframeTick10", "Label", &mut problems) },
            lblKeyframeHint: Label { id: handles::resolve(ui, root, "lblKeyframeHint", "Label", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form fraVideo não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FraVideoComponents {
            form: instance.form,
            lblCodec: instance.lblCodec,
            lyCodec: instance.lyCodec,
            imgGpu: instance.imgGpu,
            cmbCodec: instance.cmbCodec,
            lblCodecMaxRes: instance.lblCodecMaxRes,
            lylblCodecSwWarn: instance.lylblCodecSwWarn,
            barlblCodecSwWarn: instance.barlblCodecSwWarn,
            lblCodecSwWarn: instance.lblCodecSwWarn,
            lblCodecHint: instance.lblCodecHint,
            lnRule1: instance.lnRule1,
            lblQuality: instance.lblQuality,
            lyQuality: instance.lyQuality,
            lblQualityLow: instance.lblQualityLow,
            lblQualityHigh: instance.lblQualityHigh,
            lyQualityTicks: instance.lyQualityTicks,
            trkQuality: instance.trkQuality,
            lblQualityTick0: instance.lblQualityTick0,
            lblQualityTick1: instance.lblQualityTick1,
            lblQualityTick2: instance.lblQualityTick2,
            lblQualityTick3: instance.lblQualityTick3,
            lblQualityTick4: instance.lblQualityTick4,
            lblQualityTick5: instance.lblQualityTick5,
            lblQualityTick6: instance.lblQualityTick6,
            lblQualityTick7: instance.lblQualityTick7,
            lblQualityTick8: instance.lblQualityTick8,
            lblQualityTick9: instance.lblQualityTick9,
            lblQualityTick10: instance.lblQualityTick10,
            lblQualityHint: instance.lblQualityHint,
            lylblQualitySvtFloor: instance.lylblQualitySvtFloor,
            barlblQualitySvtFloor: instance.barlblQualitySvtFloor,
            lblQualitySvtFloor: instance.lblQualitySvtFloor,
            lnRule2: instance.lnRule2,
            lblVideoTest: instance.lblVideoTest,
            lblVideoTestHint: instance.lblVideoTestHint,
            lyVideoTest: instance.lyVideoTest,
            lyVtButtons: instance.lyVtButtons,
            btnVtRun: instance.btnVtRun,
            btnVtCancel: instance.btnVtCancel,
            lblVtStatus: instance.lblVtStatus,
            prgVt: instance.prgVt,
            lyVtResult: instance.lyVtResult,
            lblVtSample: instance.lblVtSample,
            lyVtLevels: instance.lyVtLevels,
            lblVtPickHint: instance.lblVtPickHint,
            lyVtTools: instance.lyVtTools,
            btnVtSwap: instance.btnVtSwap,
            btnVtZoom300: instance.btnVtZoom300,
            btnVtZoom200: instance.btnVtZoom200,
            btnVtZoom100: instance.btnVtZoom100,
            btnVtZoomFit: instance.btnVtZoomFit,
            pnlVtStage: instance.pnlVtStage,
            imgVtA: instance.imgVtA,
            lyVtB: instance.lyVtB,
            imgVtB: instance.imgVtB,
            shpVtSplit: instance.shpVtSplit,
            cirVtKnob: instance.cirVtKnob,
            lblVtKnob: instance.lblVtKnob,
            pnlVtTagA: instance.pnlVtTagA,
            lblVtTagA: instance.lblVtTagA,
            shpVtTagA: instance.shpVtTagA,
            pnlVtTagB: instance.pnlVtTagB,
            lblVtTagB: instance.lblVtTagB,
            shpVtTagB: instance.shpVtTagB,
            lblVtDragHint: instance.lblVtDragHint,
            lyVtUse: instance.lyVtUse,
            btnVtUse: instance.btnVtUse,
            lnRule3: instance.lnRule3,
            lblFps: instance.lblFps,
            lyFps: instance.lyFps,
            lblFpsLow: instance.lblFpsLow,
            lblFpsHigh: instance.lblFpsHigh,
            lyFpsTicks: instance.lyFpsTicks,
            trkFps: instance.trkFps,
            lblFpsHint: instance.lblFpsHint,
            lnRule4: instance.lnRule4,
            lblKeyframe: instance.lblKeyframe,
            lyKeyframe: instance.lyKeyframe,
            lblKeyframeLow: instance.lblKeyframeLow,
            lblKeyframeHigh: instance.lblKeyframeHigh,
            lyKeyframeTicks: instance.lyKeyframeTicks,
            trkKeyframe: instance.trkKeyframe,
            lblKeyframeTick1: instance.lblKeyframeTick1,
            lblKeyframeTick2: instance.lblKeyframeTick2,
            lblKeyframeTick3: instance.lblKeyframeTick3,
            lblKeyframeTick4: instance.lblKeyframeTick4,
            lblKeyframeTick5: instance.lblKeyframeTick5,
            lblKeyframeTick6: instance.lblKeyframeTick6,
            lblKeyframeTick7: instance.lblKeyframeTick7,
            lblKeyframeTick8: instance.lblKeyframeTick8,
            lblKeyframeTick9: instance.lblKeyframeTick9,
            lblKeyframeTick10: instance.lblKeyframeTick10,
            lblKeyframeHint: instance.lblKeyframeHint,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do fraVideo aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FraVideoComponents> {
        ui.form_components(ui.find_path("fraVideo")?)
    }

    /// Cria o fraVideo (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, iron::prelude::Error> {
        ui.create_form("fraVideo")
    }

    /// Cria mais um fraVideo dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`fraVideo`, `fraVideo2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FraVideoComponents, iron::prelude::Error> {
        let root = ui.create_form_instance("fraVideo")?;
        let name = ui.free_name_in(parent, "fraVideo");
        if let Err(e) = ui.set_name(root, &name).and_then(|_| ui.set_parent(root, Some(parent))) {
            ui.free(root);
            return Err(e);
        }
        ui.form_components(root).ok_or(iron::prelude::Error::InvalidId)
    }

    /// Os componentes de uma instância do fraVideo (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FraVideoComponents> {
        ui.form_components(root)
    }

    /// O fraVideo aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FraVideo> {
        ui.form_instance(ui.find_path("fraVideo")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FraVideo> {
        let form = ui.find_path("fraVideo")?;
        ui.form_instance_mut(form)
    }

    /// O Button deste form com o nome (`None` se não há um Button com ele).
    pub fn find_button(&self, name: &str) -> Option<Button> {
        match name {
            "btnVtRun" => Some(self.btnVtRun),
            "btnVtCancel" => Some(self.btnVtCancel),
            "btnVtSwap" => Some(self.btnVtSwap),
            "btnVtZoom300" => Some(self.btnVtZoom300),
            "btnVtZoom200" => Some(self.btnVtZoom200),
            "btnVtZoom100" => Some(self.btnVtZoom100),
            "btnVtZoomFit" => Some(self.btnVtZoomFit),
            "btnVtUse" => Some(self.btnVtUse),
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
            "cirVtKnob" => Some(self.cirVtKnob),
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
            "cmbCodec" => Some(self.cmbCodec),
            _ => None,
        }
    }

    /// O ComboBox que disparou o evento (`None` se o `sender` não é um ComboBox deste form).
    pub fn sender_as_combo_box(&self, ui: &Ui, sender: ComponentId) -> Option<ComboBox> {
        self.find_combo_box(ui.name(sender)?)
    }

    /// O Image deste form com o nome (`None` se não há um Image com ele).
    pub fn find_image(&self, name: &str) -> Option<Image> {
        match name {
            "imgGpu" => Some(self.imgGpu),
            "imgVtA" => Some(self.imgVtA),
            "imgVtB" => Some(self.imgVtB),
            _ => None,
        }
    }

    /// O Image que disparou o evento (`None` se o `sender` não é um Image deste form).
    pub fn sender_as_image(&self, ui: &Ui, sender: ComponentId) -> Option<Image> {
        self.find_image(ui.name(sender)?)
    }

    /// O Label deste form com o nome (`None` se não há um Label com ele).
    pub fn find_label(&self, name: &str) -> Option<Label> {
        match name {
            "lblCodec" => Some(self.lblCodec),
            "lblCodecMaxRes" => Some(self.lblCodecMaxRes),
            "lblCodecSwWarn" => Some(self.lblCodecSwWarn),
            "lblCodecHint" => Some(self.lblCodecHint),
            "lblQuality" => Some(self.lblQuality),
            "lblQualityLow" => Some(self.lblQualityLow),
            "lblQualityHigh" => Some(self.lblQualityHigh),
            "lblQualityTick0" => Some(self.lblQualityTick0),
            "lblQualityTick1" => Some(self.lblQualityTick1),
            "lblQualityTick2" => Some(self.lblQualityTick2),
            "lblQualityTick3" => Some(self.lblQualityTick3),
            "lblQualityTick4" => Some(self.lblQualityTick4),
            "lblQualityTick5" => Some(self.lblQualityTick5),
            "lblQualityTick6" => Some(self.lblQualityTick6),
            "lblQualityTick7" => Some(self.lblQualityTick7),
            "lblQualityTick8" => Some(self.lblQualityTick8),
            "lblQualityTick9" => Some(self.lblQualityTick9),
            "lblQualityTick10" => Some(self.lblQualityTick10),
            "lblQualityHint" => Some(self.lblQualityHint),
            "lblQualitySvtFloor" => Some(self.lblQualitySvtFloor),
            "lblVideoTest" => Some(self.lblVideoTest),
            "lblVideoTestHint" => Some(self.lblVideoTestHint),
            "lblVtStatus" => Some(self.lblVtStatus),
            "lblVtSample" => Some(self.lblVtSample),
            "lblVtPickHint" => Some(self.lblVtPickHint),
            "lblVtKnob" => Some(self.lblVtKnob),
            "lblVtTagA" => Some(self.lblVtTagA),
            "lblVtTagB" => Some(self.lblVtTagB),
            "lblVtDragHint" => Some(self.lblVtDragHint),
            "lblFps" => Some(self.lblFps),
            "lblFpsLow" => Some(self.lblFpsLow),
            "lblFpsHigh" => Some(self.lblFpsHigh),
            "lblFpsHint" => Some(self.lblFpsHint),
            "lblKeyframe" => Some(self.lblKeyframe),
            "lblKeyframeLow" => Some(self.lblKeyframeLow),
            "lblKeyframeHigh" => Some(self.lblKeyframeHigh),
            "lblKeyframeTick1" => Some(self.lblKeyframeTick1),
            "lblKeyframeTick2" => Some(self.lblKeyframeTick2),
            "lblKeyframeTick3" => Some(self.lblKeyframeTick3),
            "lblKeyframeTick4" => Some(self.lblKeyframeTick4),
            "lblKeyframeTick5" => Some(self.lblKeyframeTick5),
            "lblKeyframeTick6" => Some(self.lblKeyframeTick6),
            "lblKeyframeTick7" => Some(self.lblKeyframeTick7),
            "lblKeyframeTick8" => Some(self.lblKeyframeTick8),
            "lblKeyframeTick9" => Some(self.lblKeyframeTick9),
            "lblKeyframeTick10" => Some(self.lblKeyframeTick10),
            "lblKeyframeHint" => Some(self.lblKeyframeHint),
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
            "lyCodec" => Some(self.lyCodec),
            "lylblCodecSwWarn" => Some(self.lylblCodecSwWarn),
            "lyQuality" => Some(self.lyQuality),
            "lyQualityTicks" => Some(self.lyQualityTicks),
            "lylblQualitySvtFloor" => Some(self.lylblQualitySvtFloor),
            "lyVideoTest" => Some(self.lyVideoTest),
            "lyVtButtons" => Some(self.lyVtButtons),
            "lyVtResult" => Some(self.lyVtResult),
            "lyVtLevels" => Some(self.lyVtLevels),
            "lyVtTools" => Some(self.lyVtTools),
            "lyVtB" => Some(self.lyVtB),
            "lyVtUse" => Some(self.lyVtUse),
            "lyFps" => Some(self.lyFps),
            "lyFpsTicks" => Some(self.lyFpsTicks),
            "lyKeyframe" => Some(self.lyKeyframe),
            "lyKeyframeTicks" => Some(self.lyKeyframeTicks),
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
            "pnlVtStage" => Some(self.pnlVtStage),
            "pnlVtTagA" => Some(self.pnlVtTagA),
            "pnlVtTagB" => Some(self.pnlVtTagB),
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
            "prgVt" => Some(self.prgVt),
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
            "barlblCodecSwWarn" => Some(self.barlblCodecSwWarn),
            "lnRule1" => Some(self.lnRule1),
            "barlblQualitySvtFloor" => Some(self.barlblQualitySvtFloor),
            "lnRule2" => Some(self.lnRule2),
            "shpVtSplit" => Some(self.shpVtSplit),
            "shpVtTagA" => Some(self.shpVtTagA),
            "shpVtTagB" => Some(self.shpVtTagB),
            "lnRule3" => Some(self.lnRule3),
            "lnRule4" => Some(self.lnRule4),
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
            "trkQuality" => Some(self.trkQuality),
            "trkFps" => Some(self.trkFps),
            "trkKeyframe" => Some(self.trkKeyframe),
            _ => None,
        }
    }

    /// O TrackBar que disparou o evento (`None` se o `sender` não é um TrackBar deste form).
    pub fn sender_as_track_bar(&self, ui: &Ui, sender: ComponentId) -> Option<TrackBar> {
        self.find_track_bar(ui.name(sender)?)
    }
}
