use radiant_core::{RadiantMessage, RadiantPathNode, RadiantRectangleNode, RadiantTool};
use radiant_main::{RadiantApp, RadiantResponse};
use winit::event::Event::RedrawRequested;
use winit::{event_loop::EventLoop, window::WindowBuilder};

async fn run() {
    let env = env_logger::Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let handler: Box<dyn Fn(RadiantResponse)> = Box::new(move |response: RadiantResponse| {
        println!("Response: {:?}", response);
    });

    let mut app = RadiantApp::new(window, handler).await;
    app.scene.add(radiant_core::RadiantNodeType::Rectangle(
        RadiantRectangleNode::new(0, [100.0, 100.0]),
    ));
    app.scene
        .add(radiant_core::RadiantNodeType::Path(RadiantPathNode::new(
            1,
            [400.0, 400.0],
        )));
    // app.handle_message(RadiantMessage::SelectTool(RadiantTool::Rectangle));

    event_loop.run(move |event, _, control_flow| {
        if let Some(response) = app.handle_event(&event, control_flow) {
            println!("Response: {:?}", response);
        }

        match event {
            RedrawRequested(..) => {
                let output_frame = std::mem::replace(&mut app.scene.current_texture, None);
                output_frame.unwrap().present();
            }
            _ => {}
        }
    });
}

fn main() {
    pollster::block_on(run());
}
