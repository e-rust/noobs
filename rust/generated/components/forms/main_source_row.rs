
/// O form fraSourceRow: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FraSourceRow {
    pub form: Form,
    pub pnlRow: Panel,
    pub shpStripe: Rectangle,
    pub swOn: Switch,
    pub lyBody: Layout,
    pub lyLine: Layout,
    pub dotDefault: Circle,
    pub icoBtMic: Icon,
    pub icoBtSpeaker: Icon,
    pub lblName: Label,
    pub mtrLevel: LevelMeter,
    pub data: super::main_source_row::Data,
}

/// Os componentes do fraSourceRow sem o estado, para o código fora dos handlers
/// (`FraSourceRow::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FraSourceRowComponents {
    pub form: Form,
    pub pnlRow: Panel,
    pub shpStripe: Rectangle,
    pub swOn: Switch,
    pub lyBody: Layout,
    pub lyLine: Layout,
    pub dotDefault: Circle,
    pub icoBtMic: Icon,
    pub icoBtSpeaker: Icon,
    pub lblName: Label,
    pub mtrLevel: LevelMeter,
}

impl std::ops::Deref for FraSourceRow {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FraSourceRow {
    /// Monta a instância de um fraSourceRow recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            pnlRow: Panel { id: handles::resolve(ui, root, "pnlRow", "Panel", &mut problems) },
            shpStripe: Rectangle { id: handles::resolve(ui, root, "shpStripe", "Rectangle", &mut problems) },
            swOn: Switch { id: handles::resolve(ui, root, "swOn", "Switch", &mut problems) },
            lyBody: Layout { id: handles::resolve(ui, root, "lyBody", "Layout", &mut problems) },
            lyLine: Layout { id: handles::resolve(ui, root, "lyLine", "Layout", &mut problems) },
            dotDefault: Circle { id: handles::resolve(ui, root, "dotDefault", "Circle", &mut problems) },
            icoBtMic: Icon { id: handles::resolve(ui, root, "icoBtMic", "Icon", &mut problems) },
            icoBtSpeaker: Icon { id: handles::resolve(ui, root, "icoBtSpeaker", "Icon", &mut problems) },
            lblName: Label { id: handles::resolve(ui, root, "lblName", "Label", &mut problems) },
            mtrLevel: LevelMeter { id: handles::resolve(ui, root, "mtrLevel", "LevelMeter", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form fraSourceRow não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FraSourceRowComponents {
            form: instance.form,
            pnlRow: instance.pnlRow,
            shpStripe: instance.shpStripe,
            swOn: instance.swOn,
            lyBody: instance.lyBody,
            lyLine: instance.lyLine,
            dotDefault: instance.dotDefault,
            icoBtMic: instance.icoBtMic,
            icoBtSpeaker: instance.icoBtSpeaker,
            lblName: instance.lblName,
            mtrLevel: instance.mtrLevel,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do fraSourceRow aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FraSourceRowComponents> {
        ui.form_components(ui.find_path("fraSourceRow")?)
    }

    /// Cria o fraSourceRow (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, aylis::prelude::Error> {
        ui.create_form("fraSourceRow")
    }

    /// Cria mais um fraSourceRow dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`fraSourceRow`, `fraSourceRow2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FraSourceRowComponents, aylis::prelude::Error> {
        let root = ui.create_form_instance("fraSourceRow")?;
        let name = ui.free_name_in(parent, "fraSourceRow");
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

    /// Os componentes de uma instância do fraSourceRow (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FraSourceRowComponents> {
        ui.form_components(root)
    }

    /// O fraSourceRow aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FraSourceRow> {
        ui.form_instance(ui.find_path("fraSourceRow")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FraSourceRow> {
        let form = ui.find_path("fraSourceRow")?;
        ui.form_instance_mut(form)
    }

    /// O Circle deste form com o nome (`None` se não há um Circle com ele).
    pub fn find_circle(&self, name: &str) -> Option<Circle> {
        match name {
            "dotDefault" => Some(self.dotDefault),
            _ => None,
        }
    }

    /// O Circle que disparou o evento (`None` se o `sender` não é um Circle deste form).
    pub fn sender_as_circle(&self, ui: &Ui, sender: ComponentId) -> Option<Circle> {
        self.find_circle(ui.name(sender)?)
    }

    /// O Icon deste form com o nome (`None` se não há um Icon com ele).
    pub fn find_icon(&self, name: &str) -> Option<Icon> {
        match name {
            "icoBtMic" => Some(self.icoBtMic),
            "icoBtSpeaker" => Some(self.icoBtSpeaker),
            _ => None,
        }
    }

    /// O Icon que disparou o evento (`None` se o `sender` não é um Icon deste form).
    pub fn sender_as_icon(&self, ui: &Ui, sender: ComponentId) -> Option<Icon> {
        self.find_icon(ui.name(sender)?)
    }

    /// O Label deste form com o nome (`None` se não há um Label com ele).
    pub fn find_label(&self, name: &str) -> Option<Label> {
        match name {
            "lblName" => Some(self.lblName),
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
            "lyBody" => Some(self.lyBody),
            "lyLine" => Some(self.lyLine),
            _ => None,
        }
    }

    /// O Layout que disparou o evento (`None` se o `sender` não é um Layout deste form).
    pub fn sender_as_layout(&self, ui: &Ui, sender: ComponentId) -> Option<Layout> {
        self.find_layout(ui.name(sender)?)
    }

    /// O LevelMeter deste form com o nome (`None` se não há um LevelMeter com ele).
    pub fn find_level_meter(&self, name: &str) -> Option<LevelMeter> {
        match name {
            "mtrLevel" => Some(self.mtrLevel),
            _ => None,
        }
    }

    /// O LevelMeter que disparou o evento (`None` se o `sender` não é um LevelMeter deste form).
    pub fn sender_as_level_meter(&self, ui: &Ui, sender: ComponentId) -> Option<LevelMeter> {
        self.find_level_meter(ui.name(sender)?)
    }

    /// O Panel deste form com o nome (`None` se não há um Panel com ele).
    pub fn find_panel(&self, name: &str) -> Option<Panel> {
        match name {
            "pnlRow" => Some(self.pnlRow),
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
            "shpStripe" => Some(self.shpStripe),
            _ => None,
        }
    }

    /// O Rectangle que disparou o evento (`None` se o `sender` não é um Rectangle deste form).
    pub fn sender_as_rectangle(&self, ui: &Ui, sender: ComponentId) -> Option<Rectangle> {
        self.find_rectangle(ui.name(sender)?)
    }

    /// O Switch deste form com o nome (`None` se não há um Switch com ele).
    pub fn find_switch(&self, name: &str) -> Option<Switch> {
        match name {
            "swOn" => Some(self.swOn),
            _ => None,
        }
    }

    /// O Switch que disparou o evento (`None` se o `sender` não é um Switch deste form).
    pub fn sender_as_switch(&self, ui: &Ui, sender: ComponentId) -> Option<Switch> {
        self.find_switch(ui.name(sender)?)
    }
}
