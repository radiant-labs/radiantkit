use std::collections::BTreeMap;

use crate::{
    RadiantGroupNode, RadiantNode, RadiantSelectable, RadiantTessellatable, ScreenDescriptor,
    SelectionComponent,
};
use epaint::ClippedPrimitive;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct RadiantDocumentNode<N: RadiantNode> {
    pub counter: u64,
    pub artboards: BTreeMap<Uuid, RadiantGroupNode<N>>,
    pub active_artboard_id: Uuid,
    pub selected_node_id: Option<Uuid>,
    #[serde(skip)]
    listeners: Vec<Box<dyn RadiantDocumentListener<N>>>,
}

unsafe impl<N: RadiantNode> Send for RadiantDocumentNode<N> {}
unsafe impl<N: RadiantNode> Sync for RadiantDocumentNode<N> {}

impl<N: RadiantNode> RadiantDocumentNode<N> {
    pub fn new() -> Self {
        let artboard_id = Uuid::new_v4();
        let mut artboards = BTreeMap::new();
        artboards.insert(artboard_id, RadiantGroupNode::new(artboard_id));
        Self {
            counter: 1,
            artboards,
            active_artboard_id: artboard_id,
            selected_node_id: None,
            listeners: Vec::new(),
        }
    }

    pub fn add_listener(&mut self, listener: Box<dyn RadiantDocumentListener<N>>) {
        self.listeners.push(listener);
    }

    pub fn remove_listener(&mut self, listener: &dyn RadiantDocumentListener<N>) {
        self.listeners.retain(|l| !std::ptr::eq(&**l, listener));
    }

    pub fn add_artboard(&mut self) {
        let id = Uuid::new_v4();
        self.artboards.insert(id, RadiantGroupNode::new(id));
        self.counter += 1;
    }

    pub fn add(&mut self, node: N) {
        if let Some(artboard) = self.artboards.get_mut(&self.active_artboard_id) {
            let id = node.get_id();
            artboard.add(node);

            let mut listeners = std::mem::take(&mut self.listeners);
            listeners.iter_mut().for_each(|listener| {
                listener.on_node_added(self, id);
            });
            self.listeners = listeners;

            self.counter += 1;
        }
    }

    pub fn add_excluding_listener(&mut self, node: N) {
        //, listener: &Box<dyn RadiantDocumentListener<N>>
        if let Some(artboard) = self.artboards.get_mut(&self.active_artboard_id) {
            artboard.add(node);

            // let id = node.get_id();
            // let mut listeners = std::mem::take(&mut self.listeners);
            // listeners
            //     .iter_mut()
            //     .filter(|l| !std::ptr::eq(&**l, listener))
            //     .for_each(|listener| {
            //         listener.on_node_added(self, id);
            //     });
            // self.listeners = listeners;

            self.counter += 1;
        }
    }

    pub fn set_active_artboard(&mut self, id: Uuid) {
        self.active_artboard_id = id;
    }

    pub fn get_active_artboard(&self) -> Option<&RadiantGroupNode<N>> {
        self.artboards.get(&self.active_artboard_id)
    }

    pub fn select(&mut self, id: Option<Uuid>) {
        if id == self.selected_node_id {
            return;
        }
        self.artboards.iter_mut().for_each(|artboard| {
            if let Some(prev_selected_node_id) = self.selected_node_id {
                if let Some(node) = artboard.1.get_node_mut(prev_selected_node_id) {
                    if let Some(component) = node.get_component_mut::<SelectionComponent>() {
                        component.set_selected(false);
                        node.set_needs_tessellation();
                    }
                }
            }
            if let Some(id) = id {
                if let Some(node) = artboard.1.get_node_mut(id) {
                    if let Some(component) = node.get_component_mut::<SelectionComponent>() {
                        component.set_selected(true);
                        node.set_needs_tessellation();
                    }
                }
            }
        });
        self.selected_node_id = id
    }

    pub fn get_node(&self, id: Uuid) -> Option<&N> {
        for artboard in &self.artboards {
            if let Some(node) = artboard.1.get_node(id) {
                return Some(node);
            }
        }
        None
    }

    pub fn get_node_mut(&mut self, id: Uuid) -> Option<&mut N> {
        for artboard in &mut self.artboards {
            if let Some(node) = artboard.1.get_node_mut(id) {
                return Some(node);
            }
        }
        None
    }
}

impl<N: RadiantNode> RadiantTessellatable for RadiantDocumentNode<N> {
    fn attach(&mut self, screen_descriptor: &ScreenDescriptor) {
        for artboard in &mut self.artboards {
            artboard.1.attach(screen_descriptor);
        }
    }

    fn detach(&mut self) {
        for artboard in &mut self.artboards {
            artboard.1.detach();
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
                primitives.append(&mut artboard.1.tessellate(
                    selection,
                    screen_descriptor,
                    fonts_manager,
                ));
                primitives
            })
    }
}

pub trait RadiantDocumentListener<N: RadiantNode> {
    fn on_node_added(&mut self, document: &mut RadiantDocumentNode<N>, id: Uuid);
}
