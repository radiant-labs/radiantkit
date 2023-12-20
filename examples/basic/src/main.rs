use radiantkit::{
    run_native, RadiantPathNode, RadiantRectangleNode, RadiantResponse, RadiantRuntime, Runtime,
};
use uuid::Uuid;

async fn run() {
    let env = env_logger::Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    let handler: Box<dyn Fn(RadiantResponse)> = Box::new(move |response: RadiantResponse| {
        log::info!("Response: {:?}", response);
    });

    let mut runtime = RadiantRuntime::new(2, false, None).await;
    runtime.add(RadiantRectangleNode::new(Uuid::new_v4(), [100.0, 100.0], [200.0, 200.0]).into());
    runtime.add(RadiantPathNode::new(Uuid::new_v4(), [400.0, 400.0]).into());

    run_native(runtime, handler);
}

fn main() {
    pollster::block_on(run());
}
