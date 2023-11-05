use serde::{Deserialize, Serialize};

use crate::RadiantNode;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RadiantSceneMessage {
    AddArtboard {},
    SelectArtboard {
        id: u64,
    },
    SelectNode {
        id: Option<u64>,
    },
    TransformNode {
        id: u64,
        position: [f32; 2],
        scale: [f32; 2],
    },
    SetTransform {
        id: u64,
        position: [f32; 2],
        scale: [f32; 2],
    },
    SetFillColor {
        id: u64,
        fill_color: epaint::Color32,
    },
    SetStrokeColor {
        id: u64,
        stroke_color: epaint::Color32,
    },
    SelectTool {
        id: u32,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RadiantSceneResponse<M, N: RadiantNode> {
    Message(M),
    NodeSelected(N),
    TransformUpdated {
        id: u64,
        position: [f32; 2],
        scale: [f32; 2],
    },
}
