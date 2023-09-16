use crate::{
    RadiantNode, RadiantScene, RadiantTessellatable, RadiantTransformable, ScreenDescriptor,
    SelectionComponent, TransformComponent,
};
use epaint::{ClippedPrimitive, ClippedShape, Rect, TessellationOptions};
use serde::{Deserialize, Serialize};
use std::{
    any::{Any, TypeId},
    fmt::Debug,
};

#[derive(Serialize, Deserialize)]
pub struct RadiantPathNode {
    pub id: u64,
    pub transform: TransformComponent,
    pub selection: SelectionComponent,
    #[serde(skip)]
    pub primitives: Vec<ClippedPrimitive>,
    #[serde(skip)]
    pub selection_primitives: Vec<ClippedPrimitive>,
    #[serde(skip)]
    pub needs_tessellation: bool,
}

impl Clone for RadiantPathNode {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            transform: self.transform.clone(),
            selection: self.selection.clone(),
            primitives: Vec::new(),
            selection_primitives: Vec::new(),
            needs_tessellation: true,
        }
    }
}

impl Debug for RadiantPathNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RadiantPathNode")
            .field("id", &self.id)
            .field("transform", &self.transform)
            .field("selection", &self.selection)
            .finish()
    }
}

impl RadiantPathNode {
    pub fn new(id: u64, position: [f32; 2]) -> Self {
        let mut transform = TransformComponent::new();
        transform.set_xy(&position);

        let selection = SelectionComponent::new();

        Self {
            id,
            transform,
            selection,
            primitives: Vec::new(),
            selection_primitives: Vec::new(),
            needs_tessellation: true,
        }
    }

    fn tessellate(&mut self, pixels_per_point: f32) {
        if !self.needs_tessellation {
            return;
        }
        self.needs_tessellation = false;

        let position = self.transform.get_xy();
        let points = vec![
            epaint::Pos2::new(
                position[0] / pixels_per_point,
                position[1] / pixels_per_point,
            ),
            epaint::Pos2::new(
                position[0] / pixels_per_point + 200.0,
                position[1] / pixels_per_point + 200.0,
            ),
            epaint::Pos2::new(
                position[0] / pixels_per_point,
                position[1] / pixels_per_point + 400.0,
            ),
            epaint::Pos2::new(
                position[0] / pixels_per_point - 200.0,
                position[1] / pixels_per_point + 200.0,
            ),
        ];

        let color = if self.selection.is_selected() {
            epaint::Color32::RED
        } else {
            epaint::Color32::LIGHT_RED
        };
        let stroke = epaint::Stroke::new(1.0, color);
        let path_shape = epaint::PathShape::convex_polygon(points.clone(), color, stroke);
        let shapes = vec![ClippedShape(
            Rect::EVERYTHING,
            epaint::Shape::Path(path_shape),
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
        let stroke = epaint::Stroke::new(1.0, color);
        let path_shape = epaint::PathShape::convex_polygon(points, color, stroke);
        let shapes = vec![ClippedShape(
            Rect::EVERYTHING,
            epaint::Shape::Path(path_shape),
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

impl RadiantTessellatable for RadiantPathNode {
    fn attach_to_scene(&mut self, scene: &mut RadiantScene) {
        self.tessellate(scene.screen_descriptor.pixels_per_point);
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
    ) -> Vec<ClippedPrimitive> {
        self.tessellate(screen_descriptor.pixels_per_point);
        if selection {
            self.selection_primitives.clone()
        } else {
            self.primitives.clone()
        }
    }
}

impl RadiantNode for RadiantPathNode {
    fn get_id(&self) -> u64 {
        return self.id;
    }

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
