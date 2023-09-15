use std::fmt::Debug;

use crate::RadiantMessageHandler;
use crate::RadiantScene;
use crate::SelectionMessage;
use crate::TransformMessage;
use crate::{RadiantIdentifiable, RadiantSelectable, RenderComponent, SelectionComponent};
use crate::{RadiantRenderable, RadiantVertex, TransformComponent};
use epaint::ClippedPrimitive;
use epaint::ClippedShape;
use epaint::Color32;
use epaint::Pos2;
use epaint::Rect;
use epaint::TessellationOptions;
use epaint::Vertex;
use serde::{Deserialize, Serialize};
use crate::ScreenDescriptor;

// const VERTICES: &[RadiantVertex] = &[
//     RadiantVertex {
//         position: [0.5, 0.5, 0.0],
//         color: [0.5, 0.0, 0.5],
//     }, // A
//     RadiantVertex {
//         position: [-0.5, 0.5, 0.0],
//         color: [0.5, 0.0, 0.5],
//     }, // B
//     RadiantVertex {
//         position: [-0.5, -0.5, 0.0],
//         color: [0.5, 0.0, 0.5],
//     }, // C
//     RadiantVertex {
//         position: [0.5, -0.5, 0.0],
//         color: [0.5, 0.0, 0.5],
//     }, // D
// ];

// const VERTICES: &[Vertex] = &[
//     Vertex {
//         pos: Pos2 { x: 0.5, y: 0.5 },
//         uv: Pos2 { x: 0.0, y: 0.0 },
//         color: Color32::RED,
//     }, // A
//     Vertex {
//         pos: Pos2 { x: -0.5, y: 0.5 },
//         uv: Pos2 { x: 0.0, y: 0.0 },
//         color: Color32::RED,
//     }, // B
//     Vertex {
//         pos: Pos2 { x: -0.5, y: -0.5 },
//         uv: Pos2 { x: 0.0, y: 0.0 },
//         color: Color32::RED,
//     },  // C
//     Vertex {
//         pos: Pos2 { x: 0.5, y: -0.5 },
//         uv: Pos2 { x: 0.0, y: 0.0 },
//         color: Color32::RED,
//     }, // D
// ];

// const INDICES: &[u32] = &[0, 1, 2, 2, 3, 0];

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum RadiantRectangleMessage {
    Transform(TransformMessage),
    Selection(SelectionMessage),
}

#[derive(Serialize, Deserialize)]
pub struct RadiantRectangleNode {
    pub id: u64,
    pub transform: TransformComponent,
    pub selection: SelectionComponent,
    #[serde(skip)]
    pub renderer: Option<RenderComponent>,
    #[serde(skip)]
    pub offscreen_renderer: Option<RenderComponent>,
    #[serde(skip)]
    pub mesh: epaint::Mesh,
    #[serde(skip)]
    pub primitives: Vec<ClippedPrimitive>
}

impl Clone for RadiantRectangleNode {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            transform: self.transform.clone(),
            selection: self.selection.clone(),
            renderer: None,
            offscreen_renderer: None,
            mesh: epaint::Mesh::default(),
            primitives: Vec::new(),
        }
    }
}

impl Debug for RadiantRectangleNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RadiantRectangleNode")
            .field("id", &self.id)
            .field("transform", &self.transform)
            .field("selection", &self.selection)
            .finish()
    }
}

impl RadiantRectangleNode {
    pub fn new(id: u64, position: [f32; 2]) -> Self {
        let mut transform = TransformComponent::new();
        transform.set_xy(&position);

        let selection = SelectionComponent::new();

        Self {
            id,
            transform,
            selection,
            renderer: None,
            offscreen_renderer: None,
            mesh: epaint::Mesh::default(),
            primitives: Vec::new(),
        }
    }
}

impl RadiantIdentifiable for RadiantRectangleNode {
    fn get_id(&self) -> u64 {
        return self.id;
    }
}

impl RadiantSelectable for RadiantRectangleNode {
    fn set_selected(&mut self, selected: bool) {
        self.selection.set_selected(selected);
        self.renderer.as_mut().map_or((), |r| {
            r.set_selection_color([1.0, 0.0, 0.0, if selected { 1.0 } else { 0.0 }])
        });
    }
}

