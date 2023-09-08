use super::{RadiantNode, RadiantNodeRenderable};
use crate::RadiantArtboardNode;

pub struct RadiantDocumentNode {
    pub artboards: Vec<RadiantArtboardNode>,
    pub active_artboard_id: usize,
}

impl RadiantDocumentNode {
    pub fn new() -> Self {
        let artboards = vec![RadiantArtboardNode::new()];
        Self {
            artboards,
            active_artboard_id: 0,
        }
    }

    pub fn add(&mut self, node: Box<dyn RadiantNode>) {
        if let Some(artboard) = self.artboards.get_mut(self.active_artboard_id) {
            artboard.add(node);
        }
    }

    pub fn get_active_artboard(&self) -> Option<&RadiantArtboardNode> {
        self.artboards.get(self.active_artboard_id)
    }

    pub fn select(&mut self, id: u64) {
        if let Some(artboard) = self.artboards.get_mut(self.active_artboard_id) {
            artboard.select(id);
        }
    }
}

impl RadiantNode for RadiantDocumentNode {
    fn set_selected(&mut self, selected: bool) {

    }
    
    fn set_id(&mut self, id: u64) {

    }
}

impl RadiantNodeRenderable for RadiantDocumentNode {
    fn update(&mut self, queue: &mut wgpu::Queue) {
        if let Some(artboard) = self.artboards.get_mut(self.active_artboard_id) {
            artboard.update(queue);
        }
    }

    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, offscreen: bool) {
        log::debug!("Rendering document");
        if let Some(artboard) = self.artboards.get(self.active_artboard_id) {
            artboard.render(render_pass, offscreen);
        }
    }
}
