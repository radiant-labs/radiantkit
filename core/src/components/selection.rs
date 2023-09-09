use crate::{RadiantComponent, RadiantObservable, RadiantObserver};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum SelectionMessage {
    SetSelected(bool),
    SetId(u64),
}

pub struct SelectionComponent {
    selected: bool,
    id: u64,
    observers: Vec<Box<dyn RadiantObserver<SelectionMessage>>>,
}

impl SelectionComponent {
    pub fn new() -> Self {
        Self {
            selected: false,
            id: 0,
            observers: Vec::new(),
        }
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
        self.notify(&SelectionMessage::SetSelected(self.selected));
    }

    pub fn set_id(&mut self, id: u64) {
        self.id = id;
        self.notify(&SelectionMessage::SetId(self.id));
    }
}

impl RadiantComponent<SelectionMessage> for SelectionComponent {
    fn observers(&mut self) -> &mut Vec<Box<dyn RadiantObserver<SelectionMessage>>> {
        &mut self.observers
    }
}
