use epaint::{text::FontDefinitions, Fonts};

use crate::{
    RadiantDocumentNode, RadiantMessage, RadiantMessageHandler, RadiantNode, RadiantNodeType,
    RadiantRenderer, RadiantResponse, RadiantTessellatable, RadiantTool, RadiantTransformable,
    TransformComponent,
};

/// Information about the screen used for rendering.
pub struct ScreenDescriptor {
    /// Size of the window in physical pixels.
    pub size_in_pixels: [u32; 2],

    /// HiDPI scale factor (pixels per point).
    pub pixels_per_point: f32,
}

impl ScreenDescriptor {
    /// size in "logical" points
    pub fn screen_size_in_points(&self) -> [f32; 2] {
        [
            self.size_in_pixels[0] as f32 / self.pixels_per_point,
            self.size_in_pixels[1] as f32 / self.pixels_per_point,
        ]
    }
}

pub struct RadiantScene {
    pub config: wgpu::SurfaceConfiguration,
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub document: RadiantDocumentNode,
    pub tool: RadiantTool,
    pub handler: Box<dyn Fn(RadiantResponse)>,

    pub screen_descriptor: ScreenDescriptor,

    pub current_texture: Option<wgpu::SurfaceTexture>,
    pub current_view: Option<wgpu::TextureView>,

    pub fonts: epaint::Fonts,

    pub renderer: RadiantRenderer,
    pub offscreen_renderer: RadiantRenderer,
}

impl RadiantScene {
    pub fn new(
        config: wgpu::SurfaceConfiguration,
        surface: wgpu::Surface,
        device: wgpu::Device,
        queue: wgpu::Queue,
        screen_descriptor: ScreenDescriptor,
        handler: Box<dyn Fn(RadiantResponse)>,
    ) -> Self {
        let font_definitions = FontDefinitions::default();
        let fonts = Fonts::new(screen_descriptor.pixels_per_point, 1600, font_definitions);

        let renderer = RadiantRenderer::new(&device, config.format, None, 1);
        let offscreen_renderer =
            RadiantRenderer::new(&device, wgpu::TextureFormat::Rgba8Unorm, None, 1);

        Self {
            config,
            surface,
            device,
            queue,
            document: RadiantDocumentNode::new(),
            tool: RadiantTool::Selection,
            handler,

            screen_descriptor,

            current_texture: None,
            current_view: None,

            fonts,

            renderer,
            offscreen_renderer,
        }
    }
}

impl RadiantScene {
    pub fn add(&mut self, mut node: RadiantNodeType) {
        let id = node.get_id();
        node.attach_to_scene(self);
        self.document.add(node);

        self.fonts
            .begin_frame(self.screen_descriptor.pixels_per_point, 1600);
        if let Some(image_delta) = self.fonts.font_image_delta() {
            self.renderer.update_texture(
                &self.device,
                &self.queue,
                epaint::TextureId::default(),
                &image_delta,
            );
            self.offscreen_renderer.update_texture(
                &self.device,
                &self.queue,
                epaint::TextureId::default(),
                &image_delta,
            );
        }

        let response = self.handle_message(RadiantMessage::SelectNode(id));
        self.handle_response(response);
    }

    pub fn render(
        &mut self,
        texture_view: &Option<wgpu::TextureView>,
    ) -> Result<(), wgpu::SurfaceError> {
        let primitives = self
            .document
            .tessellate(texture_view.is_some(), &self.screen_descriptor);

        let mut current_texture = None;
        let offscreen;
        let view;
        if let Some(texture_view) = texture_view {
            self.offscreen_renderer.update_buffers(
                &self.device,
                &self.queue,
                &self.screen_descriptor,
                &primitives,
            );
            view = texture_view;
            offscreen = true;
        } else {
            self.renderer.update_buffers(
                &self.device,
                &self.queue,
                &self.screen_descriptor,
                &primitives,
            );

            let output = self.surface.get_current_texture()?;
            let v = output
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            self.current_view = Some(v);
            view = self.current_view.as_ref().unwrap();

            offscreen = false;
            current_texture = Some(output);
        }

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            if offscreen {
                self.offscreen_renderer.render(
                    &mut render_pass,
                    &self.screen_descriptor,
                    &primitives,
                );
            } else {
                self.renderer
                    .render(&mut render_pass, &self.screen_descriptor, &primitives);
            }
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));

        #[cfg(target_arch = "wasm32")]
        if !offscreen {
            current_texture.unwrap().present();
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            self.current_texture = current_texture;
        }

        Ok(())
    }

    fn handle_response(&self, response: Option<RadiantResponse>) {
        if let Some(response) = response {
            (self.handler)(response);
        }
    }
}

impl RadiantMessageHandler for RadiantScene {
    fn handle_message(&mut self, message: RadiantMessage) -> Option<RadiantResponse> {
        match message {
            RadiantMessage::AddArtboard => {
                self.document.add_artboard();
            }
            RadiantMessage::SelectArtboard(id) => {
                self.document.set_active_artboard(id);
            }
            RadiantMessage::SelectNode(id) => {
                self.document.select(id);
                if let Some(node) = self.document.get_node(id) {
                    return Some(RadiantResponse::NodeSelected(node.clone()));
                }
            }
            RadiantMessage::TransformNode {
                id,
                position,
                scale,
            } => {
                if let Some(node) = self.document.get_node_mut(id) {
                    if let Some(component) = node.get_component_mut::<TransformComponent>() {
                        component.transform_xy(&position);
                        component.set_scale(&scale);
                        node.set_needs_tessellation();
                    }
                }
            }
            RadiantMessage::SelectTool(tool) => {
                self.tool = tool;
            }
        }
        None
    }
}
