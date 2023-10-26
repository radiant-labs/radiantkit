use serde::{Deserialize, Serialize};
use radiant_core::{RadiantComponent, RadiantComponentProvider, ScreenDescriptor, RadiantNode, RadiantRectangleNode, RadiantTessellatable};
use epaint::ClippedPrimitive;
use radiant_image_node::RadiantImageNode;
use radiant_text_node::RadiantTextNode;
use radiant_path_node::RadiantPathNode;
use crate::{RadiantDocumentNode, RadiantArtboardNode};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RadiantNodeType {
    Document(RadiantDocumentNode),
    Artboard(RadiantArtboardNode),
    Rectangle(RadiantRectangleNode),
    Path(RadiantPathNode),
    Image(RadiantImageNode),
    Text(RadiantTextNode),
}

impl RadiantTessellatable for RadiantNodeType {
    fn attach(&mut self, screen_descriptor: &ScreenDescriptor) {
        match self {
            RadiantNodeType::Document(node) => node.attach(screen_descriptor),
            RadiantNodeType::Artboard(node) => node.attach(screen_descriptor),
            RadiantNodeType::Rectangle(node) => node.attach(screen_descriptor),
            RadiantNodeType::Path(node) => node.attach(screen_descriptor),
            RadiantNodeType::Image(node) => node.attach(screen_descriptor),
            RadiantNodeType::Text(node) => node.attach(screen_descriptor),
        }
    }

    fn detach(&mut self) {
        match self {
            RadiantNodeType::Document(node) => node.detach(),
            RadiantNodeType::Artboard(node) => node.detach(),
            RadiantNodeType::Rectangle(node) => node.detach(),
            RadiantNodeType::Path(node) => node.detach(),
            RadiantNodeType::Image(node) => node.detach(),
            RadiantNodeType::Text(node) => node.detach(),
        }
    }

    fn set_needs_tessellation(&mut self) {
        match self {
            RadiantNodeType::Document(node) => node.set_needs_tessellation(),
            RadiantNodeType::Artboard(node) => node.set_needs_tessellation(),
            RadiantNodeType::Rectangle(node) => node.set_needs_tessellation(),
            RadiantNodeType::Path(node) => node.set_needs_tessellation(),
            RadiantNodeType::Image(node) => node.set_needs_tessellation(),
            RadiantNodeType::Text(node) => node.set_needs_tessellation(),
        }
    }

    fn tessellate(
        &mut self,
        selection: bool,
        screen_descriptor: &ScreenDescriptor,
        fonts_manager: &epaint::text::Fonts,
    ) -> Vec<ClippedPrimitive> {
        match self {
            RadiantNodeType::Document(node) => node.tessellate(selection, screen_descriptor, fonts_manager),
            RadiantNodeType::Artboard(node) => node.tessellate(selection, screen_descriptor, fonts_manager),
            RadiantNodeType::Rectangle(node) => node.tessellate(selection, screen_descriptor, fonts_manager),
            RadiantNodeType::Path(node) => node.tessellate(selection, screen_descriptor, fonts_manager),
            RadiantNodeType::Image(node) => node.tessellate(selection, screen_descriptor, fonts_manager),
            RadiantNodeType::Text(node) => node.tessellate(selection, screen_descriptor, fonts_manager),
        }
    }
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
