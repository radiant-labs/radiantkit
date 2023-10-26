use epaint::Color32;
use serde::{Deserialize, Serialize};

use crate::RadiantToolId;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RadiantMessage {
    AddArtboard,
    SelectArtboard(u64),

    SelectNode(u64),
    AddNode {
        node_type: String,
        position: [f32; 2],
        scale: [f32; 2],
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
        fill_color: Color32,
    },
    SetStrokeColor {
        id: u64,
        stroke_color: Color32,
    },

    SelectTool {
        id: RadiantToolId,
    },

    AddText {
        text: String,
        position: [f32; 2],
    },
    AddImage {
        name: String,
        path: String,
    },
}

