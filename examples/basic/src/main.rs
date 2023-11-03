use radiant_runtime::{
    run_native, RadiantNodeType, RadiantPathNode, RadiantRectangleNode, RadiantResponse,
    RadiantRuntime,
};

async fn run() {
    let env = env_logger::Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    let handler: Box<dyn Fn(RadiantResponse)> = Box::new(move |response: RadiantResponse| {
        println!("Response: {:?}", response);
    });

    let mut runtime = RadiantRuntime::new(handler).await;
    runtime
        .app
        .scene
        .add(RadiantNodeType::Rectangle(RadiantRectangleNode::new(
            1,
            [100.0, 100.0],
            [200.0, 200.0],
        )));
    runtime
        .app
        .scene
        .add(RadiantNodeType::Path(RadiantPathNode::new(
            2,
            [400.0, 400.0],
        )));

    run_native(runtime);
}

fn main() {
    pollster::block_on(run());
}
