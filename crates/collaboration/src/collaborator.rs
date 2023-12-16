use radiantkit_core::{RadiantDocumentListener, RadiantDocumentNode, RadiantNode};
use uuid::Uuid;
use y_sync::awareness::Awareness;
use yrs::*;
use std::sync::{Arc, RwLock, Weak};

#[cfg(target_arch = "wasm32")]
use crate::wasm_connection::WasmConnection;
#[cfg(not(target_arch = "wasm32"))]
use crate::native_connection::NativeConnection;

pub struct Collaborator<N: RadiantNode> {
    _document: Weak<RwLock<RadiantDocumentNode<N>>>,
    #[cfg(target_arch = "wasm32")]
    connection: Arc<RwLock<WasmConnection>>,
    #[cfg(not(target_arch = "wasm32"))]
    connection: Arc<RwLock<NativeConnection>>,
    _sub: Option<UpdateSubscription>,
}

impl<'a, N: 'static + RadiantNode + serde::de::DeserializeOwned> Collaborator<N> {
    pub async fn new(client_id: u64, document: Weak<RwLock<RadiantDocumentNode<N>>>) -> Result<Self, ()> {
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

        let mut sub = None;
        if let Ok(connection) = connection.write() {
            let awareness = connection.awareness();
            let awareness = awareness.write();
            if let Ok(awareness) = awareness {
                let doc = awareness.doc();
                let document_clone = document.clone();
                sub = Some({
                    doc.observe_update_v1(move |txn, _e| {
                        log::info!("receiving update");
                        if let Some(root) = txn.get_map("radiantkit-root") {
                            let Some(document) = document_clone.upgrade() else { return };
                            let Ok(mut document) = document.try_write() else { return };
                            root.iter(txn).for_each(|(id, val)| {
                                let id = Uuid::parse_str(id).unwrap();
                                let node: String = val.cast().unwrap();
                                let mut node: N = serde_json::from_str(&node).unwrap();
                                node.set_needs_tessellation();
                                if document.get_node(id).is_none() {
                                    document.add_excluding_listener(node);
                                }
                            });
                        }
                    })
                    .unwrap()
                });
            }
        }
    
        Ok(Self {
            _document: document,
            connection,
            _sub: sub,
        })
    }
}

impl<N: RadiantNode> RadiantDocumentListener<N> for Collaborator<N> {
    fn on_node_added(&mut self, document: &mut RadiantDocumentNode<N>, id: Uuid) {
        let Ok(connection) = self.connection.write() else { return };
        let awareness = connection.awareness();
        let Ok(awareness) = awareness.write() else { return };

        if let Some(node) = document.get_node(id) {
            let doc = awareness.doc();
            let root = doc.get_or_insert_map("radiantkit-root");

            let mut txn = doc.transact_mut();
            root.insert(&mut txn, id.to_string(), serde_json::to_string(node).unwrap());
            txn.commit();

            log::info!("count {}", root.len(&txn));
        }
    }
}
