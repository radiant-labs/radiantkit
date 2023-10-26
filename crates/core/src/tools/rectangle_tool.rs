use crate::{RadiantMessage, RadiantTool};

pub struct RectangleTool {
    active_node_id: Option<u64>,
    start_position: [f32; 2],
    prev_position: [f32; 2],
}

impl RectangleTool {
    pub fn new() -> Self {
        Self {
            active_node_id: None,
            start_position: [0.0, 0.0],
            prev_position: [0.0, 0.0],
        }
    }
}

impl RadiantTool for RectangleTool {
    fn tool_id(&self) -> u32 {
        1
    }

    fn on_mouse_down(
        &mut self,
        node_id: u64,
        position: [f32; 2],
    ) -> Option<RadiantMessage> {
        let message = RadiantMessage::AddNode {
            node_type: String::from("Rectangle"),
            position,
            scale: [10.0, 10.0],
        };
        self.active_node_id = Some(node_id);
        self.start_position = position;
        self.prev_position = position;
        Some(message)
    }

    fn on_mouse_move(
        &mut self,
        position: [f32; 2],
    ) -> Option<RadiantMessage> {
        let result = if let Some(id) = self.active_node_id {
            let message = RadiantMessage::TransformNode {
                id: id,
                position: [0.0, 0.0],
                scale: [
                    position[0] - self.prev_position[0],
                    position[1] - self.prev_position[1],
                ],
            };
            Some(message)
        } else {
            None
        };
        self.prev_position = position;
        result
    }

    fn on_mouse_up(
        &mut self,
        _position: [f32; 2],
    ) -> Option<RadiantMessage> {
        self.active_node_id = None;
        self.start_position = [0.0, 0.0];
        self.prev_position = [0.0, 0.0];
        None
    }
}
