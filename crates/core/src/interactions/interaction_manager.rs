use epaint::ClippedPrimitive;

use crate::{BoundingBoxInteraction, RadiantMessage, RadiantNodeType, ScreenDescriptor};

pub struct RadiantInteractionManager {
    pub bounding_box_interaction: BoundingBoxInteraction,
}

impl RadiantInteractionManager {
    pub fn new() -> Self {
        Self {
            bounding_box_interaction: BoundingBoxInteraction::new(),
        }
    }

    pub fn is_interaction(&self, id: u64) -> bool {
        self.bounding_box_interaction.contains(id)
    }

    pub fn enable_interactions(
        &mut self,
        node: &RadiantNodeType,
        screen_descriptor: &ScreenDescriptor,
    ) {
        self.bounding_box_interaction
            .enable(node, screen_descriptor);
    }

    pub fn disable_interactions(&mut self) {
        self.bounding_box_interaction.disable();
    }

    pub fn update_interactions(
        &mut self,
        node: &RadiantNodeType,
        screen_descriptor: &ScreenDescriptor,
    ) {
        self.bounding_box_interaction
            .update(node, screen_descriptor);
    }

    pub fn handle_interaction(&mut self, message: RadiantMessage) -> Option<RadiantMessage> {
        match message {
            RadiantMessage::TransformNode { id, position, .. } if self.is_interaction(id) => {
                self.bounding_box_interaction.handle(id, position)
            }
            _ => None,
        }
    }

    pub fn tessellate(
        &mut self,
        selection: bool,
        screen_descriptor: &ScreenDescriptor,
        fonts_manager: &epaint::text::Fonts,
    ) -> Vec<ClippedPrimitive> {
        self.bounding_box_interaction
            .tessellate(selection, screen_descriptor, fonts_manager)
    }
}
