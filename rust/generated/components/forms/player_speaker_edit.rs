
/// O form fraSpeakerEdit: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FraSpeakerEdit {
    pub form: Form,
    pub edName: Edit,
    pub data: super::player_speaker_edit::Data,
}

/// Os componentes do fraSpeakerEdit sem o estado, para o código fora dos handlers
/// (`FraSpeakerEdit::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FraSpeakerEditComponents {
    pub form: Form,
    pub edName: Edit,
}

impl std::ops::Deref for FraSpeakerEdit {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FraSpeakerEdit {
    /// Monta a instância de um fraSpeakerEdit recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            edName: Edit { id: handles::resolve(ui, root, "edName", "Edit", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form fraSpeakerEdit não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FraSpeakerEditComponents {
            form: instance.form,
            edName: instance.edName,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do fraSpeakerEdit aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FraSpeakerEditComponents> {
        ui.form_components(ui.find_path("fraSpeakerEdit")?)
    }

    /// Cria o fraSpeakerEdit (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, aylis::prelude::Error> {
        ui.create_form("fraSpeakerEdit")
    }

    /// Cria mais um fraSpeakerEdit dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`fraSpeakerEdit`, `fraSpeakerEdit2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FraSpeakerEditComponents, aylis::prelude::Error> {
        let root = ui.create_form_instance("fraSpeakerEdit")?;
        let name = ui.free_name_in(parent, "fraSpeakerEdit");
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

    /// Os componentes de uma instância do fraSpeakerEdit (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FraSpeakerEditComponents> {
        ui.form_components(root)
    }

    /// O fraSpeakerEdit aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FraSpeakerEdit> {
        ui.form_instance(ui.find_path("fraSpeakerEdit")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FraSpeakerEdit> {
        let form = ui.find_path("fraSpeakerEdit")?;
        ui.form_instance_mut(form)
    }

    /// O Edit deste form com o nome (`None` se não há um Edit com ele).
    pub fn find_edit(&self, name: &str) -> Option<Edit> {
        match name {
            "edName" => Some(self.edName),
            _ => None,
        }
    }

    /// O Edit que disparou o evento (`None` se o `sender` não é um Edit deste form).
    pub fn sender_as_edit(&self, ui: &Ui, sender: ComponentId) -> Option<Edit> {
        self.find_edit(ui.name(sender)?)
    }
}
