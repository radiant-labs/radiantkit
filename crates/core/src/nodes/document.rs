use crate::{
    RadiantComponentProvider, RadiantNode, RadiantComponent,
    RadiantSelectable, RadiantTessellatable, ScreenDescriptor, SelectionComponent, RadiantGroupNode
};
use epaint::ClippedPrimitive;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RadiantDocumentNode<N: RadiantNode> {
    pub counter: u64,
    pub artboards: Vec<RadiantGroupNode<N>>,
    pub active_artboard_id: u64,
    pub selected_node_id: Option<u64>,
}

impl<N: RadiantNode> RadiantDocumentNode<N> {
    pub fn new() -> Self {
        let artboards = vec![RadiantGroupNode::new(0)];
        Self {
            counter: 1,
            artboards,
            active_artboard_id: 0,
            selected_node_id: None,
        }
    }

    pub fn add_artboard(&mut self) {
        self.artboards.push(RadiantGroupNode::new(self.counter));
        self.counter += 1;
    }

    pub fn add(&mut self, node: N) {
        if let Some(artboard) = self.artboards.get_mut(self.active_artboard_id as usize) {
            artboard.add(node);
            self.counter += 1;
        }
    }

    pub fn set_active_artboard(&mut self, id: u64) {
        self.active_artboard_id = id;
    }

    pub fn get_active_artboard(&self) -> Option<&RadiantGroupNode<N>> {
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

    pub fn get_node(&self, id: u64) -> Option<&N> {
        for artboard in &self.artboards {
            if let Some(node) = artboard.get_node(id) {
                return Some(node);
            }
        }
        None
    }

    pub fn get_node_mut(&mut self, id: u64) -> Option<&mut N> {
        for artboard in &mut self.artboards {
            if let Some(node) = artboard.get_node_mut(id) {
                return Some(node);
            }
        }
        None
    }
}

impl<N: RadiantNode> RadiantTessellatable for RadiantDocumentNode<N> {
    fn attach(&mut self, screen_descriptor: &ScreenDescriptor) {
        for artboard in &mut self.artboards {
            artboard.attach(screen_descriptor);
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
        fonts_manager: &epaint::text::Fonts,
    ) -> Vec<ClippedPrimitive> {
        self.artboards
            .iter_mut()
            .fold(Vec::new(), |mut primitives, artboard| {
                primitives.append(&mut artboard.tessellate(
                    selection,
                    screen_descriptor,
                    fonts_manager,
                ));
                primitives
            })
    }
}

impl<N: RadiantNode> RadiantNode for RadiantDocumentNode<N> {
    fn get_id(&self) -> u64 {
        0
    }

    fn set_id(&mut self, _id: u64) {}

    fn get_bounding_rect(&self) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }
}

impl<N: RadiantNode> RadiantComponentProvider for RadiantDocumentNode<N> {
    fn get_component<T: RadiantComponent + 'static>(&self) -> Option<&T> {
        None
    }

    fn get_component_mut<T: RadiantComponent + 'static>(&mut self) -> Option<&mut T> {
        None
    }
}
