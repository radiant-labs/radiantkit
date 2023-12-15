use macro_magic::export_tokens;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[export_tokens]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RadiantAddTextMessage {
    AddText { text: String, position: [f32; 2] },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RadiantTextMessage {
    SetText { id: Uuid, text: String },
}

impl RadiantTextMessage {
    pub fn id(&self) -> Uuid {
        match self {
            RadiantTextMessage::SetText { id, .. } => *id,
        }
    }
}
