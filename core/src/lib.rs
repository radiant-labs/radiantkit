pub mod components;
pub mod artboard;
pub mod document;
pub mod rectangle;

pub use components::*;
pub use artboard::*;
pub use document::*;
pub use rectangle::*;

pub trait RadiantObserver<M> {
    fn on_notify(&self, message: &M);
}

pub trait RadiantObservable<M> {
    fn subscribe(&mut self, observer: Box<dyn RadiantObserver<M>>);
    fn unsubscribe(&mut self, observer: Box<dyn RadiantObserver<M>>);
    fn notify(&mut self, message: &M);
}

trait RadiantComponent<M> {
    fn observers(&mut self) -> &mut Vec<Box<dyn RadiantObserver<M>>>;
}

impl<M, T: RadiantComponent<M>> RadiantObservable<M> for T {
    fn subscribe(&mut self, observer: Box<dyn RadiantObserver<M>>) {
        self.observers().push(observer);
    }

    fn unsubscribe(&mut self, observer: Box<dyn RadiantObserver<M>>) {
        // self.observers().retain(|x| *x != observer);
    }

    fn notify(&mut self, message: &M) {
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

pub trait RadiantNode: RadiantNodeRenderable {
    fn set_selected(&mut self, selected: bool);
    fn set_id(&mut self, id: u64);
}
