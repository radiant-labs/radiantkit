use epaint::ClippedPrimitive;
use radiant_core::{
    RadiantComponent, RadiantComponentProvider, RadiantNode, RadiantRectangleNode,
    RadiantTessellatable, ScreenDescriptor,
};
use radiant_core::{RadiantDocumentNode, RadiantGroupNode};
use radiant_image_node::RadiantImageNode;
use radiant_macros::RadiantTessellatable;
use radiant_path_node::RadiantPathNode;
use radiant_text_node::RadiantTextNode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[derive(RadiantTessellatable)]
pub enum RadiantNodeType {
    Document(RadiantDocumentNode<RadiantNodeType>),
    Artboard(RadiantGroupNode<RadiantNodeType>),
    Rectangle(RadiantRectangleNode),
    Path(RadiantPathNode),
    Image(RadiantImageNode),
    Text(RadiantTextNode),
}

impl RadiantNode for RadiantNodeType {
    fn get_id(&self) -> u64 {
        match self {
            RadiantNodeType::Document(node) => node.get_id(),
            RadiantNodeType::Artboard(node) => node.get_id(),
            RadiantNodeType::Rectangle(node) => node.get_id(),
            RadiantNodeType::Path(node) => node.get_id(),
            RadiantNodeType::Image(node) => node.get_id(),
            RadiantNodeType::Text(node) => node.get_id(),
        }
    }

    fn set_id(&mut self, id: u64) {
        match self {
            RadiantNodeType::Document(node) => node.set_id(id),
            RadiantNodeType::Artboard(node) => node.set_id(id),
            RadiantNodeType::Rectangle(node) => node.set_id(id),
            RadiantNodeType::Path(node) => node.set_id(id),
            RadiantNodeType::Image(node) => node.set_id(id),
            RadiantNodeType::Text(node) => node.set_id(id),
        }
    }

    fn get_bounding_rect(&self) -> [f32; 4] {
        match self {
            RadiantNodeType::Document(node) => node.get_bounding_rect(),
            RadiantNodeType::Artboard(node) => node.get_bounding_rect(),
            RadiantNodeType::Rectangle(node) => node.get_bounding_rect(),
            RadiantNodeType::Path(node) => node.get_bounding_rect(),
            RadiantNodeType::Image(node) => node.get_bounding_rect(),
            RadiantNodeType::Text(node) => node.get_bounding_rect(),
        }
    }
}

impl RadiantComponentProvider for RadiantNodeType {
    fn get_component<T: RadiantComponent + 'static>(&self) -> Option<&T> {
        match self {
            RadiantNodeType::Document(node) => node.get_component(),
            RadiantNodeType::Artboard(node) => node.get_component(),
            RadiantNodeType::Rectangle(node) => node.get_component(),
            RadiantNodeType::Path(node) => node.get_component(),
            RadiantNodeType::Image(node) => node.get_component(),
            RadiantNodeType::Text(node) => node.get_component(),
        }
    }

    fn get_component_mut<T: RadiantComponent + 'static>(&mut self) -> Option<&mut T> {
        match self {
            RadiantNodeType::Document(node) => node.get_component_mut(),
            RadiantNodeType::Artboard(node) => node.get_component_mut(),
            RadiantNodeType::Rectangle(node) => node.get_component_mut(),
            RadiantNodeType::Path(node) => node.get_component_mut(),
            RadiantNodeType::Image(node) => node.get_component_mut(),
            RadiantNodeType::Text(node) => node.get_component_mut(),
        }
    }
}
