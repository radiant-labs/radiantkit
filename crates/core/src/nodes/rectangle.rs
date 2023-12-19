use crate::{
    get_color_for_node, BaseNode, RadiantNode, RadiantTessellatable, ScreenDescriptor, Vec3,
};
use epaint::{ClippedPrimitive, ClippedShape, Rect, TessellationOptions};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RadiantRectangleNode {
    base: BaseNode,
}

impl RadiantRectangleNode {
    pub fn new_wasm(id: Uuid, position: Vec3, scale: Vec3) -> Self {
        let base = BaseNode::new(id, position, scale);
        Self { base }
    }
}

impl RadiantRectangleNode {
    pub fn new(id: Uuid, position: [f32; 2], scale: [f32; 2]) -> Self {
        let base = BaseNode::new(id, position.into(), scale.into());
        Self { base }
    }

    fn tessellate(&mut self, screen_descriptor: &ScreenDescriptor) {
        if !self.base.needs_tessellation {
            return;
        }
        self.base.needs_tessellation = false;

        let pixels_per_point = screen_descriptor.pixels_per_point;
        let position = self.base.transform.position();
        let scale = self.base.transform.scale();

        let rect = epaint::Rect::from_two_pos(
            position.into(),
            Vec3::new_with_added(&position, &scale).into(),
        );

        let rounding = epaint::Rounding::default();

        let fill_color = self.base.color.fill_color();
        let rect_shape = epaint::RectShape::filled(rect, rounding, fill_color);
        let shapes = vec![ClippedShape(
            Rect::EVERYTHING,
            epaint::Shape::Rect(rect_shape),
        )];
        self.base.primitives = epaint::tessellator::tessellate_shapes(
            pixels_per_point,
            TessellationOptions::default(),
            [1, 1],
            vec![],
            shapes,
        );

        let fill_color = get_color_for_node(self.base.id);
        let rect_shape = epaint::RectShape::filled(rect, rounding, fill_color);
        let shapes = vec![ClippedShape(
            Rect::EVERYTHING,
            epaint::Shape::Rect(rect_shape),
        )];
        self.base.selection_primitives = epaint::tessellator::tessellate_shapes(
            pixels_per_point,
            TessellationOptions::default(),
            [1, 1],
            vec![],
            shapes,
        );
    }
}

impl RadiantTessellatable for RadiantRectangleNode {
    fn attach(&mut self, screen_descriptor: &ScreenDescriptor) {
        self.tessellate(screen_descriptor);
    }

    fn detach(&mut self) {
        self.base.primitives.clear();
        self.base.selection_primitives.clear();
    }

    fn set_needs_tessellation(&mut self) {
        let position = self.base.transform.position();
        let scale = self.base.transform.scale();

        let rect = epaint::Rect::from_two_pos(
            position.into(),
            Vec3::new_with_added(&position, &scale).into(),
        );

        self.base.bounding_rect = [
            rect.left_top().x,
            rect.left_top().y,
            rect.right_bottom().x,
            rect.right_bottom().y,
        ];

        self.base.set_needs_tessellation();
        self.base.notify(serde_json::to_string(self).unwrap());
    }

    fn tessellate(
        &mut self,
        selection: bool,
        screen_descriptor: &ScreenDescriptor,
        _fonts_manager: &epaint::text::Fonts,
    ) -> Vec<ClippedPrimitive> {
        self.tessellate(screen_descriptor);
        if selection {
            self.base.selection_primitives.clone()
        } else {
            self.base.primitives.clone()
        }
    }
}

impl RadiantNode for RadiantRectangleNode {
    fn base(&self) -> &BaseNode {
        &self.base
    }

    fn base_mut(&mut self) -> &mut BaseNode {
        &mut self.base
    }
}
