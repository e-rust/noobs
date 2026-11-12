
/// O form fraTrackCard: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FraTrackCard {
    pub form: Form,
    pub pnlCard: Panel,
    pub lblTitle: Label,
    pub lyRows: Layout,
    pub recLine: Rectangle,
    pub lyVolume: Layout,
    pub btnMute: Button,
    pub lblPct: Label,
    pub trkVolume: TrackBar,
    pub data: super::player_track_card::Data,
}

/// Os componentes do fraTrackCard sem o estado, para o código fora dos handlers
/// (`FraTrackCard::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FraTrackCardComponents {
    pub form: Form,
    pub pnlCard: Panel,
    pub lblTitle: Label,
    pub lyRows: Layout,
    pub recLine: Rectangle,
    pub lyVolume: Layout,
    pub btnMute: Button,
    pub lblPct: Label,
    pub trkVolume: TrackBar,
}

impl std::ops::Deref for FraTrackCard {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FraTrackCard {
    /// Monta a instância de um fraTrackCard recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            pnlCard: Panel { id: handles::resolve(ui, root, "pnlCard", "Panel", &mut problems) },
            lblTitle: Label { id: handles::resolve(ui, root, "lblTitle", "Label", &mut problems) },
            lyRows: Layout { id: handles::resolve(ui, root, "lyRows", "Layout", &mut problems) },
            recLine: Rectangle { id: handles::resolve(ui, root, "recLine", "Rectangle", &mut problems) },
            lyVolume: Layout { id: handles::resolve(ui, root, "lyVolume", "Layout", &mut problems) },
            btnMute: Button { id: handles::resolve(ui, root, "btnMute", "Button", &mut problems) },
            lblPct: Label { id: handles::resolve(ui, root, "lblPct", "Label", &mut problems) },
            trkVolume: TrackBar { id: handles::resolve(ui, root, "trkVolume", "TrackBar", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form fraTrackCard não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FraTrackCardComponents {
            form: instance.form,
            pnlCard: instance.pnlCard,
            lblTitle: instance.lblTitle,
            lyRows: instance.lyRows,
            recLine: instance.recLine,
            lyVolume: instance.lyVolume,
            btnMute: instance.btnMute,
            lblPct: instance.lblPct,
            trkVolume: instance.trkVolume,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do fraTrackCard aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FraTrackCardComponents> {
        ui.form_components(ui.find_path("fraTrackCard")?)
    }

    /// Cria o fraTrackCard (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, aylis::prelude::Error> {
        ui.create_form("fraTrackCard")
    }

    /// Cria mais um fraTrackCard dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`fraTrackCard`, `fraTrackCard2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FraTrackCardComponents, aylis::prelude::Error> {
        let root = ui.create_form_instance("fraTrackCard")?;
        let name = ui.free_name_in(parent, "fraTrackCard");
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

    /// Os componentes de uma instância do fraTrackCard (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FraTrackCardComponents> {
        ui.form_components(root)
    }

    /// O fraTrackCard aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FraTrackCard> {
        ui.form_instance(ui.find_path("fraTrackCard")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FraTrackCard> {
        let form = ui.find_path("fraTrackCard")?;
        ui.form_instance_mut(form)
    }

    /// O Button deste form com o nome (`None` se não há um Button com ele).
    pub fn find_button(&self, name: &str) -> Option<Button> {
        match name {
            "btnMute" => Some(self.btnMute),
            _ => None,
        }
    }

    /// O Button que disparou o evento (`None` se o `sender` não é um Button deste form).
    pub fn sender_as_button(&self, ui: &Ui, sender: ComponentId) -> Option<Button> {
        self.find_button(ui.name(sender)?)
    }

    /// O Label deste form com o nome (`None` se não há um Label com ele).
    pub fn find_label(&self, name: &str) -> Option<Label> {
        match name {
            "lblTitle" => Some(self.lblTitle),
            "lblPct" => Some(self.lblPct),
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
            "lyRows" => Some(self.lyRows),
            "lyVolume" => Some(self.lyVolume),
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
            "pnlCard" => Some(self.pnlCard),
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
            "recLine" => Some(self.recLine),
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
            "trkVolume" => Some(self.trkVolume),
            _ => None,
        }
    }

    /// O TrackBar que disparou o evento (`None` se o `sender` não é um TrackBar deste form).
    pub fn sender_as_track_bar(&self, ui: &Ui, sender: ComponentId) -> Option<TrackBar> {
        self.find_track_bar(ui.name(sender)?)
    }
}
