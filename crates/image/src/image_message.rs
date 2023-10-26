use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RadiantImageMessage {
    AddImage {
        name: String,
        path: String,
    },
}

