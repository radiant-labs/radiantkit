use serde::{Deserialize, Serialize};

use crate::{RadiantComponent, RadiantSelectable};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
#[cfg_attr(not(target_arch = "wasm32"), radiantkit_macros::radiant_wasm_bindgen)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
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
