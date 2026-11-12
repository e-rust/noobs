
/// O form frmExport: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FrmExport {
    pub form: Form,
    pub pnlExport: Panel,
    pub pnlExHead: Panel,
    pub lnExHead: Rectangle,
    pub btnExClose: Button,
    pub lblExTitle: Label,
    pub scrExport: ScrollBox,
    pub lyExport: Layout,
    pub lblExRegionsHead: Label,
    pub flwExRegions: FlowLayout,
    pub lblExRegionsHint: Label,
    pub lblExRangeHead: Label,
    pub pnlExStage: Panel,
    pub fvExFrame: FrameView,
    pub crpExCrop: CropBox,
    pub lblExMsg: Label,
    pub lblExAudioOnly: Label,
    pub lblExCaption: Label,
    pub lyExCrop: Layout,
    pub btnExCropReset: Button,
    pub btnExZoomOut: Button,
    pub lblExZoom: Label,
    pub btnExZoomIn: Button,
    pub btnExZoomReset: Button,
    pub lblExCropInfo: Label,
    pub lblExCropHint: Label,
    pub lyExPlay: Layout,
    pub btnExPlay: Button,
    pub btnExMute: Button,
    pub trkExVolume: TrackBar,
    pub btnExCc: Button,
    pub lblExSpaceHint: Label,
    pub lyExTimeline: Layout,
    pub pnlExTl: Panel,
    pub shpExRuler: Rectangle,
    pub shpExRulerLine: Rectangle,
    pub imgExTl: Image,
    pub pnlExSb: Panel,
    pub shpExSbThumb: Rectangle,
    pub lyExTlLabels: Layout,
    pub lyExTicks: Layout,
    pub shpExPlayShade: Rectangle,
    pub shpExPlayLine: Rectangle,
    pub lyExTlBar: Layout,
    pub btnExCut: Button,
    pub btnExUncut: Button,
    pub btnExToggle: Button,
    pub btnExTlZoomOut: Button,
    pub lblExTlZoom: Label,
    pub btnExTlZoomIn: Button,
    pub edtExTime: Edit,
    pub lblExRangeLen: Label,
    pub lblExRangeText: Label,
    pub lblExRangeHint: Label,
    pub lyExAudio: Layout,
    pub lblExAudioHead: Label,
    pub lyExOrigRow: Layout,
    pub pnlExOrig: Panel,
    pub chkExOrig: CheckBox,
    pub lblExOrig: Label,
    pub flwExTracks: FlowLayout,
    pub lyExMixRow: Layout,
    pub pnlExMix: Panel,
    pub chkExMix: CheckBox,
    pub lblExMix: Label,
    pub lyExVideoHead: Layout,
    pub lblExResHead: Label,
    pub lblExEncHead: Label,
    pub lyExVideoRow: Layout,
    pub cmbExRes: ComboBox,
    pub cmbExEnc: ComboBox,
    pub lblExResHint: Label,
    pub lyExScale: Layout,
    pub lblExScaleHead: Label,
    pub lyExScaleRow: Layout,
    pub cmbExScale: ComboBox,
    pub lblExScaleHint: Label,
    pub lblExSpeedHead: Label,
    pub lyExSpeedRow: Layout,
    pub edtExSpeed: Edit,
    pub lblExSpeedX: Label,
    pub lblExSpeedValue: Label,
    pub lblExSpeedHint: Label,
    pub lyExFps: Layout,
    pub lblExFpsHead: Label,
    pub lyExFpsRow: Layout,
    pub lblExFps: Label,
    pub trkExFps: TrackBar,
    pub lblExFpsHint: Label,
    pub lblExQualityHead: Label,
    pub lyExQualityRow: Layout,
    pub lblExQuality: Label,
    pub trkExQuality: TrackBar,
    pub lblExQualityHint: Label,
    pub lblExCaptionHead: Label,
    pub lyExBurnRow: Layout,
    pub pnlExBurn: Panel,
    pub chkExBurn: CheckBox,
    pub lblExBurn: Label,
    pub lblExCaptionHint: Label,
    pub lblExNameHead: Label,
    pub lyExNameRow: Layout,
    pub cmbExCont: ComboBox,
    pub edtExName: Edit,
    pub lblExContHint: Label,
    pub lyExActions: Layout,
    pub btnExRun: Button,
    pub btnExPreview: Button,
    pub cmbExPrevSec: ComboBox,
    pub lyExProgress: Layout,
    pub prgExProgress: ProgressBar,
    pub lblExProgress: Label,
    pub data: super::export::Data,
}

