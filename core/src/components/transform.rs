use crate::{RadiantComponent, RadiantObservable, RadiantObserver};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum TransformMessage {
    SetPosition([f32; 3]),
    SetScale([f32; 3]),
    HandlePosition([f32; 3]),
    HandleScale([f32; 3]),
}

pub struct TransformComponent {
    position: [f32; 3],
    scale: [f32; 3],
    observers: Vec<Box<dyn RadiantObserver<TransformMessage>>>,
}

impl TransformComponent {
    pub fn new() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
            observers: Vec::new(),
        }
    }

    pub fn set_xy(&mut self, position: &[f32; 2]) {
        self.position = [position[0], position[1], 0.0];
        self.notify(&TransformMessage::SetPosition(self.position));
    }
}

impl RadiantComponent<TransformMessage> for TransformComponent {
    fn observers(&mut self) -> &mut Vec<Box<dyn RadiantObserver<TransformMessage>>> {
        &mut self.observers
    }
}
