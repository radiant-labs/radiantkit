use crate::{
    get_color_for_node, BaseNode, RadiantNode, RadiantTessellatable, ScreenDescriptor, Vec3,
};
use epaint::{ClippedPrimitive, ClippedShape, Rect, TessellationOptions};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RadiantLineNode {
    pub base: BaseNode,
    pub start: Vec3,
    pub end: Vec3,
}

impl RadiantLineNode {
    pub fn new(id: Uuid, start: [f32; 2], end: [f32; 2]) -> Self {
        let base = BaseNode::new(id, start.into(), Vec3::default());
        Self {
            base,
            start: start.into(),
            end: end.into(),
        }
    }

    fn tessellate(&mut self, screen_descriptor: &ScreenDescriptor) {
        if !self.base.needs_tessellation {
            return;
        }
        self.base.needs_tessellation = false;

        let pixels_per_point = screen_descriptor.pixels_per_point;

        let points = [self.start.into(), self.end.into()];

        let color = epaint::Color32::BLUE;
        let shapes = vec![ClippedShape(
            Rect::EVERYTHING,
            epaint::Shape::LineSegment {
                points,
                stroke: epaint::Stroke::new(1.0, color),
            },
        )];
        self.base.primitives = epaint::tessellator::tessellate_shapes(
            pixels_per_point,
            TessellationOptions::default(),
            [1, 1],
            vec![],
            shapes,
        );

        let color = get_color_for_node(self.base.id);
        let shapes = vec![ClippedShape(
            Rect::EVERYTHING,
            epaint::Shape::LineSegment {
                points,
                stroke: epaint::Stroke::new(8.0, color),
            },
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

impl RadiantTessellatable for RadiantLineNode {
    fn attach(&mut self, screen_descriptor: &ScreenDescriptor) {
        self.tessellate(screen_descriptor);
    }

    fn detach(&mut self) {
        self.base.primitives.clear();
        self.base.selection_primitives.clear();
    }

    fn set_needs_tessellation(&mut self) {
        self.base.needs_tessellation = true;
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

impl RadiantNode for RadiantLineNode {
    fn base(&self) -> &BaseNode {
        &self.base
    }

    fn base_mut(&mut self) -> &mut BaseNode {
        &mut self.base
    }
}
