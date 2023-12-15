use epaint::{ClippedPrimitive, ClippedShape, Rect, TessellationOptions};
use radiantkit_core::{
    RadiantComponent, RadiantComponentProvider, RadiantNode, RadiantTessellatable,
    ScreenDescriptor, SelectionComponent, TransformComponent, get_color_for_node,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::{
    any::{Any, TypeId},
    fmt::Debug,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RadiantPathNode {
    pub id: Uuid,
    pub transform: TransformComponent,
    pub selection: SelectionComponent,
    #[serde(skip)]
    pub primitives: Vec<ClippedPrimitive>,
    #[serde(skip)]
    pub selection_primitives: Vec<ClippedPrimitive>,
    #[serde(skip)]
    pub needs_tessellation: bool,
    #[serde(skip)]
    pub bounding_rect: [f32; 4],
}

impl RadiantPathNode {
    pub fn new(id: Uuid, position: [f32; 2]) -> Self {
        let mut transform = TransformComponent::new();
        transform.set_position(&position.into());

        let selection = SelectionComponent::new();

        Self {
            id,
            transform,
            selection,
            primitives: Vec::new(),
            selection_primitives: Vec::new(),
            needs_tessellation: true,
            bounding_rect: [0.0, 0.0, 0.0, 0.0],
        }
    }

    fn tessellate(&mut self, pixels_per_point: f32) {
        if !self.needs_tessellation {
            return;
        }
        self.needs_tessellation = false;

        let position = self.transform.position();
        let scale = self.transform.scale();

        let points = vec![
            position.into(),
            epaint::Pos2::new(position.x + scale.x + 200.0, position.y + scale.y + 200.0),
            epaint::Pos2::new(position.x + scale.x, position.y + scale.y + 400.0),
            epaint::Pos2::new(position.x - 200.0, position.y + 200.0),
        ];

        let color = epaint::Color32::LIGHT_RED;
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

        let color = get_color_for_node(self.id);
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
    fn attach(&mut self, screen_descriptor: &ScreenDescriptor) {
        self.tessellate(screen_descriptor.pixels_per_point);
    }

    fn detach(&mut self) {
        self.primitives.clear();
        self.selection_primitives.clear();
    }

    fn set_needs_tessellation(&mut self) {
        let position = self.transform.position();
        let scale = self.transform.scale();

        let points = vec![
            position.into(),
            epaint::Pos2::new(position.x + scale.x + 200.0, position.y + scale.y + 200.0),
            epaint::Pos2::new(position.x + scale.x, position.y + scale.y + 400.0),
            epaint::Pos2::new(position.x - 200.0, position.y + 200.0),
        ];

        let color = epaint::Color32::LIGHT_RED;
        let stroke = epaint::Stroke::new(1.0, color);
        let path_shape = epaint::PathShape::convex_polygon(points.clone(), color, stroke);
        let bounding_rect = path_shape.visual_bounding_rect();
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
        _fonts_manager: &epaint::text::Fonts,
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
    fn get_id(&self) -> Uuid {
        return self.id;
    }

    fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }

    fn get_bounding_rect(&self) -> [f32; 4] {
        self.bounding_rect
    }
}

impl RadiantComponentProvider for RadiantPathNode {
    fn get_component<T: RadiantComponent + 'static>(&self) -> Option<&T> {
        if TypeId::of::<T>() == TypeId::of::<SelectionComponent>() {
            unsafe { Some(&*(&self.selection as *const dyn Any as *const T)) }
        } else if TypeId::of::<T>() == TypeId::of::<TransformComponent>() {
            unsafe { Some(&*(&self.transform as *const dyn Any as *const T)) }
        } else {
            None
        }
    }

    fn get_component_mut<T: RadiantComponent + 'static>(&mut self) -> Option<&mut T> {
        if TypeId::of::<T>() == TypeId::of::<SelectionComponent>() {
            unsafe { Some(&mut *(&mut self.selection as *mut dyn Any as *mut T)) }
        } else if TypeId::of::<T>() == TypeId::of::<TransformComponent>() {
            unsafe { Some(&mut *(&mut self.transform as *mut dyn Any as *mut T)) }
        } else {
            None
        }
    }
}
