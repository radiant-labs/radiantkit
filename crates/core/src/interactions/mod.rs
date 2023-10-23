pub mod bounding_box;
pub mod interaction_manager;

pub use bounding_box::*;
pub use interaction_manager::*;

use epaint::ClippedPrimitive;

pub trait RadiantInteraction {
    fn get_primitives(&self, selection: bool) -> Vec<ClippedPrimitive>;
}
