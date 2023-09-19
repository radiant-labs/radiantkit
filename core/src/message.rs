use crate::{RadiantNodeType, RadiantToolType};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RadiantMessage {
    AddArtboard,
    SelectArtboard(u64),

    SelectNode(u64),
    AddNode(RadiantNodeType),
    TransformNode {
        id: u64,
        position: [f32; 2],
        scale: [f32; 2],
    },

    SelectTool(RadiantToolType),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RadiantResponse {
    NodeSelected(RadiantNodeType),
}