/// Os componentes do frmExport sem o estado, para o código fora dos handlers
/// (`FrmExport::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FrmExportComponents {
    pub form: Form,
    pub pnlExport: Panel,
    pub pnlExHead: Panel,
    pub lnExHead: Rectangle,
    pub btnExClose: Button,
    pub lblExTitle: Label,
    pub scrExport: ScrollBox,
    pub lyExport: Layout,
    pub lblExRegionsHead: Label,
    pub flwExRegions: FlowLayout,
    pub lblExRegionsHint: Label,
    pub lblExRangeHead: Label,
    pub pnlExStage: Panel,
    pub fvExFrame: FrameView,
    pub crpExCrop: CropBox,
    pub lblExMsg: Label,
    pub lblExAudioOnly: Label,
    pub lblExCaption: Label,
    pub lyExCrop: Layout,
    pub btnExCropReset: Button,
    pub btnExZoomOut: Button,
    pub lblExZoom: Label,
    pub btnExZoomIn: Button,
    pub btnExZoomReset: Button,
    pub lblExCropInfo: Label,
    pub lblExCropHint: Label,
    pub lyExPlay: Layout,
    pub btnExPlay: Button,
    pub btnExMute: Button,
    pub trkExVolume: TrackBar,
    pub btnExCc: Button,
    pub lblExSpaceHint: Label,
    pub lyExTimeline: Layout,
    pub pnlExTl: Panel,
    pub shpExRuler: Rectangle,
    pub shpExRulerLine: Rectangle,
    pub imgExTl: Image,
    pub pnlExSb: Panel,
    pub shpExSbThumb: Rectangle,
    pub lyExTlLabels: Layout,
    pub lyExTicks: Layout,
    pub shpExPlayShade: Rectangle,
    pub shpExPlayLine: Rectangle,
    pub lyExTlBar: Layout,
    pub btnExCut: Button,
    pub btnExUncut: Button,
    pub btnExToggle: Button,
    pub btnExTlZoomOut: Button,
    pub lblExTlZoom: Label,
    pub btnExTlZoomIn: Button,
    pub edtExTime: Edit,
    pub lblExRangeLen: Label,
    pub lblExRangeText: Label,
    pub lblExRangeHint: Label,
    pub lyExAudio: Layout,
    pub lblExAudioHead: Label,
    pub lyExOrigRow: Layout,
    pub pnlExOrig: Panel,
    pub chkExOrig: CheckBox,
    pub lblExOrig: Label,
    pub flwExTracks: FlowLayout,
    pub lyExMixRow: Layout,
    pub pnlExMix: Panel,
    pub chkExMix: CheckBox,
    pub lblExMix: Label,
    pub lyExVideoHead: Layout,
    pub lblExResHead: Label,
    pub lblExEncHead: Label,
    pub lyExVideoRow: Layout,
    pub cmbExRes: ComboBox,
    pub cmbExEnc: ComboBox,
    pub lblExResHint: Label,
    pub lyExScale: Layout,
    pub lblExScaleHead: Label,
    pub lyExScaleRow: Layout,
    pub cmbExScale: ComboBox,
    pub lblExScaleHint: Label,
    pub lblExSpeedHead: Label,
    pub lyExSpeedRow: Layout,
    pub edtExSpeed: Edit,
    pub lblExSpeedX: Label,
    pub lblExSpeedValue: Label,
    pub lblExSpeedHint: Label,
    pub lyExFps: Layout,
    pub lblExFpsHead: Label,
    pub lyExFpsRow: Layout,
    pub lblExFps: Label,
    pub trkExFps: TrackBar,
    pub lblExFpsHint: Label,
    pub lblExQualityHead: Label,
    pub lyExQualityRow: Layout,
    pub lblExQuality: Label,
    pub trkExQuality: TrackBar,
    pub lblExQualityHint: Label,
    pub lblExCaptionHead: Label,
    pub lyExBurnRow: Layout,
    pub pnlExBurn: Panel,
    pub chkExBurn: CheckBox,
    pub lblExBurn: Label,
    pub lblExCaptionHint: Label,
    pub lblExNameHead: Label,
    pub lyExNameRow: Layout,
    pub cmbExCont: ComboBox,
    pub edtExName: Edit,
    pub lblExContHint: Label,
    pub lyExActions: Layout,
    pub btnExRun: Button,
    pub btnExPreview: Button,
    pub cmbExPrevSec: ComboBox,
    pub lyExProgress: Layout,
    pub prgExProgress: ProgressBar,
    pub lblExProgress: Label,
}

