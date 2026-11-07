
/// O form frmSettings: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FrmSettings {
    pub form: Form,
    pub pnlSettings: Panel,
    pub lySetHead: Layout,
    pub lnSetHead: Rectangle,
    pub btnSetBack: Button,
    pub lblSetTitle: Label,
    pub pnlSetSide: Panel,
    pub lnSetSide: Rectangle,
    pub lySetTabs: Layout,
    pub pnlTabGeneral: Panel,
    pub lblTabGeneral: Label,
    pub pnlTabOnRecord: Panel,
    pub lblTabOnRecord: Label,
    pub pnlTabFiles: Panel,
    pub lblTabFiles: Label,
    pub pnlTabVideo: Panel,
    pub lblTabVideo: Label,
    pub pnlTabAudio: Panel,
    pub lblTabAudio: Label,
    pub pnlTabReplay: Panel,
    pub lblTabReplay: Label,
    pub pnlTabHotkeys: Panel,
    pub lblTabHotkeys: Label,
    pub pnlTabDevices: Panel,
    pub lblTabDevices: Label,
    pub pnlTabTranscribe: Panel,
    pub lblTabTranscribe: Label,
    pub pnlTabUpdates: Panel,
    pub lblTabUpdates: Label,
    pub pnlTabAbout: Panel,
    pub lblTabAbout: Label,
    pub scrSettings: ScrollBox,
    pub lySettings: Layout,
    pub tbcSettings: TabControl,
    pub tabGeneral: TabItem,
    pub frGeneral: Frame,
    pub tabOnRecord: TabItem,
    pub frOnRecord: Frame,
    pub tabFiles: TabItem,
    pub frFiles: Frame,
    pub tabVideo: TabItem,
    pub frVideo: Frame,
    pub tabAudio: TabItem,
    pub frAudio: Frame,
    pub tabReplay: TabItem,
    pub frReplay: Frame,
    pub tabHotkeys: TabItem,
    pub frHotkeys: Frame,
    pub tabDevices: TabItem,
    pub frDevices: Frame,
    pub tabTranscribe: TabItem,
    pub frTranscribe: Frame,
    pub tabUpdates: TabItem,
    pub frUpdates: Frame,
    pub tabAbout: TabItem,
    pub frAbout: Frame,
    pub data: super::settings::Data,
}

/// Os componentes do frmSettings sem o estado, para o código fora dos handlers
/// (`FrmSettings::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FrmSettingsComponents {
    pub form: Form,
    pub pnlSettings: Panel,
    pub lySetHead: Layout,
    pub lnSetHead: Rectangle,
    pub btnSetBack: Button,
    pub lblSetTitle: Label,
    pub pnlSetSide: Panel,
    pub lnSetSide: Rectangle,
    pub lySetTabs: Layout,
    pub pnlTabGeneral: Panel,
    pub lblTabGeneral: Label,
    pub pnlTabOnRecord: Panel,
    pub lblTabOnRecord: Label,
    pub pnlTabFiles: Panel,
    pub lblTabFiles: Label,
    pub pnlTabVideo: Panel,
    pub lblTabVideo: Label,
    pub pnlTabAudio: Panel,
    pub lblTabAudio: Label,
    pub pnlTabReplay: Panel,
    pub lblTabReplay: Label,
    pub pnlTabHotkeys: Panel,
    pub lblTabHotkeys: Label,
    pub pnlTabDevices: Panel,
    pub lblTabDevices: Label,
    pub pnlTabTranscribe: Panel,
    pub lblTabTranscribe: Label,
    pub pnlTabUpdates: Panel,
    pub lblTabUpdates: Label,
    pub pnlTabAbout: Panel,
    pub lblTabAbout: Label,
    pub scrSettings: ScrollBox,
    pub lySettings: Layout,
    pub tbcSettings: TabControl,
    pub tabGeneral: TabItem,
    pub frGeneral: Frame,
    pub tabOnRecord: TabItem,
    pub frOnRecord: Frame,
    pub tabFiles: TabItem,
    pub frFiles: Frame,
    pub tabVideo: TabItem,
    pub frVideo: Frame,
    pub tabAudio: TabItem,
    pub frAudio: Frame,
    pub tabReplay: TabItem,
    pub frReplay: Frame,
    pub tabHotkeys: TabItem,
    pub frHotkeys: Frame,
    pub tabDevices: TabItem,
    pub frDevices: Frame,
    pub tabTranscribe: TabItem,
    pub frTranscribe: Frame,
    pub tabUpdates: TabItem,
    pub frUpdates: Frame,
    pub tabAbout: TabItem,
    pub frAbout: Frame,
}

