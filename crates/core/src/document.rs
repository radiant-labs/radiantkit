use crate::{
    RadiantGroupNode, RadiantNode, RadiantSelectable,
    RadiantTessellatable, ScreenDescriptor, SelectionComponent,
};
use epaint::ClippedPrimitive;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RadiantDocumentNode<N: RadiantNode> {
    pub counter: u64,
    pub artboards: Vec<RadiantGroupNode<N>>,
    pub active_artboard_id: u64,
    pub selected_node_id: Option<u64>,
    #[serde(skip)]
    listeners: Vec<Box<dyn RadiantDocumentListener<N>>>,
}

unsafe impl<N: RadiantNode> Send for RadiantDocumentNode<N> {}
unsafe impl<N: RadiantNode> Sync for RadiantDocumentNode<N> {}

impl<N: RadiantNode> RadiantDocumentNode<N> {
    pub fn new() -> Self {
        let artboards = vec![RadiantGroupNode::new(0)];
        Self {
            counter: 1,
            artboards,
            active_artboard_id: 0,
            selected_node_id: None,
            listeners: Vec::new(),
        }
    }

    pub fn add_listener(&mut self, listener: Box<dyn RadiantDocumentListener<N>>) {
        self.listeners.push(listener);
    }

    pub fn remove_listener(&mut self, listener: &dyn RadiantDocumentListener<N>) {
        self.listeners
            .retain(|l| !std::ptr::eq(&**l, listener));
    }

    pub fn add_artboard(&mut self) {
        self.artboards.push(RadiantGroupNode::new(self.counter));
        self.counter += 1;
    }

    pub fn add(&mut self, node: N) {
        if let Some(artboard) = self.artboards.get_mut(self.active_artboard_id as usize) {
            artboard.add(node.clone());

            let id = self.counter;
            let mut listeners = std::mem::take(&mut self.listeners);
            listeners.iter_mut().for_each(|listener| {
                listener.on_node_added(self, id);
            });
            self.listeners = listeners;
 
            self.counter += 1;
        }
    }

    pub fn set_active_artboard(&mut self, id: u64) {
        self.active_artboard_id = id;
    }

    pub fn get_active_artboard(&self) -> Option<&RadiantGroupNode<N>> {
        self.artboards.get(self.active_artboard_id as usize)
    }

    pub fn select(&mut self, id: Option<u64>) {
        if id == self.selected_node_id {
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
            if let Some(id) = id {
                if let Some(node) = artboard.get_node_mut(id) {
                    if let Some(component) = node.get_component_mut::<SelectionComponent>() {
                        component.set_selected(true);
                        node.set_needs_tessellation();
                    }
                }
            }
        });
        self.selected_node_id = id
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

pub trait RadiantDocumentListener<N: RadiantNode> {
    fn on_node_added(&mut self, document: &mut RadiantDocumentNode<N>, id: u64);
}