impl std::ops::Deref for FrmExport {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FrmExport {
    /// Monta a instância de um frmExport recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            pnlExport: Panel { id: handles::resolve(ui, root, "pnlExport", "Panel", &mut problems) },
            pnlExHead: Panel { id: handles::resolve(ui, root, "pnlExHead", "Panel", &mut problems) },
            lnExHead: Rectangle { id: handles::resolve(ui, root, "lnExHead", "Rectangle", &mut problems) },
            btnExClose: Button { id: handles::resolve(ui, root, "btnExClose", "Button", &mut problems) },
            lblExTitle: Label { id: handles::resolve(ui, root, "lblExTitle", "Label", &mut problems) },
            scrExport: ScrollBox { id: handles::resolve(ui, root, "scrExport", "ScrollBox", &mut problems) },
            lyExport: Layout { id: handles::resolve(ui, root, "lyExport", "Layout", &mut problems) },
            lblExRegionsHead: Label { id: handles::resolve(ui, root, "lblExRegionsHead", "Label", &mut problems) },
            flwExRegions: FlowLayout { id: handles::resolve(ui, root, "flwExRegions", "FlowLayout", &mut problems) },
            lblExRegionsHint: Label { id: handles::resolve(ui, root, "lblExRegionsHint", "Label", &mut problems) },
            lblExRangeHead: Label { id: handles::resolve(ui, root, "lblExRangeHead", "Label", &mut problems) },
            pnlExStage: Panel { id: handles::resolve(ui, root, "pnlExStage", "Panel", &mut problems) },
            fvExFrame: FrameView { id: handles::resolve(ui, root, "fvExFrame", "FrameView", &mut problems) },
            crpExCrop: CropBox { id: handles::resolve(ui, root, "crpExCrop", "CropBox", &mut problems) },
            lblExMsg: Label { id: handles::resolve(ui, root, "lblExMsg", "Label", &mut problems) },
            lblExAudioOnly: Label { id: handles::resolve(ui, root, "lblExAudioOnly", "Label", &mut problems) },
            lblExCaption: Label { id: handles::resolve(ui, root, "lblExCaption", "Label", &mut problems) },
            lyExCrop: Layout { id: handles::resolve(ui, root, "lyExCrop", "Layout", &mut problems) },
            btnExCropReset: Button { id: handles::resolve(ui, root, "btnExCropReset", "Button", &mut problems) },
            btnExZoomOut: Button { id: handles::resolve(ui, root, "btnExZoomOut", "Button", &mut problems) },
            lblExZoom: Label { id: handles::resolve(ui, root, "lblExZoom", "Label", &mut problems) },
            btnExZoomIn: Button { id: handles::resolve(ui, root, "btnExZoomIn", "Button", &mut problems) },
            btnExZoomReset: Button { id: handles::resolve(ui, root, "btnExZoomReset", "Button", &mut problems) },
            lblExCropInfo: Label { id: handles::resolve(ui, root, "lblExCropInfo", "Label", &mut problems) },
            lblExCropHint: Label { id: handles::resolve(ui, root, "lblExCropHint", "Label", &mut problems) },
            lyExPlay: Layout { id: handles::resolve(ui, root, "lyExPlay", "Layout", &mut problems) },
            btnExPlay: Button { id: handles::resolve(ui, root, "btnExPlay", "Button", &mut problems) },
            btnExMute: Button { id: handles::resolve(ui, root, "btnExMute", "Button", &mut problems) },
            trkExVolume: TrackBar { id: handles::resolve(ui, root, "trkExVolume", "TrackBar", &mut problems) },
            btnExCc: Button { id: handles::resolve(ui, root, "btnExCc", "Button", &mut problems) },
            lblExSpaceHint: Label { id: handles::resolve(ui, root, "lblExSpaceHint", "Label", &mut problems) },
            lyExTimeline: Layout { id: handles::resolve(ui, root, "lyExTimeline", "Layout", &mut problems) },
            pnlExTl: Panel { id: handles::resolve(ui, root, "pnlExTl", "Panel", &mut problems) },
            shpExRuler: Rectangle { id: handles::resolve(ui, root, "shpExRuler", "Rectangle", &mut problems) },
            shpExRulerLine: Rectangle { id: handles::resolve(ui, root, "shpExRulerLine", "Rectangle", &mut problems) },
            imgExTl: Image { id: handles::resolve(ui, root, "imgExTl", "Image", &mut problems) },
            pnlExSb: Panel { id: handles::resolve(ui, root, "pnlExSb", "Panel", &mut problems) },
            shpExSbThumb: Rectangle { id: handles::resolve(ui, root, "shpExSbThumb", "Rectangle", &mut problems) },
            lyExTlLabels: Layout { id: handles::resolve(ui, root, "lyExTlLabels", "Layout", &mut problems) },
            lyExTicks: Layout { id: handles::resolve(ui, root, "lyExTicks", "Layout", &mut problems) },
            shpExPlayShade: Rectangle { id: handles::resolve(ui, root, "shpExPlayShade", "Rectangle", &mut problems) },
            shpExPlayLine: Rectangle { id: handles::resolve(ui, root, "shpExPlayLine", "Rectangle", &mut problems) },
            lyExTlBar: Layout { id: handles::resolve(ui, root, "lyExTlBar", "Layout", &mut problems) },
            btnExCut: Button { id: handles::resolve(ui, root, "btnExCut", "Button", &mut problems) },
            btnExUncut: Button { id: handles::resolve(ui, root, "btnExUncut", "Button", &mut problems) },
            btnExToggle: Button { id: handles::resolve(ui, root, "btnExToggle", "Button", &mut problems) },
            btnExTlZoomOut: Button { id: handles::resolve(ui, root, "btnExTlZoomOut", "Button", &mut problems) },
            lblExTlZoom: Label { id: handles::resolve(ui, root, "lblExTlZoom", "Label", &mut problems) },
            btnExTlZoomIn: Button { id: handles::resolve(ui, root, "btnExTlZoomIn", "Button", &mut problems) },
            edtExTime: Edit { id: handles::resolve(ui, root, "edtExTime", "Edit", &mut problems) },
            lblExRangeLen: Label { id: handles::resolve(ui, root, "lblExRangeLen", "Label", &mut problems) },
            lblExRangeText: Label { id: handles::resolve(ui, root, "lblExRangeText", "Label", &mut problems) },
            lblExRangeHint: Label { id: handles::resolve(ui, root, "lblExRangeHint", "Label", &mut problems) },
            lyExAudio: Layout { id: handles::resolve(ui, root, "lyExAudio", "Layout", &mut problems) },
            lblExAudioHead: Label { id: handles::resolve(ui, root, "lblExAudioHead", "Label", &mut problems) },
            lyExOrigRow: Layout { id: handles::resolve(ui, root, "lyExOrigRow", "Layout", &mut problems) },
            pnlExOrig: Panel { id: handles::resolve(ui, root, "pnlExOrig", "Panel", &mut problems) },
            chkExOrig: CheckBox { id: handles::resolve(ui, root, "chkExOrig", "CheckBox", &mut problems) },
            lblExOrig: Label { id: handles::resolve(ui, root, "lblExOrig", "Label", &mut problems) },
            flwExTracks: FlowLayout { id: handles::resolve(ui, root, "flwExTracks", "FlowLayout", &mut problems) },
            lyExMixRow: Layout { id: handles::resolve(ui, root, "lyExMixRow", "Layout", &mut problems) },
            pnlExMix: Panel { id: handles::resolve(ui, root, "pnlExMix", "Panel", &mut problems) },
            chkExMix: CheckBox { id: handles::resolve(ui, root, "chkExMix", "CheckBox", &mut problems) },
            lblExMix: Label { id: handles::resolve(ui, root, "lblExMix", "Label", &mut problems) },
            lyExVideoHead: Layout { id: handles::resolve(ui, root, "lyExVideoHead", "Layout", &mut problems) },
            lblExResHead: Label { id: handles::resolve(ui, root, "lblExResHead", "Label", &mut problems) },
            lblExEncHead: Label { id: handles::resolve(ui, root, "lblExEncHead", "Label", &mut problems) },
            lyExVideoRow: Layout { id: handles::resolve(ui, root, "lyExVideoRow", "Layout", &mut problems) },
            cmbExRes: ComboBox { id: handles::resolve(ui, root, "cmbExRes", "ComboBox", &mut problems) },
            cmbExEnc: ComboBox { id: handles::resolve(ui, root, "cmbExEnc", "ComboBox", &mut problems) },
            lblExResHint: Label { id: handles::resolve(ui, root, "lblExResHint", "Label", &mut problems) },
            lyExScale: Layout { id: handles::resolve(ui, root, "lyExScale", "Layout", &mut problems) },
            lblExScaleHead: Label { id: handles::resolve(ui, root, "lblExScaleHead", "Label", &mut problems) },
            lyExScaleRow: Layout { id: handles::resolve(ui, root, "lyExScaleRow", "Layout", &mut problems) },
            cmbExScale: ComboBox { id: handles::resolve(ui, root, "cmbExScale", "ComboBox", &mut problems) },
            lblExScaleHint: Label { id: handles::resolve(ui, root, "lblExScaleHint", "Label", &mut problems) },
            lblExSpeedHead: Label { id: handles::resolve(ui, root, "lblExSpeedHead", "Label", &mut problems) },
            lyExSpeedRow: Layout { id: handles::resolve(ui, root, "lyExSpeedRow", "Layout", &mut problems) },
            edtExSpeed: Edit { id: handles::resolve(ui, root, "edtExSpeed", "Edit", &mut problems) },
            lblExSpeedX: Label { id: handles::resolve(ui, root, "lblExSpeedX", "Label", &mut problems) },
            lblExSpeedValue: Label { id: handles::resolve(ui, root, "lblExSpeedValue", "Label", &mut problems) },
            lblExSpeedHint: Label { id: handles::resolve(ui, root, "lblExSpeedHint", "Label", &mut problems) },
            lyExFps: Layout { id: handles::resolve(ui, root, "lyExFps", "Layout", &mut problems) },
            lblExFpsHead: Label { id: handles::resolve(ui, root, "lblExFpsHead", "Label", &mut problems) },
            lyExFpsRow: Layout { id: handles::resolve(ui, root, "lyExFpsRow", "Layout", &mut problems) },
            lblExFps: Label { id: handles::resolve(ui, root, "lblExFps", "Label", &mut problems) },
            trkExFps: TrackBar { id: handles::resolve(ui, root, "trkExFps", "TrackBar", &mut problems) },
            lblExFpsHint: Label { id: handles::resolve(ui, root, "lblExFpsHint", "Label", &mut problems) },
            lblExQualityHead: Label { id: handles::resolve(ui, root, "lblExQualityHead", "Label", &mut problems) },
            lyExQualityRow: Layout { id: handles::resolve(ui, root, "lyExQualityRow", "Layout", &mut problems) },
            lblExQuality: Label { id: handles::resolve(ui, root, "lblExQuality", "Label", &mut problems) },
            trkExQuality: TrackBar { id: handles::resolve(ui, root, "trkExQuality", "TrackBar", &mut problems) },
            lblExQualityHint: Label { id: handles::resolve(ui, root, "lblExQualityHint", "Label", &mut problems) },
            lblExCaptionHead: Label { id: handles::resolve(ui, root, "lblExCaptionHead", "Label", &mut problems) },
            lyExBurnRow: Layout { id: handles::resolve(ui, root, "lyExBurnRow", "Layout", &mut problems) },
            pnlExBurn: Panel { id: handles::resolve(ui, root, "pnlExBurn", "Panel", &mut problems) },
            chkExBurn: CheckBox { id: handles::resolve(ui, root, "chkExBurn", "CheckBox", &mut problems) },
            lblExBurn: Label { id: handles::resolve(ui, root, "lblExBurn", "Label", &mut problems) },
            lblExCaptionHint: Label { id: handles::resolve(ui, root, "lblExCaptionHint", "Label", &mut problems) },
            lblExNameHead: Label { id: handles::resolve(ui, root, "lblExNameHead", "Label", &mut problems) },
            lyExNameRow: Layout { id: handles::resolve(ui, root, "lyExNameRow", "Layout", &mut problems) },
            cmbExCont: ComboBox { id: handles::resolve(ui, root, "cmbExCont", "ComboBox", &mut problems) },
            edtExName: Edit { id: handles::resolve(ui, root, "edtExName", "Edit", &mut problems) },
            lblExContHint: Label { id: handles::resolve(ui, root, "lblExContHint", "Label", &mut problems) },
            lyExActions: Layout { id: handles::resolve(ui, root, "lyExActions", "Layout", &mut problems) },
            btnExRun: Button { id: handles::resolve(ui, root, "btnExRun", "Button", &mut problems) },
            btnExPreview: Button { id: handles::resolve(ui, root, "btnExPreview", "Button", &mut problems) },
            cmbExPrevSec: ComboBox { id: handles::resolve(ui, root, "cmbExPrevSec", "ComboBox", &mut problems) },
            lyExProgress: Layout { id: handles::resolve(ui, root, "lyExProgress", "Layout", &mut problems) },
            prgExProgress: ProgressBar { id: handles::resolve(ui, root, "prgExProgress", "ProgressBar", &mut problems) },
            lblExProgress: Label { id: handles::resolve(ui, root, "lblExProgress", "Label", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form frmExport não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FrmExportComponents {
            form: instance.form,
            pnlExport: instance.pnlExport,
            pnlExHead: instance.pnlExHead,
            lnExHead: instance.lnExHead,
            btnExClose: instance.btnExClose,
            lblExTitle: instance.lblExTitle,
            scrExport: instance.scrExport,
            lyExport: instance.lyExport,
            lblExRegionsHead: instance.lblExRegionsHead,
            flwExRegions: instance.flwExRegions,
            lblExRegionsHint: instance.lblExRegionsHint,
            lblExRangeHead: instance.lblExRangeHead,
            pnlExStage: instance.pnlExStage,
            fvExFrame: instance.fvExFrame,
            crpExCrop: instance.crpExCrop,
            lblExMsg: instance.lblExMsg,
            lblExAudioOnly: instance.lblExAudioOnly,
            lblExCaption: instance.lblExCaption,
            lyExCrop: instance.lyExCrop,
            btnExCropReset: instance.btnExCropReset,
            btnExZoomOut: instance.btnExZoomOut,
            lblExZoom: instance.lblExZoom,
            btnExZoomIn: instance.btnExZoomIn,
            btnExZoomReset: instance.btnExZoomReset,
            lblExCropInfo: instance.lblExCropInfo,
            lblExCropHint: instance.lblExCropHint,
            lyExPlay: instance.lyExPlay,
            btnExPlay: instance.btnExPlay,
            btnExMute: instance.btnExMute,
            trkExVolume: instance.trkExVolume,
            btnExCc: instance.btnExCc,
            lblExSpaceHint: instance.lblExSpaceHint,
            lyExTimeline: instance.lyExTimeline,
            pnlExTl: instance.pnlExTl,
            shpExRuler: instance.shpExRuler,
            shpExRulerLine: instance.shpExRulerLine,
            imgExTl: instance.imgExTl,
            pnlExSb: instance.pnlExSb,
            shpExSbThumb: instance.shpExSbThumb,
            lyExTlLabels: instance.lyExTlLabels,
            lyExTicks: instance.lyExTicks,
            shpExPlayShade: instance.shpExPlayShade,
            shpExPlayLine: instance.shpExPlayLine,
            lyExTlBar: instance.lyExTlBar,
            btnExCut: instance.btnExCut,
            btnExUncut: instance.btnExUncut,
            btnExToggle: instance.btnExToggle,
            btnExTlZoomOut: instance.btnExTlZoomOut,
            lblExTlZoom: instance.lblExTlZoom,
            btnExTlZoomIn: instance.btnExTlZoomIn,
            edtExTime: instance.edtExTime,
            lblExRangeLen: instance.lblExRangeLen,
            lblExRangeText: instance.lblExRangeText,
            lblExRangeHint: instance.lblExRangeHint,
            lyExAudio: instance.lyExAudio,
            lblExAudioHead: instance.lblExAudioHead,
            lyExOrigRow: instance.lyExOrigRow,
            pnlExOrig: instance.pnlExOrig,
            chkExOrig: instance.chkExOrig,
            lblExOrig: instance.lblExOrig,
            flwExTracks: instance.flwExTracks,
            lyExMixRow: instance.lyExMixRow,
            pnlExMix: instance.pnlExMix,
            chkExMix: instance.chkExMix,
            lblExMix: instance.lblExMix,
            lyExVideoHead: instance.lyExVideoHead,
            lblExResHead: instance.lblExResHead,
            lblExEncHead: instance.lblExEncHead,
            lyExVideoRow: instance.lyExVideoRow,
            cmbExRes: instance.cmbExRes,
            cmbExEnc: instance.cmbExEnc,
            lblExResHint: instance.lblExResHint,
            lyExScale: instance.lyExScale,
            lblExScaleHead: instance.lblExScaleHead,
            lyExScaleRow: instance.lyExScaleRow,
            cmbExScale: instance.cmbExScale,
            lblExScaleHint: instance.lblExScaleHint,
            lblExSpeedHead: instance.lblExSpeedHead,
            lyExSpeedRow: instance.lyExSpeedRow,
            edtExSpeed: instance.edtExSpeed,
            lblExSpeedX: instance.lblExSpeedX,
            lblExSpeedValue: instance.lblExSpeedValue,
            lblExSpeedHint: instance.lblExSpeedHint,
            lyExFps: instance.lyExFps,
            lblExFpsHead: instance.lblExFpsHead,
            lyExFpsRow: instance.lyExFpsRow,
            lblExFps: instance.lblExFps,
            trkExFps: instance.trkExFps,
            lblExFpsHint: instance.lblExFpsHint,
            lblExQualityHead: instance.lblExQualityHead,
            lyExQualityRow: instance.lyExQualityRow,
            lblExQuality: instance.lblExQuality,
            trkExQuality: instance.trkExQuality,
            lblExQualityHint: instance.lblExQualityHint,
            lblExCaptionHead: instance.lblExCaptionHead,
            lyExBurnRow: instance.lyExBurnRow,
            pnlExBurn: instance.pnlExBurn,
            chkExBurn: instance.chkExBurn,
            lblExBurn: instance.lblExBurn,
            lblExCaptionHint: instance.lblExCaptionHint,
            lblExNameHead: instance.lblExNameHead,
            lyExNameRow: instance.lyExNameRow,
            cmbExCont: instance.cmbExCont,
            edtExName: instance.edtExName,
            lblExContHint: instance.lblExContHint,
            lyExActions: instance.lyExActions,
            btnExRun: instance.btnExRun,
            btnExPreview: instance.btnExPreview,
            cmbExPrevSec: instance.cmbExPrevSec,
            lyExProgress: instance.lyExProgress,
            prgExProgress: instance.prgExProgress,
            lblExProgress: instance.lblExProgress,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do frmExport aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FrmExportComponents> {
        ui.form_components(ui.find_path("frmExport")?)
    }

    /// Cria o frmExport (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, aylis::prelude::Error> {
        ui.create_form("frmExport")
    }

    /// Cria mais um frmExport dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`frmExport`, `frmExport2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FrmExportComponents, aylis::prelude::Error> {
        let root = ui.create_form_instance("frmExport")?;
        let name = ui.free_name_in(parent, "frmExport");
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

    /// Os componentes de uma instância do frmExport (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FrmExportComponents> {
        ui.form_components(root)
    }

    /// O frmExport aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FrmExport> {
        ui.form_instance(ui.find_path("frmExport")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FrmExport> {
        let form = ui.find_path("frmExport")?;
        ui.form_instance_mut(form)
    }

    /// O Button deste form com o nome (`None` se não há um Button com ele).
    pub fn find_button(&self, name: &str) -> Option<Button> {
        match name {
            "btnExClose" => Some(self.btnExClose),
            "btnExCropReset" => Some(self.btnExCropReset),
            "btnExZoomOut" => Some(self.btnExZoomOut),
            "btnExZoomIn" => Some(self.btnExZoomIn),
            "btnExZoomReset" => Some(self.btnExZoomReset),
            "btnExPlay" => Some(self.btnExPlay),
            "btnExMute" => Some(self.btnExMute),
            "btnExCc" => Some(self.btnExCc),
            "btnExCut" => Some(self.btnExCut),
            "btnExUncut" => Some(self.btnExUncut),
            "btnExToggle" => Some(self.btnExToggle),
            "btnExTlZoomOut" => Some(self.btnExTlZoomOut),
            "btnExTlZoomIn" => Some(self.btnExTlZoomIn),
            "btnExRun" => Some(self.btnExRun),
            "btnExPreview" => Some(self.btnExPreview),
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
            "chkExOrig" => Some(self.chkExOrig),
            "chkExMix" => Some(self.chkExMix),
            "chkExBurn" => Some(self.chkExBurn),
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
            "cmbExRes" => Some(self.cmbExRes),
            "cmbExEnc" => Some(self.cmbExEnc),
            "cmbExScale" => Some(self.cmbExScale),
            "cmbExCont" => Some(self.cmbExCont),
            "cmbExPrevSec" => Some(self.cmbExPrevSec),
            _ => None,
        }
    }

    /// O ComboBox que disparou o evento (`None` se o `sender` não é um ComboBox deste form).
    pub fn sender_as_combo_box(&self, ui: &Ui, sender: ComponentId) -> Option<ComboBox> {
        self.find_combo_box(ui.name(sender)?)
    }

    /// O CropBox deste form com o nome (`None` se não há um CropBox com ele).
    pub fn find_crop_box(&self, name: &str) -> Option<CropBox> {
        match name {
            "crpExCrop" => Some(self.crpExCrop),
            _ => None,
        }
    }

    /// O CropBox que disparou o evento (`None` se o `sender` não é um CropBox deste form).
    pub fn sender_as_crop_box(&self, ui: &Ui, sender: ComponentId) -> Option<CropBox> {
        self.find_crop_box(ui.name(sender)?)
    }

    /// O Edit deste form com o nome (`None` se não há um Edit com ele).
    pub fn find_edit(&self, name: &str) -> Option<Edit> {
        match name {
            "edtExTime" => Some(self.edtExTime),
            "edtExSpeed" => Some(self.edtExSpeed),
            "edtExName" => Some(self.edtExName),
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
            "flwExRegions" => Some(self.flwExRegions),
            "flwExTracks" => Some(self.flwExTracks),
            _ => None,
        }
    }

    /// O FlowLayout que disparou o evento (`None` se o `sender` não é um FlowLayout deste form).
    pub fn sender_as_flow_layout(&self, ui: &Ui, sender: ComponentId) -> Option<FlowLayout> {
        self.find_flow_layout(ui.name(sender)?)
    }

    /// O FrameView deste form com o nome (`None` se não há um FrameView com ele).
    pub fn find_frame_view(&self, name: &str) -> Option<FrameView> {
        match name {
            "fvExFrame" => Some(self.fvExFrame),
            _ => None,
        }
    }

    /// O FrameView que disparou o evento (`None` se o `sender` não é um FrameView deste form).
    pub fn sender_as_frame_view(&self, ui: &Ui, sender: ComponentId) -> Option<FrameView> {
        self.find_frame_view(ui.name(sender)?)
    }

    /// O Image deste form com o nome (`None` se não há um Image com ele).
    pub fn find_image(&self, name: &str) -> Option<Image> {
        match name {
            "imgExTl" => Some(self.imgExTl),
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
            "lblExTitle" => Some(self.lblExTitle),
            "lblExRegionsHead" => Some(self.lblExRegionsHead),
            "lblExRegionsHint" => Some(self.lblExRegionsHint),
            "lblExRangeHead" => Some(self.lblExRangeHead),
            "lblExMsg" => Some(self.lblExMsg),
            "lblExAudioOnly" => Some(self.lblExAudioOnly),
            "lblExCaption" => Some(self.lblExCaption),
            "lblExZoom" => Some(self.lblExZoom),
            "lblExCropInfo" => Some(self.lblExCropInfo),
            "lblExCropHint" => Some(self.lblExCropHint),
            "lblExSpaceHint" => Some(self.lblExSpaceHint),
            "lblExTlZoom" => Some(self.lblExTlZoom),
            "lblExRangeLen" => Some(self.lblExRangeLen),
            "lblExRangeText" => Some(self.lblExRangeText),
            "lblExRangeHint" => Some(self.lblExRangeHint),
            "lblExAudioHead" => Some(self.lblExAudioHead),
            "lblExOrig" => Some(self.lblExOrig),
            "lblExMix" => Some(self.lblExMix),
            "lblExResHead" => Some(self.lblExResHead),
            "lblExEncHead" => Some(self.lblExEncHead),
            "lblExResHint" => Some(self.lblExResHint),
            "lblExScaleHead" => Some(self.lblExScaleHead),
            "lblExScaleHint" => Some(self.lblExScaleHint),
            "lblExSpeedHead" => Some(self.lblExSpeedHead),
            "lblExSpeedX" => Some(self.lblExSpeedX),
            "lblExSpeedValue" => Some(self.lblExSpeedValue),
            "lblExSpeedHint" => Some(self.lblExSpeedHint),
            "lblExFpsHead" => Some(self.lblExFpsHead),
            "lblExFps" => Some(self.lblExFps),
            "lblExFpsHint" => Some(self.lblExFpsHint),
            "lblExQualityHead" => Some(self.lblExQualityHead),
            "lblExQuality" => Some(self.lblExQuality),
            "lblExQualityHint" => Some(self.lblExQualityHint),
            "lblExCaptionHead" => Some(self.lblExCaptionHead),
            "lblExBurn" => Some(self.lblExBurn),
            "lblExCaptionHint" => Some(self.lblExCaptionHint),
            "lblExNameHead" => Some(self.lblExNameHead),
            "lblExContHint" => Some(self.lblExContHint),
            "lblExProgress" => Some(self.lblExProgress),
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
            "lyExport" => Some(self.lyExport),
            "lyExCrop" => Some(self.lyExCrop),
            "lyExPlay" => Some(self.lyExPlay),
            "lyExTimeline" => Some(self.lyExTimeline),
            "lyExTlLabels" => Some(self.lyExTlLabels),
            "lyExTicks" => Some(self.lyExTicks),
            "lyExTlBar" => Some(self.lyExTlBar),
            "lyExAudio" => Some(self.lyExAudio),
            "lyExOrigRow" => Some(self.lyExOrigRow),
            "lyExMixRow" => Some(self.lyExMixRow),
            "lyExVideoHead" => Some(self.lyExVideoHead),
            "lyExVideoRow" => Some(self.lyExVideoRow),
            "lyExScale" => Some(self.lyExScale),
            "lyExScaleRow" => Some(self.lyExScaleRow),
            "lyExSpeedRow" => Some(self.lyExSpeedRow),
            "lyExFps" => Some(self.lyExFps),
            "lyExFpsRow" => Some(self.lyExFpsRow),
            "lyExQualityRow" => Some(self.lyExQualityRow),
            "lyExBurnRow" => Some(self.lyExBurnRow),
            "lyExNameRow" => Some(self.lyExNameRow),
            "lyExActions" => Some(self.lyExActions),
            "lyExProgress" => Some(self.lyExProgress),
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
            "pnlExport" => Some(self.pnlExport),
            "pnlExHead" => Some(self.pnlExHead),
            "pnlExStage" => Some(self.pnlExStage),
            "pnlExTl" => Some(self.pnlExTl),
            "pnlExSb" => Some(self.pnlExSb),
            "pnlExOrig" => Some(self.pnlExOrig),
            "pnlExMix" => Some(self.pnlExMix),
            "pnlExBurn" => Some(self.pnlExBurn),
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
            "prgExProgress" => Some(self.prgExProgress),
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
            "lnExHead" => Some(self.lnExHead),
            "shpExRuler" => Some(self.shpExRuler),
            "shpExRulerLine" => Some(self.shpExRulerLine),
            "shpExSbThumb" => Some(self.shpExSbThumb),
            "shpExPlayShade" => Some(self.shpExPlayShade),
            "shpExPlayLine" => Some(self.shpExPlayLine),
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
            "scrExport" => Some(self.scrExport),
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
            "trkExVolume" => Some(self.trkExVolume),
            "trkExFps" => Some(self.trkExFps),
            "trkExQuality" => Some(self.trkExQuality),
            _ => None,
        }
    }

    /// O TrackBar que disparou o evento (`None` se o `sender` não é um TrackBar deste form).
    pub fn sender_as_track_bar(&self, ui: &Ui, sender: ComponentId) -> Option<TrackBar> {
        self.find_track_bar(ui.name(sender)?)
    }
}
