use crate::{
    RadiantMessage, RadiantInteraction, RadiantLineNode, RadiantNode,
    RadiantRectangleNode, RadiantTessellatable, RadiantTransformable,
    ScreenDescriptor, TransformComponent,
};
use epaint::ClippedPrimitive;

const BOUNDING_BOX_TOP_ID: u64 = 201;
const BOUNDING_BOX_RIGHT_ID: u64 = 202;
const BOUNDING_BOX_BOTTOM_ID: u64 = 203;
const BOUNDING_BOX_LEFT_ID: u64 = 204;

const BOUNDING_BOX_TOP_RIGHT_ID: u64 = 205;
const BOUNDING_BOX_BOTTOM_RIGHT_ID: u64 = 206;
const BOUNDING_BOX_BOTTOM_LEFT_ID: u64 = 207;
const BOUNDING_BOX_TOP_LEFT_ID: u64 = 208;

#[derive(Debug, Clone)]
pub struct BoundingBoxInteraction {
    pub active_node_id: Option<u64>,
    pub nodes: Vec<RadiantLineNode>,
    pub corner_nodes: Vec<RadiantRectangleNode>,
    pub primitives: Vec<ClippedPrimitive>,
    pub selection_primitives: Vec<ClippedPrimitive>,
}

impl BoundingBoxInteraction {
    pub fn new() -> Self {
        let nodes = vec![
            RadiantLineNode::new(BOUNDING_BOX_TOP_ID, [0.0, 0.0], [0.0, 0.0]),
            RadiantLineNode::new(BOUNDING_BOX_RIGHT_ID, [0.0, 0.0], [0.0, 0.0]),
            RadiantLineNode::new(BOUNDING_BOX_BOTTOM_ID, [0.0, 0.0], [0.0, 0.0]),
            RadiantLineNode::new(BOUNDING_BOX_LEFT_ID, [0.0, 0.0], [0.0, 0.0]),
        ];

        let mut corner_nodes = vec![
            RadiantRectangleNode::new(BOUNDING_BOX_TOP_RIGHT_ID, [0.0, 0.0], [16.0, 16.0]),
            RadiantRectangleNode::new(BOUNDING_BOX_BOTTOM_RIGHT_ID, [0.0, 0.0], [16.0, 16.0]),
            RadiantRectangleNode::new(BOUNDING_BOX_BOTTOM_LEFT_ID, [0.0, 0.0], [16.0, 16.0]),
            RadiantRectangleNode::new(BOUNDING_BOX_TOP_LEFT_ID, [0.0, 0.0], [16.0, 16.0]),
        ];
        for node in &mut corner_nodes {
            node.color.set_fill_color(epaint::Color32::BLUE);
        }

        Self {
            active_node_id: None,
            nodes,
            corner_nodes,
            primitives: Vec::new(),
            selection_primitives: Vec::new(),
        }
    }
}

impl Default for BoundingBoxInteraction {
    fn default() -> Self {
        Self::new()
    }
}

impl BoundingBoxInteraction {
    pub fn contains(&self, id: u64) -> bool {
        return id == BOUNDING_BOX_TOP_ID
            || id == BOUNDING_BOX_RIGHT_ID
            || id == BOUNDING_BOX_BOTTOM_ID
            || id == BOUNDING_BOX_LEFT_ID
            || id == BOUNDING_BOX_TOP_RIGHT_ID
            || id == BOUNDING_BOX_BOTTOM_RIGHT_ID
            || id == BOUNDING_BOX_BOTTOM_LEFT_ID
            || id == BOUNDING_BOX_TOP_LEFT_ID;
    }

    pub fn enable(&mut self, node: &impl RadiantNode, _screen_descriptor: &ScreenDescriptor) {
        if let Some(_component) = node.get_component::<TransformComponent>() {
            let rect = node.get_bounding_rect();

            self.nodes[0].start = [rect[0], rect[1]];
            self.nodes[0].end = [rect[2], rect[1]];

            self.nodes[1].start = [rect[2], rect[1]];
            self.nodes[1].end = [rect[2], rect[3]];

            self.nodes[2].start = [rect[2], rect[3]];
            self.nodes[2].end = [rect[0], rect[3]];

            self.nodes[3].start = [rect[0], rect[3]];
            self.nodes[3].end = [rect[0], rect[1]];

            self.corner_nodes[0]
                .transform
                .set_xy(&[rect[2] - 8.0, rect[1] - 8.0]);
            self.corner_nodes[1]
                .transform
                .set_xy(&[rect[2] - 8.0, rect[3] - 8.0]);
            self.corner_nodes[2]
                .transform
                .set_xy(&[rect[0] - 8.0, rect[3] - 8.0]);
            self.corner_nodes[3]
                .transform
                .set_xy(&[rect[0] - 8.0, rect[1] - 8.0]);

            for node in &mut self.nodes {
                node.set_needs_tessellation();
            }
            for node in &mut self.corner_nodes {
                node.set_needs_tessellation();
            }

            self.active_node_id = Some(node.get_id());
        }
    }

