pub mod artboard;
pub mod document;
pub mod line;
pub mod path;
pub mod rectangle;

pub use artboard::*;
pub use document::*;
pub use line::*;
pub use path::*;
pub use rectangle::*;

use crate::{RadiantComponent, RadiantComponentProvider, RadiantScene, ScreenDescriptor};
use epaint::ClippedPrimitive;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RadiantNodeType {
    Document(RadiantDocumentNode),
    Artboard(RadiantArtboardNode),
    Rectangle(RadiantRectangleNode),
    Path(RadiantPathNode),
}

impl RadiantNodeType {
    fn get_node(&self) -> &dyn RadiantNode {
        match self {
            RadiantNodeType::Document(node) => node,
            RadiantNodeType::Artboard(node) => node,
            RadiantNodeType::Rectangle(node) => node,
            RadiantNodeType::Path(node) => node,
        }
    }

    fn get_node_mut(&mut self) -> &mut dyn RadiantNode {
        match self {
            RadiantNodeType::Document(node) => node,
            RadiantNodeType::Artboard(node) => node,
            RadiantNodeType::Rectangle(node) => node,
            RadiantNodeType::Path(node) => node,
        }
    }
}

impl RadiantTessellatable for RadiantNodeType {
    fn attach_to_scene(&mut self, scene: &mut RadiantScene) {
        self.get_node_mut().attach_to_scene(scene);
    }

    fn detach(&mut self) {
        self.get_node_mut().detach();
    }

    fn set_needs_tessellation(&mut self) {
        self.get_node_mut().set_needs_tessellation();
    }

    fn tessellate(
        &mut self,
        selection: bool,
        screen_descriptor: &ScreenDescriptor,
    ) -> Vec<ClippedPrimitive> {
        self.get_node_mut().tessellate(selection, screen_descriptor)
    }
}

impl RadiantNode for RadiantNodeType {
    fn get_id(&self) -> u64 {
        self.get_node().get_id()
    }

    fn set_id(&mut self, id: u64) {
        self.get_node_mut().set_id(id);
    }

    fn get_bounding_rect(&self) -> [f32; 4] {
        self.get_node().get_bounding_rect()
    }
}

impl RadiantComponentProvider for RadiantNodeType {
    fn get_component<T: RadiantComponent + 'static>(&self) -> Option<&T> {
        match self {
            RadiantNodeType::Document(node) => node.get_component(),
            RadiantNodeType::Artboard(node) => node.get_component(),
            RadiantNodeType::Rectangle(node) => node.get_component(),
            RadiantNodeType::Path(node) => node.get_component(),
        }
    }

    fn get_component_mut<T: RadiantComponent + 'static>(&mut self) -> Option<&mut T> {
        match self {
            RadiantNodeType::Document(node) => node.get_component_mut(),
            RadiantNodeType::Artboard(node) => node.get_component_mut(),
            RadiantNodeType::Rectangle(node) => node.get_component_mut(),
            RadiantNodeType::Path(node) => node.get_component_mut(),
        }
    }
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
