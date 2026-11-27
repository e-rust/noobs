
/// Um Taskbar de um form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Taskbar {
    pub id: ComponentId,
}

impl Handle for Taskbar {
    fn id(&self) -> ComponentId {
        self.id
    }
}

/// Quem disparou um handler tipado (`sender: Taskbar`): só um Taskbar.
impl aylis::prelude::FromSender for Taskbar {
    fn from_sender(ui: &Ui, id: ComponentId) -> Option<Self> {
        (ui.class_name(id) == Some("Taskbar")).then_some(Self { id })
    }
}

impl Taskbar {
    pub fn progress(self, ui: &Ui) -> aylis::prelude::TaskbarProgress {
        handles::read_enum::<aylis::prelude::TaskbarProgress>(ui, self.id, "progress")
    }
    pub fn set_progress(self, ui: &mut Ui, value: aylis::prelude::TaskbarProgress) {
        handles::write(ui, self.id, "progress", PropValue::enum_value(value))
    }
    pub fn value(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "value").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_value(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "value", value)
    }
    pub fn badge(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "badge"))
    }
    pub fn set_badge(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "badge", Into::<String>::into(value))
    }
    pub fn badge_color(self, ui: &Ui) -> aylis::prelude::Color {
        handles::read(ui, self.id, "badge_color").and_then(|v| v.to_color().ok()).unwrap_or_default()
    }
    pub fn set_badge_color(self, ui: &mut Ui, value: impl Into<handles::ColorRef>) {
        handles::write(ui, self.id, "badge_color", Into::<handles::ColorRef>::into(value))
    }
    pub fn buttons(self, ui: &Ui) -> Vec<ThumbButton> {
        ThumbButton::list_from_text(&handles::text(ui, handles::read(ui, self.id, "buttons")))
    }
    pub fn set_buttons(self, ui: &mut Ui, value: &[ThumbButton]) {
        handles::write(ui, self.id, "buttons", PropValue::String(ThumbButton::list_text(value)))
    }
    pub fn tasks(self, ui: &Ui) -> Vec<JumpTask> {
        JumpTask::list_from_text(&handles::text(ui, handles::read(ui, self.id, "tasks")))
    }
    pub fn set_tasks(self, ui: &mut Ui, value: &[JumpTask]) {
        handles::write(ui, self.id, "tasks", PropValue::String(JumpTask::list_text(value)))
    }
    pub fn on_button(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_button"))
    }
    pub fn set_on_button(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_button", Into::<String>::into(value))
    }
}
