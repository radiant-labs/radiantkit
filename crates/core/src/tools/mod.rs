pub mod rectangle_tool;
pub mod selection_tool;
pub mod tool_manager;

pub use rectangle_tool::*;
pub use selection_tool::*;
pub use tool_manager::*;

pub trait RadiantTool<M> {
    fn on_mouse_down(&mut self, node_id: u64, node_count: u64, position: [f32; 2]) -> Option<M>;
    fn on_mouse_move(&mut self, position: [f32; 2]) -> Option<M>;
    fn on_mouse_up(&mut self, position: [f32; 2]) -> Option<M>;
}
