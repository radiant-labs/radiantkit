use serde::{Deserialize, Serialize};

use crate::{RadiantComponent, RadiantSelectable};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SelectionComponent {
    selected: bool,
}

impl SelectionComponent {
    pub fn new() -> Self {
        Self { selected: false }
    }

    pub fn is_selected(&self) -> bool {
        self.selected
    }
}

impl RadiantSelectable for SelectionComponent {
    fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }
}

impl RadiantComponent for SelectionComponent {}
