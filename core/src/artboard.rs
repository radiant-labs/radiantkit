use super::{RadiantNodeType, RadiantRenderable, RadiantMessageHandler, RadiantNodeMessage, RadiantIdentifiable, RadiantSelectable};
use std::collections::HashSet;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum RadiantArtboardMessage {
    SelectNode(u64),
    Node(u64, RadiantNodeMessage),
}

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
}

impl RadiantIdentifiable for RadiantArtboardNode {
    fn get_id(&self) -> u64 { 0 }
}

impl RadiantSelectable for RadiantArtboardNode {
    fn set_selected(&mut self, selected: bool) {
        self.is_active = selected;
    }
}

impl RadiantRenderable for RadiantArtboardNode {
    fn update(&mut self, queue: &mut wgpu::Queue) {
        for node in &mut self.nodes {
            node.update(queue);
        }
    }

    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, offscreen: bool) {
        log::debug!("Rendering artboard");
        for node in &self.nodes {
            node.render(render_pass, offscreen);
        }
    }
}

impl RadiantMessageHandler<RadiantArtboardMessage> for RadiantArtboardNode {
    fn handle_message(&mut self, message: RadiantArtboardMessage) {
        match message {
            RadiantArtboardMessage::SelectNode(id) => {
                self.select(id);
            }
            RadiantArtboardMessage::Node(id, message) => {
                if let Some(node) = self.nodes.get_mut(id as usize) {
                    node.handle_message(message);
                }
            }
        }
    }
}