    pub fn disable(&mut self) {
        self.active_node_id = None;
    }

    pub fn update(&mut self, node: &impl RadiantNode, screen_descriptor: &ScreenDescriptor) {
        self.enable(node, screen_descriptor);
    }
}

impl BoundingBoxInteraction {
    pub fn tessellate(
        &mut self,
        selection: bool,
        screen_descriptor: &crate::ScreenDescriptor,
        fonts_manager: &epaint::text::Fonts,
    ) -> Vec<ClippedPrimitive> {
        if self.active_node_id.is_none() {
            return Vec::new();
        }

        let primitives = self
            .nodes
            .iter_mut()
            .fold(Vec::new(), |mut primitives, node| {
                primitives.append(&mut node.tessellate(
                    selection,
                    screen_descriptor,
                    fonts_manager,
                ));
                primitives
            });
        self.primitives = self
            .corner_nodes
            .iter_mut()
            .fold(primitives, |mut primitives, node| {
                primitives.append(&mut node.tessellate(
                    selection,
                    screen_descriptor,
                    fonts_manager,
                ));
                primitives
            });

        let selection_primitives =
            self.nodes
                .iter_mut()
                .fold(Vec::new(), |mut primitives, node| {
                    primitives.append(&mut node.tessellate(true, screen_descriptor, fonts_manager));
                    primitives
                });
        self.selection_primitives =
            self.corner_nodes
                .iter_mut()
                .fold(selection_primitives, |mut primitives, node| {
                    primitives.append(&mut node.tessellate(true, screen_descriptor, fonts_manager));
                    primitives
                });

        if selection {
            self.selection_primitives.clone()
        } else {
            self.primitives.clone()
        }
    }
}

impl RadiantInteraction for BoundingBoxInteraction {
    fn get_primitives(&self, selection: bool) -> Vec<ClippedPrimitive> {
        if selection {
            self.selection_primitives.clone()
        } else {
            self.primitives.clone()
        }
    }
}

impl BoundingBoxInteraction {
    pub fn handle(&mut self, id: u64, transform: [f32; 2]) -> Option<RadiantMessage> {
        let Some(node_id) = self.active_node_id else { return None; };
        match id {
            BOUNDING_BOX_TOP_ID => Some(RadiantMessage::TransformNode {
                id: node_id,
                position: [0.0, transform[1]],
                scale: [0.0, -transform[1]],
            }),
            BOUNDING_BOX_RIGHT_ID => Some(RadiantMessage::TransformNode {
                id: node_id,
                position: [0.0, 0.0],
                scale: [transform[0], 0.0],
            }),
            BOUNDING_BOX_BOTTOM_ID => Some(RadiantMessage::TransformNode {
                id: node_id,
                position: [0.0, 0.0],
                scale: [0.0, transform[1]],
            }),
            BOUNDING_BOX_LEFT_ID => Some(RadiantMessage::TransformNode {
                id: node_id,
                position: [transform[0], 0.0],
                scale: [-transform[0], 0.0],
            }),
            BOUNDING_BOX_TOP_RIGHT_ID => Some(RadiantMessage::TransformNode {
                id: node_id,
                position: [0.0, transform[1]],
                scale: [transform[0], -transform[1]],
            }),
            BOUNDING_BOX_BOTTOM_RIGHT_ID => Some(RadiantMessage::TransformNode {
                id: node_id,
                position: [0.0, 0.0],
                scale: [transform[0], transform[1]],
            }),
            BOUNDING_BOX_BOTTOM_LEFT_ID => Some(RadiantMessage::TransformNode {
                id: node_id,
                position: [transform[0], 0.0],
                scale: [-transform[0], transform[1]],
            }),
            BOUNDING_BOX_TOP_LEFT_ID => Some(RadiantMessage::TransformNode {
                id: node_id,
                position: [transform[0], transform[1]],
                scale: [-transform[0], -transform[1]],
            }),
            _ => None,
        }
    }
}
