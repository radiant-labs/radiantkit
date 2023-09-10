use winit::{event_loop::EventLoop, window::WindowBuilder};

async fn run() {
    let env = env_logger::Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut app = radiant_main::RadiantApp::new(window).await;

    event_loop.run(move |event, _, control_flow| {
        app.handle_event(event, control_flow);
    });
}

fn main() {
    pollster::block_on(run());
}
