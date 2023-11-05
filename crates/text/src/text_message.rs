use macro_magic::export_tokens;
use serde::{Deserialize, Serialize};

#[export_tokens]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RadiantTextMessage {
    AddText { text: String, position: [f32; 2] },
}
