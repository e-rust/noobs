
/// O form fraRecCard: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FraRecCard {
    pub form: Form,
    pub mcCard: MediaCard,
    pub data: super::library_rec_card::Data,
}

/// Os componentes do fraRecCard sem o estado, para o código fora dos handlers
/// (`FraRecCard::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FraRecCardComponents {
    pub form: Form,
    pub mcCard: MediaCard,
}

impl std::ops::Deref for FraRecCard {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FraRecCard {
    /// Monta a instância de um fraRecCard recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            mcCard: MediaCard { id: handles::resolve(ui, root, "mcCard", "MediaCard", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form fraRecCard não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FraRecCardComponents {
            form: instance.form,
            mcCard: instance.mcCard,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do fraRecCard aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FraRecCardComponents> {
        ui.form_components(ui.find_path("fraRecCard")?)
    }

    /// Cria o fraRecCard (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, aylis::prelude::Error> {
        ui.create_form("fraRecCard")
    }

    /// Cria mais um fraRecCard dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`fraRecCard`, `fraRecCard2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FraRecCardComponents, aylis::prelude::Error> {
        let root = ui.create_form_instance("fraRecCard")?;
        let name = ui.free_name_in(parent, "fraRecCard");
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

    /// Os componentes de uma instância do fraRecCard (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FraRecCardComponents> {
        ui.form_components(root)
    }

    /// O fraRecCard aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FraRecCard> {
        ui.form_instance(ui.find_path("fraRecCard")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FraRecCard> {
        let form = ui.find_path("fraRecCard")?;
        ui.form_instance_mut(form)
    }

    /// O MediaCard deste form com o nome (`None` se não há um MediaCard com ele).
    pub fn find_media_card(&self, name: &str) -> Option<MediaCard> {
        match name {
            "mcCard" => Some(self.mcCard),
            _ => None,
        }
    }

    /// O MediaCard que disparou o evento (`None` se o `sender` não é um MediaCard deste form).
    pub fn sender_as_media_card(&self, ui: &Ui, sender: ComponentId) -> Option<MediaCard> {
        self.find_media_card(ui.name(sender)?)
    }
}
