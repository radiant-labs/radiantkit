use epaint::Color32;
use radiant_path_node::PathToolMessage;
use serde::{Deserialize, Serialize};

use radiant_core::{InteractionMessage, RectangleToolMessage, SelectionToolMessage};

use crate::RadiantNodeType;

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
        id: u32,
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

impl From<RectangleToolMessage> for RadiantMessage {
    fn from(message: RectangleToolMessage) -> Self {
        match message {
            RectangleToolMessage::AddNode {
                node_type,
                position,
                scale,
            } => Self::AddNode {
                node_type,
                position,
                scale,
            },
            RectangleToolMessage::TransformNode {
                id,
                position,
                scale,
            } => Self::TransformNode {
                id,
                position,
                scale,
            },
        }
    }
}

impl From<SelectionToolMessage> for RadiantMessage {
    fn from(message: SelectionToolMessage) -> Self {
        match message {
            SelectionToolMessage::SelectNode(id) => Self::SelectNode(id),
            SelectionToolMessage::TransformNode {
                id,
                position,
                scale,
            } => Self::TransformNode {
                id,
                position,
                scale,
            },
        }
    }
}

impl From<PathToolMessage> for RadiantMessage {
    fn from(message: PathToolMessage) -> Self {
        match message {
            PathToolMessage::SelectNode(id) => Self::SelectNode(id),
            PathToolMessage::TransformNode {
                id,
                position,
                scale,
            } => Self::TransformNode {
                id,
                position,
                scale,
            },
        }
    }
}

impl From<InteractionMessage> for RadiantMessage {
    fn from(message: InteractionMessage) -> Self {
        match message {
            InteractionMessage::TransformNode {
                id,
                position,
                scale,
            } => Self::TransformNode {
                id,
                position,
                scale,
            },
        }
    }
}

impl TryInto<InteractionMessage> for RadiantMessage {
    type Error = ();

    fn try_into(self) -> Result<InteractionMessage, Self::Error> {
        match self {
            Self::TransformNode {
                id,
                position,
                scale,
            } => Ok(InteractionMessage::TransformNode {
                id,
                position,
                scale,
            }),
            _ => Err(()),
        }
    }
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
