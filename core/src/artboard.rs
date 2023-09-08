use std::collections::{HashMap, HashSet};
use super::{RadiantNode, RadiantNodeRenderable};

pub struct RadiantArtboardNode {
    pub counter: u64,
    pub nodes: HashMap<u64, Box<dyn RadiantNode>>,
    pub selected_node_ids: HashSet<u64>
}

impl RadiantArtboardNode {
    pub fn new() -> Self {
        Self { 
            counter: 0,
            nodes: HashMap::new(),
            selected_node_ids: HashSet::new()
        }
    }

    pub fn add(&mut self, node: Box<dyn RadiantNode>) {
        self.nodes.insert(self.counter, node);
        self.counter += 1;
    }

    pub fn select(&mut self, id: u64) {
        self.selected_node_ids.iter().for_each(|id| {
            if let Some(node) = self.nodes.get_mut(id) {
                node.set_selected(false);
            }
        });
        if let Some(node) = self.nodes.get_mut(&id) {
            node.set_selected(true);
        }
        self.selected_node_ids.insert(id);
    }
}

impl RadiantNode for RadiantArtboardNode {
    fn set_selected(&mut self, selected: bool) {

    }
    
    fn set_id(&mut self, id: u64) {

    }
}

impl RadiantNodeRenderable for RadiantArtboardNode {
    fn update(&mut self, queue: &mut wgpu::Queue) {
        for node in &mut self.nodes.values_mut() {
            node.update(queue);
        }
    }

    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, offscreen: bool) {
        log::debug!("Rendering artboard");
        for node in self.nodes.values() {
            node.render(render_pass, offscreen);
        }
    }
}
