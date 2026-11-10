
/// O form frmPicker: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FrmPicker {
    pub form: Form,
    pub pnlPicker: Panel,
    pub pnlPickerDialog: Panel,
    pub lyPickerHeader: Layout,
    pub btnPickerClose: Button,
    pub btnPickerRefresh: Button,
    pub lyPickerTitles: Layout,
    pub lblPickerTitle: Label,
    pub lblPickerSubtitle: Label,
    pub lnPicker: Rectangle,
    pub lyPickerMode: Layout,
    pub segPickerMode: SegmentedControl,
    pub lyPickerWindow: Layout,
    pub lyPickerTools: Layout,
    pub chipAppAudio: Chip,
    pub edPickerSearch: Edit,
    pub lblPickerEmpty: Label,
    pub scrPickerGrid: ScrollBox,
    pub flowPickerGrid: FlowLayout,
    pub lyPickerArea: Layout,
    pub lblAreaStep1: Label,
    pub lblAreaStep2: Label,
    pub lyAreaOptions: FlowLayout,
    pub chipTimelapse: Chip,
    pub chipTimelapseCursor: Chip,
    pub lyAreaButtons: FlowLayout,
    pub btnSelectArea: Button,
    pub data: super::picker::Data,
}

/// Os componentes do frmPicker sem o estado, para o código fora dos handlers
/// (`FrmPicker::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FrmPickerComponents {
    pub form: Form,
    pub pnlPicker: Panel,
    pub pnlPickerDialog: Panel,
    pub lyPickerHeader: Layout,
    pub btnPickerClose: Button,
    pub btnPickerRefresh: Button,
    pub lyPickerTitles: Layout,
    pub lblPickerTitle: Label,
    pub lblPickerSubtitle: Label,
    pub lnPicker: Rectangle,
    pub lyPickerMode: Layout,
    pub segPickerMode: SegmentedControl,
    pub lyPickerWindow: Layout,
    pub lyPickerTools: Layout,
    pub chipAppAudio: Chip,
    pub edPickerSearch: Edit,
    pub lblPickerEmpty: Label,
    pub scrPickerGrid: ScrollBox,
    pub flowPickerGrid: FlowLayout,
    pub lyPickerArea: Layout,
    pub lblAreaStep1: Label,
    pub lblAreaStep2: Label,
    pub lyAreaOptions: FlowLayout,
    pub chipTimelapse: Chip,
    pub chipTimelapseCursor: Chip,
    pub lyAreaButtons: FlowLayout,
    pub btnSelectArea: Button,
}

