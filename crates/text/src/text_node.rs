use epaint::emath::NumExt;
use epaint::{
    text::{LayoutJob, TextFormat},
    ClippedPrimitive, ClippedShape, Color32, FontFamily, FontId, Fonts, Rect, TessellationOptions,
};
use once_cell::sync::Lazy;
use radiantkit_core::{
    get_color_for_node, BaseNode, RadiantLineNode, RadiantNode, RadiantTessellatable,
    ScreenDescriptor,
};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

use crate::RadiantTextMessage;

const CURSOR_NODE_ID: Lazy<Uuid> = Lazy::new(|| Uuid::new_v4());

#[derive(Serialize, Deserialize, Clone)]
pub struct RadiantTextNode {
    pub base: BaseNode,
    pub text: String,
    #[serde(skip)]
    pub cursor_node: RadiantLineNode,
}

impl Debug for RadiantTextNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RadiantTextNode")
            .field("base", &self.base)
            .finish()
    }
}

impl RadiantTextNode {
    pub fn new(id: Uuid, text: String, position: [f32; 2], scale: [f32; 2]) -> Self {
        let base = BaseNode::new(id, position.into(), scale.into());
        let cursor_node = RadiantLineNode::new(*CURSOR_NODE_ID, [0.0, 0.0], [0.0, 0.0]);

        Self {
            base,
            text,
            cursor_node,
        }
    }

    fn tessellate(&mut self, screen_descriptor: &ScreenDescriptor, fonts: &Fonts) {
        if !self.base.needs_tessellation {
            return;
        }
        self.base.needs_tessellation = false;

        let pixels_per_point = screen_descriptor.pixels_per_point;
        let position = self.base.transform.position();
        // let scale = self.transform.scale();

        let font_id = FontId::new(24.0, FontFamily::Proportional);
        let row_height = fonts.row_height(&font_id);

        let mut job = LayoutJob::default();
        job.append(
            &self.text,
            0.0,
            TextFormat {
                font_id,
                color: Color32::WHITE,
                ..Default::default()
            },
        );
        // job.append(
        //     "Hello ",
        //     0.0,
        //     TextFormat {
        //         font_id: FontId::new(14.0, FontFamily::Proportional),
        //         color: Color32::WHITE,
        //         ..Default::default()
        //     },
        // );
        // job.append(
        //     "World!",
        //     0.0,
        //     TextFormat {
        //         font_id: FontId::new(14.0, FontFamily::Monospace),
        //         color: Color32::BLACK,
        //         ..Default::default()
        //     },
        // );

        let galley = fonts.layout_job(job);

        let cursor = galley.end();
        let mut cursor_pos = galley.pos_from_cursor(&cursor); //.translate(pos.to_vec2());
        cursor_pos.max.y = cursor_pos.max.y.at_least(cursor_pos.min.y + row_height); // Handle completely empty galleys
        cursor_pos = cursor_pos.expand(1.5); // slightly above/below row

        let top = cursor_pos.center_top();
        let bottom = cursor_pos.center_bottom();
        self.cursor_node.start = [top.x + position.x, top.y + position.y].into();
        self.cursor_node.end = [bottom.x + position.x, bottom.y + position.y].into();
        // self.cursor_node.transform = self.transform.clone();

        let shape = epaint::TextShape::new(position.into(), galley);

        let texture_atlas = fonts.texture_atlas();
        let (font_tex_size, prepared_discs) = {
            let atlas = texture_atlas.lock();
            (atlas.size(), atlas.prepared_discs())
        };

        let rect: Rect = shape.visual_bounding_rect();
        self.base.bounding_rect = [
            rect.left_top().x,
            rect.left_top().y,
            rect.right_bottom().x + 10.0,
            rect.right_bottom().y,
        ];

        let rounding = epaint::Rounding::default();

        let shapes = vec![ClippedShape(Rect::EVERYTHING, epaint::Shape::Text(shape))];
        self.base.primitives = epaint::tessellator::tessellate_shapes(
            pixels_per_point,
            TessellationOptions::default(),
            font_tex_size,
            prepared_discs,
            shapes,
        );

        if self.base.selection.is_selected() {
            self.base
                .primitives
                .append(&mut self.cursor_node.tessellate(false, screen_descriptor, fonts));
        }

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

impl RadiantTessellatable for RadiantTextNode {
    fn attach(&mut self, _screen_descriptor: &ScreenDescriptor) {}

    fn detach(&mut self) {
        self.base.primitives.clear();
        self.base.selection_primitives.clear();
    }

    fn set_needs_tessellation(&mut self, notify: bool) {
        self.base.set_needs_tessellation();
        self.cursor_node.set_needs_tessellation(notify);
        if notify {
            self.base.notify(serde_json::to_string(self).unwrap());
        }
    }

    fn tessellate(
        &mut self,
        selection: bool,
        screen_descriptor: &ScreenDescriptor,
        fonts: &Fonts,
    ) -> Vec<ClippedPrimitive> {
        self.tessellate(screen_descriptor, fonts);
        if selection {
            self.base.selection_primitives.clone()
        } else {
            self.base.primitives.clone()
        }
    }
}

impl RadiantNode for RadiantTextNode {
    fn base(&self) -> &BaseNode {
        &self.base
    }

    fn base_mut(&mut self) -> &mut BaseNode {
        &mut self.base
    }

    fn handle_key_down(&mut self, key: radiantkit_core::KeyCode) -> bool {
        let did_update = match key {
            radiantkit_core::KeyCode::Backspace => {
                self.text.pop();
                true
            }
            radiantkit_core::KeyCode::Enter => {
                self.text.push('\n');
                true
            }
            radiantkit_core::KeyCode::Space => {
                self.text.push(' ');
                true
            }
            radiantkit_core::KeyCode::Char(c) => {
                self.text.push_str(&c);
                true
            }
            _ => false,
        };
        if did_update {
            self.set_needs_tessellation(true);
        }
        did_update
    }
}

impl RadiantTextNode {
    pub fn handle_message(&mut self, message: RadiantTextMessage) -> bool {
        match message {
            RadiantTextMessage::SetText { text, .. } => {
                self.text = text;
                self.set_needs_tessellation(true);
                true
            }
        }
    }
}
