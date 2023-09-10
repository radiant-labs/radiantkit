use radiant_core::{RadiantNodeType, RadiantRectangleNode, RadiantScene, RadiantRenderable};
use winit::window::Window;
use winit::{event::*, event_loop::ControlFlow};

pub struct RadiantApp {
    pub window: Window,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub scene: RadiantScene,

    config: wgpu::SurfaceConfiguration,
    offscreen_texture: Option<wgpu::Texture>,
    offscreen_texture_view: Option<wgpu::TextureView>,
    offscreen_buffer: Option<wgpu::Buffer>,

    mouse_position: [f32; 2],
}

impl RadiantApp {
    pub async fn new(window: Window) -> Self {
        let size = window.inner_size();

        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        // The surface needs to live as long as the window that created it.
        // State owns the window so this should be safe.
        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web we'll have to disable some.
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label: None,
                },
                None, // Trace path
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        let scene = RadiantScene::new(surface, device, queue);
        let mouse_position = [0.0, 0.0];

        Self {
            window,
            size,
            scene,
            config,
            mouse_position,
            offscreen_texture: None,
            offscreen_texture_view: None,
            offscreen_buffer: None,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.scene.surface.configure(&self.scene.device, &self.config);

            let texture_width = self.size.width;
            let texture_height = self.size.height;

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
            let texture = self.scene.device.create_texture(&texture_desc);
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
            self.offscreen_buffer = Some(self.scene.device.create_buffer(&output_buffer_desc));
        }
    }

    pub fn input(&mut self, _event: &WindowEvent) -> bool {
        false
    }

    pub async fn select(&self) -> u64 {
        log::info!("Selecting...");
    
        let texture_width = self.size.width;
        let texture_height = self.size.height;
        let u32_size = std::mem::size_of::<u32>() as u32;

        let mut encoder = self.
            scene.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let render_pass_desc = wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.offscreen_texture_view.as_ref().unwrap(),
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
            };
            let mut render_pass = encoder.begin_render_pass(&render_pass_desc);

            self.scene.document.render(&mut render_pass, true);
        }

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

        self.scene.queue.submit(Some(encoder.finish()));

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
            self.scene.device.poll(wgpu::Maintain::Wait);
            rx.receive().await.unwrap().unwrap();

            let data = buffer_slice.get_mapped_range();

            let posx: u32 = self.mouse_position[0].round() as u32;
            let posy: u32 = self.mouse_position[1].round() as u32;
            let index = (posy * texture_width * 4 + posx * 4) as usize;

            id = *data.get(index).unwrap() as u64;
            id += (*data.get(index+1).unwrap() as u64) << 8;
            id += (*data.get(index+2).unwrap() as u64) << 16;
            log::info!("id: {}", id);

            // use image::{ImageBuffer, Rgba};
            // let buffer =
            //     ImageBuffer::<Rgba<u8>, _>::from_raw(texture_width, texture_height, data).unwrap();

            // #[cfg(not(target_arch = "wasm32"))]
            // buffer.save("image.png").unwrap();
        }
        self.offscreen_buffer.as_ref().unwrap().unmap();

        id   
    }

    pub fn handle_event(&mut self, event: Event<()>, control_flow: &mut ControlFlow) {
        log::debug!("Event: {:?}", event);
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == self.window.id() => {
                if !self.input(event) {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            self.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            self.resize(**new_inner_size);
                        }
                        WindowEvent::MouseInput { state, button, .. } => {
                            let is_pressed = *state == ElementState::Pressed;
                            if button == &MouseButton::Left && is_pressed {
                                let node = RadiantRectangleNode::new(
                                    self.scene.document.counter,
                                    &self.scene.device,
                                    &self.config,
                                    [
                                        (self.mouse_position[0]
                                            / self.size.width as f32
                                            - 0.5)
                                            * 2.0,
                                        (0.5 - self.mouse_position[1]
                                            / self.size.height as f32)
                                            * 2.0,
                                    ],
                                );
                                self.scene.document.add(RadiantNodeType::Rectangle(node));
                                self.window.request_redraw();
                            }
                        }
                        WindowEvent::CursorMoved { position, .. } => {
                            self.mouse_position = [position.x as f32, position.y as f32];
                            let id = pollster::block_on(
                                self.select(),
                            );
                            if id > 0 {
                                self.scene.document.select(id - 1);
                            }
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == self.window.id() => {
                match self.scene.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => self.resize(self.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                self.window.request_redraw();
            }
            _ => {}
        }
    }
}
