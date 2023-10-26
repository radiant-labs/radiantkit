use radiant_runtime::{RadiantRuntime, RadiantNodeType, RadiantRectangleNode, RadiantPathNode, RadiantResponse};
use radiant_winit::RedrawRequested;

async fn run() {
    let env = env_logger::Env::default()
    .filter_or("MY_LOG_LEVEL", "info")
    .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    let handler: Box<dyn Fn(RadiantResponse)> = Box::new(move |response: RadiantResponse| {
        println!("Response: {:?}", response);
    });

    let mut runtime = RadiantRuntime::new(handler).await;
    runtime.app.scene.add(RadiantNodeType::Rectangle(
        RadiantRectangleNode::new(1, [100.0, 100.0], [200.0, 200.0]),
    ));
    runtime.app.scene
        .add(RadiantNodeType::Path(RadiantPathNode::new(
            2,
            [400.0, 400.0],
        )));
    if let Some(event_loop) = std::mem::replace(&mut runtime.app.event_loop, None) {
        event_loop.run(move |event, _, control_flow| {
            if let Some(message) = runtime.app.handle_event(&event, control_flow) {
                runtime.handle_message(message);
            }

            match event {
                RedrawRequested(..) => {
                    let output_frame =
                        std::mem::replace(&mut runtime.app.scene.render_manager.current_texture, None);
                    output_frame.unwrap().present();
                }
                _ => {}
            }
        });
    }
}

fn main() {
    pollster::block_on(run());
}
