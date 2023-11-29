use macro_magic::export_tokens;
use serde::{Deserialize, Serialize};

#[export_tokens]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RadiantAddTextMessage {
    AddText { text: String, position: [f32; 2] },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RadiantTextMessage {
    SetText { id: u64, text: String },
}

impl RadiantTextMessage {
    pub fn id(&self) -> u64 {
        match self {
            RadiantTextMessage::SetText { id, .. } => *id,
        }
    }
}
