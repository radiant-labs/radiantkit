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

#[derive(Serialize, Deserialize)]
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
    AddArtboard,
    SelectArtboard(u64),

    SelectNode(u64),

    Rectangle(u64, RadiantRectangleMessage),
}

pub struct RadiantScene {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub document: RadiantDocumentNode,
}

impl RadiantScene {
    pub fn new(surface: wgpu::Surface, device: wgpu::Device, queue: wgpu::Queue) -> Self {
        Self {
            surface,
            device,
            queue,
            document: RadiantDocumentNode::new(),
        }
    }
}

impl RadiantScene {
    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.document.update(&mut self.queue);

        let output = self.surface.get_current_texture()?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            self.document.render(&mut render_pass, false);
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

impl RadiantMessageHandler<RadiantMessage> for RadiantDocumentNode {
    fn handle_message(&mut self, message: RadiantMessage) {
        match message {
            RadiantMessage::AddArtboard => {
                self.add_artboard();
            }
            RadiantMessage::SelectArtboard(id) => {
                self.set_active_artboard(id);
            }
            RadiantMessage::SelectNode(id) => {
                self.select(id);
            }
            RadiantMessage::Rectangle(id, message) => {
                if let Some(node) = self.get_node_mut(id) {
                    node.handle_message(RadiantNodeMessage::Rectangle(message));
                }
            }
        }
    }
}