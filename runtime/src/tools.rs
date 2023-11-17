use radiantkit_core::{SelectionTool, RectangleTool};

use radiantkit_macros::radiant_wasm_bindgen;

#[radiant_wasm_bindgen]
pub struct Tools {
    pub selection: SelectionTool,
    pub rectangle: RectangleTool,
}