impl std::ops::Deref for FrmSettings {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FrmSettings {
    /// Monta a instância de um frmSettings recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            pnlSettings: Panel { id: handles::resolve(ui, root, "pnlSettings", "Panel", &mut problems) },
            lySetHead: Layout { id: handles::resolve(ui, root, "lySetHead", "Layout", &mut problems) },
            lnSetHead: Rectangle { id: handles::resolve(ui, root, "lnSetHead", "Rectangle", &mut problems) },
            btnSetBack: Button { id: handles::resolve(ui, root, "btnSetBack", "Button", &mut problems) },
            lblSetTitle: Label { id: handles::resolve(ui, root, "lblSetTitle", "Label", &mut problems) },
            pnlSetSide: Panel { id: handles::resolve(ui, root, "pnlSetSide", "Panel", &mut problems) },
            lnSetSide: Rectangle { id: handles::resolve(ui, root, "lnSetSide", "Rectangle", &mut problems) },
            lySetTabs: Layout { id: handles::resolve(ui, root, "lySetTabs", "Layout", &mut problems) },
            pnlTabGeneral: Panel { id: handles::resolve(ui, root, "pnlTabGeneral", "Panel", &mut problems) },
            lblTabGeneral: Label { id: handles::resolve(ui, root, "lblTabGeneral", "Label", &mut problems) },
            pnlTabOnRecord: Panel { id: handles::resolve(ui, root, "pnlTabOnRecord", "Panel", &mut problems) },
            lblTabOnRecord: Label { id: handles::resolve(ui, root, "lblTabOnRecord", "Label", &mut problems) },
            pnlTabFiles: Panel { id: handles::resolve(ui, root, "pnlTabFiles", "Panel", &mut problems) },
            lblTabFiles: Label { id: handles::resolve(ui, root, "lblTabFiles", "Label", &mut problems) },
            pnlTabVideo: Panel { id: handles::resolve(ui, root, "pnlTabVideo", "Panel", &mut problems) },
            lblTabVideo: Label { id: handles::resolve(ui, root, "lblTabVideo", "Label", &mut problems) },
            pnlTabAudio: Panel { id: handles::resolve(ui, root, "pnlTabAudio", "Panel", &mut problems) },
            lblTabAudio: Label { id: handles::resolve(ui, root, "lblTabAudio", "Label", &mut problems) },
            pnlTabReplay: Panel { id: handles::resolve(ui, root, "pnlTabReplay", "Panel", &mut problems) },
            lblTabReplay: Label { id: handles::resolve(ui, root, "lblTabReplay", "Label", &mut problems) },
            pnlTabHotkeys: Panel { id: handles::resolve(ui, root, "pnlTabHotkeys", "Panel", &mut problems) },
            lblTabHotkeys: Label { id: handles::resolve(ui, root, "lblTabHotkeys", "Label", &mut problems) },
            pnlTabDevices: Panel { id: handles::resolve(ui, root, "pnlTabDevices", "Panel", &mut problems) },
            lblTabDevices: Label { id: handles::resolve(ui, root, "lblTabDevices", "Label", &mut problems) },
            pnlTabTranscribe: Panel { id: handles::resolve(ui, root, "pnlTabTranscribe", "Panel", &mut problems) },
            lblTabTranscribe: Label { id: handles::resolve(ui, root, "lblTabTranscribe", "Label", &mut problems) },
            pnlTabUpdates: Panel { id: handles::resolve(ui, root, "pnlTabUpdates", "Panel", &mut problems) },
            lblTabUpdates: Label { id: handles::resolve(ui, root, "lblTabUpdates", "Label", &mut problems) },
            pnlTabAbout: Panel { id: handles::resolve(ui, root, "pnlTabAbout", "Panel", &mut problems) },
            lblTabAbout: Label { id: handles::resolve(ui, root, "lblTabAbout", "Label", &mut problems) },
            scrSettings: ScrollBox { id: handles::resolve(ui, root, "scrSettings", "ScrollBox", &mut problems) },
            lySettings: Layout { id: handles::resolve(ui, root, "lySettings", "Layout", &mut problems) },
            tbcSettings: TabControl { id: handles::resolve(ui, root, "tbcSettings", "TabControl", &mut problems) },
            tabGeneral: TabItem { id: handles::resolve(ui, root, "tabGeneral", "TabItem", &mut problems) },
            frGeneral: Frame { id: handles::resolve(ui, root, "frGeneral", "Frame", &mut problems) },
            tabOnRecord: TabItem { id: handles::resolve(ui, root, "tabOnRecord", "TabItem", &mut problems) },
            frOnRecord: Frame { id: handles::resolve(ui, root, "frOnRecord", "Frame", &mut problems) },
            tabFiles: TabItem { id: handles::resolve(ui, root, "tabFiles", "TabItem", &mut problems) },
            frFiles: Frame { id: handles::resolve(ui, root, "frFiles", "Frame", &mut problems) },
            tabVideo: TabItem { id: handles::resolve(ui, root, "tabVideo", "TabItem", &mut problems) },
            frVideo: Frame { id: handles::resolve(ui, root, "frVideo", "Frame", &mut problems) },
            tabAudio: TabItem { id: handles::resolve(ui, root, "tabAudio", "TabItem", &mut problems) },
            frAudio: Frame { id: handles::resolve(ui, root, "frAudio", "Frame", &mut problems) },
            tabReplay: TabItem { id: handles::resolve(ui, root, "tabReplay", "TabItem", &mut problems) },
            frReplay: Frame { id: handles::resolve(ui, root, "frReplay", "Frame", &mut problems) },
            tabHotkeys: TabItem { id: handles::resolve(ui, root, "tabHotkeys", "TabItem", &mut problems) },
            frHotkeys: Frame { id: handles::resolve(ui, root, "frHotkeys", "Frame", &mut problems) },
            tabDevices: TabItem { id: handles::resolve(ui, root, "tabDevices", "TabItem", &mut problems) },
            frDevices: Frame { id: handles::resolve(ui, root, "frDevices", "Frame", &mut problems) },
            tabTranscribe: TabItem { id: handles::resolve(ui, root, "tabTranscribe", "TabItem", &mut problems) },
            frTranscribe: Frame { id: handles::resolve(ui, root, "frTranscribe", "Frame", &mut problems) },
            tabUpdates: TabItem { id: handles::resolve(ui, root, "tabUpdates", "TabItem", &mut problems) },
            frUpdates: Frame { id: handles::resolve(ui, root, "frUpdates", "Frame", &mut problems) },
            tabAbout: TabItem { id: handles::resolve(ui, root, "tabAbout", "TabItem", &mut problems) },
            frAbout: Frame { id: handles::resolve(ui, root, "frAbout", "Frame", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form frmSettings não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FrmSettingsComponents {
            form: instance.form,
            pnlSettings: instance.pnlSettings,
            lySetHead: instance.lySetHead,
            lnSetHead: instance.lnSetHead,
            btnSetBack: instance.btnSetBack,
            lblSetTitle: instance.lblSetTitle,
            pnlSetSide: instance.pnlSetSide,
            lnSetSide: instance.lnSetSide,
            lySetTabs: instance.lySetTabs,
            pnlTabGeneral: instance.pnlTabGeneral,
            lblTabGeneral: instance.lblTabGeneral,
            pnlTabOnRecord: instance.pnlTabOnRecord,
            lblTabOnRecord: instance.lblTabOnRecord,
            pnlTabFiles: instance.pnlTabFiles,
            lblTabFiles: instance.lblTabFiles,
            pnlTabVideo: instance.pnlTabVideo,
            lblTabVideo: instance.lblTabVideo,
            pnlTabAudio: instance.pnlTabAudio,
            lblTabAudio: instance.lblTabAudio,
            pnlTabReplay: instance.pnlTabReplay,
            lblTabReplay: instance.lblTabReplay,
            pnlTabHotkeys: instance.pnlTabHotkeys,
            lblTabHotkeys: instance.lblTabHotkeys,
            pnlTabDevices: instance.pnlTabDevices,
            lblTabDevices: instance.lblTabDevices,
            pnlTabTranscribe: instance.pnlTabTranscribe,
            lblTabTranscribe: instance.lblTabTranscribe,
            pnlTabUpdates: instance.pnlTabUpdates,
            lblTabUpdates: instance.lblTabUpdates,
            pnlTabAbout: instance.pnlTabAbout,
            lblTabAbout: instance.lblTabAbout,
            scrSettings: instance.scrSettings,
            lySettings: instance.lySettings,
            tbcSettings: instance.tbcSettings,
            tabGeneral: instance.tabGeneral,
            frGeneral: instance.frGeneral,
            tabOnRecord: instance.tabOnRecord,
            frOnRecord: instance.frOnRecord,
            tabFiles: instance.tabFiles,
            frFiles: instance.frFiles,
            tabVideo: instance.tabVideo,
            frVideo: instance.frVideo,
            tabAudio: instance.tabAudio,
            frAudio: instance.frAudio,
            tabReplay: instance.tabReplay,
            frReplay: instance.frReplay,
            tabHotkeys: instance.tabHotkeys,
            frHotkeys: instance.frHotkeys,
            tabDevices: instance.tabDevices,
            frDevices: instance.frDevices,
            tabTranscribe: instance.tabTranscribe,
            frTranscribe: instance.frTranscribe,
            tabUpdates: instance.tabUpdates,
            frUpdates: instance.frUpdates,
            tabAbout: instance.tabAbout,
            frAbout: instance.frAbout,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do frmSettings aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FrmSettingsComponents> {
        ui.form_components(ui.find_path("frmSettings")?)
    }

    /// Cria o frmSettings (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, iron::prelude::Error> {
        ui.create_form("frmSettings")
    }

    /// Cria mais um frmSettings dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`frmSettings`, `frmSettings2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FrmSettingsComponents, iron::prelude::Error> {
        let root = ui.create_form_instance("frmSettings")?;
        let name = ui.free_name_in(parent, "frmSettings");
        if let Err(e) = ui.set_name(root, &name).and_then(|_| ui.set_parent(root, Some(parent))) {
            ui.free(root);
            return Err(e);
        }
        ui.form_components(root).ok_or(iron::prelude::Error::InvalidId)
    }

    /// Os componentes de uma instância do frmSettings (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FrmSettingsComponents> {
        ui.form_components(root)
    }

