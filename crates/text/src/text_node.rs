use epaint::{
    text::{LayoutJob, TextFormat},
    ClippedPrimitive, ClippedShape, Color32, FontFamily, FontId, Fonts, Rect, TessellationOptions,
};
use radiantkit_core::{
    ColorComponent, RadiantComponent, RadiantComponentProvider, RadiantNode, RadiantTessellatable,
    RadiantTransformable, ScreenDescriptor, SelectionComponent, TransformComponent,
};
use serde::{Deserialize, Serialize};
use std::{
    any::{Any, TypeId},
    fmt::Debug,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct RadiantTextNode {
    pub id: u64,
    pub text: String,
    pub transform: TransformComponent,
    pub selection: SelectionComponent,
    pub color: ColorComponent,
    #[serde(skip)]
    pub primitives: Vec<ClippedPrimitive>,
    #[serde(skip)]
    pub selection_primitives: Vec<ClippedPrimitive>,
    #[serde(skip)]
    pub needs_tessellation: bool,
    #[serde(skip)]
    pub bounding_rect: [f32; 4],
}

impl Debug for RadiantTextNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RadiantTextNode")
            .field("id", &self.id)
            .field("transform", &self.transform)
            .field("selection", &self.selection)
            .field("needs_tessellation", &self.needs_tessellation)
            .field("bounding_rect", &self.bounding_rect)
            .finish()
    }
}

impl RadiantTextNode {
    pub fn new(id: u64, text: String, position: [f32; 2], scale: [f32; 2]) -> Self {
        let mut transform = TransformComponent::new();
        transform.set_xy(&position);
        transform.set_scale(&scale);

        let selection = SelectionComponent::new();
        let color = ColorComponent::new();

        Self {
            id,
            text,
            transform,
            selection,
            color,
            primitives: Vec::new(),
            selection_primitives: Vec::new(),
            needs_tessellation: true,
            bounding_rect: [0.0, 0.0, 0.0, 0.0],
        }
    }

    fn tessellate(&mut self, screen_descriptor: &ScreenDescriptor, fonts: &Fonts) {
        if !self.needs_tessellation {
            return;
        }
        self.needs_tessellation = false;

        let pixels_per_point = screen_descriptor.pixels_per_point;
        let position = self.transform.get_xy();
        let scale = self.transform.get_scale();

        let mut job = LayoutJob::default();
        job.append(
            &self.text, 
            0.0, 
            TextFormat {
                font_id: FontId::new(14.0, FontFamily::Proportional),
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

        let shape = epaint::TextShape::new(
            epaint::Pos2::new(
                position[0] / pixels_per_point,
                position[1] / pixels_per_point,
            ),
            galley,
        );

        let texture_atlas = fonts.texture_atlas();
        let (font_tex_size, prepared_discs) = {
            let atlas = texture_atlas.lock();
            (atlas.size(), atlas.prepared_discs())
        };

        let rect = epaint::Rect::from_two_pos(
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

        let shapes = vec![ClippedShape(Rect::EVERYTHING, epaint::Shape::Text(shape))];
        self.primitives = epaint::tessellator::tessellate_shapes(
            pixels_per_point,
            TessellationOptions::default(),
            font_tex_size,
            prepared_discs,
            shapes,
        );

        let fill_color = epaint::Color32::from_rgb(
            (self.id + 1 >> 0) as u8 & 0xFF,
            (self.id + 1 >> 8) as u8 & 0xFF,
            (self.id + 1 >> 16) as u8 & 0xFF,
        );
        let rect_shape = epaint::RectShape::filled(rect, rounding, fill_color);
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

impl RadiantTessellatable for RadiantTextNode {
    fn attach(&mut self, _screen_descriptor: &ScreenDescriptor) {}

    fn detach(&mut self) {
        self.primitives.clear();
        self.selection_primitives.clear();
    }

    fn set_needs_tessellation(&mut self) {
        let position = self.transform.get_xy();
        let scale = self.transform.get_scale();

        let rect = epaint::Rect::from_two_pos(
            epaint::Pos2::new(position[0], position[1]),
            epaint::Pos2::new(position[0] + scale[0], position[1] + scale[1]),
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
        fonts: &Fonts,
    ) -> Vec<ClippedPrimitive> {
        self.tessellate(screen_descriptor, fonts);
        if selection {
            self.selection_primitives.clone()
        } else {
            self.primitives.clone()
        }
    }
}

impl RadiantNode for RadiantTextNode {
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

impl RadiantComponentProvider for RadiantTextNode {
    fn get_component<T: RadiantComponent + 'static>(&self) -> Option<&T> {
        if TypeId::of::<T>() == TypeId::of::<SelectionComponent>() {
            unsafe { Some(&*(&self.selection as *const dyn Any as *const T)) }
        } else if TypeId::of::<T>() == TypeId::of::<TransformComponent>() {
            unsafe { Some(&*(&self.transform as *const dyn Any as *const T)) }
        } else if TypeId::of::<T>() == TypeId::of::<ColorComponent>() {
            unsafe { Some(&*(&self.color as *const dyn Any as *const T)) }
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
            unsafe { Some(&mut *(&mut self.color as *mut dyn Any as *mut T)) }
        } else {
            None
        }
    }
}
