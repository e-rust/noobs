
/// Um MenuItem de um form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MenuItem {
    pub id: ComponentId,
}

impl Handle for MenuItem {
    fn id(&self) -> ComponentId {
        self.id
    }
}

impl MenuItem {
    pub fn text(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "text"))
    }
    pub fn set_text(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "text", Into::<String>::into(value))
    }
    pub fn shortcut(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "shortcut"))
    }
    pub fn set_shortcut(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "shortcut", Into::<String>::into(value))
    }
    pub fn enabled(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "enabled").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_enabled(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "enabled", value)
    }
    pub fn checked(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "checked").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_checked(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "checked", value)
    }
    pub fn auto_check(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "auto_check").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_auto_check(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "auto_check", value)
    }
    pub fn icon(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "icon"))
    }
    pub fn set_icon(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "icon", Into::<String>::into(value))
    }
    pub fn visible(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "visible").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_visible(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "visible", value)
    }
    pub fn danger(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "danger").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_danger(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "danger", value)
    }
}
