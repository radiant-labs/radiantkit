use std::sync::{Arc, RwLock};

use radiant_main::{RadiantApp, RadiantMessage, RadiantTool};
use wasm_bindgen::prelude::*;
use winit::platform::web::EventLoopExtWebSys;
use winit::{event_loop::EventLoop, window::WindowBuilder};

#[wasm_bindgen(js_name = RadiantAppController)]
pub struct RadiantAppController {
    app: Arc<RwLock<RadiantApp>>,
    js_callback: js_sys::Function,
}

#[wasm_bindgen(js_class = RadiantAppController)]
impl RadiantAppController {
    #[wasm_bindgen(constructor)]
    pub async fn new(f: &js_sys::Function) -> Self {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Info).expect("Couldn't initialize logger");
        log::info!("Hello from rust!");

        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        // Winit prevents sizing with CSS, so we have to set
        // the size manually when on web.
        use winit::dpi::PhysicalSize;
        window.set_inner_size(PhysicalSize::new(1600, 1200));

        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("root")?;
                let canvas = web_sys::Element::from(window.canvas());
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");

        let app = Arc::new(RwLock::new(radiant_main::RadiantApp::new(window).await));
        let weak_app = Arc::downgrade(&app);

        let f2 = f.clone();

        event_loop.spawn(move |event, _, control_flow| {
            if let Some(app) = weak_app.upgrade() {
                if let Ok(mut app) = app.write() {
                    if let Some(response) = app.handle_event(event, control_flow) {
                        let this = JsValue::null();
                        let _ = f2.call1(&this, &serde_wasm_bindgen::to_value(&response).unwrap());
                    }
                }
            }
        });

        Self {
            app,
            js_callback: f.clone(),
        }
    }

    #[wasm_bindgen(js_name = handleMessage)]
    pub fn handle_message(&mut self, message: JsValue) {
        let v = serde_wasm_bindgen::to_value(&RadiantMessage::SelectTool(RadiantTool::Selection)).unwrap();
        log::info!("Tried: {:?}", v);
        log::info!("Received message: {:?}", message);
        // let v: RadiantMessage = serde_wasm_bindgen::from_value(message).unwrap();
        if let Ok(message) =  serde_wasm_bindgen::from_value(message) {
            if let Ok(mut app) = self.app.write() {
                app.handle_message(message);
            }
        }
    }
}
