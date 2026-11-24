
/// O form frmPrint: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FrmPrint {
    pub form: Form,
    pub wsShot: WindowSettings,
    pub imgShot: Image,
    pub pnlShotStage: Panel,
    pub lyShotTexts: Layout,
    pub pnlShotBar: Panel,
    pub memShotModel: Memo,
    pub data: super::print::Data,
}

/// Os componentes do frmPrint sem o estado, para o código fora dos handlers
/// (`FrmPrint::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FrmPrintComponents {
    pub form: Form,
    pub wsShot: WindowSettings,
    pub imgShot: Image,
    pub pnlShotStage: Panel,
    pub lyShotTexts: Layout,
    pub pnlShotBar: Panel,
    pub memShotModel: Memo,
}

impl std::ops::Deref for FrmPrint {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FrmPrint {
    /// Monta a instância de um frmPrint recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        #[allow(unused_mut)] // um form sem componentes não acrescenta nada
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            wsShot: WindowSettings { id: handles::resolve(ui, root, "wsShot", "WindowSettings", &mut problems) },
            imgShot: Image { id: handles::resolve(ui, root, "imgShot", "Image", &mut problems) },
            pnlShotStage: Panel { id: handles::resolve(ui, root, "pnlShotStage", "Panel", &mut problems) },
            lyShotTexts: Layout { id: handles::resolve(ui, root, "lyShotTexts", "Layout", &mut problems) },
            pnlShotBar: Panel { id: handles::resolve(ui, root, "pnlShotBar", "Panel", &mut problems) },
            memShotModel: Memo { id: handles::resolve(ui, root, "memShotModel", "Memo", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form frmPrint não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FrmPrintComponents {
            form: instance.form,
            wsShot: instance.wsShot,
            imgShot: instance.imgShot,
            pnlShotStage: instance.pnlShotStage,
            lyShotTexts: instance.lyShotTexts,
            pnlShotBar: instance.pnlShotBar,
            memShotModel: instance.memShotModel,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do frmPrint aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`. É o caminho no
    /// código que um handler do próprio frmPrint chama (no handler mesmo, o `self`).
    pub fn components(ui: &Ui) -> Option<FrmPrintComponents> {
        ui.form_components(ui.find_path("frmPrint")?)
    }

    /// Cria o frmPrint (ou devolve o que já está criado); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, aylis::prelude::Error> {
        ui.create_form("frmPrint")
    }

    /// Cria mais um frmPrint dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`frmPrint`, `frmPrint2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FrmPrintComponents, aylis::prelude::Error> {
        let root = ui.create_form_instance("frmPrint")?;
        let name = ui.free_name_in(parent, "frmPrint");
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

    /// Os componentes de uma instância do frmPrint (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FrmPrintComponents> {
        ui.form_components(root)
    }

    /// O frmPrint aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    ///
    /// Dentro de um handler do próprio frmPrint, dá `None`: a instância está emprestada ao
    /// handler que roda. Ali, use o `self` do handler ou, no código chamado por ele,
    /// [`components`](Self::components) (em debug, o `None` por empréstimo avisa no stderr).
    pub fn get(ui: &Ui) -> Option<&FrmPrint> {
        ui.form_instance(ui.find_path("frmPrint")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FrmPrint> {
        let form = ui.find_path("frmPrint")?;
        ui.form_instance_mut(form)
    }

    /// O Image deste form com o nome (`None` se não há um Image com ele).
    pub fn find_image(&self, name: &str) -> Option<Image> {
        match name {
            "imgShot" => Some(self.imgShot),
            _ => None,
        }
    }

    /// O Image que disparou o evento (`None` se o `sender` não é um Image deste form).
    pub fn sender_as_image(&self, ui: &Ui, sender: ComponentId) -> Option<Image> {
        self.find_image(ui.name(sender)?)
    }

    /// O Layout deste form com o nome (`None` se não há um Layout com ele).
    pub fn find_layout(&self, name: &str) -> Option<Layout> {
        match name {
            "lyShotTexts" => Some(self.lyShotTexts),
            _ => None,
        }
    }

    /// O Layout que disparou o evento (`None` se o `sender` não é um Layout deste form).
    pub fn sender_as_layout(&self, ui: &Ui, sender: ComponentId) -> Option<Layout> {
        self.find_layout(ui.name(sender)?)
    }

    /// O Memo deste form com o nome (`None` se não há um Memo com ele).
    pub fn find_memo(&self, name: &str) -> Option<Memo> {
        match name {
            "memShotModel" => Some(self.memShotModel),
            _ => None,
        }
    }

    /// O Memo que disparou o evento (`None` se o `sender` não é um Memo deste form).
    pub fn sender_as_memo(&self, ui: &Ui, sender: ComponentId) -> Option<Memo> {
        self.find_memo(ui.name(sender)?)
    }

    /// O Panel deste form com o nome (`None` se não há um Panel com ele).
    pub fn find_panel(&self, name: &str) -> Option<Panel> {
        match name {
            "pnlShotStage" => Some(self.pnlShotStage),
            "pnlShotBar" => Some(self.pnlShotBar),
            _ => None,
        }
    }

    /// O Panel que disparou o evento (`None` se o `sender` não é um Panel deste form).
    pub fn sender_as_panel(&self, ui: &Ui, sender: ComponentId) -> Option<Panel> {
        self.find_panel(ui.name(sender)?)
    }

    /// O WindowSettings deste form com o nome (`None` se não há um WindowSettings com ele).
    pub fn find_window_settings(&self, name: &str) -> Option<WindowSettings> {
        match name {
            "wsShot" => Some(self.wsShot),
            _ => None,
        }
    }

    /// O WindowSettings que disparou o evento (`None` se o `sender` não é um WindowSettings deste form).
    pub fn sender_as_window_settings(&self, ui: &Ui, sender: ComponentId) -> Option<WindowSettings> {
        self.find_window_settings(ui.name(sender)?)
    }
}
