use uuid::Uuid;

use crate::{RadiantSceneMessage, RadiantTool};

pub struct SelectionTool {
    active_node_id: Option<Uuid>,
    prev_position: [f32; 2],
    is_mouse_down: bool,
}

impl SelectionTool {
    pub fn new() -> Self {
        Self {
            active_node_id: None,
            prev_position: [0.0, 0.0],
            is_mouse_down: false,
        }
    }
}

impl<M: From<RadiantSceneMessage>> RadiantTool<M> for SelectionTool {
    fn on_mouse_down(&mut self, node_id: Option<Uuid>, position: [f32; 2]) -> Option<M> {
        self.prev_position = position;
        self.is_mouse_down = true;
        self.active_node_id = node_id;
        Some(RadiantSceneMessage::SelectNode { id: node_id }.into())
    }

    fn on_mouse_move(&mut self, position: [f32; 2]) -> Option<M> {
        if !self.is_mouse_down {
            return None;
        }
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
        self.is_mouse_down = false;
        self.prev_position = [0.0, 0.0];
        None
    }

    fn on_key_down(&mut self, key: crate::KeyCode) -> Option<M> {
        return Some(RadiantSceneMessage::HandleKey { id: None, key }.into());
    }
}