impl std::ops::Deref for FrmPicker {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FrmPicker {
    /// Monta a instância de um frmPicker recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            pnlPicker: Panel { id: handles::resolve(ui, root, "pnlPicker", "Panel", &mut problems) },
            pnlPickerDialog: Panel { id: handles::resolve(ui, root, "pnlPickerDialog", "Panel", &mut problems) },
            lyPickerHeader: Layout { id: handles::resolve(ui, root, "lyPickerHeader", "Layout", &mut problems) },
            btnPickerClose: Button { id: handles::resolve(ui, root, "btnPickerClose", "Button", &mut problems) },
            btnPickerRefresh: Button { id: handles::resolve(ui, root, "btnPickerRefresh", "Button", &mut problems) },
            lyPickerTitles: Layout { id: handles::resolve(ui, root, "lyPickerTitles", "Layout", &mut problems) },
            lblPickerTitle: Label { id: handles::resolve(ui, root, "lblPickerTitle", "Label", &mut problems) },
            lblPickerSubtitle: Label { id: handles::resolve(ui, root, "lblPickerSubtitle", "Label", &mut problems) },
            lnPicker: Rectangle { id: handles::resolve(ui, root, "lnPicker", "Rectangle", &mut problems) },
            lyPickerMode: Layout { id: handles::resolve(ui, root, "lyPickerMode", "Layout", &mut problems) },
            segPickerMode: SegmentedControl { id: handles::resolve(ui, root, "segPickerMode", "SegmentedControl", &mut problems) },
            lyPickerWindow: Layout { id: handles::resolve(ui, root, "lyPickerWindow", "Layout", &mut problems) },
            lyPickerTools: Layout { id: handles::resolve(ui, root, "lyPickerTools", "Layout", &mut problems) },
            chipAppAudio: Chip { id: handles::resolve(ui, root, "chipAppAudio", "Chip", &mut problems) },
            edPickerSearch: Edit { id: handles::resolve(ui, root, "edPickerSearch", "Edit", &mut problems) },
            lblPickerEmpty: Label { id: handles::resolve(ui, root, "lblPickerEmpty", "Label", &mut problems) },
            scrPickerGrid: ScrollBox { id: handles::resolve(ui, root, "scrPickerGrid", "ScrollBox", &mut problems) },
            flowPickerGrid: FlowLayout { id: handles::resolve(ui, root, "flowPickerGrid", "FlowLayout", &mut problems) },
            lyPickerArea: Layout { id: handles::resolve(ui, root, "lyPickerArea", "Layout", &mut problems) },
            lblAreaStep1: Label { id: handles::resolve(ui, root, "lblAreaStep1", "Label", &mut problems) },
            lblAreaStep2: Label { id: handles::resolve(ui, root, "lblAreaStep2", "Label", &mut problems) },
            lyAreaOptions: FlowLayout { id: handles::resolve(ui, root, "lyAreaOptions", "FlowLayout", &mut problems) },
            chipTimelapse: Chip { id: handles::resolve(ui, root, "chipTimelapse", "Chip", &mut problems) },
            chipTimelapseCursor: Chip { id: handles::resolve(ui, root, "chipTimelapseCursor", "Chip", &mut problems) },
            lyAreaButtons: FlowLayout { id: handles::resolve(ui, root, "lyAreaButtons", "FlowLayout", &mut problems) },
            btnSelectArea: Button { id: handles::resolve(ui, root, "btnSelectArea", "Button", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form frmPicker não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FrmPickerComponents {
            form: instance.form,
            pnlPicker: instance.pnlPicker,
            pnlPickerDialog: instance.pnlPickerDialog,
            lyPickerHeader: instance.lyPickerHeader,
            btnPickerClose: instance.btnPickerClose,
            btnPickerRefresh: instance.btnPickerRefresh,
            lyPickerTitles: instance.lyPickerTitles,
            lblPickerTitle: instance.lblPickerTitle,
            lblPickerSubtitle: instance.lblPickerSubtitle,
            lnPicker: instance.lnPicker,
            lyPickerMode: instance.lyPickerMode,
            segPickerMode: instance.segPickerMode,
            lyPickerWindow: instance.lyPickerWindow,
            lyPickerTools: instance.lyPickerTools,
            chipAppAudio: instance.chipAppAudio,
            edPickerSearch: instance.edPickerSearch,
            lblPickerEmpty: instance.lblPickerEmpty,
            scrPickerGrid: instance.scrPickerGrid,
            flowPickerGrid: instance.flowPickerGrid,
            lyPickerArea: instance.lyPickerArea,
            lblAreaStep1: instance.lblAreaStep1,
            lblAreaStep2: instance.lblAreaStep2,
            lyAreaOptions: instance.lyAreaOptions,
            chipTimelapse: instance.chipTimelapse,
            chipTimelapseCursor: instance.chipTimelapseCursor,
            lyAreaButtons: instance.lyAreaButtons,
            btnSelectArea: instance.btnSelectArea,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do frmPicker aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FrmPickerComponents> {
        ui.form_components(ui.find_path("frmPicker")?)
    }

    /// Cria o frmPicker (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, aylis::prelude::Error> {
        ui.create_form("frmPicker")
    }

    /// Cria mais um frmPicker dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`frmPicker`, `frmPicker2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FrmPickerComponents, aylis::prelude::Error> {
        let root = ui.create_form_instance("frmPicker")?;
        let name = ui.free_name_in(parent, "frmPicker");
        if let Err(e) = ui.set_name(root, &name).and_then(|_| ui.set_parent(root, Some(parent))) {
            ui.free(root);
            return Err(e);
        }
        ui.form_components(root).ok_or(aylis::prelude::Error::InvalidId)
    }

    /// Os componentes de uma instância do frmPicker (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FrmPickerComponents> {
        ui.form_components(root)
    }

