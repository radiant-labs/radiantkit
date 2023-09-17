pub mod rect;
pub mod select;

pub use rect::*;
pub use select::*;

use crate::RadiantScene;

pub enum RadiantToolType {
    Selection(SelectionTool),
    Rectangle(RectangleTool),
}

pub trait RadiantTool {
    fn on_mouse_down(&mut self, scene: &mut RadiantScene, position: [f32; 2]) -> bool;
    fn on_mouse_move(&mut self, scene: &mut RadiantScene, position: [f32; 2]) -> bool;
    fn on_mouse_up(&mut self, scene: &mut RadiantScene, position: [f32; 2]) -> bool;
}

impl RadiantToolType {
    fn get_tool_mut(&mut self) -> &mut dyn RadiantTool {
        match self {
            RadiantToolType::Selection(tool) => tool,
            RadiantToolType::Rectangle(tool) => tool,
        }
    }
}

impl RadiantTool for RadiantToolType {
    fn on_mouse_down(&mut self, scene: &mut RadiantScene, position: [f32; 2]) -> bool {
        self.get_tool_mut().on_mouse_down(scene, position)
    }

    fn on_mouse_move(&mut self, scene: &mut RadiantScene, position: [f32; 2]) -> bool {
        self.get_tool_mut().on_mouse_move(scene, position)
    }

    fn on_mouse_up(&mut self, scene: &mut RadiantScene, position: [f32; 2]) -> bool {
        self.get_tool_mut().on_mouse_up(scene, position)
    }
}
