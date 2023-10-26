pub mod line;
pub mod rectangle;

pub use line::*;
pub use rectangle::*;

use epaint::ClippedPrimitive;
use crate::{ScreenDescriptor, RadiantComponentProvider};

pub trait RadiantTessellatable {
    fn attach(&mut self, screen_descriptor: &ScreenDescriptor);
    fn detach(&mut self);

    fn set_needs_tessellation(&mut self);
    fn tessellate(
        &mut self,
        selection: bool,
        screen_descriptor: &ScreenDescriptor,
        fonts_manager: &epaint::text::Fonts,
    ) -> Vec<ClippedPrimitive>;
}

pub trait RadiantNode: RadiantTessellatable + RadiantComponentProvider {
    fn get_id(&self) -> u64;
    fn set_id(&mut self, id: u64);
    fn get_bounding_rect(&self) -> [f32; 4];
}
