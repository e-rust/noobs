
/// O form fraAbout: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FraAbout {
    pub form: Form,
    pub imgAbout: Image,
    pub lblAboutName: Label,
    pub lblAboutTagline: Label,
    pub lblAboutIntro1: Label,
    pub lblAboutIntro2: Label,
    pub lblAboutIntro3: Label,
    pub lyAboutAuthor: Layout,
    pub lblAboutAuthor: Label,
    pub lblAboutAuthorValue: Label,
    pub lyAboutCoAuthor: Layout,
    pub lblAboutCoAuthor: Label,
    pub lblAboutCoAuthorValue: Label,
    pub lblAboutCoAuthorTag: Label,
    pub lyAboutRepository: Layout,
    pub lblAboutRepository: Label,
    pub lblAboutRepositoryValue: Label,
    pub lyAboutLists: Layout,
    pub lblAboutFeatures: Label,
    pub lyAboutFeatures: Layout,
    pub lblAboutBuiltWith: Label,
    pub lyAboutBuiltWith: Layout,
    pub lblAboutFooter: Label,
    pub data: super::settings_about::Data,
}

/// Os componentes do fraAbout sem o estado, para o código fora dos handlers
/// (`FraAbout::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FraAboutComponents {
    pub form: Form,
    pub imgAbout: Image,
    pub lblAboutName: Label,
    pub lblAboutTagline: Label,
    pub lblAboutIntro1: Label,
    pub lblAboutIntro2: Label,
    pub lblAboutIntro3: Label,
    pub lyAboutAuthor: Layout,
    pub lblAboutAuthor: Label,
    pub lblAboutAuthorValue: Label,
    pub lyAboutCoAuthor: Layout,
    pub lblAboutCoAuthor: Label,
    pub lblAboutCoAuthorValue: Label,
    pub lblAboutCoAuthorTag: Label,
    pub lyAboutRepository: Layout,
    pub lblAboutRepository: Label,
    pub lblAboutRepositoryValue: Label,
    pub lyAboutLists: Layout,
    pub lblAboutFeatures: Label,
    pub lyAboutFeatures: Layout,
    pub lblAboutBuiltWith: Label,
    pub lyAboutBuiltWith: Layout,
    pub lblAboutFooter: Label,
}

