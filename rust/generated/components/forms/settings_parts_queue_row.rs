
/// O form fraQueueRow: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FraQueueRow {
    pub form: Form,
    pub pnlRow: Panel,
    pub shpLine: Rectangle,
    pub lblDot: Label,
    pub lblNow: Label,
    pub btnRemove: Button,
    pub lblDuration: Label,
    pub lblName: Label,
    pub data: super::settings_parts_queue_row::Data,
}

/// Os componentes do fraQueueRow sem o estado, para o código fora dos handlers
/// (`FraQueueRow::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FraQueueRowComponents {
    pub form: Form,
    pub pnlRow: Panel,
    pub shpLine: Rectangle,
    pub lblDot: Label,
    pub lblNow: Label,
    pub btnRemove: Button,
    pub lblDuration: Label,
    pub lblName: Label,
}

impl std::ops::Deref for FraQueueRow {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FraQueueRow {
    /// Monta a instância de um fraQueueRow recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            pnlRow: Panel { id: handles::resolve(ui, root, "pnlRow", "Panel", &mut problems) },
            shpLine: Rectangle { id: handles::resolve(ui, root, "shpLine", "Rectangle", &mut problems) },
            lblDot: Label { id: handles::resolve(ui, root, "lblDot", "Label", &mut problems) },
            lblNow: Label { id: handles::resolve(ui, root, "lblNow", "Label", &mut problems) },
            btnRemove: Button { id: handles::resolve(ui, root, "btnRemove", "Button", &mut problems) },
            lblDuration: Label { id: handles::resolve(ui, root, "lblDuration", "Label", &mut problems) },
            lblName: Label { id: handles::resolve(ui, root, "lblName", "Label", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form fraQueueRow não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FraQueueRowComponents {
            form: instance.form,
            pnlRow: instance.pnlRow,
            shpLine: instance.shpLine,
            lblDot: instance.lblDot,
            lblNow: instance.lblNow,
            btnRemove: instance.btnRemove,
            lblDuration: instance.lblDuration,
            lblName: instance.lblName,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do fraQueueRow aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FraQueueRowComponents> {
        ui.form_components(ui.find_path("fraQueueRow")?)
    }

    /// Cria o fraQueueRow (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, aylis::prelude::Error> {
        ui.create_form("fraQueueRow")
    }

    /// Cria mais um fraQueueRow dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`fraQueueRow`, `fraQueueRow2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FraQueueRowComponents, aylis::prelude::Error> {
        let root = ui.create_form_instance("fraQueueRow")?;
        let name = ui.free_name_in(parent, "fraQueueRow");
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

    /// Os componentes de uma instância do fraQueueRow (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FraQueueRowComponents> {
        ui.form_components(root)
    }

    /// O fraQueueRow aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FraQueueRow> {
        ui.form_instance(ui.find_path("fraQueueRow")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FraQueueRow> {
        let form = ui.find_path("fraQueueRow")?;
        ui.form_instance_mut(form)
    }

    /// O Button deste form com o nome (`None` se não há um Button com ele).
    pub fn find_button(&self, name: &str) -> Option<Button> {
        match name {
            "btnRemove" => Some(self.btnRemove),
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
            "lblDot" => Some(self.lblDot),
            "lblNow" => Some(self.lblNow),
            "lblDuration" => Some(self.lblDuration),
            "lblName" => Some(self.lblName),
            _ => None,
        }
    }

    /// O Label que disparou o evento (`None` se o `sender` não é um Label deste form).
    pub fn sender_as_label(&self, ui: &Ui, sender: ComponentId) -> Option<Label> {
        self.find_label(ui.name(sender)?)
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
            "shpLine" => Some(self.shpLine),
            _ => None,
        }
    }

    /// O Rectangle que disparou o evento (`None` se o `sender` não é um Rectangle deste form).
    pub fn sender_as_rectangle(&self, ui: &Ui, sender: ComponentId) -> Option<Rectangle> {
        self.find_rectangle(ui.name(sender)?)
    }
}
