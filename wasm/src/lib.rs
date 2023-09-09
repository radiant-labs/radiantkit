use wasm_bindgen::prelude::*;
use winit::platform::web::EventLoopExtWebSys;
use winit::{event_loop::EventLoop, window::WindowBuilder};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum RadiantMessage {
    Render,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    RadiantMessage(RadiantMessage),
}

#[wasm_bindgen(js_name = MessageController)]
pub struct MessageController;

#[wasm_bindgen(js_class = MessageController)]
impl MessageController {
    #[wasm_bindgen(js_name = handleMessage)]
    pub fn handle_message(message: JsValue) {
        log::info!("Received message: {:?}", message);
        let v: Message = serde_wasm_bindgen::from_value(message).unwrap();
        log::info!("Deserialized message: {:?}", v);
    }

    #[wasm_bindgen(js_name = setJSMessageHandler)]
    pub fn set_js_message_handler(f: &js_sys::Function) {
        let message = Message::RadiantMessage(RadiantMessage::Render);
        let this = JsValue::null();
        let _ = f.call1(&this, &serde_wasm_bindgen::to_value(&message).unwrap());
    }
}

#[wasm_bindgen]
pub fn hello() {
    // println!("Hello from Rust!");
    // let _ = RadiantApp::default();
    // radiant_main::run();
}

async fn run() {
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

    let mut app = radiant_main::RadiantApp::default();
    app.init(window).await;

    event_loop.spawn(move |event, _, control_flow| {
        app.handle_event(event, control_flow);
    });
}

#[wasm_bindgen(start)]
pub fn start() {
    pollster::block_on(run());
}
