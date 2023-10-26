use epaint::ClippedPrimitive;
use serde::{Deserialize, Serialize};
use crate::{BoundingBoxInteraction, RadiantNode, ScreenDescriptor};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum InteractionMessage {
    TransformNode {
        id: u64,
        position: [f32; 2],
        scale: [f32; 2],
    },
}

pub struct RadiantInteractionManager<M> {
    pub bounding_box_interaction: BoundingBoxInteraction,
    _phantom: std::marker::PhantomData<M>,
}

impl<M: From<InteractionMessage> + TryInto<InteractionMessage>> RadiantInteractionManager<M> {
    pub fn new() -> Self {
        Self {
            bounding_box_interaction: BoundingBoxInteraction::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn is_interaction(&self, id: u64) -> bool {
        self.bounding_box_interaction.contains(id)
    }

    pub fn enable_interactions(
        &mut self,
        node: &impl RadiantNode,
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
        node: &impl RadiantNode,
        screen_descriptor: &ScreenDescriptor,
    ) {
        self.bounding_box_interaction
            .update(node, screen_descriptor);
    }

    pub fn handle_interaction(&mut self, message: M) -> Option<M> {
        match message.try_into() {
            Ok(InteractionMessage::TransformNode { id, position, .. }) if self.is_interaction(id) => {
                if let Some(m) = self.bounding_box_interaction.handle(id, position) {
                    Some(m.into())
                } else {
                    None
                }
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
