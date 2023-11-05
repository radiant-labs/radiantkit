use crate::{RadiantSceneMessage, RadiantTool};

pub struct SelectionTool {
    active_node_id: Option<u64>,
    prev_position: [f32; 2],
}

impl SelectionTool {
    pub fn new() -> Self {
        Self {
            active_node_id: None,
            prev_position: [0.0, 0.0],
        }
    }
}

impl<M: From<RadiantSceneMessage>> RadiantTool<M> for SelectionTool {
    fn on_mouse_down(&mut self, node_id: u64, _node_count: u64, _position: [f32; 2]) -> Option<M> {
        Some(
            if node_id > 0 {
                self.active_node_id = Some(node_id - 1);
                RadiantSceneMessage::SelectNode {
                    id: Some(node_id - 1),
                }
            } else {
                self.active_node_id = None;
                RadiantSceneMessage::SelectNode { id: None }
            }
            .into(),
        )
    }

    fn on_mouse_move(&mut self, position: [f32; 2]) -> Option<M> {
        let result = if let Some(id) = self.active_node_id {
            let message = RadiantSceneMessage::TransformNode {
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
