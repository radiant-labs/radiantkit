use serde::{Deserialize, Serialize};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
#[cfg_attr(not(target_arch = "wasm32"), radiantkit_macros::radiant_wasm_bindgen)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
impl Vec3 {
    pub fn new() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn new_with_min(min: f32) -> Self {
        Self { x: min, y: min, z: min }
    }

    pub fn new_with_added(first: &Vec3, second: &Vec3) -> Self {
        Self {
            x: first.x + second.x,
            y: first.y + second.y,
            z: first.z + second.z,
        }
    }

    pub fn add(&mut self, other: &Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }

    pub fn add_with_min(&mut self, other: &Vec3, min: f32) {
        self.x = (self.x + other.x).max(min);
        self.y = (self.y + other.y).max(min);
        self.z = (self.z + other.z).max(min);
    }

    pub fn add_scalar(&mut self, scalar: f32) {
        self.x += scalar;
        self.y += scalar;
        self.z += scalar;
    }

    pub fn set_with_min(&mut self, other: &Vec3, min: f32) {
        self.x = other.x.max(min);
        self.y = other.y.max(min);
        self.z = other.z.max(min);
    }
}

impl From<[f32; 2]> for Vec3 {
    fn from([x, y]: [f32; 2]) -> Self {
        Self { x, y, z: 0.0 }
    }
}

impl Into<[f32; 2]> for Vec3 {
    fn into(self) -> [f32; 2] {
        [self.x, self.y]
    }
}

impl From<[f32; 3]> for Vec3 {
    fn from([x, y, z]: [f32; 3]) -> Self {
        Self { x, y, z }
    }
}

impl Into<epaint::Pos2> for Vec3 {
    fn into(self) -> epaint::Pos2 {
        epaint::Pos2::new(self.x, self.y)
    }
}