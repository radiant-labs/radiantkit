use crate::{
    RadiantArtboardNode, RadiantNode, RadiantNodeType, RadiantScene, RadiantSelectable,
    RadiantTessellatable, ScreenDescriptor, SelectionComponent,
};
use epaint::ClippedPrimitive;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RadiantDocumentNode {
    pub counter: u64,
    pub artboards: Vec<RadiantArtboardNode>,
    pub active_artboard_id: u64,
    pub selected_node_id: Option<u64>,
}

impl RadiantDocumentNode {
    pub fn new() -> Self {
        let artboards = vec![RadiantArtboardNode::new(0)];
        Self {
            counter: 1,
            artboards,
            active_artboard_id: 0,
            selected_node_id: None,
        }
    }

    pub fn add_artboard(&mut self) {
        self.artboards.push(RadiantArtboardNode::new(self.counter));
        self.counter += 1;
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
        if Some(id) == self.selected_node_id {
            return;
        }
        self.artboards.iter_mut().for_each(|artboard| {
            if let Some(prev_selected_node_id) = self.selected_node_id {
                if let Some(node) = artboard.get_node_mut(prev_selected_node_id) {
                    if let Some(component) = node.get_component_mut::<SelectionComponent>() {
                        component.set_selected(false);
                        node.set_needs_tessellation();
                    }
                }
            }
            if let Some(node) = artboard.get_node_mut(id) {
                if let Some(component) = node.get_component_mut::<SelectionComponent>() {
                    component.set_selected(true);
                    node.set_needs_tessellation();
                }
            }
        });
        self.selected_node_id = Some(id);
    }

    pub fn get_node(&self, id: u64) -> Option<&RadiantNodeType> {
        for artboard in &self.artboards {
            if let Some(node) = artboard.get_node(id) {
                return Some(node);
            }
        }
        None
    }

    pub fn get_node_mut(&mut self, id: u64) -> Option<&mut RadiantNodeType> {
        for artboard in &mut self.artboards {
            if let Some(node) = artboard.get_node_mut(id) {
                return Some(node);
            }
        }
        None
    }
}

impl RadiantTessellatable for RadiantDocumentNode {
    fn attach_to_scene(&mut self, scene: &mut RadiantScene) {
        for artboard in &mut self.artboards {
            artboard.attach_to_scene(scene);
        }
    }

    fn detach(&mut self) {
        for artboard in &mut self.artboards {
            artboard.detach();
        }
    }

    fn set_needs_tessellation(&mut self) {}

    fn tessellate(
        &mut self,
        selection: bool,
        screen_descriptor: &ScreenDescriptor,
    ) -> Vec<ClippedPrimitive> {
        self.artboards
            .iter_mut()
            .fold(Vec::new(), |mut primitives, artboard| {
                primitives.append(&mut artboard.tessellate(selection, screen_descriptor));
                primitives
            })
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
