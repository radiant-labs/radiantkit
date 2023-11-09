use radiantkit_core::{RadiantSceneMessage, RadiantSceneResponse};
use radiantkit_macros::{combine_enum, combine_response, nested_message};

use crate::RadiantNodeType;
use serde::{Deserialize, Serialize};

#[nested_message]
#[combine_enum(radiantkit_core::RadiantRectangleMessage)]
#[combine_enum(radiantkit_image::RadiantImageMessage)]
#[combine_enum(radiantkit_text::RadiantTextMessage)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RadiantMessage {
    SceneMessage(RadiantSceneMessage),
}

#[combine_response(radiantkit_core::RadiantSceneResponse<RadiantMessage, RadiantNodeType>)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RadiantResponse {
    NoOp,
}
