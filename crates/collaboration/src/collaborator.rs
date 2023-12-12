use radiantkit_core::{RadiantDocumentListener, RadiantDocumentNode, RadiantNode};
use y_sync::awareness::Awareness;
use yrs::*;
use std::sync::{Arc, RwLock};

#[cfg(target_arch = "wasm32")]
use crate::wasm_connection::WasmConnection;
#[cfg(not(target_arch = "wasm32"))]
use crate::native_connection::NativeConnection;

pub struct Collaborator {
    #[cfg(target_arch = "wasm32")]
    connection: Arc<RwLock<WasmConnection>>,
    #[cfg(not(target_arch = "wasm32"))]
    connection: Arc<RwLock<NativeConnection>>,
}

impl Collaborator {
    pub async fn new(client_id: u64) -> Result<Self, ()> {
        let url = "ws://localhost:8000/sync";

        let doc = Doc::with_client_id(client_id);
        let _map = doc.get_or_insert_map("radiantkit-root");

        let connection;

        #[cfg(target_arch = "wasm32")]
        {
            let awareness = Arc::new(RwLock::new(Awareness::new(doc)));
            match WasmConnection::new(awareness.clone(), url) {
                Ok(conn) => connection = conn,
                Err(_) => return Err(()),
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            use tokio::sync::RwLock;
            let awareness = Arc::new(RwLock::new(Awareness::new(doc)));
            match NativeConnection::new(awareness.clone(), url).await {
                Ok(conn) => connection = conn,
                Err(_) => return Err(()),
            }
        }
    
        Ok(Self {
            connection,
        })
    }
}

impl<N: RadiantNode> RadiantDocumentListener<N> for Collaborator {
    fn on_node_added(&mut self, document: &mut RadiantDocumentNode<N>, id: u64) {
        let Ok(connection) = self.connection.write() else { return };
        let awareness = connection.awareness();
        let Ok(awareness) = awareness.write() else { return };

        if let Some(node) = document.get_node(id) {
            let doc = awareness.doc();
            let root = doc.get_or_insert_map("radiantkit-root");

            let mut txn = doc.transact_mut();
            root.insert(&mut txn, id.to_string(), serde_json::to_string(node).unwrap());

            log::info!("count {}", root.len(&txn));
        }
    }
}
