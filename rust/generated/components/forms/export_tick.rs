
/// O form fraExTick: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FraExTick {
    pub form: Form,
    pub shpTick: Rectangle,
    pub lblTick: Label,
    pub data: super::export_tick::Data,
}

/// Os componentes do fraExTick sem o estado, para o código fora dos handlers
/// (`FraExTick::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FraExTickComponents {
    pub form: Form,
    pub shpTick: Rectangle,
    pub lblTick: Label,
}

impl std::ops::Deref for FraExTick {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FraExTick {
    /// Monta a instância de um fraExTick recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            shpTick: Rectangle { id: handles::resolve(ui, root, "shpTick", "Rectangle", &mut problems) },
            lblTick: Label { id: handles::resolve(ui, root, "lblTick", "Label", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form fraExTick não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FraExTickComponents {
            form: instance.form,
            shpTick: instance.shpTick,
            lblTick: instance.lblTick,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do fraExTick aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FraExTickComponents> {
        ui.form_components(ui.find_path("fraExTick")?)
    }

    /// Cria o fraExTick (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, aylis::prelude::Error> {
        ui.create_form("fraExTick")
    }

    /// Cria mais um fraExTick dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`fraExTick`, `fraExTick2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FraExTickComponents, aylis::prelude::Error> {
        let root = ui.create_form_instance("fraExTick")?;
        let name = ui.free_name_in(parent, "fraExTick");
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

    /// Os componentes de uma instância do fraExTick (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FraExTickComponents> {
        ui.form_components(root)
    }

    /// O fraExTick aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FraExTick> {
        ui.form_instance(ui.find_path("fraExTick")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FraExTick> {
        let form = ui.find_path("fraExTick")?;
        ui.form_instance_mut(form)
    }

    /// O Label deste form com o nome (`None` se não há um Label com ele).
    pub fn find_label(&self, name: &str) -> Option<Label> {
        match name {
            "lblTick" => Some(self.lblTick),
            _ => None,
        }
    }

    /// O Label que disparou o evento (`None` se o `sender` não é um Label deste form).
    pub fn sender_as_label(&self, ui: &Ui, sender: ComponentId) -> Option<Label> {
        self.find_label(ui.name(sender)?)
    }

    /// O Rectangle deste form com o nome (`None` se não há um Rectangle com ele).
    pub fn find_rectangle(&self, name: &str) -> Option<Rectangle> {
        match name {
            "shpTick" => Some(self.shpTick),
            _ => None,
        }
    }

    /// O Rectangle que disparou o evento (`None` se o `sender` não é um Rectangle deste form).
    pub fn sender_as_rectangle(&self, ui: &Ui, sender: ComponentId) -> Option<Rectangle> {
        self.find_rectangle(ui.name(sender)?)
    }
}
