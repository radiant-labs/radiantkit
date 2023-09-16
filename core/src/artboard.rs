use super::{
    RadiantNode, RadiantNodeType, RadiantScene, RadiantSelectable, RadiantTessellatable,
    ScreenDescriptor, SelectionComponent,
};
use epaint::ClippedPrimitive;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RadiantArtboardNode {
    pub is_active: bool,
    pub nodes: Vec<RadiantNodeType>,
    pub selected_node_ids: HashSet<u64>,
}

impl RadiantArtboardNode {
    pub fn new() -> Self {
        Self {
            is_active: true,
            nodes: Vec::new(),
            selected_node_ids: HashSet::new(),
        }
    }

    pub fn add(&mut self, node: RadiantNodeType) {
        self.nodes.push(node);
    }

    pub fn select(&mut self, id: u64) {
        self.selected_node_ids.iter().for_each(|id| {
            if let Some(node) = self.nodes.get_mut(*id as usize) {
                if let Some(component) = node.get_component_mut::<SelectionComponent>() {
                    component.set_selected(false);
                    node.set_needs_tessellation();
                }
            }
        });
        if let Some(node) = self.nodes.get_mut(id as usize) {
            if let Some(component) = node.get_component_mut::<SelectionComponent>() {
                component.set_selected(true);
                node.set_needs_tessellation();
            }
        }
        self.selected_node_ids.insert(id);
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
        0
    }

    fn get_component<T: crate::RadiantComponent + 'static>(&self) -> Option<&T> {
        None
    }

    fn get_component_mut<T: crate::RadiantComponent + 'static>(&mut self) -> Option<&mut T> {
        None
    }
}
