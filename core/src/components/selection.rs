use crate::{RadiantComponent, RadiantObservable, RadiantObserver, RadiantMessage};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum SelectionMessage {
    SetSelected(u64, bool),
    HandleSelection(u64, bool),
}

impl From<SelectionMessage> for RadiantMessage {
    fn from(message: SelectionMessage) -> Self {
        RadiantMessage::Selection(message)
    }
}

impl TryFrom<RadiantMessage> for SelectionMessage {
    type Error = ();

    fn try_from(message: RadiantMessage) -> Result<Self, Self::Error> {
        match message {
            RadiantMessage::Selection(message) => Ok(message),
            _ => Err(()),
        }
    }
}

pub struct SelectionComponent {
    id: u64,
    selected: bool,
    observers: Vec<Box<dyn RadiantObserver<RadiantMessage>>>,
}

impl SelectionComponent {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            selected: false,
            observers: Vec::new(),
        }
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
        self.notify(SelectionMessage::HandleSelection(self.id, self.selected).into());
    }
}

impl RadiantComponent<RadiantMessage> for SelectionComponent {
    fn observers(&mut self) -> &mut Vec<Box<dyn RadiantObserver<RadiantMessage>>> {
        &mut self.observers
    }
}
