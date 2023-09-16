use super::{
    RadiantNode, RadiantNodeType, RadiantScene, RadiantTessellatable, ScreenDescriptor,
    SelectionComponent,
};
use epaint::ClippedPrimitive;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RadiantArtboardNode {
    pub id: u64,
    pub selection: SelectionComponent,
    pub nodes: Vec<RadiantNodeType>,
}

impl RadiantArtboardNode {
    pub fn new(id: u64) -> Self {
        let selection = SelectionComponent::new();
        Self {
            id,
            selection,
            nodes: Vec::new(),
        }
    }

    pub fn add(&mut self, node: RadiantNodeType) {
        self.nodes.push(node);
    }

    pub fn get_node(&self, id: u64) -> Option<&RadiantNodeType> {
        self.nodes.get(id as usize)
    }

    pub fn get_node_mut(&mut self, id: u64) -> Option<&mut RadiantNodeType> {
        self.nodes.get_mut(id as usize)
    }
}

impl RadiantTessellatable for RadiantArtboardNode {
    fn attach_to_scene(&mut self, scene: &mut RadiantScene) {
        for node in &mut self.nodes {
            node.attach_to_scene(scene);
        }
    }

    fn detach(&mut self) {
        for node in &mut self.nodes {
            node.detach();
        }
    }

    fn set_needs_tessellation(&mut self) {}

    fn tessellate(
        &mut self,
        selection: bool,
        screen_descriptor: &ScreenDescriptor,
    ) -> Vec<ClippedPrimitive> {
        let mut primitives = Vec::new();
        for node in &mut self.nodes {
            primitives.append(&mut node.tessellate(selection, screen_descriptor));
        }
        primitives
    }
}

impl RadiantNode for RadiantArtboardNode {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn get_component<T: crate::RadiantComponent + 'static>(&self) -> Option<&T> {
        None
    }

    fn get_component_mut<T: crate::RadiantComponent + 'static>(&mut self) -> Option<&mut T> {
        None
    }
}
