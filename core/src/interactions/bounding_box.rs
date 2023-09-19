use crate::{
    RadiantComponentProvider, RadiantInteraction, RadiantLineNode, RadiantNode, RadiantNodeType,
    RadiantSceneMessage, RadiantTessellatable, ScreenDescriptor, TransformComponent,
};
use epaint::ClippedPrimitive;

const BOUNDING_BOX_TOP_ID: u64 = 201;
const BOUNDING_BOX_RIGHT_ID: u64 = 202;
const BOUNDING_BOX_BOTTOM_ID: u64 = 203;
const BOUNDING_BOX_LEFT_ID: u64 = 204;

#[derive(Debug, Clone)]
pub struct BoundingBoxInteraction {
    pub active_node_id: Option<u64>,
    pub nodes: Vec<RadiantLineNode>,
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

        Self {
            active_node_id: None,
            nodes,
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
            || id == BOUNDING_BOX_LEFT_ID;
    }

    pub fn attached_to(&mut self, node: &RadiantNodeType, _screen_descriptor: &ScreenDescriptor) {
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

            for node in &mut self.nodes {
                node.set_needs_tessellation();
            }

            self.active_node_id = Some(node.get_id());
        }
    }

    pub fn detached(&mut self) {
        self.active_node_id = None;
    }

    pub fn update(&mut self, node: &RadiantNodeType, screen_descriptor: &ScreenDescriptor) {
        self.attached_to(node, screen_descriptor);
    }
}

impl RadiantTessellatable for BoundingBoxInteraction {
    fn attach_to_scene(&mut self, _scene: &mut crate::RadiantScene) {}

    fn detach(&mut self) {}

    fn set_needs_tessellation(&mut self) {}

    fn tessellate(
        &mut self,
        selection: bool,
        screen_descriptor: &crate::ScreenDescriptor,
    ) -> Vec<ClippedPrimitive> {
        if self.active_node_id.is_none() {
            return Vec::new();
        }

        self.primitives = self
            .nodes
            .iter_mut()
            .fold(Vec::new(), |mut primitives, node| {
                primitives.append(&mut node.tessellate(selection, screen_descriptor));
                primitives
            });

        self.selection_primitives =
            self.nodes
                .iter_mut()
                .fold(Vec::new(), |mut primitives, node| {
                    primitives.append(&mut node.tessellate(true, screen_descriptor));
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
    pub fn handle(&mut self, id: u64, transform: [f32; 2]) -> Option<RadiantSceneMessage> {
        let Some(node_id) = self.active_node_id else { return None; };
        match id {
            BOUNDING_BOX_TOP_ID => Some(RadiantSceneMessage::TransformNode {
                id: node_id,
                position: [0.0, 0.0],
                scale: [0.0, transform[1]],
            }),
            BOUNDING_BOX_RIGHT_ID => Some(RadiantSceneMessage::TransformNode {
                id: node_id,
                position: [0.0, 0.0],
                scale: [transform[0], 0.0],
            }),
            BOUNDING_BOX_BOTTOM_ID => Some(RadiantSceneMessage::TransformNode {
                id: node_id,
                position: [0.0, 0.0],
                scale: [0.0, transform[1]],
            }),
            BOUNDING_BOX_LEFT_ID => Some(RadiantSceneMessage::TransformNode {
                id: node_id,
                position: [0.0, 0.0],
                scale: [transform[0], 0.0],
            }),
            _ => None,
        }
    }
}
