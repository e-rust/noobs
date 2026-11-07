
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

impl PopupMenu {
}
