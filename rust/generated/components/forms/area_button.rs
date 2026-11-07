
/// O form fraAreaButton: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FraAreaButton {
    pub form: Form,
    pub btnArea: Button,
    pub data: super::area_button::Data,
}

/// Os componentes do fraAreaButton sem o estado, para o código fora dos handlers
/// (`FraAreaButton::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FraAreaButtonComponents {
    pub form: Form,
    pub btnArea: Button,
}

impl std::ops::Deref for FraAreaButton {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FraAreaButton {
    /// Monta a instância de um fraAreaButton recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            btnArea: Button { id: handles::resolve(ui, root, "btnArea", "Button", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form fraAreaButton não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FraAreaButtonComponents {
            form: instance.form,
            btnArea: instance.btnArea,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do fraAreaButton aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FraAreaButtonComponents> {
        ui.form_components(ui.find_path("fraAreaButton")?)
    }

    /// Cria o fraAreaButton (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, iron::prelude::Error> {
        ui.create_form("fraAreaButton")
    }

    /// Cria mais um fraAreaButton dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`fraAreaButton`, `fraAreaButton2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FraAreaButtonComponents, iron::prelude::Error> {
        let root = ui.create_form_instance("fraAreaButton")?;
        let name = ui.free_name_in(parent, "fraAreaButton");
        if let Err(e) = ui.set_name(root, &name).and_then(|_| ui.set_parent(root, Some(parent))) {
            ui.free(root);
            return Err(e);
        }
        ui.form_components(root).ok_or(iron::prelude::Error::InvalidId)
    }

    /// Os componentes de uma instância do fraAreaButton (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FraAreaButtonComponents> {
        ui.form_components(root)
    }

    /// O fraAreaButton aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FraAreaButton> {
        ui.form_instance(ui.find_path("fraAreaButton")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FraAreaButton> {
        let form = ui.find_path("fraAreaButton")?;
        ui.form_instance_mut(form)
    }

    /// O Button deste form com o nome (`None` se não há um Button com ele).
    pub fn find_button(&self, name: &str) -> Option<Button> {
        match name {
            "btnArea" => Some(self.btnArea),
            _ => None,
        }
    }

    /// O Button que disparou o evento (`None` se o `sender` não é um Button deste form).
    pub fn sender_as_button(&self, ui: &Ui, sender: ComponentId) -> Option<Button> {
        self.find_button(ui.name(sender)?)
    }
}
