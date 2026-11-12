
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

impl FrameTimer {
    pub fn enabled(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "enabled").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_enabled(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "enabled", value)
    }
}
