use epaint::{ClippedPrimitive, ClippedShape, Rect, TessellationOptions};
use radiantkit_core::{
    get_color_for_node, BaseNode, RadiantNode, RadiantTessellatable, ScreenDescriptor, Vec3,
};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RadiantPathNode {
    pub base: BaseNode,
}

impl RadiantPathNode {
    pub fn new(id: Uuid, position: [f32; 2]) -> Self {
        let base = BaseNode::new(id, position.into(), Vec3::default());
        Self { base }
    }

    fn tessellate(&mut self, pixels_per_point: f32) {
        if !self.base.needs_tessellation {
            return;
        }
        self.base.needs_tessellation = false;

        let position = self.base.transform.position();
        let scale = self.base.transform.scale();

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
        self.base.primitives = epaint::tessellator::tessellate_shapes(
            pixels_per_point,
            TessellationOptions::default(),
            [1, 1],
            vec![],
            shapes,
        );

        let color = get_color_for_node(self.base.id);
        let stroke = epaint::Stroke::new(1.0, color);
        let path_shape = epaint::PathShape::convex_polygon(points, color, stroke);
        let shapes = vec![ClippedShape(
            Rect::EVERYTHING,
            epaint::Shape::Path(path_shape),
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

impl RadiantTessellatable for RadiantPathNode {
    fn attach(&mut self, screen_descriptor: &ScreenDescriptor) {
        self.tessellate(screen_descriptor.pixels_per_point);
    }

    fn detach(&mut self) {
        self.base.primitives.clear();
        self.base.selection_primitives.clear();
    }

    fn set_needs_tessellation(&mut self, notify: bool) {
        let position = self.base.transform.position();
        let scale = self.base.transform.scale();

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
        self.base.bounding_rect = [
            bounding_rect.min.x,
            bounding_rect.min.y,
            bounding_rect.max.x,
            bounding_rect.max.y,
        ];

        self.base.set_needs_tessellation();
        if notify {
            self.base.notify(serde_json::to_string(self).unwrap());
        }
    }

    fn tessellate(
        &mut self,
        selection: bool,
        screen_descriptor: &ScreenDescriptor,
        _fonts_manager: &epaint::text::Fonts,
    ) -> Vec<ClippedPrimitive> {
        self.tessellate(screen_descriptor.pixels_per_point);
        if selection {
            self.base.selection_primitives.clone()
        } else {
            self.base.primitives.clone()
        }
    }
}

impl RadiantNode for RadiantPathNode {
    fn base(&self) -> &BaseNode {
        &self.base
    }

    fn base_mut(&mut self) -> &mut BaseNode {
        &mut self.base
    }
}
