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

impl RadiantTool for SelectionTool {
    fn on_mouse_down(&mut self, scene: &mut crate::RadiantScene, position: [f32; 2]) -> bool {
        self.prev_position = position;
        let id = pollster::block_on(scene.select(position));
        if id > 0 {
            self.active_node_id = Some(id - 1);
            let message = RadiantSceneMessage::SelectNode(id - 1);
            scene.process_message(message);
            true
        } else {
            false
        }
    }

    fn on_mouse_move(&mut self, scene: &mut crate::RadiantScene, position: [f32; 2]) -> bool {
        let result = if let Some(id) = self.active_node_id {
            let message = RadiantSceneMessage::TransformNode {
                id: id,
                position: [
                    position[0] - self.prev_position[0],
                    position[1] - self.prev_position[1],
                ],
                scale: [0.0, 0.0],
            };
            scene.process_message(message);
            true
        } else {
            false
        };
        self.prev_position = position;
        result
    }

    fn on_mouse_up(&mut self, _scene: &mut crate::RadiantScene, _position: [f32; 2]) -> bool {
        self.active_node_id = None;
        self.prev_position = [0.0, 0.0];
        false
    }
}
