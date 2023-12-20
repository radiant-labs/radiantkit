use epaint::{
    pos2, ClippedPrimitive, ClippedShape, Color32, Rect, TessellationOptions, TextureHandle,
};
use radiantkit_core::{
    get_color_for_node, BaseNode, RadiantNode, RadiantTessellatable, ScreenDescriptor, Vec3,
};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct RadiantImageNode {
    pub base: BaseNode,
    #[serde(skip)]
    pub texture_handle: Option<TextureHandle>,
}

impl Debug for RadiantImageNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RadiantImageNode")
            .field("base", &self.base)
            .finish()
    }
}

impl RadiantImageNode {
    pub fn new(
        id: Uuid,
        position: [f32; 2],
        scale: [f32; 2],
        texture_handle: TextureHandle,
    ) -> Self {
        let mut base = BaseNode::new(id, position.into(), scale.into());
        base.color.set_fill_color(Color32::WHITE);

        Self {
            base,
            texture_handle: Some(texture_handle),
        }
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

        let mut mesh = epaint::Mesh::with_texture(self.texture_handle.clone().unwrap().id());
        let uv = Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0));
        mesh.add_rect_with_uv(rect, uv, self.base.color.fill_color());
        let shapes = vec![ClippedShape(Rect::EVERYTHING, epaint::Shape::Mesh(mesh))];
        self.base.primitives = epaint::tessellator::tessellate_shapes(
            pixels_per_point,
            TessellationOptions::default(),
            [1, 1],
            vec![],
            shapes,
        );

        let color = get_color_for_node(self.base.id);
        let rect_shape = epaint::RectShape::filled(rect, rounding, color);
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

impl RadiantTessellatable for RadiantImageNode {
    fn attach(&mut self, screen_descriptor: &ScreenDescriptor) {
        self.tessellate(screen_descriptor);
    }

    fn detach(&mut self) {
        self.base.primitives.clear();
        self.base.selection_primitives.clear();
    }

    fn set_needs_tessellation(&mut self, notify: bool) {
        let position = self.base.transform.position();
        let scale = self.base.transform.scale();

        let rect = epaint::Rect::from_min_max(
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
        self.tessellate(screen_descriptor);
        if selection {
            self.base.selection_primitives.clone()
        } else {
            self.base.primitives.clone()
        }
    }
}

impl RadiantNode for RadiantImageNode {
    fn base(&self) -> &BaseNode {
        &self.base
    }

    fn base_mut(&mut self) -> &mut BaseNode {
        &mut self.base
    }
}
