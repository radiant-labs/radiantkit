use radiant_core::{RadiantMessage, RadiantTool};

// Todo: This is a stub. Implement the PathTool.
pub struct PathTool {
    active_node_id: Option<u64>,
    prev_position: [f32; 2],
}

impl PathTool {
    pub fn new() -> Self {
        Self {
            active_node_id: None,
            prev_position: [0.0, 0.0],
        }
    }
}

impl RadiantTool for PathTool {
    fn tool_id(&self) -> u32 {
        2
    }

    fn on_mouse_down(
        &mut self,
        node_id: u64,
        _position: [f32; 2],
    ) -> Option<RadiantMessage> {
        if node_id > 0 {
            self.active_node_id = Some(node_id - 1);
            let message = RadiantMessage::SelectNode(node_id - 1);
            Some(message)
        } else {
            None
        }
    }

    fn on_mouse_move(
        &mut self,
        position: [f32; 2],
    ) -> Option<RadiantMessage> {
        let result = if let Some(id) = self.active_node_id {
            let message = RadiantMessage::TransformNode {
                id: id,
                position: [
                    position[0] - self.prev_position[0],
                    position[1] - self.prev_position[1],
                ],
                scale: [0.0, 0.0],
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
        self.prev_position = [0.0, 0.0];
        None
    }
}
