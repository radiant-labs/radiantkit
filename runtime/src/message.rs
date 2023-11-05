use radiant_core::{RadiantSceneMessage, RadiantSceneResponse};
use radiant_macros::{combine_enum, combine_response, nested_message};

use crate::RadiantNodeType;
use serde::{Deserialize, Serialize};

#[nested_message]
#[combine_enum(radiant_core::RadiantRectangleMessage)]
#[combine_enum(radiant_image_node::RadiantImageMessage)]
#[combine_enum(radiant_text_node::RadiantTextMessage)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RadiantMessage {
    SceneMessage(RadiantSceneMessage),
}

#[combine_response(radiant_core::RadiantSceneResponse<RadiantMessage, RadiantNodeType>)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RadiantResponse {
    NoOp,
}
