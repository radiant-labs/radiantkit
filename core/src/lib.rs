pub mod artboard;
pub mod components;
pub mod document;
pub mod rectangle;

pub use artboard::*;
pub use components::*;
pub use document::*;
pub use rectangle::*;

use serde::{Deserialize, Serialize};

pub trait RadiantObserver<M> {
    fn on_notify(&mut self, message: M);
}

pub trait RadiantObservable<M> {
    fn subscribe(&mut self, observer: Box<dyn RadiantObserver<M>>);
    fn unsubscribe(&mut self, observer: Box<dyn RadiantObserver<M>>);
    fn notify(&mut self, message: M);
}

trait RadiantComponent<M> {
    fn observers(&mut self) -> &mut Vec<Box<dyn RadiantObserver<M>>>;
}

impl<M: Copy, T: RadiantComponent<M>> RadiantObservable<M> for T {
    fn subscribe(&mut self, observer: Box<dyn RadiantObserver<M>>) {
        self.observers().push(observer);
    }

    fn unsubscribe(&mut self, observer: Box<dyn RadiantObserver<M>>) {
        // self.observers().retain(|x| *x != observer);
    }

    fn notify(&mut self, message: M) {
        for observer in self.observers() {
            observer.on_notify(message);
        }
    }
}

pub trait RadiantNodeRenderable {
    // fn new(device: wgpu::Device, config: wgpu::SurfaceConfiguration) -> Self;
    fn update(&mut self, queue: &mut wgpu::Queue);
    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, offscreen: bool);
}

pub trait RadiantNode: RadiantNodeRenderable  {
    fn get_id(&self) -> u64;
    fn set_selected(&mut self, selected: bool);
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
pub enum RadiantMessage {
    Transform(TransformMessage),
    Selection(SelectionMessage),
}