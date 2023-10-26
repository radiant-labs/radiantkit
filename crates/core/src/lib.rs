pub mod components;
pub mod interactions;
pub mod nodes;
pub mod render;
pub mod texture;
pub mod tools;

pub use components::*;
pub use interactions::*;
pub use nodes::*;
pub use render::*;
pub use texture::*;
pub use tools::*;

/// Information about the screen used for rendering.
pub struct ScreenDescriptor {
    /// Size of the window in physical pixels.
    pub size_in_pixels: [u32; 2],

    /// HiDPI scale factor (pixels per point).
    pub pixels_per_point: f32,
}

impl ScreenDescriptor {
    /// size in "logical" points
    pub fn screen_size_in_points(&self) -> [f32; 2] {
        [
            self.size_in_pixels[0] as f32 / self.pixels_per_point,
            self.size_in_pixels[1] as f32 / self.pixels_per_point,
        ]
    }
}