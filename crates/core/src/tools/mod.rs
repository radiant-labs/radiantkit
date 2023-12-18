pub mod rectangle_tool;
pub mod selection_tool;
pub mod tool_manager;

pub use rectangle_tool::*;
pub use selection_tool::*;
pub use tool_manager::*;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum KeyCode {
    Backspace,
    Delete,
    Enter,
    Escape,
    Space,
    Tab,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    Char(String),
}

pub trait RadiantTool<M>: Send + Sync {
    fn on_mouse_down(&mut self, _node_id: Option<Uuid>, _position: [f32; 2]) -> Option<M> {
        None
    }
    fn on_mouse_move(&mut self, _position: [f32; 2]) -> Option<M> {
        None
    }
    fn on_mouse_up(&mut self, _position: [f32; 2]) -> Option<M> {
        None
    }
    fn on_key_down(&mut self, _key: KeyCode) -> Option<M> {
        None
    }
}
