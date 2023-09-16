pub mod artboard;
pub mod components;
pub mod document;
pub mod message;
pub mod nodes;
pub mod renderer;
pub mod scene;
pub mod tools;

pub use artboard::*;
pub use components::*;
pub use document::*;
use epaint::ClippedPrimitive;
pub use message::*;
pub use nodes::*;
pub use renderer::*;
pub use scene::*;
pub use tools::*;

use serde::{Deserialize, Serialize};

pub trait RadiantComponent {}

pub trait RadiantSelectable: RadiantComponent {
    fn set_selected(&mut self, selected: bool);
}

pub trait RadiantTransformable: RadiantComponent {
    fn set_xy(&mut self, position: &[f32; 2]);
    fn set_scale(&mut self, scale: &[f32; 2]);
    fn set_rotation(&mut self, rotation: f32);
    fn get_xy(&self) -> [f32; 2];
    fn get_scale(&self) -> [f32; 2];
    fn get_rotation(&self) -> f32;
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
    fn get_component<T: RadiantComponent + 'static>(&self) -> Option<&T>;
    fn get_component_mut<T: RadiantComponent + 'static>(&mut self) -> Option<&mut T>;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RadiantNodeType {
    Document(RadiantDocumentNode),
    Artboard(RadiantArtboardNode),
    Rectangle(RadiantRectangleNode),
    Path(RadiantPathNode),
}

impl RadiantNodeType {
    fn get_tessellatable_mut(&mut self) -> &mut dyn RadiantTessellatable {
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
        self.get_tessellatable_mut().attach_to_scene(scene);
    }

    fn detach(&mut self) {
        self.get_tessellatable_mut().detach();
    }

    fn set_needs_tessellation(&mut self) {
        self.get_tessellatable_mut().set_needs_tessellation();
    }

    fn tessellate(
        &mut self,
        selection: bool,
        screen_descriptor: &ScreenDescriptor,
    ) -> Vec<ClippedPrimitive> {
        self.get_tessellatable_mut()
            .tessellate(selection, screen_descriptor)
    }
}

impl RadiantNode for RadiantNodeType {
    fn get_id(&self) -> u64 {
        match self {
            RadiantNodeType::Document(node) => node.get_id(),
            RadiantNodeType::Artboard(node) => node.get_id(),
            RadiantNodeType::Rectangle(node) => node.get_id(),
            RadiantNodeType::Path(node) => node.get_id(),
        }
    }

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
