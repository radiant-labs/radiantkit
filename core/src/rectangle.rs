use crate::RenderComponent;

use super::{RadiantNode, RadiantNodeRenderable, RadiantVertex, TransformComponent};
use std::sync::Arc;
use wgpu::util::DeviceExt;

const VERTICES: &[RadiantVertex] = &[
    RadiantVertex {
        position: [0.5, 0.5, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // A
    RadiantVertex {
        position: [-0.5, 0.5, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // B
    RadiantVertex {
        position: [-0.5, -0.5, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // C
    RadiantVertex {
        position: [0.5, -0.5, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // D
];

const INDICES: &[u16] = &[0, 1, 2, 2, 3, 0];

pub struct RadiantRectangleNode {
    pub id: u64,
    pub transform: TransformComponent,
    pub renderer: RenderComponent,
    pub offscreen_renderer: RenderComponent,
}

impl RadiantRectangleNode {
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        position: [f32; 2],
    ) -> Self {
        let mut transform = TransformComponent::new();
        transform.set_xy(&position);

        let mut renderer = RenderComponent::new(device, config.format, &VERTICES, &INDICES);
        renderer.set_position(&position);

        let mut offscreen_renderer =
            RenderComponent::new(device, wgpu::TextureFormat::Rgba8Unorm, &VERTICES, &INDICES);
        offscreen_renderer.set_position(&position);

        Self {
            id: 0,
            transform,
            renderer,
            offscreen_renderer,
        }
    }
}

impl RadiantNode for RadiantRectangleNode {
    fn set_selected(&mut self, selected: bool) {
        self.renderer
            .set_selection_color([1.0, 0.0, 0.0, if selected { 1.0 } else { 0.0 }]);
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
        self.offscreen_renderer.set_selection_color([
            ((id + 1 >> 0) & 0xFF) as f32 / 0xFF as f32,
            ((id + 1 >> 8) & 0xFF) as f32 / 0xFF as f32,
            ((id + 1 >> 16) & 0xFF) as f32 / 0xFF as f32,
            1.0,
        ]);
    }
}

impl RadiantNodeRenderable for RadiantRectangleNode {
    fn update(&mut self, queue: &mut wgpu::Queue) {
        self.renderer.update(queue);
        self.offscreen_renderer.update(queue);
    }

    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, offscreen: bool) {
        log::debug!("Rendering rectangle");

        if offscreen {
            self.offscreen_renderer.render(render_pass, offscreen);
        } else {
            self.renderer.render(render_pass, offscreen);
        }
    }
}
