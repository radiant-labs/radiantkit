pub mod color;
pub mod selection;
pub mod transform;

pub use color::*;
pub use selection::*;
pub use transform::*;

pub trait RadiantComponent {}

pub trait RadiantSelectable: RadiantComponent {
    fn set_selected(&mut self, selected: bool);
}

pub trait RadiantComponentProvider {
    fn get_component<T: RadiantComponent + 'static>(&self) -> Option<&T>;
    fn get_component_mut<T: RadiantComponent + 'static>(&mut self) -> Option<&mut T>;
}
