use super::{
    RadiantArtboardNode, RadiantNode, RadiantNodeType, RadiantScene, RadiantTessellatable,
    ScreenDescriptor,
};
use epaint::ClippedPrimitive;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RadiantDocumentNode {
    pub counter: u64,
    pub artboards: Vec<RadiantArtboardNode>,
    pub active_artboard_id: u64,
}

impl RadiantDocumentNode {
    pub fn new() -> Self {
        let artboards = vec![RadiantArtboardNode::new()];
        Self {
            counter: 0,
            artboards,
            active_artboard_id: 0,
        }
    }

    pub fn add_artboard(&mut self) {
        self.artboards.push(RadiantArtboardNode::new());
    }

    pub fn add(&mut self, node: RadiantNodeType) {
        if let Some(artboard) = self.artboards.get_mut(self.active_artboard_id as usize) {
            artboard.add(node);
            self.counter += 1;
        }
    }

    pub fn set_active_artboard(&mut self, id: u64) {
        self.active_artboard_id = id;
    }

    pub fn get_active_artboard(&self) -> Option<&RadiantArtboardNode> {
        self.artboards.get(self.active_artboard_id as usize)
    }

    pub fn select(&mut self, id: u64) {
        if let Some(artboard) = self.artboards.get_mut(self.active_artboard_id as usize) {
            artboard.select(id);
        }
    }

    pub fn get_node(&self, id: u64) -> Option<&RadiantNodeType> {
        if let Some(artboard) = self.artboards.get(self.active_artboard_id as usize) {
            return artboard.get_node(id);
        }
        None
    }

    pub fn get_node_mut(&mut self, id: u64) -> Option<&mut RadiantNodeType> {
        if let Some(artboard) = self.artboards.get_mut(self.active_artboard_id as usize) {
            return artboard.get_node_mut(id);
        }
        None
    }
}

impl RadiantTessellatable for RadiantDocumentNode {
    fn attach_to_scene(&mut self, scene: &mut RadiantScene) {
        if let Some(artboard) = self.artboards.get_mut(self.active_artboard_id as usize) {
            artboard.attach_to_scene(scene);
        }
    }

    fn detach(&mut self) {
        if let Some(artboard) = self.artboards.get_mut(self.active_artboard_id as usize) {
            artboard.detach();
        }
    }

    fn set_needs_tessellation(&mut self) {}

    fn tessellate(
        &mut self,
        selection: bool,
        screen_descriptor: &ScreenDescriptor,
    ) -> Vec<ClippedPrimitive> {
        if let Some(artboard) = self.artboards.get_mut(self.active_artboard_id as usize) {
            return artboard.tessellate(selection, screen_descriptor);
        }
        Vec::new()
    }
}

impl RadiantNode for RadiantDocumentNode {
    fn get_id(&self) -> u64 {
        0
    }

    fn get_component<T: crate::RadiantComponent + 'static>(&self) -> Option<&T> {
        None
    }

    fn get_component_mut<T: crate::RadiantComponent + 'static>(&mut self) -> Option<&mut T> {
        None
    }
}
