use epaint::{
    pos2, ClippedPrimitive, ClippedShape, Color32, Rect, TessellationOptions, TextureHandle,
};
use radiantkit_core::{
    ColorComponent, RadiantComponent, RadiantComponentProvider, RadiantNode, RadiantTessellatable,
    ScreenDescriptor, SelectionComponent, TransformComponent, Vec3,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::{
    any::{Any, TypeId},
    fmt::Debug,
};

use crate::Player;

#[cfg(feature = "av")]
use crate::AudioDevice;

#[derive(Serialize, Deserialize)]
pub struct RadiantVideoNode {
    pub id: Uuid,
    pub transform: TransformComponent,
    pub selection: SelectionComponent,
    pub tint: ColorComponent,
    #[serde(skip)]
    pub player: Option<Player>,
    #[serde(skip)]
    #[cfg(feature = "av")]
    pub audio_device: Option<AudioDevice>,
    #[serde(skip)]
    pub primitives: Vec<ClippedPrimitive>,
    #[serde(skip)]
    pub selection_primitives: Vec<ClippedPrimitive>,
    #[serde(skip)]
    pub needs_tessellation: bool,
    #[serde(skip)]
    pub bounding_rect: [f32; 4],
}

impl Clone for RadiantVideoNode {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            transform: self.transform.clone(),
            selection: self.selection.clone(),
            tint: self.tint.clone(),
            player: None,
            #[cfg(feature = "av")]
            audio_device: None,
            primitives: self.primitives.clone(),
            selection_primitives: self.selection_primitives.clone(),
            needs_tessellation: self.needs_tessellation,
            bounding_rect: self.bounding_rect,
        }
    }
}

impl Debug for RadiantVideoNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RadiantVideoNode")
            .field("id", &self.id)
            .field("transform", &self.transform)
            .field("selection", &self.selection)
            .field("needs_tessellation", &self.needs_tessellation)
            .field("bounding_rect", &self.bounding_rect)
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

        let selection = SelectionComponent::new();
        let mut tint = ColorComponent::new();
        tint.set_fill_color(Color32::WHITE);

        Self {
            id,
            transform,
            selection,
            tint,
            player,
            #[cfg(feature = "av")]
            audio_device,
            primitives: Vec::new(),
            selection_primitives: Vec::new(),
            needs_tessellation: true,
            bounding_rect: [0.0, 0.0, 0.0, 0.0],
        }
    }

    pub fn play(&mut self) {
        if let Some(player) = self.player.as_mut() {
            player.start();
        }
    }

    fn tessellate(&mut self, screen_descriptor: &ScreenDescriptor) {
        if !self.needs_tessellation {
            return;
        }
        self.needs_tessellation = false;

        let Some(player) = self.player.as_mut() else { return; };

        let pixels_per_point = screen_descriptor.pixels_per_point;
        let position = self.transform.position();
        let scale = self.transform.scale();

        let rect = epaint::Rect::from_two_pos(
            position.into(),
            Vec3::new_with_added(&position, &scale).into(),
        );
        let rounding = epaint::Rounding::default();

        let mut mesh = epaint::Mesh::with_texture(player.texture_handle.clone().id());
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
            (self.id.as_u128() + 1 >> 0) as u8 & 0xFF,
            (self.id.as_u128() + 1 >> 8) as u8 & 0xFF,
            (self.id.as_u128() + 1 >> 16) as u8 & 0xFF,
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

impl RadiantTessellatable for RadiantVideoNode {
    fn attach(&mut self, screen_descriptor: &ScreenDescriptor) {
        self.tessellate(screen_descriptor);
    }

    fn detach(&mut self) {
        self.primitives.clear();
        self.selection_primitives.clear();
    }

    fn set_needs_tessellation(&mut self) {
        let position = self.transform.position();
        let scale = self.transform.scale();

        let rect = epaint::Rect::from_min_max(
            position.into(),
            Vec3::new_with_added(&position, &scale).into(),
        );
        self.bounding_rect = [
            rect.left_top().x,
            rect.left_top().y,
            rect.right_bottom().x,
            rect.right_bottom().y,
        ];

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

impl RadiantNode for RadiantVideoNode {
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

impl RadiantComponentProvider for RadiantVideoNode {
    fn get_component<T: RadiantComponent + 'static>(&self) -> Option<&T> {
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

    fn get_component_mut<T: RadiantComponent + 'static>(&mut self) -> Option<&mut T> {
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
