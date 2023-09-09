use super::{RadiantNode, RadiantNodeRenderable};
use crate::RadiantArtboardNode;

pub struct RadiantDocumentNode {
    pub counter: u64,
    pub artboards: Vec<RadiantArtboardNode>,
    pub active_artboard_id: usize,
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

    pub fn add(&mut self, node: Box<dyn RadiantNode>) {
        if let Some(artboard) = self.artboards.get_mut(self.active_artboard_id) {
            artboard.add(node);
            self.counter += 1;
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
    fn set_selected(&mut self, selected: bool) {}
    fn get_id(&self) -> u64 { 0 }
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
