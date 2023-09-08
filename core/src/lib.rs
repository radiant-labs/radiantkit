pub mod artboard;
pub mod document;
pub mod rectangle;

pub use artboard::*;
pub use document::*;
pub use rectangle::*;

pub trait RadiantNodeRenderable {
    // fn new(device: wgpu::Device, config: wgpu::SurfaceConfiguration) -> Self;
    fn update(&mut self, queue: &mut wgpu::Queue);
    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, offscreen: bool);
}

pub trait RadiantNode: RadiantNodeRenderable {
    fn set_selected(&mut self, selected: bool);
    fn set_id(&mut self, id: u64);
}
