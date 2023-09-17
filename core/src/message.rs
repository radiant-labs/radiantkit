use crate::RadiantNodeType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RadiantSceneMessage {
    AddArtboard,
    SelectArtboard(u64),

    SelectNode(u64),
    AddNode(RadiantNodeType),
    TransformNode {
        id: u64,
        position: [f32; 2],
        scale: [f32; 2],
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RadiantMessage {
    Scene(RadiantSceneMessage),

    SelectTool(u64),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RadiantResponse {
    NodeSelected(RadiantNodeType),
}