    /// O frmPicker aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FrmPicker> {
        ui.form_instance(ui.find_path("frmPicker")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FrmPicker> {
        let form = ui.find_path("frmPicker")?;
        ui.form_instance_mut(form)
    }

    /// O Button deste form com o nome (`None` se não há um Button com ele).
    pub fn find_button(&self, name: &str) -> Option<Button> {
        match name {
            "btnPickerClose" => Some(self.btnPickerClose),
            "btnPickerRefresh" => Some(self.btnPickerRefresh),
            "btnSelectArea" => Some(self.btnSelectArea),
            _ => None,
        }
    }

    /// O Button que disparou o evento (`None` se o `sender` não é um Button deste form).
    pub fn sender_as_button(&self, ui: &Ui, sender: ComponentId) -> Option<Button> {
        self.find_button(ui.name(sender)?)
    }

    /// O Chip deste form com o nome (`None` se não há um Chip com ele).
    pub fn find_chip(&self, name: &str) -> Option<Chip> {
        match name {
            "chipAppAudio" => Some(self.chipAppAudio),
            "chipTimelapse" => Some(self.chipTimelapse),
            "chipTimelapseCursor" => Some(self.chipTimelapseCursor),
            _ => None,
        }
    }

    /// O Chip que disparou o evento (`None` se o `sender` não é um Chip deste form).
    pub fn sender_as_chip(&self, ui: &Ui, sender: ComponentId) -> Option<Chip> {
        self.find_chip(ui.name(sender)?)
    }

    /// O Edit deste form com o nome (`None` se não há um Edit com ele).
    pub fn find_edit(&self, name: &str) -> Option<Edit> {
        match name {
            "edPickerSearch" => Some(self.edPickerSearch),
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
            "flowPickerGrid" => Some(self.flowPickerGrid),
            "lyAreaOptions" => Some(self.lyAreaOptions),
            "lyAreaButtons" => Some(self.lyAreaButtons),
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
            "lblPickerTitle" => Some(self.lblPickerTitle),
            "lblPickerSubtitle" => Some(self.lblPickerSubtitle),
            "lblPickerEmpty" => Some(self.lblPickerEmpty),
            "lblAreaStep1" => Some(self.lblAreaStep1),
            "lblAreaStep2" => Some(self.lblAreaStep2),
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
            "lyPickerHeader" => Some(self.lyPickerHeader),
            "lyPickerTitles" => Some(self.lyPickerTitles),
            "lyPickerMode" => Some(self.lyPickerMode),
            "lyPickerWindow" => Some(self.lyPickerWindow),
            "lyPickerTools" => Some(self.lyPickerTools),
            "lyPickerArea" => Some(self.lyPickerArea),
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
            "pnlPicker" => Some(self.pnlPicker),
            "pnlPickerDialog" => Some(self.pnlPickerDialog),
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
            "lnPicker" => Some(self.lnPicker),
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
            "scrPickerGrid" => Some(self.scrPickerGrid),
            _ => None,
        }
    }

    /// O ScrollBox que disparou o evento (`None` se o `sender` não é um ScrollBox deste form).
    pub fn sender_as_scroll_box(&self, ui: &Ui, sender: ComponentId) -> Option<ScrollBox> {
        self.find_scroll_box(ui.name(sender)?)
    }

    /// O SegmentedControl deste form com o nome (`None` se não há um SegmentedControl com ele).
    pub fn find_segmented_control(&self, name: &str) -> Option<SegmentedControl> {
        match name {
            "segPickerMode" => Some(self.segPickerMode),
            _ => None,
        }
    }

    /// O SegmentedControl que disparou o evento (`None` se o `sender` não é um SegmentedControl deste form).
    pub fn sender_as_segmented_control(&self, ui: &Ui, sender: ComponentId) -> Option<SegmentedControl> {
        self.find_segmented_control(ui.name(sender)?)
    }
}
