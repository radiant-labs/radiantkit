pub mod rectangle_tool;
pub mod selection_tool;
pub mod tool_manager;

pub use rectangle_tool::*;
pub use selection_tool::*;
pub use tool_manager::*;

use crate::{RadiantDocumentNode, RadiantMessage};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum RadiantToolType {
    Selection,
    Rectangle,
}

impl RadiantToolType {
    pub fn get_tool(&self) -> Box<dyn RadiantTool> {
        match self {
            RadiantToolType::Selection => Box::new(SelectionTool::new()),
            RadiantToolType::Rectangle => Box::new(RectangleTool::new()),
        }
    }
}

pub trait RadiantTool {
    fn on_mouse_down(
        &mut self,
        node_id: u64,
        document: &RadiantDocumentNode,
        position: [f32; 2],
    ) -> Option<RadiantMessage>;
    fn on_mouse_move(
        &mut self,
        document: &RadiantDocumentNode,
        position: [f32; 2],
    ) -> Option<RadiantMessage>;
    fn on_mouse_up(
        &mut self,
        document: &RadiantDocumentNode,
        position: [f32; 2],
    ) -> Option<RadiantMessage>;
}
