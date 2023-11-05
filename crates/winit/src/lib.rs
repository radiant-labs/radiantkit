use radiant_core::{InteractionMessage, RadiantNode, RadiantScene, RadiantTool, ScreenDescriptor};
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};
use winit::{event::*, event_loop::ControlFlow};

pub use winit::event::Event::RedrawRequested;

#[cfg(target_arch = "wasm32")]
use std::sync::{Arc, RwLock};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use winit::platform::web::EventLoopExtWebSys;

pub struct RadiantView<M, N: RadiantNode> {
    pub window: Window,
    pub event_loop: Option<EventLoop<()>>,

    pub size: winit::dpi::PhysicalSize<u32>,

    pub scene: RadiantScene<M, N>,

    mouse_position: [f32; 2],
    mouse_dragging: bool,
}

impl<M: From<InteractionMessage> + TryInto<InteractionMessage>, N: RadiantNode> RadiantView<M, N> {
    pub async fn new(default_tool: impl RadiantTool<M> + 'static) -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        #[cfg(target_arch = "wasm32")]
        {
            // Winit prevents sizing with CSS, so we have to set
            // the size manually when on web.
            use winit::dpi::PhysicalSize;
            window.set_inner_size(PhysicalSize::new(1600, 1200));

            use winit::platform::web::WindowExtWebSys;
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let dst = doc.get_element_by_id("canvas-container")?;
                    let canvas = web_sys::Element::from(window.canvas());
                    dst.append_child(&canvas).ok()?;
                    Some(())
                })
                .expect("Couldn't append canvas to document body.");
        }

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
            pixels_per_point: window.scale_factor() as f32,
        };

        let scene = RadiantScene::new(
            config,
            surface,
            device,
            queue,
            screen_descriptor,
            default_tool,
        );

        Self {
            window,
            event_loop: Some(event_loop),

            size,
            scene,
            mouse_position: [0.0, 0.0],
            mouse_dragging: false,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.scene.resize([new_size.width, new_size.height]);
    }

    pub fn input(&mut self, _event: &WindowEvent) -> bool {
        false
    }

    pub fn handle_event(&mut self, event: &Event<()>, control_flow: &mut ControlFlow) -> Option<M> {
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
                            if button == &MouseButton::Left {
                                let is_pressed = *state == ElementState::Pressed;
                                self.mouse_dragging = is_pressed;
                                if is_pressed {
                                    return self.on_mouse_down(self.mouse_position);
                                    //     self.window.request_redraw();
                                    // }
                                } else {
                                    return self.on_mouse_up(self.mouse_position);
                                    //     self.window.request_redraw();
                                    // }
                                }
                            }
                        }
                        WindowEvent::CursorMoved { position, .. } => {
                            let current_position = [position.x as f32, position.y as f32];
                            self.mouse_position = current_position;
                            return self.on_mouse_move(self.mouse_position);
                            //     self.window.request_redraw();
                            // }
                        }
                        WindowEvent::Touch(Touch {
                            location, phase, ..
                        }) => {
                            let current_position = [location.x as f32, location.y as f32];
                            self.mouse_position = current_position;
                            match phase {
                                TouchPhase::Started => {
                                    return self.on_mouse_down(self.mouse_position);
                                    //     self.window.request_redraw();
                                    // }
                                }
                                TouchPhase::Moved => {
                                    return self.on_mouse_move(self.mouse_position);
                                    //     self.window.request_redraw();
                                    // }
                                }
                                TouchPhase::Ended => {
                                    return self.on_mouse_up(self.mouse_position);
                                    //     self.window.request_redraw();
                                    // }
                                }
                                TouchPhase::Cancelled => {
                                    return self.on_mouse_up(self.mouse_position);
                                    //     self.window.request_redraw();
                                    // }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == &self.window.id() => {
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
        None
    }
}

impl<M: From<InteractionMessage> + TryInto<InteractionMessage>, N: RadiantNode> RadiantView<M, N> {
    pub fn on_mouse_down(&mut self, position: [f32; 2]) -> Option<M> {
        let mut id = pollster::block_on(self.scene.select(position));
        // Todo: Hack - To be removed
        if id > 1000 {
            id = self.scene.document.counter;
        }
        self.scene
            .tool_manager
            .active_tool()
            .on_mouse_down(id, position)
    }

    pub fn on_mouse_move(&mut self, position: [f32; 2]) -> Option<M> {
        self.scene
            .tool_manager
            .active_tool()
            .on_mouse_move(position)
    }

    pub fn on_mouse_up(&mut self, position: [f32; 2]) -> Option<M> {
        self.scene.tool_manager.active_tool().on_mouse_up(position)
    }
}

pub trait Runtime<M: From<InteractionMessage> + TryInto<InteractionMessage>, N: RadiantNode, R> {
    fn view(&mut self) -> &mut RadiantView<M, N>;
    fn handle_message(&mut self, message: M) -> Option<R>;

    fn scene(&mut self) -> &mut RadiantScene<M, N> { &mut self.view().scene }
    fn add(&mut self, node: N) { self.scene().add(node); }
}

pub fn run_native<
    M: From<InteractionMessage> + TryInto<InteractionMessage> + 'static,
    N: RadiantNode + 'static,
    R: 'static,
>(
    mut runtime: impl Runtime<M, N, R> + 'static,
    handler: Box<dyn Fn(R)>,
) {
    if let Some(event_loop) = std::mem::replace(&mut runtime.view().event_loop, None) {
        event_loop.run(move |event, _, control_flow| {
            if let Some(message) = runtime.view().handle_event(&event, control_flow) {
                if let Some(response) = runtime.handle_message(message) {
                    handler(response);
                }
            }

            match event {
                RedrawRequested(..) => {
                    let output_frame = std::mem::replace(
                        &mut runtime.view().scene.render_manager.current_texture,
                        None,
                    );
                    output_frame.unwrap().present();
                }
                _ => {}
            }
        });
    }
}

#[cfg(target_arch = "wasm32")]
pub fn run_wasm<
    M: From<InteractionMessage> + TryInto<InteractionMessage> + 'static,
    N: RadiantNode + 'static,
    R: serde::ser::Serialize,
>(
    runtime: Arc<RwLock<impl Runtime<M, N, R> + 'static>>,
    f: js_sys::Function,
) {
    let event_loop;
    {
        let Ok(mut runtime_) = runtime.write() else { return; };
        event_loop = std::mem::replace(&mut runtime_.view().event_loop, None);
    }

    let weak_runtime = Arc::downgrade(&runtime);

    if let Some(event_loop) = event_loop {
        event_loop.spawn(move |event, _, control_flow| {
            if let Some(runtime) = weak_runtime.upgrade() {
                if let Ok(mut runtime) = runtime.write() {
                    if let Some(message) = runtime.view().handle_event(&event, control_flow) {
                        if let Some(response) = runtime.handle_message(message) {
                            let this = JsValue::null();
                            let _ =
                                f.call1(&this, &serde_wasm_bindgen::to_value(&response).unwrap());
                        }
                    }
                }
            }
        });
    }
}
