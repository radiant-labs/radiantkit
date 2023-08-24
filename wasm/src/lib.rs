use wasm_bindgen::prelude::*;
use rucan_main::RucanApp;

#[wasm_bindgen]
pub fn hello() {
    let _ = RucanApp::default();
}