impl std::ops::Deref for FraAbout {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FraAbout {
    /// Monta a instância de um fraAbout recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            imgAbout: Image { id: handles::resolve(ui, root, "imgAbout", "Image", &mut problems) },
            lblAboutName: Label { id: handles::resolve(ui, root, "lblAboutName", "Label", &mut problems) },
            lblAboutTagline: Label { id: handles::resolve(ui, root, "lblAboutTagline", "Label", &mut problems) },
            lblAboutIntro1: Label { id: handles::resolve(ui, root, "lblAboutIntro1", "Label", &mut problems) },
            lblAboutIntro2: Label { id: handles::resolve(ui, root, "lblAboutIntro2", "Label", &mut problems) },
            lblAboutIntro3: Label { id: handles::resolve(ui, root, "lblAboutIntro3", "Label", &mut problems) },
            lyAboutAuthor: Layout { id: handles::resolve(ui, root, "lyAboutAuthor", "Layout", &mut problems) },
            lblAboutAuthor: Label { id: handles::resolve(ui, root, "lblAboutAuthor", "Label", &mut problems) },
            lblAboutAuthorValue: Label { id: handles::resolve(ui, root, "lblAboutAuthorValue", "Label", &mut problems) },
            lyAboutCoAuthor: Layout { id: handles::resolve(ui, root, "lyAboutCoAuthor", "Layout", &mut problems) },
            lblAboutCoAuthor: Label { id: handles::resolve(ui, root, "lblAboutCoAuthor", "Label", &mut problems) },
            lblAboutCoAuthorValue: Label { id: handles::resolve(ui, root, "lblAboutCoAuthorValue", "Label", &mut problems) },
            lblAboutCoAuthorTag: Label { id: handles::resolve(ui, root, "lblAboutCoAuthorTag", "Label", &mut problems) },
            lyAboutRepository: Layout { id: handles::resolve(ui, root, "lyAboutRepository", "Layout", &mut problems) },
            lblAboutRepository: Label { id: handles::resolve(ui, root, "lblAboutRepository", "Label", &mut problems) },
            lblAboutRepositoryValue: Label { id: handles::resolve(ui, root, "lblAboutRepositoryValue", "Label", &mut problems) },
            lyAboutLists: Layout { id: handles::resolve(ui, root, "lyAboutLists", "Layout", &mut problems) },
            lblAboutFeatures: Label { id: handles::resolve(ui, root, "lblAboutFeatures", "Label", &mut problems) },
            lyAboutFeatures: Layout { id: handles::resolve(ui, root, "lyAboutFeatures", "Layout", &mut problems) },
            lblAboutBuiltWith: Label { id: handles::resolve(ui, root, "lblAboutBuiltWith", "Label", &mut problems) },
            lyAboutBuiltWith: Layout { id: handles::resolve(ui, root, "lyAboutBuiltWith", "Layout", &mut problems) },
            lblAboutFooter: Label { id: handles::resolve(ui, root, "lblAboutFooter", "Label", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form fraAbout não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FraAboutComponents {
            form: instance.form,
            imgAbout: instance.imgAbout,
            lblAboutName: instance.lblAboutName,
            lblAboutTagline: instance.lblAboutTagline,
            lblAboutIntro1: instance.lblAboutIntro1,
            lblAboutIntro2: instance.lblAboutIntro2,
            lblAboutIntro3: instance.lblAboutIntro3,
            lyAboutAuthor: instance.lyAboutAuthor,
            lblAboutAuthor: instance.lblAboutAuthor,
            lblAboutAuthorValue: instance.lblAboutAuthorValue,
            lyAboutCoAuthor: instance.lyAboutCoAuthor,
            lblAboutCoAuthor: instance.lblAboutCoAuthor,
            lblAboutCoAuthorValue: instance.lblAboutCoAuthorValue,
            lblAboutCoAuthorTag: instance.lblAboutCoAuthorTag,
            lyAboutRepository: instance.lyAboutRepository,
            lblAboutRepository: instance.lblAboutRepository,
            lblAboutRepositoryValue: instance.lblAboutRepositoryValue,
            lyAboutLists: instance.lyAboutLists,
            lblAboutFeatures: instance.lblAboutFeatures,
            lyAboutFeatures: instance.lyAboutFeatures,
            lblAboutBuiltWith: instance.lblAboutBuiltWith,
            lyAboutBuiltWith: instance.lyAboutBuiltWith,
            lblAboutFooter: instance.lblAboutFooter,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do fraAbout aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FraAboutComponents> {
        ui.form_components(ui.find_path("fraAbout")?)
    }

    /// Cria o fraAbout (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, aylis::prelude::Error> {
        ui.create_form("fraAbout")
    }

    /// Cria mais um fraAbout dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`fraAbout`, `fraAbout2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FraAboutComponents, aylis::prelude::Error> {
        let root = ui.create_form_instance("fraAbout")?;
        let name = ui.free_name_in(parent, "fraAbout");
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

    /// Os componentes de uma instância do fraAbout (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FraAboutComponents> {
        ui.form_components(root)
    }

    /// O fraAbout aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FraAbout> {
        ui.form_instance(ui.find_path("fraAbout")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FraAbout> {
        let form = ui.find_path("fraAbout")?;
        ui.form_instance_mut(form)
    }

    /// O Image deste form com o nome (`None` se não há um Image com ele).
    pub fn find_image(&self, name: &str) -> Option<Image> {
        match name {
            "imgAbout" => Some(self.imgAbout),
            _ => None,
        }
    }

    /// O Image que disparou o evento (`None` se o `sender` não é um Image deste form).
    pub fn sender_as_image(&self, ui: &Ui, sender: ComponentId) -> Option<Image> {
        self.find_image(ui.name(sender)?)
    }

    /// O Label deste form com o nome (`None` se não há um Label com ele).
    pub fn find_label(&self, name: &str) -> Option<Label> {
        match name {
            "lblAboutName" => Some(self.lblAboutName),
            "lblAboutTagline" => Some(self.lblAboutTagline),
            "lblAboutIntro1" => Some(self.lblAboutIntro1),
            "lblAboutIntro2" => Some(self.lblAboutIntro2),
            "lblAboutIntro3" => Some(self.lblAboutIntro3),
            "lblAboutAuthor" => Some(self.lblAboutAuthor),
            "lblAboutAuthorValue" => Some(self.lblAboutAuthorValue),
            "lblAboutCoAuthor" => Some(self.lblAboutCoAuthor),
            "lblAboutCoAuthorValue" => Some(self.lblAboutCoAuthorValue),
            "lblAboutCoAuthorTag" => Some(self.lblAboutCoAuthorTag),
            "lblAboutRepository" => Some(self.lblAboutRepository),
            "lblAboutRepositoryValue" => Some(self.lblAboutRepositoryValue),
            "lblAboutFeatures" => Some(self.lblAboutFeatures),
            "lblAboutBuiltWith" => Some(self.lblAboutBuiltWith),
            "lblAboutFooter" => Some(self.lblAboutFooter),
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
            "lyAboutAuthor" => Some(self.lyAboutAuthor),
            "lyAboutCoAuthor" => Some(self.lyAboutCoAuthor),
            "lyAboutRepository" => Some(self.lyAboutRepository),
            "lyAboutLists" => Some(self.lyAboutLists),
            "lyAboutFeatures" => Some(self.lyAboutFeatures),
            "lyAboutBuiltWith" => Some(self.lyAboutBuiltWith),
            _ => None,
        }
    }

    /// O Layout que disparou o evento (`None` se o `sender` não é um Layout deste form).
    pub fn sender_as_layout(&self, ui: &Ui, sender: ComponentId) -> Option<Layout> {
        self.find_layout(ui.name(sender)?)
    }
}
