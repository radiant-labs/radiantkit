use std::sync::{Arc, RwLock};

use radiant_runtime::RadiantRuntime;
use radiant_runtime::{RadiantNodeType, RadiantRectangleNode, RadiantResponse};
use wasm_bindgen::prelude::*;
use winit::platform::web::EventLoopExtWebSys;

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
            .add(RadiantNodeType::Rectangle(RadiantRectangleNode::new(
                1,
                [100.0, 100.0],
                [100.0, 100.0],
            )));

        let event_loop = std::mem::replace(&mut runtime.app.event_loop, None);

        let runtime = Arc::new(RwLock::new(runtime));
        let weak_runtime = Arc::downgrade(&runtime);
        let f3 = f.clone();

        if let Some(event_loop) = event_loop {
            event_loop.spawn(move |event, _, control_flow| {
                if let Some(runtime) = weak_runtime.upgrade() {
                    if let Ok(mut runtime) = runtime.write() {
                        if let Some(message) = runtime.app.handle_event(&event, control_flow) {
                            if let Some(response) = runtime.handle_message(message) {
                                let this = JsValue::null();
                                let _ = f3.call1(
                                    &this,
                                    &serde_wasm_bindgen::to_value(&response).unwrap(),
                                );
                            }
                        }
                    }
                }
            });
        }

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
