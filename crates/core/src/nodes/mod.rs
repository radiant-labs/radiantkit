pub mod group;
pub mod line;
pub mod rectangle;

pub use group::*;
pub use line::*;
pub use rectangle::*;
use serde::Serialize;
use uuid::Uuid;

use crate::{RadiantComponentProvider, ScreenDescriptor};
use epaint::ClippedPrimitive;

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

pub trait RadiantNode: Serialize + Clone + RadiantTessellatable + RadiantComponentProvider {
    fn get_id(&self) -> Uuid;
    fn set_id(&mut self, id: Uuid);
    fn get_bounding_rect(&self) -> [f32; 4];
    fn handle_key_down(&mut self, _key: crate::KeyCode) -> bool { false }
}
