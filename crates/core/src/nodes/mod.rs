pub mod base_node;
pub mod group;
pub mod line;
pub mod rectangle;

use std::sync::Arc;

pub use base_node::*;
pub use group::*;
pub use line::*;
pub use rectangle::*;

use serde::Serialize;
use uuid::Uuid;

use crate::{ColorComponent, ScreenDescriptor, TransformComponent, Subscription, SubscriptionId};
use epaint::ClippedPrimitive;

pub trait RadiantTessellatable {
    fn attach(&mut self, screen_descriptor: &ScreenDescriptor);
    fn detach(&mut self);

    fn set_needs_tessellation(&mut self);
    fn tessellate(
        &mut self,
        selection: bool,
        screen_descriptor: &ScreenDescriptor,
        fonts_manager: &epaint::text::Fonts,
    ) -> Vec<ClippedPrimitive>;
}

pub trait RadiantNode: 'static + Serialize + Clone + RadiantTessellatable {
    fn base(&self) -> &BaseNode;
    fn base_mut(&mut self) -> &mut BaseNode;

    fn get_id(&self) -> Uuid {
        self.base().id
    }
    fn set_id(&mut self, id: Uuid) {
        self.base_mut().id = id;
    }

    fn get_bounding_rect(&self) -> [f32; 4] {
        self.base().bounding_rect
    }

    fn transform(&self) -> &TransformComponent {
        &self.base().transform
    }
    fn transform_mut(&mut self) -> &mut TransformComponent {
        &mut self.base_mut().transform
    }

    fn color(&self) -> ColorComponent {
        self.base().color
    }
    fn color_mut(&mut self) -> &mut ColorComponent {
        &mut self.base_mut().color
    }

    fn handle_key_down(&mut self, _key: crate::KeyCode) -> bool {
        false
    }

    fn get_component<T: crate::RadiantComponent + 'static>(&self) -> Option<&T> {
        self.base().get_component::<T>()
    }
    fn get_component_mut<T: crate::RadiantComponent + 'static>(&mut self) -> Option<&mut T> {
        self.base_mut().get_component_mut::<T>()
    }

    fn observe<F>(&mut self, f: F) -> Subscription<Arc<SubscriptionCallback>> where F: Fn(&str)->() + 'static {
        self.base_mut().observers.subscribe(Arc::new(f))
    }
    fn unobserve(&self, subscription_id: SubscriptionId) {
        self.base().observers.unsubscribe(subscription_id);
    }
}