impl RadiantRenderable for RadiantRectangleNode {
    fn attach_to_scene(&mut self, scene: &mut RadiantScene) {
        let pixels_per_point = scene.screen_descriptor.pixels_per_point;
        let tessellator = epaint::Tessellator::new(pixels_per_point, Default::default(), [1, 1], vec![]);

        let position = self.transform.get_xy();
        let rect = epaint::Rect::from_min_max(
            epaint::Pos2::new(position[0] / pixels_per_point, position[1] / pixels_per_point),
            epaint::Pos2::new(position[0] / pixels_per_point + 200.0, position[1] / pixels_per_point + 200.0),
        );
        let rounding = epaint::Rounding::default();
        let color = epaint::Color32::RED;
        let rect_shape = epaint::RectShape::filled(rect, rounding, color);

        let mut mesh = epaint::Mesh::default();
        // tessellator.tessellate_rect(&rect, &mut mesh);

        let shapes = vec![ClippedShape(Rect::EVERYTHING, epaint::Shape::Rect(rect_shape))];
        self.primitives = epaint::tessellator::tessellate_shapes(
            pixels_per_point,
            TessellationOptions::default(),
            [1, 1],
            vec![],
            shapes,
        );

        // self.primitives = vec![
        //     ClippedPrimitive {
        //         clip_rect: Rect::EVERYTHING,
        //         primitive: epaint::Primitive::Mesh(epaint::Mesh { indices: INDICES.to_vec(), vertices: VERTICES.to_vec(), texture_id: epaint::TextureId::Managed(0) })
        //     }
        // ];

        let mut renderer =
            RenderComponent::new(&scene.device, scene.config.format, None, 1);
        renderer.set_position(&self.transform.get_xy());

        let mut offscreen_renderer = RenderComponent::new(
            &scene.device,
            wgpu::TextureFormat::Rgba8Unorm,
            None, 
            1
        );
        offscreen_renderer.set_position(&self.transform.get_xy());
        offscreen_renderer.set_selection_color([
            ((self.id + 1 >> 0) & 0xFF) as f32 / 0xFF as f32,
            ((self.id + 1 >> 8) & 0xFF) as f32 / 0xFF as f32,
            ((self.id + 1 >> 16) & 0xFF) as f32 / 0xFF as f32,
            1.0,
        ]);

        self.renderer = Some(renderer);
        self.offscreen_renderer = Some(offscreen_renderer);
    }

    fn detach(&mut self) {
        self.renderer = None;
        self.offscreen_renderer = None;
    }

    fn update_texture(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        id: epaint::TextureId,
        image_delta: &epaint::ImageDelta,
    ) {
        self.renderer.as_mut().map_or((), |r| r.update_texture(device, queue, id, image_delta));
        self.offscreen_renderer.as_mut().map_or((), |r| r.update_texture(device, queue, id, image_delta));
    }

    fn update_buffers(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, screen_descriptor: &ScreenDescriptor) {
        self.renderer.as_mut().map_or((), |r| r.update_buffers(device, queue, screen_descriptor, &self.primitives));
        self.offscreen_renderer
            .as_mut()
            .map_or((), |r| r.update_buffers(device, queue, screen_descriptor, &self.primitives));
    }

    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, screen_descriptor: &ScreenDescriptor, offscreen: bool) {
        log::debug!("Rendering rectangle");
        if offscreen {
            self.offscreen_renderer
                .as_ref()
                .map_or((), |r| r.render(render_pass, screen_descriptor, &self.primitives));
        } else {
            self.renderer.as_ref().map_or((), |r| r.render(render_pass, screen_descriptor, &self.primitives));
        }
    }
}

impl RadiantMessageHandler<RadiantRectangleMessage> for RadiantRectangleNode {
    fn handle_message(&mut self, message: RadiantRectangleMessage) {
        match message {
            RadiantRectangleMessage::Transform(message) => {
                self.transform.handle_message(message);
                // self.renderer.set_position(&self.transform.get_xy());
                // self.offscreen_renderer.set_position(&self.transform.get_xy());
            }
            RadiantRectangleMessage::Selection(message) => {
                self.selection.handle_message(message);
                // self.renderer
                //     .set_selection_color([1.0, 0.0, 0.0, if self.selection.is_selected() { 1.0 } else { 0.0 }]);
            }
        }
    }
}
