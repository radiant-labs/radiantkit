use epaint::ClippedPrimitive;
use radiant_core::{
    RadiantComponent, RadiantComponentProvider, RadiantNode, RadiantRectangleNode,
    RadiantTessellatable, ScreenDescriptor,
};
use radiant_core::{RadiantDocumentNode, RadiantGroupNode};
use radiant_image_node::RadiantImageNode;
use radiant_macros::{RadiantComponentProvider, RadiantNode, RadiantTessellatable};
use radiant_path_node::RadiantPathNode;
use radiant_text_node::RadiantTextNode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[derive(RadiantComponentProvider, RadiantNode, RadiantTessellatable)]
pub enum RadiantNodeType {
    Document(RadiantDocumentNode<RadiantNodeType>),
    Artboard(RadiantGroupNode<RadiantNodeType>),
    Rectangle(RadiantRectangleNode),
    Path(RadiantPathNode),
    Image(RadiantImageNode),
    Text(RadiantTextNode),
}
