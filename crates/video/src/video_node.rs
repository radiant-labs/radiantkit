use epaint::{
    pos2, ClippedPrimitive, ClippedShape, Color32, Rect, TessellationOptions, TextureHandle,
};
use radiantkit_core::{
    BaseNode, ColorComponent, RadiantNode, RadiantTessellatable, ScreenDescriptor,
    TransformComponent, Vec3,
};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

use crate::Player;

#[cfg(feature = "av")]
use crate::AudioDevice;

#[derive(Serialize, Deserialize)]
pub struct RadiantVideoNode {
    pub base: BaseNode,
    #[serde(skip)]
    pub player: Option<Player>,
    #[serde(skip)]
    #[cfg(feature = "av")]
    pub audio_device: Option<AudioDevice>,
}

impl Clone for RadiantVideoNode {
    fn clone(&self) -> Self {
        let base = self.base.clone();
        Self {
            base,
            player: None,
            #[cfg(feature = "av")]
            audio_device: None,
        }
    }
}

impl Debug for RadiantVideoNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RadiantVideoNode")
            .field("base", &self.base)
            .finish()
    }
}

impl RadiantVideoNode {
    pub fn new(
        id: Uuid,
        position: [f32; 2],
        scale: [f32; 2],
        path: String,
        texture_handle: TextureHandle,
    ) -> Self {
        let mut base = BaseNode::new(id, position.into(), scale.into());
        base.color.set_fill_color(Color32::WHITE);

        #[cfg(feature = "av")]
        let mut audio_device = None;

        let player = if let Ok(player) = Player::new(&path, texture_handle) {
            #[cfg(feature = "av")]
            if let Ok(device) = AudioDevice::new() {
                println!("Audio device created");
                audio_device = Some(device);
                player.with_audio(&mut audio_device.as_mut().unwrap()).ok()
            } else {
                Some(player)
            }

            #[cfg(not(feature = "av"))]
            Some(player)
        } else {
            None
        };

        let mut transform = TransformComponent::new();
        transform.set_position(&position.into());
        transform.set_scale(&scale.into());

        let mut tint = ColorComponent::new();
        tint.set_fill_color(Color32::WHITE);

        Self {
            base,
            player,
            #[cfg(feature = "av")]
            audio_device,
        }
    }

    pub fn play(&mut self) {
        if let Some(player) = self.player.as_mut() {
            player.start();
        }
    }

    fn tessellate(&mut self, screen_descriptor: &ScreenDescriptor) {
        if !self.base.needs_tessellation {
            return;
        }
        self.base.needs_tessellation = false;

        let Some(player) = self.player.as_mut() else {
            return;
        };

        let pixels_per_point = screen_descriptor.pixels_per_point;
        let position = self.base.transform.position();
        let scale = self.base.transform.scale();

        let rect = epaint::Rect::from_two_pos(
            position.into(),
            Vec3::new_with_added(&position, &scale).into(),
        );
        let rounding = epaint::Rounding::default();

        let mut mesh = epaint::Mesh::with_texture(player.texture_handle.clone().id());
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

        let color = epaint::Color32::from_rgb(
            (self.base.id.as_u128() + 1 >> 0) as u8 & 0xFF,
            (self.base.id.as_u128() + 1 >> 8) as u8 & 0xFF,
            (self.base.id.as_u128() + 1 >> 16) as u8 & 0xFF,
        );
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

impl RadiantTessellatable for RadiantVideoNode {
    fn attach(&mut self, screen_descriptor: &ScreenDescriptor) {
        self.tessellate(screen_descriptor);
    }

    fn detach(&mut self) {
        self.base.primitives.clear();
        self.base.selection_primitives.clear();
    }

    fn set_needs_tessellation(&mut self, _notify: bool) {
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

impl RadiantNode for RadiantVideoNode {
    fn base(&self) -> &BaseNode {
        &self.base
    }

    fn base_mut(&mut self) -> &mut BaseNode {
        &mut self.base
    }
}