    /// O frmSettings aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FrmSettings> {
        ui.form_instance(ui.find_path("frmSettings")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FrmSettings> {
        let form = ui.find_path("frmSettings")?;
        ui.form_instance_mut(form)
    }

    /// O Button deste form com o nome (`None` se não há um Button com ele).
    pub fn find_button(&self, name: &str) -> Option<Button> {
        match name {
            "btnSetBack" => Some(self.btnSetBack),
            _ => None,
        }
    }

    /// O Button que disparou o evento (`None` se o `sender` não é um Button deste form).
    pub fn sender_as_button(&self, ui: &Ui, sender: ComponentId) -> Option<Button> {
        self.find_button(ui.name(sender)?)
    }

    /// O Frame deste form com o nome (`None` se não há um Frame com ele).
    pub fn find_frame(&self, name: &str) -> Option<Frame> {
        match name {
            "frGeneral" => Some(self.frGeneral),
            "frOnRecord" => Some(self.frOnRecord),
            "frFiles" => Some(self.frFiles),
            "frVideo" => Some(self.frVideo),
            "frAudio" => Some(self.frAudio),
            "frReplay" => Some(self.frReplay),
            "frHotkeys" => Some(self.frHotkeys),
            "frDevices" => Some(self.frDevices),
            "frTranscribe" => Some(self.frTranscribe),
            "frUpdates" => Some(self.frUpdates),
            "frAbout" => Some(self.frAbout),
            _ => None,
        }
    }

    /// O Frame que disparou o evento (`None` se o `sender` não é um Frame deste form).
    pub fn sender_as_frame(&self, ui: &Ui, sender: ComponentId) -> Option<Frame> {
        self.find_frame(ui.name(sender)?)
    }

    /// O Label deste form com o nome (`None` se não há um Label com ele).
    pub fn find_label(&self, name: &str) -> Option<Label> {
        match name {
            "lblSetTitle" => Some(self.lblSetTitle),
            "lblTabGeneral" => Some(self.lblTabGeneral),
            "lblTabOnRecord" => Some(self.lblTabOnRecord),
            "lblTabFiles" => Some(self.lblTabFiles),
            "lblTabVideo" => Some(self.lblTabVideo),
            "lblTabAudio" => Some(self.lblTabAudio),
            "lblTabReplay" => Some(self.lblTabReplay),
            "lblTabHotkeys" => Some(self.lblTabHotkeys),
            "lblTabDevices" => Some(self.lblTabDevices),
            "lblTabTranscribe" => Some(self.lblTabTranscribe),
            "lblTabUpdates" => Some(self.lblTabUpdates),
            "lblTabAbout" => Some(self.lblTabAbout),
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
            "lySetHead" => Some(self.lySetHead),
            "lySetTabs" => Some(self.lySetTabs),
            "lySettings" => Some(self.lySettings),
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
            "pnlSettings" => Some(self.pnlSettings),
            "pnlSetSide" => Some(self.pnlSetSide),
            "pnlTabGeneral" => Some(self.pnlTabGeneral),
            "pnlTabOnRecord" => Some(self.pnlTabOnRecord),
            "pnlTabFiles" => Some(self.pnlTabFiles),
            "pnlTabVideo" => Some(self.pnlTabVideo),
            "pnlTabAudio" => Some(self.pnlTabAudio),
            "pnlTabReplay" => Some(self.pnlTabReplay),
            "pnlTabHotkeys" => Some(self.pnlTabHotkeys),
            "pnlTabDevices" => Some(self.pnlTabDevices),
            "pnlTabTranscribe" => Some(self.pnlTabTranscribe),
            "pnlTabUpdates" => Some(self.pnlTabUpdates),
            "pnlTabAbout" => Some(self.pnlTabAbout),
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
            "lnSetHead" => Some(self.lnSetHead),
            "lnSetSide" => Some(self.lnSetSide),
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
            "scrSettings" => Some(self.scrSettings),
            _ => None,
        }
    }

    /// O ScrollBox que disparou o evento (`None` se o `sender` não é um ScrollBox deste form).
    pub fn sender_as_scroll_box(&self, ui: &Ui, sender: ComponentId) -> Option<ScrollBox> {
        self.find_scroll_box(ui.name(sender)?)
    }

    /// O TabControl deste form com o nome (`None` se não há um TabControl com ele).
    pub fn find_tab_control(&self, name: &str) -> Option<TabControl> {
        match name {
            "tbcSettings" => Some(self.tbcSettings),
            _ => None,
        }
    }

    /// O TabControl que disparou o evento (`None` se o `sender` não é um TabControl deste form).
    pub fn sender_as_tab_control(&self, ui: &Ui, sender: ComponentId) -> Option<TabControl> {
        self.find_tab_control(ui.name(sender)?)
    }

    /// O TabItem deste form com o nome (`None` se não há um TabItem com ele).
    pub fn find_tab_item(&self, name: &str) -> Option<TabItem> {
        match name {
            "tabGeneral" => Some(self.tabGeneral),
            "tabOnRecord" => Some(self.tabOnRecord),
            "tabFiles" => Some(self.tabFiles),
            "tabVideo" => Some(self.tabVideo),
            "tabAudio" => Some(self.tabAudio),
            "tabReplay" => Some(self.tabReplay),
            "tabHotkeys" => Some(self.tabHotkeys),
            "tabDevices" => Some(self.tabDevices),
            "tabTranscribe" => Some(self.tabTranscribe),
            "tabUpdates" => Some(self.tabUpdates),
            "tabAbout" => Some(self.tabAbout),
            _ => None,
        }
    }

    /// O TabItem que disparou o evento (`None` se o `sender` não é um TabItem deste form).
    pub fn sender_as_tab_item(&self, ui: &Ui, sender: ComponentId) -> Option<TabItem> {
        self.find_tab_item(ui.name(sender)?)
    }
}
