
/// O form fraGeneral: os componentes (resolvidos ao abrir; os que só existem em algumas
/// plataformas são `Option`) e o estado dele (`Data`, no arquivo do form). Cada janela
/// aberta tem a sua; os handlers são métodos dela.
pub struct FraGeneral {
    pub form: Form,
    pub lblTheme: Label,
    pub pnlTheme: Panel,
    pub btnThemeSystem: Button,
    pub btnThemeLight: Button,
    pub btnThemeDark: Button,
    pub lnRule1: Rectangle,
    pub lblLanguage: Label,
    pub cmbLanguage: ComboBox,
    pub lblLanguageHint: Label,
    pub lnRule2: Rectangle,
    pub lblWindowTitle: Label,
    pub edtWindowTitle: Edit,
    pub lblWindowTitleHint: Label,
    pub lnRule3: Rectangle,
    pub chkAutostart: CheckBox,
    pub lblAutostartHint: Label,
    pub lnRule4: Rectangle,
    pub chkRunAsAdmin: CheckBox,
    pub lblRunAsAdminHint: Label,
    pub lblRunAsAdminState: Label,
    pub lnRule5: Rectangle,
    pub chkCloseToTray: CheckBox,
    pub lblCloseToTrayHint: Label,
    pub lnRuleHibernate: Rectangle,
    pub chkHibernate: CheckBox,
    pub lblHibernateHint: Label,
    pub lnRule6: Rectangle,
    pub lblReset: Label,
    pub lblResetHint: Label,
    pub lyReset: Layout,
    pub btnReset: Button,
    pub data: super::general::Data,
}

/// Os componentes do fraGeneral sem o estado, para o código fora dos handlers
/// (`FraGeneral::components(ui)`): uma cópia, que se usa à vontade com o `ui`.
#[derive(Clone, Copy, Debug)]
pub struct FraGeneralComponents {
    pub form: Form,
    pub lblTheme: Label,
    pub pnlTheme: Panel,
    pub btnThemeSystem: Button,
    pub btnThemeLight: Button,
    pub btnThemeDark: Button,
    pub lnRule1: Rectangle,
    pub lblLanguage: Label,
    pub cmbLanguage: ComboBox,
    pub lblLanguageHint: Label,
    pub lnRule2: Rectangle,
    pub lblWindowTitle: Label,
    pub edtWindowTitle: Edit,
    pub lblWindowTitleHint: Label,
    pub lnRule3: Rectangle,
    pub chkAutostart: CheckBox,
    pub lblAutostartHint: Label,
    pub lnRule4: Rectangle,
    pub chkRunAsAdmin: CheckBox,
    pub lblRunAsAdminHint: Label,
    pub lblRunAsAdminState: Label,
    pub lnRule5: Rectangle,
    pub chkCloseToTray: CheckBox,
    pub lblCloseToTrayHint: Label,
    pub lnRuleHibernate: Rectangle,
    pub chkHibernate: CheckBox,
    pub lblHibernateHint: Label,
    pub lnRule6: Rectangle,
    pub lblReset: Label,
    pub lblResetHint: Label,
    pub lyReset: Layout,
    pub btnReset: Button,
}

impl std::ops::Deref for FraGeneral {
    type Target = Form;
    fn deref(&self) -> &Form {
        &self.form
    }
}

