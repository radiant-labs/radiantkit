use std::{collections::BTreeMap, cell::RefCell, rc::Rc};

use crate::{
    RadiantGroupNode, RadiantNode, RadiantSelectable, RadiantTessellatable, ScreenDescriptor,
    SelectionComponent, SubscriptionId,
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
    listeners: Rc<RefCell<Vec<Box<dyn RadiantDocumentListener<N>>>>>,
    #[serde(skip)]
    subscriptions: Vec<SubscriptionId>,
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
            listeners: Rc::new(Vec::new().into()),
            subscriptions: Vec::new(),
        }
    }

    pub fn add_listener(&mut self, listener: Box<dyn RadiantDocumentListener<N>>) {
        self.listeners.borrow_mut().push(listener);
    }

    pub fn remove_listener(&mut self, listener: &dyn RadiantDocumentListener<N>) {
        self.listeners.borrow_mut().retain(|l| !std::ptr::eq(&**l, listener));
    }

    pub fn add_artboard(&mut self) {
        let id = Uuid::new_v4();
        self.artboards.insert(id, RadiantGroupNode::new(id));
        self.counter += 1;
    }

    pub fn add(&mut self, node: N) {
        self.add_node(node, None);
    }

    pub fn add_excluding_listener(&mut self, node: N, listener_id: Uuid) {
        self.add_node(node, Some(listener_id));
    }

    fn add_node(&mut self, mut node: N, exclude_listener_id: Option<Uuid>) {
        if let Some(artboard) = self.artboards.get_mut(&self.active_artboard_id) {
            let id = node.get_id();
            let listeners = self.listeners.clone();
            let subscription = node.observe(move |data| {
                listeners.borrow_mut().iter_mut().for_each(|listener| {
                    listener.on_node_changed(id, data);
                });
            });
            artboard.add(node);

            let listeners = self.listeners.clone();
            listeners.borrow_mut()
                .iter_mut()
                .filter(|l| match exclude_listener_id {
                    Some(id) => l.get_id() != id,
                    None => true,
                })
                .for_each(|listener| {
                    listener.on_node_added(self, id);
                });

            self.subscriptions.push(subscription.into());
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
                        node.set_needs_tessellation(true);
                    }
                }
            }
            if let Some(id) = id {
                if let Some(node) = artboard.1.get_node_mut(id) {
                    if let Some(component) = node.get_component_mut::<SelectionComponent>() {
                        component.set_selected(true);
                        node.set_needs_tessellation(true);
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

    pub fn replace_node(&mut self, id: Uuid, node: N) {
        for artboard in &mut self.artboards {
            if artboard.1.get_node_mut(id).is_some() {
                artboard.1.replace_node(id, node);
                return;
            }
        }
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

    fn set_needs_tessellation(&mut self, _notify: bool) {}

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
    fn get_id(&self) -> Uuid;
    fn on_node_added(&mut self, document: &RadiantDocumentNode<N>, node: Uuid);
    fn on_node_changed(&mut self, id: Uuid, data: &str);
}
