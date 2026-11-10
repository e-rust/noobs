
/// O form fraLegendRow: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FraLegendRow {
    pub form: Form,
    pub lyRow: Layout,
    pub dotDefault: Circle,
    pub shpTrack: Rectangle,
    pub lblTrack: Label,
    pub lblName: Label,
    pub data: super::legend_row::Data,
}

/// Os componentes do fraLegendRow sem o estado, para o código fora dos handlers
/// (`FraLegendRow::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FraLegendRowComponents {
    pub form: Form,
    pub lyRow: Layout,
    pub dotDefault: Circle,
    pub shpTrack: Rectangle,
    pub lblTrack: Label,
    pub lblName: Label,
}

impl std::ops::Deref for FraLegendRow {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FraLegendRow {
    /// Monta a instância de um fraLegendRow recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            lyRow: Layout { id: handles::resolve(ui, root, "lyRow", "Layout", &mut problems) },
            dotDefault: Circle { id: handles::resolve(ui, root, "dotDefault", "Circle", &mut problems) },
            shpTrack: Rectangle { id: handles::resolve(ui, root, "shpTrack", "Rectangle", &mut problems) },
            lblTrack: Label { id: handles::resolve(ui, root, "lblTrack", "Label", &mut problems) },
            lblName: Label { id: handles::resolve(ui, root, "lblName", "Label", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form fraLegendRow não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FraLegendRowComponents {
            form: instance.form,
            lyRow: instance.lyRow,
            dotDefault: instance.dotDefault,
            shpTrack: instance.shpTrack,
            lblTrack: instance.lblTrack,
            lblName: instance.lblName,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do fraLegendRow aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FraLegendRowComponents> {
        ui.form_components(ui.find_path("fraLegendRow")?)
    }

    /// Cria o fraLegendRow (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, aylis::prelude::Error> {
        ui.create_form("fraLegendRow")
    }

    /// Cria mais um fraLegendRow dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`fraLegendRow`, `fraLegendRow2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FraLegendRowComponents, aylis::prelude::Error> {
        let root = ui.create_form_instance("fraLegendRow")?;
        let name = ui.free_name_in(parent, "fraLegendRow");
        if let Err(e) = ui.set_name(root, &name).and_then(|_| ui.set_parent(root, Some(parent))) {
            ui.free(root);
            return Err(e);
        }
        ui.form_components(root).ok_or(aylis::prelude::Error::InvalidId)
    }

    /// Os componentes de uma instância do fraLegendRow (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FraLegendRowComponents> {
        ui.form_components(root)
    }

    /// O fraLegendRow aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FraLegendRow> {
        ui.form_instance(ui.find_path("fraLegendRow")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FraLegendRow> {
        let form = ui.find_path("fraLegendRow")?;
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

    /// O Label deste form com o nome (`None` se não há um Label com ele).
    pub fn find_label(&self, name: &str) -> Option<Label> {
        match name {
            "lblTrack" => Some(self.lblTrack),
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
            "lyRow" => Some(self.lyRow),
            _ => None,
        }
    }

    /// O Layout que disparou o evento (`None` se o `sender` não é um Layout deste form).
    pub fn sender_as_layout(&self, ui: &Ui, sender: ComponentId) -> Option<Layout> {
        self.find_layout(ui.name(sender)?)
    }

    /// O Rectangle deste form com o nome (`None` se não há um Rectangle com ele).
    pub fn find_rectangle(&self, name: &str) -> Option<Rectangle> {
        match name {
            "shpTrack" => Some(self.shpTrack),
            _ => None,
        }
    }

    /// O Rectangle que disparou o evento (`None` se o `sender` não é um Rectangle deste form).
    pub fn sender_as_rectangle(&self, ui: &Ui, sender: ComponentId) -> Option<Rectangle> {
        self.find_rectangle(ui.name(sender)?)
    }
}
