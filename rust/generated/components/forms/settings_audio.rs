
/// O form fraAudio: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FraAudio {
    pub form: Form,
    pub lblAudioBitrate: Label,
    pub cmbAudioBitrate: ComboBox,
    pub lblAudioBitrateHint: Label,
    pub lnRule1: Rectangle,
    pub chkMuteWhenMuted: CheckBox,
    pub lblMuteWhenMutedHint: Label,
    pub data: super::settings_audio::Data,
}

/// Os componentes do fraAudio sem o estado, para o código fora dos handlers
/// (`FraAudio::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FraAudioComponents {
    pub form: Form,
    pub lblAudioBitrate: Label,
    pub cmbAudioBitrate: ComboBox,
    pub lblAudioBitrateHint: Label,
    pub lnRule1: Rectangle,
    pub chkMuteWhenMuted: CheckBox,
    pub lblMuteWhenMutedHint: Label,
}

impl std::ops::Deref for FraAudio {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FraAudio {
    /// Monta a instância de um fraAudio recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            lblAudioBitrate: Label { id: handles::resolve(ui, root, "lblAudioBitrate", "Label", &mut problems) },
            cmbAudioBitrate: ComboBox { id: handles::resolve(ui, root, "cmbAudioBitrate", "ComboBox", &mut problems) },
            lblAudioBitrateHint: Label { id: handles::resolve(ui, root, "lblAudioBitrateHint", "Label", &mut problems) },
            lnRule1: Rectangle { id: handles::resolve(ui, root, "lnRule1", "Rectangle", &mut problems) },
            chkMuteWhenMuted: CheckBox { id: handles::resolve(ui, root, "chkMuteWhenMuted", "CheckBox", &mut problems) },
            lblMuteWhenMutedHint: Label { id: handles::resolve(ui, root, "lblMuteWhenMutedHint", "Label", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form fraAudio não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FraAudioComponents {
            form: instance.form,
            lblAudioBitrate: instance.lblAudioBitrate,
            cmbAudioBitrate: instance.cmbAudioBitrate,
            lblAudioBitrateHint: instance.lblAudioBitrateHint,
            lnRule1: instance.lnRule1,
            chkMuteWhenMuted: instance.chkMuteWhenMuted,
            lblMuteWhenMutedHint: instance.lblMuteWhenMutedHint,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do fraAudio aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FraAudioComponents> {
        ui.form_components(ui.find_path("fraAudio")?)
    }

    /// Cria o fraAudio (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, aylis::prelude::Error> {
        ui.create_form("fraAudio")
    }

    /// Cria mais um fraAudio dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`fraAudio`, `fraAudio2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FraAudioComponents, aylis::prelude::Error> {
        let root = ui.create_form_instance("fraAudio")?;
        let name = ui.free_name_in(parent, "fraAudio");
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

    /// Os componentes de uma instância do fraAudio (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FraAudioComponents> {
        ui.form_components(root)
    }

    /// O fraAudio aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FraAudio> {
        ui.form_instance(ui.find_path("fraAudio")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FraAudio> {
        let form = ui.find_path("fraAudio")?;
        ui.form_instance_mut(form)
    }

    /// O CheckBox deste form com o nome (`None` se não há um CheckBox com ele).
    pub fn find_check_box(&self, name: &str) -> Option<CheckBox> {
        match name {
            "chkMuteWhenMuted" => Some(self.chkMuteWhenMuted),
            _ => None,
        }
    }

    /// O CheckBox que disparou o evento (`None` se o `sender` não é um CheckBox deste form).
    pub fn sender_as_check_box(&self, ui: &Ui, sender: ComponentId) -> Option<CheckBox> {
        self.find_check_box(ui.name(sender)?)
    }

    /// O ComboBox deste form com o nome (`None` se não há um ComboBox com ele).
    pub fn find_combo_box(&self, name: &str) -> Option<ComboBox> {
        match name {
            "cmbAudioBitrate" => Some(self.cmbAudioBitrate),
            _ => None,
        }
    }

    /// O ComboBox que disparou o evento (`None` se o `sender` não é um ComboBox deste form).
    pub fn sender_as_combo_box(&self, ui: &Ui, sender: ComponentId) -> Option<ComboBox> {
        self.find_combo_box(ui.name(sender)?)
    }

    /// O Label deste form com o nome (`None` se não há um Label com ele).
    pub fn find_label(&self, name: &str) -> Option<Label> {
        match name {
            "lblAudioBitrate" => Some(self.lblAudioBitrate),
            "lblAudioBitrateHint" => Some(self.lblAudioBitrateHint),
            "lblMuteWhenMutedHint" => Some(self.lblMuteWhenMutedHint),
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
            "lnRule1" => Some(self.lnRule1),
            _ => None,
        }
    }

    /// O Rectangle que disparou o evento (`None` se o `sender` não é um Rectangle deste form).
    pub fn sender_as_rectangle(&self, ui: &Ui, sender: ComponentId) -> Option<Rectangle> {
        self.find_rectangle(ui.name(sender)?)
    }
}
