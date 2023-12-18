use serde::{Deserialize, Serialize};

use crate::{RadiantComponent, Vec3};

const MIN_SIZE: f32 = 8.0;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
#[cfg_attr(not(target_arch = "wasm32"), radiantkit_macros::radiant_wasm_bindgen)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct TransformComponent {
    position: Vec3,
    scale: Vec3,
    rotation: f32,
}

impl TransformComponent {
    pub fn new() -> Self {
        Self {
            position: Vec3::zero(),
            scale: Vec3::new_with_min(MIN_SIZE),
            rotation: 0.0,
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
impl TransformComponent {
    pub fn transform_xy(&mut self, position: &Vec3) {
        self.position.add(position);
    }

    pub fn transform_scale(&mut self, scale: &Vec3) {
        self.scale.add_with_min(scale, MIN_SIZE)
    }

    pub fn set_position(&mut self, position: &Vec3) {
        self.position = position.clone();
    }

    pub fn set_scale(&mut self, scale: &Vec3) {
        self.scale.set_with_min(scale, MIN_SIZE);
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }

    pub fn position(&self) -> Vec3 {
        self.position
    }

    pub fn scale(&self) -> Vec3 {
        self.scale
    }

    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }
}

impl RadiantComponent for TransformComponent {}
