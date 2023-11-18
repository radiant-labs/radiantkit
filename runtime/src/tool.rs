#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub enum RadiantToolType {
    Select = 0, // Default
    Rectangle = 1,
}