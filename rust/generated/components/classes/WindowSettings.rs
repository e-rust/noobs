
/// Um WindowSettings de um form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WindowSettings {
    pub id: ComponentId,
}

impl Handle for WindowSettings {
    fn id(&self) -> ComponentId {
        self.id
    }
}

/// Quem disparou um handler tipado (`sender: WindowSettings`): só um WindowSettings.
impl aylis::prelude::FromSender for WindowSettings {
    fn from_sender(ui: &Ui, id: ComponentId) -> Option<Self> {
        (ui.class_name(id) == Some("WindowSettings")).then_some(Self { id })
    }
}

impl WindowSettings {
    pub fn scope(self, ui: &Ui) -> aylis::prelude::WindowSettingsScope {
        handles::read_enum::<aylis::prelude::WindowSettingsScope>(ui, self.id, "scope")
    }
    pub fn set_scope(self, ui: &mut Ui, value: aylis::prelude::WindowSettingsScope) {
        handles::write(ui, self.id, "scope", PropValue::enum_value(value))
    }
    pub fn animation(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "animation").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_animation(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "animation", value)
    }
    pub fn capture_excluded(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "capture_excluded").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_capture_excluded(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "capture_excluded", value)
    }
    pub fn click_through(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "click_through").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_click_through(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "click_through", value)
    }
    pub fn minimize_button(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "minimize_button").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_minimize_button(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "minimize_button", value)
    }
    pub fn maximize_button(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "maximize_button").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_maximize_button(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "maximize_button", value)
    }
    pub fn backdrop(self, ui: &Ui) -> aylis::prelude::WindowBackdrop {
        handles::read_enum::<aylis::prelude::WindowBackdrop>(ui, self.id, "backdrop")
    }
    pub fn set_backdrop(self, ui: &mut Ui, value: aylis::prelude::WindowBackdrop) {
        handles::write(ui, self.id, "backdrop", PropValue::enum_value(value))
    }
    pub fn title_color(self, ui: &Ui) -> aylis::prelude::Color {
        handles::read(ui, self.id, "title_color").and_then(|v| v.to_color().ok()).unwrap_or_default()
    }
    pub fn set_title_color(self, ui: &mut Ui, value: impl Into<handles::ColorRef>) {
        handles::write(ui, self.id, "title_color", Into::<handles::ColorRef>::into(value))
    }
    pub fn title_text_color(self, ui: &Ui) -> aylis::prelude::Color {
        handles::read(ui, self.id, "title_text_color").and_then(|v| v.to_color().ok()).unwrap_or_default()
    }
    pub fn set_title_text_color(self, ui: &mut Ui, value: impl Into<handles::ColorRef>) {
        handles::write(ui, self.id, "title_text_color", Into::<handles::ColorRef>::into(value))
    }
    pub fn border_color(self, ui: &Ui) -> aylis::prelude::Color {
        handles::read(ui, self.id, "border_color").and_then(|v| v.to_color().ok()).unwrap_or_default()
    }
    pub fn set_border_color(self, ui: &mut Ui, value: impl Into<handles::ColorRef>) {
        handles::write(ui, self.id, "border_color", Into::<handles::ColorRef>::into(value))
    }
    pub fn shadow(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "shadow").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_shadow(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "shadow", value)
    }
    pub fn blur(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "blur").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_blur(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "blur", value)
    }
    pub fn no_activate(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "no_activate").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_no_activate(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "no_activate", value)
    }
    pub fn alt_tab(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "alt_tab").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_alt_tab(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "alt_tab", value)
    }
    pub fn tag(self, ui: &Ui) -> i64 {
        handles::read(ui, self.id, "tag").and_then(|v| v.to_i64().ok()).unwrap_or_default()
    }
    pub fn set_tag(self, ui: &mut Ui, value: i64) {
        handles::write(ui, self.id, "tag", value)
    }
}
