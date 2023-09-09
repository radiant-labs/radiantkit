use crate::{RadiantComponent, RadiantObservable, RadiantObserver, RadiantMessage};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum TransformMessage {
    SetPosition(u64, [f32; 3]),
    SetScale(u64, [f32; 3]),
    HandlePosition(u64, [f32; 3]),
    HandleScale(u64, [f32; 3]),
}

impl From<TransformMessage> for RadiantMessage {
    fn from(message: TransformMessage) -> Self {
        RadiantMessage::Transform(message)
    }
}

impl TryFrom<RadiantMessage> for TransformMessage {
    type Error = ();

    fn try_from(message: RadiantMessage) -> Result<Self, Self::Error> {
        match message {
            RadiantMessage::Transform(message) => Ok(message),
            _ => Err(()),
        }
    }
}

pub struct TransformComponent {
    id: u64,
    position: [f32; 3],
    scale: [f32; 3],
    observers: Vec<Box<dyn RadiantObserver<RadiantMessage>>>,
}

impl TransformComponent {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            position: [0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
            observers: Vec::new(),
        }
    }

    pub fn set_xy(&mut self, position: &[f32; 2]) {
        self.position = [position[0], position[1], 0.0];
        self.notify(TransformMessage::HandlePosition(self.id, self.position).into());
    }
}

impl RadiantComponent<RadiantMessage> for TransformComponent {
    fn observers(&mut self) -> &mut Vec<Box<dyn RadiantObserver<RadiantMessage>>> {
        &mut self.observers
    }
}

impl RadiantObserver<RadiantMessage> for TransformComponent {
    fn on_notify(&mut self, message: RadiantMessage) {
        if let Ok(message) = message.try_into() {
            match message {
                TransformMessage::SetPosition(id, position) => {
                    if id == self.id {
                        self.set_xy(&[position[0], position[1]]);
                    }
                }
                TransformMessage::SetScale(id, scale) => {
                    if id == self.id {
                        
                    }
                }
                _ => {}
            }
        }
    }
}
