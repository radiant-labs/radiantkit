use std::sync::{Arc, RwLock};

use crate::RadiantRuntime;
use crate::{RadiantNodeType, RadiantRectangleNode, RadiantResponse, RectangleTool};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = RadiantAppController)]
pub struct RadiantAppController {
    runtime: Arc<RwLock<RadiantRuntime>>,
}

#[wasm_bindgen(js_class = RadiantAppController)]
impl RadiantAppController {
    #[wasm_bindgen(constructor)]
    pub async fn new(f: &js_sys::Function) -> Self {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Info).expect("Couldn't initialize logger");
        log::info!("Hello from rust!");

        let f2 = f.clone();
        let handler: Box<dyn Fn(RadiantResponse)> = Box::new(move |response: RadiantResponse| {
            let this = JsValue::null();
            let _ = f2.call1(&this, &serde_wasm_bindgen::to_value(&response).unwrap());
        });

        let mut runtime = RadiantRuntime::new(handler).await;
        runtime
            .app
            .scene
            .tool_manager
            .register_tool(Box::new(RectangleTool::new()));
        runtime
            .app
            .scene
            .add(RadiantNodeType::Rectangle(RadiantRectangleNode::new(
                1,
                [100.0, 100.0],
                [100.0, 100.0],
            )));

        let runtime = Arc::new(RwLock::new(runtime));

        radiant_winit::run_wasm(runtime.clone(), f.clone());

        Self { runtime }
    }

    #[wasm_bindgen(js_name = handleMessage)]
    pub fn handle_message(&mut self, message: JsValue) {
        if let Ok(message) = serde_wasm_bindgen::from_value(message) {
            if let Ok(mut runtime) = self.runtime.write() {
                runtime.handle_message(message);
            }
        }
    }
}
