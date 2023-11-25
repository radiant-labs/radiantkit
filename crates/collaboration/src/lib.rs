use radiantkit_core::{RadiantDocumentListener, RadiantDocumentNode, RadiantNode};
use yrs::*;

pub struct Collaborator {
    doc: Doc,
}

impl Collaborator {
    pub fn new() -> Self {
        Self {
            doc: Doc::new(),
        }
    }
}

impl<N: RadiantNode> RadiantDocumentListener<N> for Collaborator {
    fn on_node_added(&mut self, document: &mut RadiantDocumentNode<N>, id: u64) {
        let root = self.doc.get_or_insert_text("radiantkit-root");
        let mut txn = self.doc.transact_mut();
        if let Some(node) = document.get_node(id) {
            root.push(&mut txn, &serde_json::to_string(node).unwrap());
        }
    }
}
