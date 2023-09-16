use crate::{RadiantNodeType, RadiantTool};
use serde::{Deserialize, Serialize};

pub trait RadiantMessageHandler {
    fn handle_message(&mut self, message: RadiantMessage) -> Option<RadiantResponse>;
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum RadiantMessage {
    AddArtboard,
    SelectArtboard(u64),

    SelectNode(u64),
    TransformNode {
        id: u64,
        position: [f32; 2],
        scale: [f32; 2],
    },

    SelectTool(RadiantTool),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RadiantResponse {
    NodeSelected(RadiantNodeType),
}
