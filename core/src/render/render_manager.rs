use crate::{RadiantRenderer, ScreenDescriptor};
use epaint::{ClippedPrimitive, ImageDelta};

pub struct RadiantRenderManager {
    pub config: wgpu::SurfaceConfiguration,
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,

    pub renderer: RadiantRenderer,
    pub offscreen_renderer: RadiantRenderer,

    offscreen_texture: Option<wgpu::Texture>,
    offscreen_texture_view: Option<wgpu::TextureView>,
    offscreen_buffer: Option<wgpu::Buffer>,

    pub current_texture: Option<wgpu::SurfaceTexture>,
    pub current_view: Option<wgpu::TextureView>,
}

impl RadiantRenderManager {
    pub fn new(
        config: wgpu::SurfaceConfiguration,
        surface: wgpu::Surface,
        device: wgpu::Device,
        queue: wgpu::Queue,
        font_image_delta: Option<ImageDelta>,
    ) -> Self {
        let mut renderer = RadiantRenderer::new(&device, config.format, None, 1);
        let mut offscreen_renderer =
            RadiantRenderer::new(&device, wgpu::TextureFormat::Rgba8Unorm, None, 1);

        if let Some(image_delta) = font_image_delta {
            renderer.update_texture(&device, &queue, epaint::TextureId::default(), &image_delta);
            offscreen_renderer.update_texture(
                &device,
                &queue,
                epaint::TextureId::default(),
                &image_delta,
            );
        }

        Self {
            config,
            surface,
            device,
            queue,

            renderer,
            offscreen_renderer,

            offscreen_texture: None,
            offscreen_texture_view: None,
            offscreen_buffer: None,

            current_texture: None,
            current_view: None,
        }
    }

    pub fn resize(&mut self, new_size: [u32; 2]) {
        if new_size[0] > 0 && new_size[1] > 0 {
            self.config.width = new_size[0];
            self.config.height = new_size[1];
            self.surface.configure(&self.device, &self.config);

            let texture_width = new_size[0];
            let texture_height = new_size[1];

            let texture_desc = wgpu::TextureDescriptor {
                size: wgpu::Extent3d {
                    width: texture_width,
                    height: texture_height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8Unorm,
                usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
                label: None,
                view_formats: &[],
            };
            let texture = self.device.create_texture(&texture_desc);
            self.offscreen_texture_view = Some(texture.create_view(&Default::default()));
            self.offscreen_texture = Some(texture);

            // we need to store this for later
            let u32_size = std::mem::size_of::<u32>() as u32;

            let output_buffer_size =
                (u32_size * texture_width * texture_height) as wgpu::BufferAddress;
            let output_buffer_desc = wgpu::BufferDescriptor {
                size: output_buffer_size,
                usage: wgpu::BufferUsages::COPY_DST
                    // this tells wpgu that we want to read this buffer from the cpu
                    | wgpu::BufferUsages::MAP_READ,
                label: None,
                mapped_at_creation: false,
            };
            self.offscreen_buffer = Some(self.device.create_buffer(&output_buffer_desc));
        }
    }

    pub fn render(
        &mut self,
        primitives: Vec<ClippedPrimitive>,
        screen_descriptor: &ScreenDescriptor,
        selection: bool,
    ) -> Result<(), wgpu::SurfaceError> {
        let mut current_texture = None;
        let view;
        if selection {
            self.offscreen_renderer.update_buffers(
                &self.device,
                &self.queue,
                screen_descriptor,
                &primitives,
            );
            view = self.offscreen_texture_view.as_ref().unwrap();
        } else {
            self.renderer
                .update_buffers(&self.device, &self.queue, screen_descriptor, &primitives);

            let output = self.surface.get_current_texture()?;
            let v = output
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            self.current_view = Some(v);
            view = self.current_view.as_ref().unwrap();

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

            if selection {
                self.offscreen_renderer
                    .render(&mut render_pass, screen_descriptor, &primitives);
            } else {
                self.renderer
                    .render(&mut render_pass, screen_descriptor, &primitives);
            }
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));

        #[cfg(target_arch = "wasm32")]
        if !selection {
            current_texture.unwrap().present();
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            self.current_texture = current_texture;
        }

        Ok(())
    }

    pub async fn render_offscreen(
        &mut self,
        primitives: Vec<ClippedPrimitive>,
        screen_descriptor: &ScreenDescriptor,
        selection: bool,
        mouse_position: [f32; 2],
    ) -> Result<u64, wgpu::SurfaceError> {
        let texture_width = screen_descriptor.size_in_pixels[0];
        let texture_height = screen_descriptor.size_in_pixels[1];
        let u32_size = std::mem::size_of::<u32>() as u32;

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        self.render(primitives, screen_descriptor, selection)?;

        encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &self.offscreen_texture.as_ref().unwrap(),
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::ImageCopyBuffer {
                buffer: &self.offscreen_buffer.as_ref().unwrap(),
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(u32_size * texture_width),
                    rows_per_image: Some(texture_height),
                },
            },
            wgpu::Extent3d {
                width: texture_width,
                height: texture_height,
                depth_or_array_layers: 1,
            },
        );

        self.queue.submit(Some(encoder.finish()));

        let mut id: u64;

        // We need to scope the mapping variables so that we can
        // unmap the buffer
        {
            let buffer = self.offscreen_buffer.as_ref().unwrap();
            let buffer_slice = buffer.slice(..);

            // NOTE: We have to create the mapping THEN device.poll() before await
            // the future. Otherwise the application will freeze.
            let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
            buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
                tx.send(result).unwrap();
            });
            self.device.poll(wgpu::Maintain::Wait);
            rx.receive().await.unwrap().unwrap();

            let data = buffer_slice.get_mapped_range();

            let posx: u32 = mouse_position[0].round() as u32;
            let posy: u32 = mouse_position[1].round() as u32;
            let index = (posy * texture_width * 4 + posx * 4) as usize;

            id = *data.get(index).unwrap() as u64;
            id += (*data.get(index + 1).unwrap() as u64) << 8;
            id += (*data.get(index + 2).unwrap() as u64) << 16;

            log::info!("id: {}", id);

            // use image::{ImageBuffer, Rgba};
            // let buffer =
            //     ImageBuffer::<Rgba<u8>, _>::from_raw(texture_width, texture_height, data).unwrap();

            // #[cfg(not(target_arch = "wasm32"))]
            // buffer.save("image.png").unwrap();
        }
        self.offscreen_buffer.as_ref().unwrap().unmap();

        Ok(id)
    }
}
