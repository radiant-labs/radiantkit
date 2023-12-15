use crate::{RadiantSceneMessage, RadiantTool};
use macro_magic::export_tokens;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[export_tokens]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RadiantRectangleMessage {
    AddRectangle { id: Option<uuid::Uuid>, position: [f32; 2], scale: [f32; 2] },
}

pub struct RectangleTool {
    active_node_id: Option<Uuid>,
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

impl<M: From<RadiantRectangleMessage> + From<RadiantSceneMessage>> RadiantTool<M>
    for RectangleTool
{
    fn on_mouse_down(&mut self, _node_id: Option<Uuid>,position: [f32; 2]) -> Option<M> {
        let id = Uuid::new_v4();
        let message = RadiantRectangleMessage::AddRectangle {
            id: Some(id),
            position,
            scale: [10.0, 10.0],
        };
        self.active_node_id = Some(id);
        self.start_position = position;
        self.prev_position = position;
        Some(message.into())
    }

    fn on_mouse_move(&mut self, position: [f32; 2]) -> Option<M> {
        let result = if let Some(id) = self.active_node_id {
            let message = RadiantSceneMessage::TransformNode {
                id: id,
                position: [0.0, 0.0],
                scale: [
                    position[0] - self.prev_position[0],
                    position[1] - self.prev_position[1],
                ],
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
        self.start_position = [0.0, 0.0];
        self.prev_position = [0.0, 0.0];
        None
    }
}
