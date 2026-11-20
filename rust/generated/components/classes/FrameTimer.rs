
/// Um FrameTimer de um form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrameTimer {
    pub id: ComponentId,
}

impl Handle for FrameTimer {
    fn id(&self) -> ComponentId {
        self.id
    }
}

/// Quem disparou um handler tipado (`sender: FrameTimer`): só um FrameTimer.
impl aylis::prelude::FromSender for FrameTimer {
    fn from_sender(ui: &Ui, id: ComponentId) -> Option<Self> {
        (ui.class_name(id) == Some("FrameTimer")).then_some(Self { id })
    }
}

impl FrameTimer {
    pub fn enabled(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "enabled").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_enabled(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "enabled", value)
    }
    pub fn on_frame(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_frame"))
    }
    pub fn set_on_frame(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_frame", Into::<String>::into(value))
    }
}