impl FraGeneral {
    /// Monta a instância de um fraGeneral recém-criado (o `register` liga). Um componente que
    /// sumiu do form ou mudou de classe impede o form de abrir, com a lista do que não bate.
    pub fn bind(ui: &mut Ui, root: ComponentId) -> Result<Box<dyn std::any::Any>, String> {
        let mut problems: Vec<String> = Vec::new();
        let instance = Self {
            form: Form { id: root },
            lblTheme: Label { id: handles::resolve(ui, root, "lblTheme", "Label", &mut problems) },
            pnlTheme: Panel { id: handles::resolve(ui, root, "pnlTheme", "Panel", &mut problems) },
            btnThemeSystem: Button { id: handles::resolve(ui, root, "btnThemeSystem", "Button", &mut problems) },
            btnThemeLight: Button { id: handles::resolve(ui, root, "btnThemeLight", "Button", &mut problems) },
            btnThemeDark: Button { id: handles::resolve(ui, root, "btnThemeDark", "Button", &mut problems) },
            lnRule1: Rectangle { id: handles::resolve(ui, root, "lnRule1", "Rectangle", &mut problems) },
            lblLanguage: Label { id: handles::resolve(ui, root, "lblLanguage", "Label", &mut problems) },
            cmbLanguage: ComboBox { id: handles::resolve(ui, root, "cmbLanguage", "ComboBox", &mut problems) },
            lblLanguageHint: Label { id: handles::resolve(ui, root, "lblLanguageHint", "Label", &mut problems) },
            lnRule2: Rectangle { id: handles::resolve(ui, root, "lnRule2", "Rectangle", &mut problems) },
            lblWindowTitle: Label { id: handles::resolve(ui, root, "lblWindowTitle", "Label", &mut problems) },
            edtWindowTitle: Edit { id: handles::resolve(ui, root, "edtWindowTitle", "Edit", &mut problems) },
            lblWindowTitleHint: Label { id: handles::resolve(ui, root, "lblWindowTitleHint", "Label", &mut problems) },
            lnRule3: Rectangle { id: handles::resolve(ui, root, "lnRule3", "Rectangle", &mut problems) },
            chkAutostart: CheckBox { id: handles::resolve(ui, root, "chkAutostart", "CheckBox", &mut problems) },
            lblAutostartHint: Label { id: handles::resolve(ui, root, "lblAutostartHint", "Label", &mut problems) },
            lnRule4: Rectangle { id: handles::resolve(ui, root, "lnRule4", "Rectangle", &mut problems) },
            chkRunAsAdmin: CheckBox { id: handles::resolve(ui, root, "chkRunAsAdmin", "CheckBox", &mut problems) },
            lblRunAsAdminHint: Label { id: handles::resolve(ui, root, "lblRunAsAdminHint", "Label", &mut problems) },
            lblRunAsAdminState: Label { id: handles::resolve(ui, root, "lblRunAsAdminState", "Label", &mut problems) },
            lnRule5: Rectangle { id: handles::resolve(ui, root, "lnRule5", "Rectangle", &mut problems) },
            chkCloseToTray: CheckBox { id: handles::resolve(ui, root, "chkCloseToTray", "CheckBox", &mut problems) },
            lblCloseToTrayHint: Label { id: handles::resolve(ui, root, "lblCloseToTrayHint", "Label", &mut problems) },
            lnRuleHibernate: Rectangle { id: handles::resolve(ui, root, "lnRuleHibernate", "Rectangle", &mut problems) },
            chkHibernate: CheckBox { id: handles::resolve(ui, root, "chkHibernate", "CheckBox", &mut problems) },
            lblHibernateHint: Label { id: handles::resolve(ui, root, "lblHibernateHint", "Label", &mut problems) },
            lnRule6: Rectangle { id: handles::resolve(ui, root, "lnRule6", "Rectangle", &mut problems) },
            lblReset: Label { id: handles::resolve(ui, root, "lblReset", "Label", &mut problems) },
            lblResetHint: Label { id: handles::resolve(ui, root, "lblResetHint", "Label", &mut problems) },
            lyReset: Layout { id: handles::resolve(ui, root, "lyReset", "Layout", &mut problems) },
            btnReset: Button { id: handles::resolve(ui, root, "btnReset", "Button", &mut problems) },
            data: Default::default(),
        };
        if !problems.is_empty() {
            return Err(format!("o form fraGeneral não bate com o código: {}", problems.join("; ")));
        }
        ui.set_form_components(root, Box::new(FraGeneralComponents {
            form: instance.form,
            lblTheme: instance.lblTheme,
            pnlTheme: instance.pnlTheme,
            btnThemeSystem: instance.btnThemeSystem,
            btnThemeLight: instance.btnThemeLight,
            btnThemeDark: instance.btnThemeDark,
            lnRule1: instance.lnRule1,
            lblLanguage: instance.lblLanguage,
            cmbLanguage: instance.cmbLanguage,
            lblLanguageHint: instance.lblLanguageHint,
            lnRule2: instance.lnRule2,
            lblWindowTitle: instance.lblWindowTitle,
            edtWindowTitle: instance.edtWindowTitle,
            lblWindowTitleHint: instance.lblWindowTitleHint,
            lnRule3: instance.lnRule3,
            chkAutostart: instance.chkAutostart,
            lblAutostartHint: instance.lblAutostartHint,
            lnRule4: instance.lnRule4,
            chkRunAsAdmin: instance.chkRunAsAdmin,
            lblRunAsAdminHint: instance.lblRunAsAdminHint,
            lblRunAsAdminState: instance.lblRunAsAdminState,
            lnRule5: instance.lnRule5,
            chkCloseToTray: instance.chkCloseToTray,
            lblCloseToTrayHint: instance.lblCloseToTrayHint,
            lnRuleHibernate: instance.lnRuleHibernate,
            chkHibernate: instance.chkHibernate,
            lblHibernateHint: instance.lblHibernateHint,
            lnRule6: instance.lnRule6,
            lblReset: instance.lblReset,
            lblResetHint: instance.lblResetHint,
            lyReset: instance.lyReset,
            btnReset: instance.btnReset,
        }));
        Ok(Box::new(instance))
    }

