pub mod artboard;
pub mod document;
pub mod rectangle;

pub use artboard::*;
pub use document::*;
pub use rectangle::*;

pub trait RadiantNodeRenderable {
    // fn new(device: wgpu::Device, config: wgpu::SurfaceConfiguration) -> Self;
    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>);
}

pub trait RadiantNode: RadiantNodeRenderable {}

impl<T: RadiantNodeRenderable> RadiantNode for T {}
