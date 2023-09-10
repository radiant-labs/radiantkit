use crate::RadiantMessageHandler;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum SelectionMessage {
    SetSelected(bool),
}

pub struct SelectionComponent {
    selected: bool,
}

impl SelectionComponent {
    pub fn new() -> Self {
        Self {
            selected: false,
        }
    }

    pub fn is_selected(&self) -> bool {
        self.selected
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }
}

impl RadiantMessageHandler<SelectionMessage> for SelectionComponent {
    fn handle_message(&mut self, message: SelectionMessage) {
        match message {
            SelectionMessage::SetSelected(selected) => self.set_selected(selected),
        }
    }
}