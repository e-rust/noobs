
/// Um PopupMenu de um form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PopupMenu {
    pub id: ComponentId,
}

impl Handle for PopupMenu {
    fn id(&self) -> ComponentId {
        self.id
    }
}

/// Quem disparou um handler tipado (`sender: PopupMenu`): só um PopupMenu.
impl aylis::prelude::FromSender for PopupMenu {
    fn from_sender(ui: &Ui, id: ComponentId) -> Option<Self> {
        (ui.class_name(id) == Some("PopupMenu")).then_some(Self { id })
    }
}

impl PopupMenu {
    pub fn close(self, ui: &mut Ui) -> Result<(), String> {
        handles::try_call(ui, self.id, "close", &[]).map(|_| ())
    }
}
