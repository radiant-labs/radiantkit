use super::{RadiantNodeType, RadiantRenderable};
use crate::{RadiantArtboardNode, RadiantArtboardMessage, RadiantMessageHandler, RadiantIdentifiable, RadiantSelectable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum RadiantDocumentMessage {
    AddArtboard,
    SelectArtboard(u64),
    Artboard(u64, RadiantArtboardMessage),
}

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

    pub fn add(&mut self, node: RadiantNodeType) {
        if let Some(artboard) = self.artboards.get_mut(self.active_artboard_id as usize) {
            artboard.add(node);
            self.counter += 1;
        }
    }

    pub fn get_active_artboard(&self) -> Option<&RadiantArtboardNode> {
        self.artboards.get(self.active_artboard_id as usize)
    }

    pub fn select(&mut self, id: u64) {
        if let Some(artboard) = self.artboards.get_mut(self.active_artboard_id as usize) {
            artboard.select(id);
        }
    }
}

impl RadiantIdentifiable for RadiantDocumentNode {
    fn get_id(&self) -> u64 { 0 }
}

impl RadiantSelectable for RadiantDocumentNode {
    fn set_selected(&mut self, _selected: bool) {}
}

impl RadiantRenderable for RadiantDocumentNode {
    fn update(&mut self, queue: &mut wgpu::Queue) {
        if let Some(artboard) = self.artboards.get_mut(self.active_artboard_id as usize) {
            artboard.update(queue);
        }
    }

    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, offscreen: bool) {
        log::debug!("Rendering document");
        if let Some(artboard) = self.artboards.get(self.active_artboard_id as usize) {
            artboard.render(render_pass, offscreen);
        }
    }
}

impl RadiantMessageHandler<RadiantDocumentMessage> for RadiantDocumentNode {
    fn handle_message(&mut self, message: RadiantDocumentMessage) {
        match message {
            RadiantDocumentMessage::AddArtboard => {
                self.artboards.push(RadiantArtboardNode::new());
            }
            RadiantDocumentMessage::SelectArtboard(id) => {
                self.active_artboard_id = id;
            }
            RadiantDocumentMessage::Artboard(id, message) => {
                if let Some(artboard) = self.artboards.get_mut(id as usize) {
                    artboard.handle_message(message);
                }
            }
        }
    }
}
