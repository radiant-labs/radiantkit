use macro_magic::export_tokens;
use serde::{Deserialize, Serialize};

#[export_tokens]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RadiantVideoMessage {
    AddVideo { name: String, path: String },
    PlayVideo { id: u64 },
}