    /// Os componentes do fraGeneral aberto (o primeiro, se houver dois), sem o estado: valem
    /// também enquanto um handler dele roda, quando o `get` dá `None`.
    pub fn components(ui: &Ui) -> Option<FraGeneralComponents> {
        ui.form_components(ui.find_path("fraGeneral")?)
    }

    /// Cria o fraGeneral (ou devolve o que já está criado, como o `CreateForm`); mostrar é com
    /// `ui.show`.
    pub fn create(ui: &mut Ui) -> Result<ComponentId, aylis::prelude::Error> {
        ui.create_form("fraGeneral")
    }

    /// Cria mais um fraGeneral dentro de `parent` (uma linha, um cartão: cada um com a sua
    /// instância e os handlers falando com ela) e devolve os componentes dele. O nome
    /// segue livre no form de `parent` (`fraGeneral`, `fraGeneral2`...).
    pub fn create_in(ui: &mut Ui, parent: ComponentId) -> Result<FraGeneralComponents, aylis::prelude::Error> {
        let root = ui.create_form_instance("fraGeneral")?;
        let name = ui.free_name_in(parent, "fraGeneral");
        if let Err(e) = ui.set_name(root, &name).and_then(|_| ui.set_parent(root, Some(parent))) {
            ui.free(root);
            return Err(e);
        }
        ui.form_components(root).ok_or(aylis::prelude::Error::InvalidId)
    }

    /// Os componentes de uma instância do fraGeneral (a raiz dela: o `form.id` de um
    /// [`create_in`](Self::create_in)).
    pub fn components_of(ui: &Ui, root: ComponentId) -> Option<FraGeneralComponents> {
        ui.form_components(root)
    }

    /// O fraGeneral aberto (o primeiro, se houver dois), para outro form ler ou mexer nele.
    pub fn get(ui: &Ui) -> Option<&FraGeneral> {
        ui.form_instance(ui.find_path("fraGeneral")?)
    }

    pub fn get_mut(ui: &mut Ui) -> Option<&mut FraGeneral> {
        let form = ui.find_path("fraGeneral")?;
        ui.form_instance_mut(form)
    }

    /// O Button deste form com o nome (`None` se não há um Button com ele).
    pub fn find_button(&self, name: &str) -> Option<Button> {
        match name {
            "btnThemeSystem" => Some(self.btnThemeSystem),
            "btnThemeLight" => Some(self.btnThemeLight),
            "btnThemeDark" => Some(self.btnThemeDark),
            "btnReset" => Some(self.btnReset),
            _ => None,
        }
    }

