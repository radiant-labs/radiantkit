use crate::{RadiantRectangleNode, RadiantSceneMessage, RadiantTool};

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
    fn on_mouse_down(&mut self, scene: &mut crate::RadiantScene, position: [f32; 2]) -> bool {
        let node_id = scene.document.counter;
        let message = RadiantSceneMessage::AddNode(crate::RadiantNodeType::Rectangle(
            RadiantRectangleNode::new(node_id, position, [10.0, 10.0]),
        ));
        scene.process_message(message);
        self.active_node_id = Some(node_id);
        self.start_position = position;
        self.prev_position = position;
        true
    }

    fn on_mouse_move(&mut self, scene: &mut crate::RadiantScene, position: [f32; 2]) -> bool {
        let result = if let Some(id) = self.active_node_id {
            let message = RadiantSceneMessage::TransformNode {
                id: id,
                position: [0.0, 0.0],
                scale: [
                    position[0] - self.prev_position[0],
                    position[1] - self.prev_position[1],
                ],
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
        self.start_position = [0.0, 0.0];
        self.prev_position = [0.0, 0.0];
        false
    }
}
