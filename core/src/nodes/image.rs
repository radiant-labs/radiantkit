use crate::{
    ColorComponent, RadiantComponentProvider, RadiantNode, RadiantScene, RadiantTessellatable,
    RadiantTransformable, ScreenDescriptor, SelectionComponent, TransformComponent,
};
use epaint::{
    pos2, ClippedPrimitive, ClippedShape, Color32, Rect, TessellationOptions, TextureHandle,
};
use serde::{Deserialize, Serialize};
use std::{
    any::{Any, TypeId},
    fmt::Debug,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct RadiantImageNode {
    pub id: u64,
    pub transform: TransformComponent,
    pub selection: SelectionComponent,
    pub tint: ColorComponent,
    #[serde(skip)]
    pub texture_handle: Option<TextureHandle>,
    #[serde(skip)]
    pub primitives: Vec<ClippedPrimitive>,
    #[serde(skip)]
    pub selection_primitives: Vec<ClippedPrimitive>,
    #[serde(skip)]
    pub needs_tessellation: bool,
    #[serde(skip)]
    pub bounding_rect: [f32; 4],
}

impl Debug for RadiantImageNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RadiantImageNode")
            .field("id", &self.id)
            .field("transform", &self.transform)
            .field("selection", &self.selection)
            .field("needs_tessellation", &self.needs_tessellation)
            .field("bounding_rect", &self.bounding_rect)
            .finish()
    }
}

impl RadiantImageNode {
    pub fn new(
        id: u64,
        position: [f32; 2],
        scale: [f32; 2],
        texture_handle: TextureHandle,
    ) -> Self {
        let mut transform = TransformComponent::new();
        transform.set_xy(&position);
        transform.set_scale(&scale);

        let selection = SelectionComponent::new();
        let mut tint = ColorComponent::new();
        tint.set_fill_color(Color32::WHITE);

        Self {
            id,
            transform,
            selection,
            tint,
            texture_handle: Some(texture_handle),
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
        let position = self.transform.get_xy();
        let scale = self.transform.get_scale();

        let rect = epaint::Rect::from_min_max(
            epaint::Pos2::new(
                position[0] / pixels_per_point,
                position[1] / pixels_per_point,
            ),
            epaint::Pos2::new(
                (position[0] + scale[0]) / pixels_per_point,
                (position[1] + scale[1]) / pixels_per_point,
            ),
        );
        let rounding = epaint::Rounding::default();

        let mut mesh = epaint::Mesh::with_texture(self.texture_handle.clone().unwrap().id());
        let uv = Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0));
        mesh.add_rect_with_uv(rect, uv, self.tint.fill_color());
        let shapes = vec![ClippedShape(Rect::EVERYTHING, epaint::Shape::Mesh(mesh))];
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
        let rect_shape = epaint::RectShape::filled(rect, rounding, color);
        let shapes = vec![ClippedShape(
            Rect::EVERYTHING,
            epaint::Shape::Rect(rect_shape),
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

impl RadiantTessellatable for RadiantImageNode {
    fn attach_to_scene(&mut self, scene: &mut RadiantScene) {
        self.tessellate(&scene.screen_descriptor);
    }

    fn detach(&mut self) {
        self.primitives.clear();
        self.selection_primitives.clear();
    }

    fn set_needs_tessellation(&mut self) {
        let position = self.transform.get_xy();
        let scale = self.transform.get_scale();

        let rect = epaint::Rect::from_min_max(
            epaint::Pos2::new(position[0], position[1]),
            epaint::Pos2::new(position[0] + scale[0], position[1] + scale[1]),
        );
        let rounding = epaint::Rounding::default();

        let rect_shape = epaint::RectShape::filled(rect, rounding, epaint::Color32::WHITE);
        let bounding_rect = rect_shape.visual_bounding_rect();
        self.bounding_rect = [
            bounding_rect.min.x,
            bounding_rect.min.y,
            bounding_rect.max.x,
            bounding_rect.max.y,
        ];

        self.needs_tessellation = true;
    }

    fn tessellate(
        &mut self,
        selection: bool,
        screen_descriptor: &ScreenDescriptor,
    ) -> Vec<ClippedPrimitive> {
        self.tessellate(screen_descriptor);
        if selection {
            self.selection_primitives.clone()
        } else {
            self.primitives.clone()
        }
    }
}

impl RadiantNode for RadiantImageNode {
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

impl RadiantComponentProvider for RadiantImageNode {
    fn get_component<T: crate::RadiantComponent + 'static>(&self) -> Option<&T> {
        if TypeId::of::<T>() == TypeId::of::<SelectionComponent>() {
            unsafe { Some(&*(&self.selection as *const dyn Any as *const T)) }
        } else if TypeId::of::<T>() == TypeId::of::<TransformComponent>() {
            unsafe { Some(&*(&self.transform as *const dyn Any as *const T)) }
        } else if TypeId::of::<T>() == TypeId::of::<ColorComponent>() {
            unsafe { Some(&*(&self.tint as *const dyn Any as *const T)) }
        } else {
            None
        }
    }

    fn get_component_mut<T: crate::RadiantComponent + 'static>(&mut self) -> Option<&mut T> {
        if TypeId::of::<T>() == TypeId::of::<SelectionComponent>() {
            unsafe { Some(&mut *(&mut self.selection as *mut dyn Any as *mut T)) }
        } else if TypeId::of::<T>() == TypeId::of::<TransformComponent>() {
            unsafe { Some(&mut *(&mut self.transform as *mut dyn Any as *mut T)) }
        } else if TypeId::of::<T>() == TypeId::of::<ColorComponent>() {
            unsafe { Some(&mut *(&mut self.tint as *mut dyn Any as *mut T)) }
        } else {
            None
        }
    }
}