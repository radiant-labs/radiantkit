use macro_magic::export_tokens;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{RadiantNode, KeyCode};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RadiantSceneMessage {
    AddArtboard {},
    SelectArtboard {
        id: Uuid,
    },
    SelectNode {
        id: Option<Uuid>,
    },
    TransformNode {
        id: Uuid,
        position: [f32; 2],
        scale: [f32; 2],
    },
    SetTransform {
        id: Uuid,
        position: [f32; 2],
        scale: [f32; 2],
    },
    SetFillColor {
        id: Uuid,
        fill_color: epaint::Color32,
    },
    SetStrokeColor {
        id: Uuid,
        stroke_color: epaint::Color32,
    },
    SelectTool {
        id: u32,
    },
    HandleKey {
        id: Option<Uuid>,
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
        id: uuid::Uuid,
        position: [f32; 2],
        scale: [f32; 2],
    },
}
