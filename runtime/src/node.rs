use epaint::ClippedPrimitive;
use radiantkit_core::{
    RadiantGroupNode, RadiantNode, RadiantRectangleNode, RadiantTessellatable, ScreenDescriptor,
};
use radiantkit_image::RadiantImageNode;
use radiantkit_macros::{RadiantNode, RadiantTessellatable};
use radiantkit_path::RadiantPathNode;
use radiantkit_text::RadiantTextNode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, RadiantNode, RadiantTessellatable)]
pub enum RadiantNodeType {
    Artboard(RadiantGroupNode<RadiantNodeType>),
    Rectangle(RadiantRectangleNode),
    Path(RadiantPathNode),
    Image(RadiantImageNode),
    Text(RadiantTextNode),
    #[cfg(all(not(target_arch = "wasm32"), feature = "video"))]
    Video(radiantkit_video::RadiantVideoNode),
}
