use crate::RadiantMessageHandler;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum TransformMessage {
    SetPosition([f32; 3]),
    SetScale([f32; 3]),
}

pub struct TransformComponent {
    position: [f32; 3],
    scale: [f32; 3],
}

impl TransformComponent {
    pub fn new() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
        }
    }

    pub fn get_xy(&self) -> [f32; 2] {
        [self.position[0], self.position[1]]
    }

    pub fn set_xy(&mut self, position: &[f32; 2]) {
        self.position = [position[0], position[1], 0.0];
    }

    pub fn set_scale_xy(&mut self, scale: &[f32; 2]) {
        self.scale = [scale[0], scale[1], 0.0];
    }
}

impl RadiantMessageHandler<TransformMessage> for TransformComponent {
    fn handle_message(&mut self, message: TransformMessage) {
        match message {
            TransformMessage::SetPosition(position) => self.position = position,
            TransformMessage::SetScale(scale) => self.scale = scale,
        }
    }
}
