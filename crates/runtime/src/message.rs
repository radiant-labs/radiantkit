use radiant_core::{RadiantSceneMessage, RadiantSceneResponse};
use radiant_macros::combine_enum;

use crate::RadiantNodeType;
use serde::{Deserialize, Serialize};

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

impl From<RadiantSceneMessage> for RadiantMessage {
    fn from(message: RadiantSceneMessage) -> Self {
        Self::SceneMessage(message)
    }
}

impl TryFrom<RadiantMessage> for RadiantSceneMessage {
    type Error = ();

    fn try_from(message: RadiantMessage) -> Result<Self, Self::Error> {
        match message {
            RadiantMessage::SceneMessage(message) => Ok(message),
            _ => Err(()),
        }
    }
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
