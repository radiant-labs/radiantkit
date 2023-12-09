use macro_magic::export_tokens;
use serde::{Deserialize, Serialize};

use crate::{RadiantNode, KeyCode};

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
    HandleKey {
        id: Option<u64>,
        key: KeyCode,
    },
}

#[export_tokens]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RadiantSceneResponse<M, N: RadiantNode> {
    Message {
        message: M,
    },
    Selected {
        node: N,
    },
    TransformUpdated {
        id: u64,
        position: [f32; 2],
        scale: [f32; 2],
    },
}
