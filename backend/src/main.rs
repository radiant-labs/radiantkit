use futures_util::{FutureExt, StreamExt};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use warp::Filter;
use y_sync::awareness::Awareness;
use y_sync::net::BroadcastGroup;
use yrs::Doc;
use yrs_warp::{
    ws::{WarpSink, WarpStream},
    AwarenessRef,
};

#[tokio::main]
async fn main() {
    let env = env_logger::Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    // We're using a single static document shared among all the peers.
    let awareness: AwarenessRef = {
        let doc = Doc::with_client_id(1);
        {
            // pre-initialize code mirror document with some text
            let _map = doc.get_or_insert_map("radiantkit-root");
        }
        Arc::new(RwLock::new(Awareness::new(doc)))
    };

    let bcast = Arc::new(BroadcastGroup::new(awareness.clone(), 32).await);

    let routes = warp::path("sync")
        .and(warp::ws())
        .and(warp::any().map(move || bcast.clone()))
        .map(|ws: warp::ws::Ws, bcast: Arc<BroadcastGroup>| {
            ws.on_upgrade(move |websocket| {
                log::info!("websocket connection opened");
                let (tx, rx) = websocket.split();
                let sink = Arc::new(Mutex::new(WarpSink::from(tx)));
                let stream = WarpStream::from(rx);
                let sub = bcast.subscribe(sink, stream);
                sub.completed().then(|result| async move {
                    match result {
                        Ok(_) => println!("broadcasting for channel finished successfully"),
                        Err(e) => eprintln!("broadcasting for channel finished abruptly: {}", e),
                    }
                })
            })
        });

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
