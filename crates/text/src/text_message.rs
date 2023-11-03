use serde::{Deserialize, Serialize};
use macro_magic::export_tokens;

#[export_tokens]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RadiantTextMessage {
    AddText {
        text: String,
        position: [f32; 2],
    },
}

