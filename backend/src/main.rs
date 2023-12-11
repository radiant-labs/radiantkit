use futures_util::{FutureExt, StreamExt};
use warp::Filter;
use tokio::sync::{RwLock, Mutex};
use yrs_warp::{AwarenessRef, ws::{WarpSink, WarpStream}};
use y_sync::awareness::Awareness;
use y_sync::net::BroadcastGroup;
use yrs::Doc;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // We're using a single static document shared among all the peers.
    let awareness: AwarenessRef = {
        let doc = Doc::with_client_id(1);
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
