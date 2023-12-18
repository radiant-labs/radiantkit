use serde::{Deserialize, Serialize};

use crate::RadiantComponent;
use epaint::Color32;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
#[cfg_attr(not(target_arch = "wasm32"), radiantkit_macros::radiant_wasm_bindgen)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct ColorComponent {
    fill_color: Color32,
    stroke_color: Color32,
}

impl ColorComponent {
    pub fn new() -> Self {
        Self {
            fill_color: Color32::LIGHT_RED,
            stroke_color: Color32::TRANSPARENT,
        }
    }

    pub fn fill_color(&self) -> Color32 {
        self.fill_color
    }

    pub fn stroke_color(&self) -> Color32 {
        self.stroke_color
    }
}

impl ColorComponent {
    pub fn set_fill_color(&mut self, color: Color32) {
        self.fill_color = color;
    }

    pub fn set_stroke_color(&mut self, color: Color32) {
        self.stroke_color = color;
    }
}

impl RadiantComponent for ColorComponent {}
