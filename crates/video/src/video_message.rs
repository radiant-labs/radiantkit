use macro_magic::export_tokens;
use serde::{Deserialize, Serialize};

#[export_tokens]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RadiantVideoMessage {
    AddVideo { id: Option<uuid::Uuid>, name: String, path: String },
    PlayVideo { id: uuid::Uuid },
}
