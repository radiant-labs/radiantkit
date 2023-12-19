use std::{any::{Any, TypeId}, sync::Arc, fmt::Debug};

use crate::{ColorComponent, SelectionComponent, TransformComponent, Vec3, Observer};
use epaint::ClippedPrimitive;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type SubscriptionCallback = dyn Fn(&str)->() + 'static;

#[derive(Serialize, Deserialize, Default)]
pub struct BaseNode {
    pub id: Uuid,
    pub transform: TransformComponent,
    pub selection: SelectionComponent,
    pub color: ColorComponent,
    #[serde(skip)]
    pub primitives: Vec<ClippedPrimitive>,
    #[serde(skip)]
    pub selection_primitives: Vec<ClippedPrimitive>,
    #[serde(skip)]
    pub needs_tessellation: bool,
    #[serde(skip)]
    pub bounding_rect: [f32; 4],
    #[serde(skip)]
    pub observers: Observer<Arc<SubscriptionCallback>>,
}

impl Clone for BaseNode {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            transform: self.transform.clone(),
            selection: self.selection.clone(),
            color: self.color.clone(),
            primitives: Vec::new(),
            selection_primitives: Vec::new(),
            needs_tessellation: true,
            bounding_rect: [0.0, 0.0, 0.0, 0.0],
            observers: Observer::default(),
        }
    }
}

impl Debug for BaseNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BaseNode")
            .field("id", &self.id)
            .field("transform", &self.transform)
            .field("selection", &self.selection)
            .field("color", &self.color)
            .field("primitives", &self.primitives)
            .field("selection_primitives", &self.selection_primitives)
            .field("needs_tessellation", &self.needs_tessellation)
            .field("bounding_rect", &self.bounding_rect)
            .finish()
    }
}

impl BaseNode {
    pub fn new(id: Uuid, position: Vec3, scale: Vec3) -> Self {
        let mut transform = TransformComponent::new();
        transform.set_position(&position);
        transform.set_scale(&scale);

        let selection = SelectionComponent::new();
        let color = ColorComponent::new();

        Self {
            id,
            transform,
            selection,
            color,
            primitives: Vec::new(),
            selection_primitives: Vec::new(),
            needs_tessellation: true,
            bounding_rect: [0.0, 0.0, 0.0, 0.0],
            observers: Observer::default(),
        }
    }

    pub fn set_needs_tessellation(&mut self) {
        self.needs_tessellation = true;
    }

    pub fn notify(&self, message: String) {
        for cb in self.observers.callbacks() {
            cb(&message);
        }
    }
}

impl BaseNode {
    pub fn get_component<T: crate::RadiantComponent + 'static>(&self) -> Option<&T> {
        if TypeId::of::<T>() == TypeId::of::<SelectionComponent>() {
            unsafe { Some(&*(&self.selection as *const dyn Any as *const T)) }
        } else if TypeId::of::<T>() == TypeId::of::<TransformComponent>() {
            unsafe { Some(&*(&self.transform as *const dyn Any as *const T)) }
        } else if TypeId::of::<T>() == TypeId::of::<ColorComponent>() {
            unsafe { Some(&*(&self.color as *const dyn Any as *const T)) }
        } else {
            None
        }
    }

    pub fn get_component_mut<T: crate::RadiantComponent + 'static>(&mut self) -> Option<&mut T> {
        if TypeId::of::<T>() == TypeId::of::<SelectionComponent>() {
            unsafe { Some(&mut *(&mut self.selection as *mut dyn Any as *mut T)) }
        } else if TypeId::of::<T>() == TypeId::of::<TransformComponent>() {
            unsafe { Some(&mut *(&mut self.transform as *mut dyn Any as *mut T)) }
        } else if TypeId::of::<T>() == TypeId::of::<ColorComponent>() {
            unsafe { Some(&mut *(&mut self.color as *mut dyn Any as *mut T)) }
        } else {
            None
        }
    }
}
