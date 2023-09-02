use winit::{event_loop::EventLoop, window::WindowBuilder};

async fn run() {
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut app = radiant_main::RadiantApp::default();
    app.init(window).await;

    event_loop.run(move |event, _, control_flow| {
        app.handle_event(event, control_flow);
    });
}

fn main() {
    pollster::block_on(run());
}
