
/// O form fraSliderTick: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FraSliderTick {
    pub form: Form,
    pub lblTick: Label,
    pub data: super::slider_tick::Data,
}

/// Os componentes do fraSliderTick sem o estado, para o código fora dos handlers
/// (`FraSliderTick::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FraSliderTickComponents {
    pub form: Form,
    pub lblTick: Label,
}

impl std::ops::Deref for FraSliderTick {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FraSliderTick {
    /// Monta a instância de um fraSliderTick recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            lblTick: Label { id: handles::resolve(ui, root, "lblTick", "Label", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form fraSliderTick não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FraSliderTickComponents {
            form: instance.form,
            lblTick: instance.lblTick,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do fraSliderTick aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FraSliderTickComponents> {
        ui.form_components(ui.find_path("fraSliderTick")?)
    }

    /// Cria o fraSliderTick (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, aylis::prelude::Error> {
        ui.create_form("fraSliderTick")
    }

    /// Cria mais um fraSliderTick dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`fraSliderTick`, `fraSliderTick2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FraSliderTickComponents, aylis::prelude::Error> {
        let root = ui.create_form_instance("fraSliderTick")?;
        let name = ui.free_name_in(parent, "fraSliderTick");
        if let Err(e) = ui.set_name(root, &name).and_then(|_| ui.set_parent(root, Some(parent))) {
            ui.free(root);
            return Err(e);
        }
        ui.form_components(root).ok_or(aylis::prelude::Error::InvalidId)
    }

    /// Os componentes de uma instância do fraSliderTick (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FraSliderTickComponents> {
        ui.form_components(root)
    }

    /// O fraSliderTick aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FraSliderTick> {
        ui.form_instance(ui.find_path("fraSliderTick")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FraSliderTick> {
        let form = ui.find_path("fraSliderTick")?;
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
}
