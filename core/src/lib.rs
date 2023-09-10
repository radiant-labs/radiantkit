pub mod artboard;
pub mod components;
pub mod document;
pub mod nodes;

pub use artboard::*;
pub use components::*;
pub use document::*;
pub use nodes::*;

use serde::{Deserialize, Serialize};

pub trait RadiantMessageHandler<M> {
    fn handle_message(&mut self, message: M);
}

pub trait RadiantIdentifiable {
    fn get_id(&self) -> u64;
}

pub trait RadiantSelectable {
    fn set_selected(&mut self, selected: bool);
}

pub trait RadiantRenderable {
    fn update(&mut self, queue: &mut wgpu::Queue);
    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, offscreen: bool);
}

pub enum RadiantNodeType {
    Document(RadiantDocumentNode),
    Artboard(RadiantArtboardNode),
    Rectangle(RadiantRectangleNode),
}

impl RadiantIdentifiable for RadiantNodeType {
    fn get_id(&self) -> u64 {
        match self {
            RadiantNodeType::Document(node) => node.get_id(),
            RadiantNodeType::Artboard(node) => node.get_id(),
            RadiantNodeType::Rectangle(node) => node.get_id(),
        }
    }
}

impl RadiantSelectable for RadiantNodeType {
    fn set_selected(&mut self, selected: bool) {
        match self {
            RadiantNodeType::Document(node) => node.set_selected(selected),
            RadiantNodeType::Artboard(node) => node.set_selected(selected),
            RadiantNodeType::Rectangle(node) => node.set_selected(selected),
        }
    }
}

impl RadiantNodeType {
    pub fn update(&mut self, queue: &mut wgpu::Queue) {
        match self {
            RadiantNodeType::Document(node) => node.update(queue),
            RadiantNodeType::Artboard(node) => node.update(queue),
            RadiantNodeType::Rectangle(node) => node.update(queue),
        }
    }

    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, offscreen: bool) {
        match self {
            RadiantNodeType::Document(node) => node.render(render_pass, offscreen),
            RadiantNodeType::Artboard(node) => node.render(render_pass, offscreen),
            RadiantNodeType::Rectangle(node) => node.render(render_pass, offscreen),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RadiantVertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl RadiantVertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum RadiantNodeMessage {
    Rectangle(RadiantRectangleMessage)
}

impl RadiantMessageHandler<RadiantNodeMessage> for RadiantNodeType {
    fn handle_message(&mut self, message: RadiantNodeMessage) {
        match message {
            RadiantNodeMessage::Rectangle(message) => {
                if let RadiantNodeType::Rectangle(node) = self {
                    node.handle_message(message);
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum RadiantMessage {
    Document(RadiantDocumentMessage),
}