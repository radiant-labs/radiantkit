use crate::{RadiantDocumentNode, RadiantIdentifiable, RadiantMessage, RadiantRenderable};
use crate::{RadiantNodeType, RadiantResponse, RadiantTool};

pub struct RadiantScene {
    pub config: wgpu::SurfaceConfiguration,
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub document: RadiantDocumentNode,
    pub tool: RadiantTool,
    pub handler: Box<dyn Fn(RadiantResponse)>,

    pub current_texture: Option<wgpu::SurfaceTexture>,
    pub current_view: Option<wgpu::TextureView>,
}

impl RadiantScene {
    pub fn new(
        config: wgpu::SurfaceConfiguration,
        surface: wgpu::Surface,
        device: wgpu::Device,
        queue: wgpu::Queue,
        handler: Box<dyn Fn(RadiantResponse)>,
    ) -> Self {
        Self {
            config,
            surface,
            device,
            queue,
            document: RadiantDocumentNode::new(),
            tool: RadiantTool::Selection,
            handler,

            current_texture: None,
            current_view: None,
        }
    }
}

impl RadiantScene {
    pub fn add(&mut self, mut node: RadiantNodeType) {
        let id = node.get_id();
        node.attach_to_scene(self);
        self.document.add(node);

        let response = self.handle_message(RadiantMessage::SelectNode(id));
        self.handle_response(response);
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.document.update(&mut self.queue);

        let output = self.surface.get_current_texture()?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
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

            self.document.render(&mut render_pass, false);
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));

        #[cfg(target_arch = "wasm32")]
        output.present();

        #[cfg(not(target_arch = "wasm32"))]
        {
            self.current_texture = Some(output);
            self.current_view = Some(view);
        }

        Ok(())
    }

    fn handle_response(&self, response: Option<RadiantResponse>) {
        if let Some(response) = response {
            (self.handler)(response);
        }
    }
}
