pub mod components;
pub mod interactions;
pub mod message;
pub mod nodes;
pub mod render;
pub mod document;
pub mod scene;
pub mod texture;
pub mod tools;
pub mod utils;

use std::{sync::{RwLockReadGuard, RwLockWriteGuard}, collections::HashMap};

pub use components::*;
use epaint::Color32;
pub use interactions::*;
pub use message::*;
pub use nodes::*;
pub use render::*;
pub use document::*;
pub use scene::*;
pub use texture::*;
pub use tools::*;
pub use utils::*;

/// Information about the screen used for rendering.
#[derive(Clone, Copy)]
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

pub trait View<M: From<RadiantSceneMessage> + TryInto<RadiantSceneMessage>, N: RadiantNode> {
    fn scene(&self) -> RwLockReadGuard<RadiantScene<M, N>>;
    fn scene_mut(&mut self) -> RwLockWriteGuard<RadiantScene<M, N>>;
}

pub trait Runtime<
    'a,
    M: From<RadiantSceneMessage> + TryInto<RadiantSceneMessage> + 'a,
    N: RadiantNode + 'a,
    R: 'a,
>
{
    type View: View<M, N>;

    fn view(&self) -> &Self::View;
    fn view_mut(&mut self) -> &mut Self::View;

    fn handle_message(&mut self, message: M) -> Option<R>;

    fn scene(&'a self) -> RwLockReadGuard<RadiantScene<M, N>> {
        self.view().scene()
    }
    fn scene_mut(&'a mut self) -> RwLockWriteGuard<RadiantScene<M, N>> {
        self.view_mut().scene_mut()
    }

    fn add(&'a mut self, node: N) {
        self.scene_mut().add(node);
    }
}

use once_cell::sync::Lazy;
use uuid::Uuid; 
use std::sync::Mutex;

static COUNTER : Lazy<Mutex<u64>> = Lazy::new(|| Mutex::new(0));
static NODE_TO_COLORS: Lazy<Mutex<HashMap<Uuid, Color32>>> = Lazy::new(|| Mutex::new(HashMap::new()));
static COLOR_TO_NODES: Lazy<Mutex<HashMap<Color32, Uuid>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub fn get_color_for_node(node_id: Uuid) -> Color32 {
    let mut colors = NODE_TO_COLORS.lock().unwrap();
    if let Some(color) = colors.get(&node_id) {
        return *color;
    }
    let mut counter = COUNTER.lock().unwrap();
    *counter += 1;
    let c = *counter;
    let color = Color32::from_rgb(
        (c >> 0) as u8 & 0xFF,
        (c >> 8) as u8 & 0xFF,
        (c >> 16) as u8 & 0xFF,
    );
    colors.insert(node_id, color);

    let mut color_to_nodes = COLOR_TO_NODES.lock().unwrap();
    color_to_nodes.insert(color, node_id);

    color
}

pub fn get_node_for_color(color: Color32) -> Option<Uuid> {
    let color_to_nodes = COLOR_TO_NODES.lock().unwrap();
    color_to_nodes.get(&color).copied()
}