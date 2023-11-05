use macro_magic::export_tokens;
use radiant_core::RadiantTool;
use serde::{Deserialize, Serialize};

// Todo: This is a stub. Implement the PathTool.
#[export_tokens]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PathToolMessage {
    SelectNode {
        id: u64,
    },
    TransformNode {
        id: u64,
        position: [f32; 2],
        scale: [f32; 2],
    },
}

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

impl<M: From<PathToolMessage>> RadiantTool<M> for PathTool {
    fn on_mouse_down(&mut self, node_id: u64, _position: [f32; 2]) -> Option<M> {
        if node_id > 0 {
            self.active_node_id = Some(node_id - 1);
            let message = PathToolMessage::SelectNode { id: node_id - 1 };
            Some(message.into())
        } else {
            None
        }
    }

    fn on_mouse_move(&mut self, position: [f32; 2]) -> Option<M> {
        let result = if let Some(id) = self.active_node_id {
            let message = PathToolMessage::TransformNode {
                id: id,
                position: [
                    position[0] - self.prev_position[0],
                    position[1] - self.prev_position[1],
                ],
                scale: [0.0, 0.0],
            };
            Some(message.into())
        } else {
            None
        };
        self.prev_position = position;
        result
    }

    fn on_mouse_up(&mut self, _position: [f32; 2]) -> Option<M> {
        self.active_node_id = None;
        self.prev_position = [0.0, 0.0];
        None
    }
}
