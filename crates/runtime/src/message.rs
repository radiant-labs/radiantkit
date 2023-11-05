use epaint::Color32;
use radiant_macros::combine_enum;

use crate::RadiantNodeType;
use serde::{Deserialize, Serialize};

#[combine_enum(radiant_core::RectangleToolMessage)]
#[combine_enum(radiant_core::SelectionToolMessage)]
#[combine_enum(radiant_core::InteractionMessage)]
#[combine_enum(radiant_image_node::RadiantImageMessage)]
#[combine_enum(radiant_text_node::RadiantTextMessage)]
#[combine_enum(radiant_path_node::PathToolMessage)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RadiantMessage {
    AddArtboard {},
    SelectArtboard {
        id: u64,
    },
    SelectNode {
        id: u64,
    },
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
        id: u32,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RadiantResponse {
    NodeSelected(RadiantNodeType),
    TransformUpdated {
        id: u64,
        position: [f32; 2],
        scale: [f32; 2],
    },
}
