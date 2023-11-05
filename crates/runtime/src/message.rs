use radiant_core::{RadiantSceneMessage, RadiantSceneResponse};
use radiant_macros::{combine_enum, RadiantMessage};

use crate::RadiantNodeType;
use serde::{Deserialize, Serialize};

#[derive(RadiantMessage)]
#[combine_enum(radiant_core::RadiantRectangleMessage)]
#[combine_enum(radiant_image_node::RadiantImageMessage)]
#[combine_enum(radiant_text_node::RadiantTextMessage)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RadiantMessage {
    SceneMessage(RadiantSceneMessage),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RadiantResponse {
    NodeSelected(RadiantNodeType),
    TransformUpdated {
        id: u64,
        position: [f32; 2],
        scale: [f32; 2],
    },

    NoOp,
}

impl From<RadiantSceneResponse<RadiantMessage, RadiantNodeType>> for RadiantResponse {
    fn from(response: RadiantSceneResponse<RadiantMessage, RadiantNodeType>) -> Self {
        match response {
            RadiantSceneResponse::NodeSelected(node) => Self::NodeSelected(node),
            RadiantSceneResponse::TransformUpdated {
                id,
                position,
                scale,
            } => Self::TransformUpdated {
                id,
                position,
                scale,
            },
            _ => Self::NoOp,
        }
    }
}
