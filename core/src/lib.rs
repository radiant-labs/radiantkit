pub mod components;
pub mod interactions;
pub mod message;
pub mod nodes;
pub mod render;
pub mod scene;
pub mod tools;

pub use components::*;
pub use interactions::*;
pub use message::*;
pub use nodes::*;
pub use render::*;
pub use scene::*;
pub use tools::*;

use epaint::ClippedPrimitive;

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

pub trait RadiantTessellatable {
    fn attach_to_scene(&mut self, scene: &mut RadiantScene);
    fn detach(&mut self);

    fn set_needs_tessellation(&mut self);
    fn tessellate(
        &mut self,
        selection: bool,
        screen_descriptor: &ScreenDescriptor,
    ) -> Vec<ClippedPrimitive>;
}

pub trait RadiantNode: RadiantTessellatable {
    fn get_id(&self) -> u64;
    fn set_id(&mut self, id: u64);
    fn get_bounding_rect(&self) -> [f32; 4];
}

pub trait RadiantComponentProvider {
    fn get_component<T: RadiantComponent + 'static>(&self) -> Option<&T>;
    fn get_component_mut<T: RadiantComponent + 'static>(&mut self) -> Option<&mut T>;
}

pub trait RadiantInteraction {
    fn get_primitives(&self, selection: bool) -> Vec<ClippedPrimitive>;
}
