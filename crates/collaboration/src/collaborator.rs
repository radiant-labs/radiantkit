use parking_lot::RwLock;
use pollster::block_on;
use radiantkit_core::{RadiantDocumentListener, RadiantDocumentNode, RadiantNode};
use std::sync::{Arc, Weak};
use uuid::Uuid;
use y_sync::awareness::{Awareness, UpdateSubscription as AwarenessUpdateSubscription};
use yrs::{
    types::{map::MapEvent, EntryChange},
    *,
};

#[cfg(not(target_arch = "wasm32"))]
use crate::native_connection::NativeConnection;
#[cfg(target_arch = "wasm32")]
use crate::wasm_connection::WasmConnection;

#[cfg(not(target_arch = "wasm32"))]
type Connection = NativeConnection;
#[cfg(target_arch = "wasm32")]
type Connection = WasmConnection;

pub struct Collaborator<N: RadiantNode> {
    id: Uuid,
    _document: Weak<RwLock<RadiantDocumentNode<N>>>,
    connection: Arc<RwLock<Connection>>,
    _awareness_sub: Option<AwarenessUpdateSubscription>,
    _root_sub: Subscription<Arc<dyn Fn(&TransactionMut, &MapEvent)>>,
}

impl<'a, N: 'static + RadiantNode + serde::de::DeserializeOwned> Collaborator<N> {
    pub async fn new(
        client_id: u64,
        document: Weak<RwLock<RadiantDocumentNode<N>>>,
    ) -> Result<Self, ()> {
        let extension_id = Uuid::new_v4();
        let url = "ws://localhost:8000/sync";

        let doc = Doc::with_client_id(client_id);
        let mut root = doc.get_or_insert_map("radiantkit-root");
        let document_clone = document.clone();
        let root_sub = root.observe(move |txn, event| {
            log::error!("root event received");
            let Some(document) = document_clone.upgrade() else {
                return;
            };
            let Some(mut document) = document.try_write() else {
                return;
            };
            event
                .keys(txn)
                .iter()
                .for_each(|(key, change)| match change {
                    EntryChange::Inserted(val) => {
                        log::error!("inserted {} {}", key, val);
                        let id = Uuid::parse_str(key).unwrap();
                        let node: String = val.clone().cast().unwrap();
                        let mut node: N = serde_json::from_str(&node).unwrap();
                        node.set_needs_tessellation(false);
                        if document.get_node(id).is_none() {
                            document.add_excluding_listener(node, extension_id);
                        }
                    }
                    EntryChange::Removed(_val) => {}
                    EntryChange::Updated(_old, new) => {
                        log::error!("updated {} {}", key, new);
                        let id = Uuid::parse_str(key).unwrap();
                        if let Some(node) = document.get_node_mut(id) {
                            log::error!("replacing node");
                            let n: String = new.clone().cast().unwrap();
                            node.replace(&n);
                        }
                    }
                });
        });

        let connection;

        let mut awareness = Awareness::new(doc);
        let awareness_sub = Some(awareness.on_update(|_a, e| {
            log::error!("awareness event {:?}", e);
        }));

        #[cfg(target_arch = "wasm32")]
        {
            let awareness = Arc::new(RwLock::new(awareness));
            match WasmConnection::new(awareness.clone(), url) {
                Ok(conn) => connection = conn,
                Err(_) => return Err(()),
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            use tokio::sync::RwLock;
            let awareness = Arc::new(RwLock::new(awareness));
            match NativeConnection::new(awareness.clone(), url).await {
                Ok(conn) => connection = conn,
                Err(_) => return Err(()),
            }
        }

        Ok(Self {
            id: extension_id,
            _document: document,
            connection,
            _awareness_sub: awareness_sub,
            _root_sub: root_sub,
        })
    }
}

impl<N: RadiantNode> RadiantDocumentListener<N> for Collaborator<N> {
    fn get_id(&self) -> Uuid {
        self.id
    }

    fn on_node_added(&mut self, document: &RadiantDocumentNode<N>, id: Uuid) {
        block_on(async {
            let connection = self.connection.write();
            let awareness = connection.awareness();
            #[cfg(not(target_arch = "wasm32"))]
            let awareness = awareness.write().await;
            #[cfg(target_arch = "wasm32")]
            let Some(awareness) = awareness.try_write() else {
                return;
            };
            if let Some(node) = document.get_node(id) {
                let doc = awareness.doc();
                let Ok(mut txn) = doc.try_transact_mut() else {
                    log::error!("Failed to transact");
                    return;
                };
                if let Some(root) = txn.get_map("radiantkit-root") {
                    root.insert(
                        &mut txn,
                        id.to_string(),
                        serde_json::to_string(node).unwrap(),
                    );
                }
                txn.commit();
                log::error!("Added node {:?}", id);
            }
        });
    }

    fn on_node_changed(&mut self, id: Uuid, data: &str) {
        let connection_clone = self.connection.clone();
        #[cfg(not(target_arch = "wasm32"))]
        {
            let data = data.to_string();
            tokio::spawn(async move {
               handle_node_change(connection_clone, id, &data);
            });
        }
        
        #[cfg(target_arch = "wasm32")]
        handle_node_change(connection_clone, id, data);
    }
}

fn handle_node_change(connection: Arc<RwLock<Connection>>, id: Uuid, data: &str) {
    let connection = connection.write();
    let awareness = connection.awareness();
    #[cfg(not(target_arch = "wasm32"))]
    let Ok(awareness) = awareness.try_write() else { return };
    #[cfg(target_arch = "wasm32")]
    let Some(awareness) = awareness.try_write() else { return };
    let doc = awareness.doc();
    let Ok(mut txn) = doc.try_transact_mut() else {
        log::error!("Failed to transact");
        return;
    };
    if let Some(root) = txn.get_map("radiantkit-root") {
        root.insert(
            &mut txn,
            id.to_string(),
            data,
        );
    }
    txn.commit();
    log::error!("Updated node {:?}", id);
}
