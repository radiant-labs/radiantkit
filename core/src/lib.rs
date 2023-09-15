pub mod artboard;
pub mod components;
pub mod document;
pub mod nodes;
pub mod scene;
pub mod tools;
pub mod renderer;

pub use artboard::*;
pub use components::*;
pub use document::*;
use epaint::ClippedPrimitive;
pub use nodes::*;
pub use scene::*;
pub use tools::*;
pub use renderer::*;

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

/// Information about the screen used for rendering.
pub struct ScreenDescriptor {
    /// Size of the window in physical pixels.
    pub size_in_pixels: [u32; 2],

    /// HiDPI scale factor (pixels per point).
    pub pixels_per_point: f32,
}

impl ScreenDescriptor {
    /// size in "logical" points
    fn screen_size_in_points(&self) -> [f32; 2] {
        [
            self.size_in_pixels[0] as f32 / self.pixels_per_point,
            self.size_in_pixels[1] as f32 / self.pixels_per_point,
        ]
    }
}

pub trait RadiantRenderable {
    fn attach_to_scene(&mut self, scene: &mut RadiantScene);
    fn detach(&mut self);
    fn update_texture(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        id: epaint::TextureId,
        image_delta: &epaint::ImageDelta,
    );
    fn update_buffers(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, screen_descriptor: &ScreenDescriptor);
    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, screen_descriptor: &ScreenDescriptor, offscreen: bool);
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    pub fn attach_to_scene(&mut self, scene: &mut RadiantScene) {
        match self {
            RadiantNodeType::Document(node) => node.attach_to_scene(scene),
            RadiantNodeType::Artboard(node) => node.attach_to_scene(scene),
            RadiantNodeType::Rectangle(node) => node.attach_to_scene(scene),
        }
    }

    pub fn detach(&mut self) {
        match self {
            RadiantNodeType::Document(node) => node.detach(),
            RadiantNodeType::Artboard(node) => node.detach(),
            RadiantNodeType::Rectangle(node) => node.detach(),
        }
    }

    pub fn update_buffers(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, screen_descriptor: &ScreenDescriptor) {
        match self {
            RadiantNodeType::Document(node) => node.update_buffers(device, queue, screen_descriptor),
            RadiantNodeType::Artboard(node) => node.update_buffers(device, queue, screen_descriptor),
            RadiantNodeType::Rectangle(node) => node.update_buffers(device, queue, screen_descriptor),
        }
    }

    pub fn update_texture(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        id: epaint::TextureId,
        image_delta: &epaint::ImageDelta,
    ) {
        match self {
            RadiantNodeType::Document(node) => node.update_texture(device, queue, id, image_delta),
            RadiantNodeType::Artboard(node) => node.update_texture(device, queue, id, image_delta),
            RadiantNodeType::Rectangle(node) => node.update_texture(device, queue, id, image_delta),
        }
    }

    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, screen_descriptor: &ScreenDescriptor, offscreen: bool) {
        match self {
            RadiantNodeType::Document(node) => node.render(render_pass, screen_descriptor, offscreen),
            RadiantNodeType::Artboard(node) => node.render(render_pass, screen_descriptor, offscreen),
            RadiantNodeType::Rectangle(node) => node.render(render_pass, screen_descriptor, offscreen),
        }
    }

    pub fn get_primitives(&self) -> Vec<ClippedPrimitive> {
        match self {
            RadiantNodeType::Document(node) => vec![],
            RadiantNodeType::Artboard(node) => vec![],
            RadiantNodeType::Rectangle(node) => node.primitives.clone(),
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
    Rectangle(RadiantRectangleMessage),
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
    AddArtboard,
    SelectArtboard(u64),

    SelectNode(u64),

    Rectangle(u64, RadiantRectangleMessage),

    SelectTool(RadiantTool),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RadiantResponse {
    NodeSelected(RadiantNodeType),
}

impl RadiantScene {
    pub fn handle_message(&mut self, message: RadiantMessage) -> Option<RadiantResponse> {
        match message {
            RadiantMessage::AddArtboard => {
                self.document.add_artboard();
            }
            RadiantMessage::SelectArtboard(id) => {
                self.document.set_active_artboard(id);
            }
            RadiantMessage::SelectNode(id) => {
                self.document.select(id);
                if let Some(node) = self.document.get_node(id) {
                    return Some(RadiantResponse::NodeSelected(node.clone()));
                }
            }
            RadiantMessage::Rectangle(id, message) => {
                if let Some(node) = self.document.get_node_mut(id) {
                    node.handle_message(RadiantNodeMessage::Rectangle(message));
                }
            }
            RadiantMessage::SelectTool(tool) => {
                self.tool = tool;
            }
        }
        None
    }
}
