use super::artboard::RadiantArtboardNode;
use super::{RadiantNode, RadiantNodeRenderable};

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
            artboard.add(node.into());
        }
    }
}

impl RadiantNodeRenderable for RadiantDocumentNode {
    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, offscreen: bool) {
        log::debug!("Rendering document");
        if let Some(artboard) = self.artboards.get(self.active_artboard_id) {
            artboard.render(render_pass, offscreen);
        }
    }
}
