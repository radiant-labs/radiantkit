use crate::{
    RadiantComponentProvider, RadiantNode, RadiantTessellatable,
    ScreenDescriptor, SelectionComponent, TransformComponent, Vec3,
};
use epaint::{ClippedPrimitive, ClippedShape, Rect, TessellationOptions};
use serde::{Deserialize, Serialize};
use std::{
    any::{Any, TypeId},
    fmt::Debug,
};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
#[cfg_attr(not(target_arch = "wasm32"), radiantkit_macros::radiant_wasm_bindgen)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RadiantLineNode {
    pub id: u64,
    pub start: Vec3,
    pub end: Vec3,
    pub transform: TransformComponent,
    pub selection: SelectionComponent,
    #[serde(skip)]
    #[wasm_bindgen(skip)]
    pub primitives: Vec<ClippedPrimitive>,
    #[serde(skip)]
    #[wasm_bindgen(skip)]
    pub selection_primitives: Vec<ClippedPrimitive>,
    #[serde(skip)]
    #[wasm_bindgen(skip)]
    pub needs_tessellation: bool,
    #[serde(skip)]
    #[wasm_bindgen(skip)]
    pub bounding_rect: [f32; 4],
}

impl RadiantLineNode {
    pub fn new(id: u64, start: [f32; 2], end: [f32; 2]) -> Self {
        let mut transform = TransformComponent::new();
        transform.set_position(&start.into());

        let selection = SelectionComponent::new();

        Self {
            id,
            start: start.into(),
            end: end.into(),
            transform,
            selection,
            primitives: Vec::new(),
            selection_primitives: Vec::new(),
            needs_tessellation: true,
            bounding_rect: [0.0, 0.0, 0.0, 0.0],
        }
    }

    fn tessellate(&mut self, screen_descriptor: &ScreenDescriptor) {
        if !self.needs_tessellation {
            return;
        }
        self.needs_tessellation = false;

        let pixels_per_point = screen_descriptor.pixels_per_point;

        let points = [
            self.start.into(),
            self.end.into(),
        ];

        let color = epaint::Color32::BLUE;
        let shapes = vec![ClippedShape(
            Rect::EVERYTHING,
            epaint::Shape::LineSegment {
                points,
                stroke: epaint::Stroke::new(1.0, color),
            },
        )];
        self.primitives = epaint::tessellator::tessellate_shapes(
            pixels_per_point,
            TessellationOptions::default(),
            [1, 1],
            vec![],
            shapes,
        );

        let color = epaint::Color32::from_rgb(
            (self.id + 1 >> 0) as u8 & 0xFF,
            (self.id + 1 >> 8) as u8 & 0xFF,
            (self.id + 1 >> 16) as u8 & 0xFF,
        );
        let shapes = vec![ClippedShape(
            Rect::EVERYTHING,
            epaint::Shape::LineSegment {
                points,
                stroke: epaint::Stroke::new(8.0, color),
            },
        )];
        self.selection_primitives = epaint::tessellator::tessellate_shapes(
            pixels_per_point,
            TessellationOptions::default(),
            [1, 1],
            vec![],
            shapes,
        );
    }
}

impl RadiantTessellatable for RadiantLineNode {
    fn attach(&mut self, screen_descriptor: &ScreenDescriptor) {
        self.tessellate(screen_descriptor);
    }

    fn detach(&mut self) {
        self.primitives.clear();
        self.selection_primitives.clear();
    }

    fn set_needs_tessellation(&mut self) {
        self.needs_tessellation = true;
    }

    fn tessellate(
        &mut self,
        selection: bool,
        screen_descriptor: &ScreenDescriptor,
        _fonts_manager: &epaint::text::Fonts,
    ) -> Vec<ClippedPrimitive> {
        self.tessellate(screen_descriptor);
        if selection {
            self.selection_primitives.clone()
        } else {
            self.primitives.clone()
        }
    }
}

impl RadiantNode for RadiantLineNode {
    fn get_id(&self) -> u64 {
        return self.id;
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn get_bounding_rect(&self) -> [f32; 4] {
        self.bounding_rect
    }
}

impl RadiantComponentProvider for RadiantLineNode {
    fn get_component<T: crate::RadiantComponent + 'static>(&self) -> Option<&T> {
        if TypeId::of::<T>() == TypeId::of::<SelectionComponent>() {
            unsafe { Some(&*(&self.selection as *const dyn Any as *const T)) }
        } else if TypeId::of::<T>() == TypeId::of::<TransformComponent>() {
            unsafe { Some(&*(&self.transform as *const dyn Any as *const T)) }
        } else {
            None
        }
    }

    fn get_component_mut<T: crate::RadiantComponent + 'static>(&mut self) -> Option<&mut T> {
        if TypeId::of::<T>() == TypeId::of::<SelectionComponent>() {
            unsafe { Some(&mut *(&mut self.selection as *mut dyn Any as *mut T)) }
        } else if TypeId::of::<T>() == TypeId::of::<TransformComponent>() {
            unsafe { Some(&mut *(&mut self.transform as *mut dyn Any as *mut T)) }
        } else {
            None
        }
    }
}