    /// O Button que disparou o evento (`None` se o `sender` não é um Button deste form).
    pub fn sender_as_button(&self, ui: &Ui, sender: ComponentId) -> Option<Button> {
        self.find_button(ui.name(sender)?)
    }

    /// O CheckBox deste form com o nome (`None` se não há um CheckBox com ele).
    pub fn find_check_box(&self, name: &str) -> Option<CheckBox> {
        match name {
            "chkAutostart" => Some(self.chkAutostart),
            "chkRunAsAdmin" => Some(self.chkRunAsAdmin),
            "chkCloseToTray" => Some(self.chkCloseToTray),
            "chkHibernate" => Some(self.chkHibernate),
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
            "cmbLanguage" => Some(self.cmbLanguage),
            _ => None,
        }
    }

    /// O ComboBox que disparou o evento (`None` se o `sender` não é um ComboBox deste form).
    pub fn sender_as_combo_box(&self, ui: &Ui, sender: ComponentId) -> Option<ComboBox> {
        self.find_combo_box(ui.name(sender)?)
    }

    /// O Edit deste form com o nome (`None` se não há um Edit com ele).
    pub fn find_edit(&self, name: &str) -> Option<Edit> {
        match name {
            "edtWindowTitle" => Some(self.edtWindowTitle),
            _ => None,
        }
    }

    /// O Edit que disparou o evento (`None` se o `sender` não é um Edit deste form).
    pub fn sender_as_edit(&self, ui: &Ui, sender: ComponentId) -> Option<Edit> {
        self.find_edit(ui.name(sender)?)
    }

    /// O Label deste form com o nome (`None` se não há um Label com ele).
    pub fn find_label(&self, name: &str) -> Option<Label> {
        match name {
            "lblTheme" => Some(self.lblTheme),
            "lblLanguage" => Some(self.lblLanguage),
            "lblLanguageHint" => Some(self.lblLanguageHint),
            "lblWindowTitle" => Some(self.lblWindowTitle),
            "lblWindowTitleHint" => Some(self.lblWindowTitleHint),
            "lblAutostartHint" => Some(self.lblAutostartHint),
            "lblRunAsAdminHint" => Some(self.lblRunAsAdminHint),
            "lblRunAsAdminState" => Some(self.lblRunAsAdminState),
            "lblCloseToTrayHint" => Some(self.lblCloseToTrayHint),
            "lblHibernateHint" => Some(self.lblHibernateHint),
            "lblReset" => Some(self.lblReset),
            "lblResetHint" => Some(self.lblResetHint),
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
            "lyReset" => Some(self.lyReset),
            _ => None,
        }
    }

    /// O Layout que disparou o evento (`None` se o `sender` não é um Layout deste form).
    pub fn sender_as_layout(&self, ui: &Ui, sender: ComponentId) -> Option<Layout> {
        self.find_layout(ui.name(sender)?)
    }

    /// O Panel deste form com o nome (`None` se não há um Panel com ele).
    pub fn find_panel(&self, name: &str) -> Option<Panel> {
        match name {
            "pnlTheme" => Some(self.pnlTheme),
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
            "lnRule1" => Some(self.lnRule1),
            "lnRule2" => Some(self.lnRule2),
            "lnRule3" => Some(self.lnRule3),
            "lnRule4" => Some(self.lnRule4),
            "lnRule5" => Some(self.lnRule5),
            "lnRuleHibernate" => Some(self.lnRuleHibernate),
            "lnRule6" => Some(self.lnRule6),
            _ => None,
        }
    }

    /// O Rectangle que disparou o evento (`None` se o `sender` não é um Rectangle deste form).
    pub fn sender_as_rectangle(&self, ui: &Ui, sender: ComponentId) -> Option<Rectangle> {
        self.find_rectangle(ui.name(sender)?)
    }
}
