use serde::{Deserialize, Serialize};

use crate::{RadiantComponent, RadiantTransformable};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransformComponent {
    position: [f32; 3],
    scale: [f32; 3],
    rotation: f32,
}

impl TransformComponent {
    pub fn new() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
            rotation: 0.0,
        }
    }
}

impl RadiantTransformable for TransformComponent {
    fn set_xy(&mut self, position: &[f32; 2]) {
        self.position = [position[0], position[1], 0.0];
    }

    fn set_scale(&mut self, scale: &[f32; 2]) {
        self.scale = [scale[0], scale[1], 0.0];
    }

    fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }

    fn get_xy(&self) -> [f32; 2] {
        [self.position[0], self.position[1]]
    }

    fn get_scale(&self) -> [f32; 2] {
        [self.scale[0], self.scale[1]]
    }

    fn get_rotation(&self) -> f32 {
        self.rotation
    }
}

impl RadiantComponent for TransformComponent {}
