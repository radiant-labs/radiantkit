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

pub trait RadiantTransformable: RadiantComponent {
    fn transform_xy(&mut self, position: &[f32; 2]);
    fn transform_scale(&mut self, scale: &[f32; 2]);
    fn set_xy(&mut self, position: &[f32; 2]);
    fn set_scale(&mut self, scale: &[f32; 2]);
    fn set_rotation(&mut self, rotation: f32);
    fn get_xy(&self) -> [f32; 2];
    fn get_scale(&self) -> [f32; 2];
    fn get_rotation(&self) -> f32;
}

pub trait RadiantComponentProvider {
    fn get_component<T: RadiantComponent + 'static>(&self) -> Option<&T>;
    fn get_component_mut<T: RadiantComponent + 'static>(&mut self) -> Option<&mut T>;
}
