use radiant_core::{
    RadiantMessage, RadiantNodeType, RadiantRectangleNode, RadiantRenderable, RadiantResponse,
    RadiantScene, RadiantTool, ScreenDescriptor,
};
use winit::window::Window;
use winit::{event::*, event_loop::ControlFlow};

pub struct RadiantApp {
    pub window: Window,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub scene: RadiantScene,

    offscreen_texture: Option<wgpu::Texture>,
    offscreen_texture_view: Option<wgpu::TextureView>,
    offscreen_buffer: Option<wgpu::Buffer>,

    mouse_position: [f32; 2],
}

impl RadiantApp {
    pub async fn new(window: Window, handler: Box<dyn Fn(RadiantResponse)>) -> Self {
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

        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [size.width, size.height],
            pixels_per_point: window.scale_factor() as f32
        };
        let scene = RadiantScene::new(config, surface, device, queue, screen_descriptor, handler);
        let mouse_position = [0.0, 0.0];

        Self {
            window,
            size,
            scene,
            mouse_position,
            offscreen_texture: None,
            offscreen_texture_view: None,
            offscreen_buffer: None,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.scene.config.width = new_size.width;
            self.scene.config.height = new_size.height;
            self.scene
                .surface
                .configure(&self.scene.device, &self.scene.config);
            self.scene.screen_descriptor = ScreenDescriptor {
                size_in_pixels: [new_size.width, new_size.height],
                pixels_per_point: self.window.scale_factor() as f32
            };

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

    pub async fn select(&mut self) -> u64 {
        log::info!("Selecting...");

        let texture_width = self.size.width;
        let texture_height = self.size.height;
        let u32_size = std::mem::size_of::<u32>() as u32;

        let mut encoder = self
            .scene
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        self.scene.render(&self.offscreen_texture_view);
        
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

        id
    }

    pub fn handle_event(
        &mut self,
        event: &Event<()>,
        control_flow: &mut ControlFlow,
    ) -> Option<RadiantResponse> {
        log::debug!("Event: {:?}", event);
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == &self.window.id() => {
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
                            if button == &MouseButton::Left
                                && is_pressed
                                && self.scene.tool == RadiantTool::Rectangle
                            {
                                let node = RadiantRectangleNode::new(
                                    self.scene.document.counter,
                                    self.mouse_position,
                                    // [
                                    //     (self.mouse_position[0] / self.size.width as f32 - 0.5)
                                    //         * 2.0,
                                    //     (0.5 - self.mouse_position[1] / self.size.height as f32)
                                    //         * 2.0,
                                    // ],
                                );
                                self.scene.add(RadiantNodeType::Rectangle(node));
                                self.window.request_redraw();
                            }
                        }
                        WindowEvent::CursorMoved { position, .. } => {
                            self.mouse_position = [position.x as f32, position.y as f32];
                            if self.scene.tool == RadiantTool::Selection {
                                let id = pollster::block_on(self.select());
                                if id > 0 {
                                    let message = RadiantMessage::SelectNode(id - 1);
                                    return self.scene.handle_message(message);
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == &self.window.id() => {
                match self.scene.render(&None) {
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
        None
    }
}

impl RadiantApp {
    pub fn handle_message(&mut self, message: RadiantMessage) -> Option<RadiantResponse> {
        self.scene.handle_message(message)
    }
}
