use super::{RadiantIdentifiable, RadiantNodeType, RadiantRenderable, RadiantSelectable};
use crate::RadiantScene;
use epaint::ClippedPrimitive;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use crate::ScreenDescriptor;

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
                node.set_selected(false);
            }
        });
        if let Some(node) = self.nodes.get_mut(id as usize) {
            node.set_selected(true);
        }
        self.selected_node_ids.insert(id);
    }

    pub fn get_node(&self, id: u64) -> Option<&RadiantNodeType> {
        self.nodes.get(id as usize)
    }

    pub fn get_node_mut(&mut self, id: u64) -> Option<&mut RadiantNodeType> {
        self.nodes.get_mut(id as usize)
    }

    pub fn get_primitives(&self, selection: bool) -> Vec<ClippedPrimitive> {
        let mut primitives = Vec::new();
        for node in &self.nodes {
            primitives.append(&mut node.get_primitives(selection));
        }
        primitives
    }
}

impl RadiantIdentifiable for RadiantArtboardNode {
    fn get_id(&self) -> u64 {
        0
    }
}

impl RadiantSelectable for RadiantArtboardNode {
    fn set_selected(&mut self, selected: bool) {
        self.is_active = selected;
    }
}

impl RadiantRenderable for RadiantArtboardNode {
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
}
