use crate::{BaseNode, RadiantNode, RadiantTessellatable, ScreenDescriptor};
use epaint::ClippedPrimitive;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RadiantGroupNode<N: RadiantNode> {
    pub base: BaseNode,
    pub nodes: BTreeMap<Uuid, N>,
}

impl<N: RadiantNode> RadiantGroupNode<N> {
    pub fn new(id: Uuid) -> Self {
        let base = BaseNode::new(id, [0.0, 0.0].into(), [0.0, 0.0].into());
        Self {
            base,
            nodes: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, node: N) {
        self.nodes.insert(node.get_id(), node);
    }

    pub fn get_node(&self, id: Uuid) -> Option<&N> {
        self.nodes.get(&id)
    }

    pub fn get_node_mut(&mut self, id: Uuid) -> Option<&mut N> {
        self.nodes.get_mut(&id)
    }
}

impl<N: RadiantNode> RadiantTessellatable for RadiantGroupNode<N> {
    fn attach(&mut self, screen_descriptor: &ScreenDescriptor) {
        for node in &mut self.nodes.values_mut() {
            node.attach(screen_descriptor);
        }
    }

    fn detach(&mut self) {
        for node in &mut self.nodes.values_mut() {
            node.detach();
        }
    }

    fn set_needs_tessellation(&mut self) {}

    fn tessellate(
        &mut self,
        selection: bool,
        screen_descriptor: &ScreenDescriptor,
        fonts_manager: &epaint::text::Fonts,
    ) -> Vec<ClippedPrimitive> {
        let mut primitives = Vec::new();
        for node in &mut self.nodes.values_mut() {
            primitives.append(&mut node.tessellate(selection, screen_descriptor, fonts_manager));
        }
        primitives
    }
}

impl<N: RadiantNode> RadiantNode for RadiantGroupNode<N> {
    fn base(&self) -> &BaseNode {
        &self.base
    }

    fn base_mut(&mut self) -> &mut BaseNode {
        &mut self.base
    }

    fn handle_key_down(&mut self, key: crate::KeyCode) -> bool {
        for node in &mut self.nodes.values_mut() {
            if node.handle_key_down(key.clone()) {
                return true;
            }
        }
        false
    }
}
