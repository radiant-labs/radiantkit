use crate::{
    RadiantComponentProvider, RadiantNode, RadiantTessellatable,
    ScreenDescriptor, SelectionComponent, RadiantComponent,
};
use epaint::ClippedPrimitive;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RadiantGroupNode<N: RadiantNode> {
    pub id: u64,
    pub selection: SelectionComponent,
    pub nodes: BTreeMap<u64, N>,
}

impl<N: RadiantNode> RadiantGroupNode<N> {
    pub fn new(id: u64) -> Self {
        let selection = SelectionComponent::new();
        Self {
            id,
            selection,
            nodes: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, node: N) {
        self.nodes.insert(node.get_id(), node);
    }

    pub fn get_node(&self, id: u64) -> Option<&N> {
        self.nodes.get(&id)
    }

    pub fn get_node_mut(&mut self, id: u64) -> Option<&mut N> {
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
    fn get_id(&self) -> u64 {
        self.id
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn get_bounding_rect(&self) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }
}

impl<N: RadiantNode> RadiantComponentProvider for RadiantGroupNode<N> {
    fn get_component<T: RadiantComponent + 'static>(&self) -> Option<&T> {
        None
    }

    fn get_component_mut<T: RadiantComponent + 'static>(&mut self) -> Option<&mut T> {
        None
    }
}
