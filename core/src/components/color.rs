use serde::{Deserialize, Serialize};

use crate::RadiantComponent;
use epaint::Color32;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ColorComponent {
    color: Color32,
}

impl ColorComponent {
    pub fn new() -> Self {
        Self { 
            color: Color32::LIGHT_RED,
        }
    }

    pub fn color(&self) -> Color32 {
        self.color
    }
}

impl ColorComponent {
    pub fn set_color(&mut self, color: Color32) {
        self.color = color;
    }
}

impl RadiantComponent for ColorComponent {}
