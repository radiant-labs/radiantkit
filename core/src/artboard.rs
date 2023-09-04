use super::{RadiantNode, RadiantNodeRenderable};
use std::sync::Arc;

pub struct RadiantArtboardNode {
    pub nodes: Vec<Arc<dyn RadiantNode>>,
}

impl RadiantArtboardNode {
    pub fn new() -> Self {
        Self { nodes: vec![] }
    }

    pub fn add(&mut self, node: Arc<dyn RadiantNode>) {
        self.nodes.push(node);
    }
}

impl RadiantNodeRenderable for RadiantArtboardNode {
    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        log::debug!("Rendering artboard");
        // self.nodes.iter().for_each(|node| node.render(render_pass));
        for node in &self.nodes {
            node.render(render_pass);
        }
    }
}
