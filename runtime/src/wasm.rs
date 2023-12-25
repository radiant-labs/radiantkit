use crate::{RadiantRuntime, Runtime, Vec3};
use std::sync::{Arc, RwLock};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = RadiantKitAppController)]
pub struct RadiantKitAppController {
    runtime: Arc<RwLock<RadiantRuntime>>,
    callback: js_sys::Function,
}

#[wasm_bindgen(js_class = RadiantKitAppController)]
impl RadiantKitAppController {
    #[wasm_bindgen(constructor)]
    pub async fn new(
        client_id: u64,
        collaborate: bool,
        f: &js_sys::Function,
        width: Option<f32>,
        height: Option<f32>,
        padding_x: Option<f32>,
        padding_y: Option<f32>,
    ) -> Self {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Error).expect("Couldn't initialize logger");

        let size = match (width, height) {
            (Some(width), Some(height)) => Some(Vec3::new(width, height, 0.0)),
            _ => None,
        };
        let padding = match (padding_x, padding_y) {
            (Some(padding_x), Some(padding_y)) => Vec3::new(padding_x, padding_y, 0.0),
            _ => Vec3::zero(),
        };
        let runtime = RadiantRuntime::new(client_id, collaborate, size, padding).await;
        let runtime = Arc::new(RwLock::new(runtime));

        radiantkit_winit::run_wasm(runtime.clone(), f.clone());

        Self {
            runtime,
            callback: f.clone(),
        }
    }

    #[wasm_bindgen(js_name = handleMessage)]
    pub fn handle_message(&mut self, message: JsValue) {
        if let Ok(message) = serde_wasm_bindgen::from_value(message.clone()) {
            if let Ok(mut runtime) = self.runtime.write() {
                if let Some(response) = runtime.handle_message(message) {
                    let this = JsValue::null();
                    let _ = self
                        .callback
                        .call1(&this, &serde_wasm_bindgen::to_value(&response).unwrap());
                }
            }
        } else {
            log::error!("Couldn't deserialize message {:?}", message);
        }
    }
}
