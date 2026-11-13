
/// O form frmViewer: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FrmViewer {
    pub form: Form,
    pub pnlViewer: Panel,
    pub lyViewStage: Layout,
    pub imgView: Image,
    pub lyViewChrome: Layout,
    pub pnlViewHead: Panel,
    pub btnViewClose: Button,
    pub lblViewTitle: Label,
    pub pnlViewBar: Panel,
    pub btnViewPrev: Button,
    pub lblViewPos: Label,
    pub btnViewNext: Button,
    pub btnViewCopy: Button,
    pub btnViewFit: Button,
    pub btnView100: Button,
    pub btnViewIn: Button,
    pub lblViewZoom: Label,
    pub btnViewOut: Button,
    pub lblViewInfo: Label,
    pub data: super::viewer::Data,
}

/// Os componentes do frmViewer sem o estado, para o código fora dos handlers
/// (`FrmViewer::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FrmViewerComponents {
    pub form: Form,
    pub pnlViewer: Panel,
    pub lyViewStage: Layout,
    pub imgView: Image,
    pub lyViewChrome: Layout,
    pub pnlViewHead: Panel,
    pub btnViewClose: Button,
    pub lblViewTitle: Label,
    pub pnlViewBar: Panel,
    pub btnViewPrev: Button,
    pub lblViewPos: Label,
    pub btnViewNext: Button,
    pub btnViewCopy: Button,
    pub btnViewFit: Button,
    pub btnView100: Button,
    pub btnViewIn: Button,
    pub lblViewZoom: Label,
    pub btnViewOut: Button,
    pub lblViewInfo: Label,
}

impl std::ops::Deref for FrmViewer {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FrmViewer {
    /// Monta a instância de um frmViewer recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            pnlViewer: Panel { id: handles::resolve(ui, root, "pnlViewer", "Panel", &mut problems) },
            lyViewStage: Layout { id: handles::resolve(ui, root, "lyViewStage", "Layout", &mut problems) },
            imgView: Image { id: handles::resolve(ui, root, "imgView", "Image", &mut problems) },
            lyViewChrome: Layout { id: handles::resolve(ui, root, "lyViewChrome", "Layout", &mut problems) },
            pnlViewHead: Panel { id: handles::resolve(ui, root, "pnlViewHead", "Panel", &mut problems) },
            btnViewClose: Button { id: handles::resolve(ui, root, "btnViewClose", "Button", &mut problems) },
            lblViewTitle: Label { id: handles::resolve(ui, root, "lblViewTitle", "Label", &mut problems) },
            pnlViewBar: Panel { id: handles::resolve(ui, root, "pnlViewBar", "Panel", &mut problems) },
            btnViewPrev: Button { id: handles::resolve(ui, root, "btnViewPrev", "Button", &mut problems) },
            lblViewPos: Label { id: handles::resolve(ui, root, "lblViewPos", "Label", &mut problems) },
            btnViewNext: Button { id: handles::resolve(ui, root, "btnViewNext", "Button", &mut problems) },
            btnViewCopy: Button { id: handles::resolve(ui, root, "btnViewCopy", "Button", &mut problems) },
            btnViewFit: Button { id: handles::resolve(ui, root, "btnViewFit", "Button", &mut problems) },
            btnView100: Button { id: handles::resolve(ui, root, "btnView100", "Button", &mut problems) },
            btnViewIn: Button { id: handles::resolve(ui, root, "btnViewIn", "Button", &mut problems) },
            lblViewZoom: Label { id: handles::resolve(ui, root, "lblViewZoom", "Label", &mut problems) },
            btnViewOut: Button { id: handles::resolve(ui, root, "btnViewOut", "Button", &mut problems) },
            lblViewInfo: Label { id: handles::resolve(ui, root, "lblViewInfo", "Label", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form frmViewer não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FrmViewerComponents {
            form: instance.form,
            pnlViewer: instance.pnlViewer,
            lyViewStage: instance.lyViewStage,
            imgView: instance.imgView,
            lyViewChrome: instance.lyViewChrome,
            pnlViewHead: instance.pnlViewHead,
            btnViewClose: instance.btnViewClose,
            lblViewTitle: instance.lblViewTitle,
            pnlViewBar: instance.pnlViewBar,
            btnViewPrev: instance.btnViewPrev,
            lblViewPos: instance.lblViewPos,
            btnViewNext: instance.btnViewNext,
            btnViewCopy: instance.btnViewCopy,
            btnViewFit: instance.btnViewFit,
            btnView100: instance.btnView100,
            btnViewIn: instance.btnViewIn,
            lblViewZoom: instance.lblViewZoom,
            btnViewOut: instance.btnViewOut,
            lblViewInfo: instance.lblViewInfo,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do frmViewer aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FrmViewerComponents> {
        ui.form_components(ui.find_path("frmViewer")?)
    }

    /// Cria o frmViewer (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, aylis::prelude::Error> {
        ui.create_form("frmViewer")
    }

    /// Cria mais um frmViewer dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`frmViewer`, `frmViewer2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FrmViewerComponents, aylis::prelude::Error> {
        let root = ui.create_form_instance("frmViewer")?;
        let name = ui.free_name_in(parent, "frmViewer");
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

    /// Os componentes de uma instância do frmViewer (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FrmViewerComponents> {
        ui.form_components(root)
    }

    /// O frmViewer aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FrmViewer> {
        ui.form_instance(ui.find_path("frmViewer")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FrmViewer> {
        let form = ui.find_path("frmViewer")?;
        ui.form_instance_mut(form)
    }

    /// O Button deste form com o nome (`None` se não há um Button com ele).
    pub fn find_button(&self, name: &str) -> Option<Button> {
        match name {
            "btnViewClose" => Some(self.btnViewClose),
            "btnViewPrev" => Some(self.btnViewPrev),
            "btnViewNext" => Some(self.btnViewNext),
            "btnViewCopy" => Some(self.btnViewCopy),
            "btnViewFit" => Some(self.btnViewFit),
            "btnView100" => Some(self.btnView100),
            "btnViewIn" => Some(self.btnViewIn),
            "btnViewOut" => Some(self.btnViewOut),
            _ => None,
        }
    }

    /// O Button que disparou o evento (`None` se o `sender` não é um Button deste form).
    pub fn sender_as_button(&self, ui: &Ui, sender: ComponentId) -> Option<Button> {
        self.find_button(ui.name(sender)?)
    }

    /// O Image deste form com o nome (`None` se não há um Image com ele).
    pub fn find_image(&self, name: &str) -> Option<Image> {
        match name {
            "imgView" => Some(self.imgView),
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
            "lblViewTitle" => Some(self.lblViewTitle),
            "lblViewPos" => Some(self.lblViewPos),
            "lblViewZoom" => Some(self.lblViewZoom),
            "lblViewInfo" => Some(self.lblViewInfo),
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
            "lyViewStage" => Some(self.lyViewStage),
            "lyViewChrome" => Some(self.lyViewChrome),
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
            "pnlViewer" => Some(self.pnlViewer),
            "pnlViewHead" => Some(self.pnlViewHead),
            "pnlViewBar" => Some(self.pnlViewBar),
            _ => None,
        }
    }

    /// O Panel que disparou o evento (`None` se o `sender` não é um Panel deste form).
    pub fn sender_as_panel(&self, ui: &Ui, sender: ComponentId) -> Option<Panel> {
        self.find_panel(ui.name(sender)?)
    }
}
