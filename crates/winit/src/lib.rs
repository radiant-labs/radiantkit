use radiantkit_core::{
    RadiantNode, RadiantScene, RadiantSceneMessage, Runtime, ScreenDescriptor, Vec3, View,
};
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};
use winit::{event::*, event_loop::ControlFlow};

use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use winit::dpi::PhysicalSize;
pub use winit::event::Event::RedrawRequested;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use winit::platform::web::EventLoopExtWebSys;

pub struct RadiantView<M, N: RadiantNode> {
    pub window: Arc<Window>,
    pub event_loop: Option<EventLoop<()>>,

    pub size: winit::dpi::PhysicalSize<u32>,

    pub scene: Arc<RwLock<RadiantScene<M, N>>>,

    mouse_position: [f32; 2],
    mouse_dragging: bool,
}

impl<M: From<RadiantSceneMessage> + TryInto<RadiantSceneMessage>, N: RadiantNode>
    RadiantView<M, N>
{
    pub async fn new(size: Option<Vec3>) -> Self {
        let event_loop = EventLoop::new();
        let window;

        #[cfg(not(target_arch = "wasm32"))]
        {
            window = WindowBuilder::new().build(&event_loop).unwrap();
            if let Some(size) = size {
                window.set_inner_size(PhysicalSize::new(size.x, size.y));
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            use winit::platform::web::WindowExtWebSys;
            let win = web_sys::window().unwrap();
            let doc = win.document().unwrap();

            let canvas = doc
                .query_selector("#radiant-canvas")
                .expect("Cannot query for canvas element.");
            if let Some(canvas) = canvas {
                let canvas = canvas.dyn_into::<web_sys::HtmlCanvasElement>().ok();
                use winit::platform::web::WindowBuilderExtWebSys;
                window = WindowBuilder::new()
                    .with_canvas(canvas)
                    .build(&event_loop)
                    .unwrap();
            } else {
                window = WindowBuilder::new().build(&event_loop).unwrap();

                let dst = doc
                    .get_element_by_id("canvas-container")
                    .expect("Couldn't append canvas to document body.");
                let canvas = web_sys::Element::from(window.canvas());
                dst.append_child(&canvas)
                    .ok()
                    .expect("Couldn't append canvas to document body.");
            }
        }

        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        // The surface needs to live as long as the window that created it.
        // State owns the window so this should be safe.
        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let window = Arc::new(window);
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(size) = size {
                window.set_inner_size(PhysicalSize::new(size.x, size.y));
            } else {
                let window_clone = window.clone();

                let closure =
                    wasm_bindgen::closure::Closure::wrap(Box::new(move |_: web_sys::Event| {
                        let win = web_sys::window().unwrap();
                        let w = win.inner_width().unwrap().as_f64().unwrap() as u32;
                        let h = win.inner_height().unwrap().as_f64().unwrap() as u32;
                        let scale_factor = window_clone.scale_factor() as u32;
                        window_clone
                            .set_inner_size(PhysicalSize::new(w * scale_factor, h * scale_factor));
                    })
                        as Box<dyn FnMut(_)>);
                let win = web_sys::window().unwrap();
                win.add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
                    .unwrap();
                closure.forget();

                // Winit prevents sizing with CSS, so we have to set
                // the size manually when on web.
                let win = web_sys::window().unwrap();
                let w = win.inner_width().unwrap().as_f64().unwrap() as u32;
                let h = win.inner_height().unwrap().as_f64().unwrap() as u32;
                let scale_factor = window.scale_factor() as u32;
                window.set_inner_size(PhysicalSize::new(w * scale_factor, h * scale_factor));
            }
        }

        let size = window.inner_size();

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
                        wgpu::Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits())
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

        let scene = RadiantScene::new(config, surface, device, queue, screen_descriptor);

        Self {
            window,
            event_loop: Some(event_loop),

            size,
            scene: Arc::new(RwLock::new(scene)),
            mouse_position: [0.0, 0.0],
            mouse_dragging: false,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.scene_mut().resize([new_size.width, new_size.height]);
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
                            let ScreenDescriptor {
                                pixels_per_point, ..
                            } = self.scene().screen_descriptor;
                            let current_position = [
                                position.x as f32 / pixels_per_point,
                                position.y as f32 / pixels_per_point,
                            ];
                            self.mouse_position = current_position;
                            return self.on_mouse_move(self.mouse_position);
                            //     self.window.request_redraw();
                            // }
                        }
                        WindowEvent::Touch(Touch {
                            location, phase, ..
                        }) => {
                            let ScreenDescriptor {
                                pixels_per_point, ..
                            } = self.scene().screen_descriptor;
                            let window_origin = self.window.outer_position().unwrap_or_default();
                            let current_position = [
                                (location.x as f32 - window_origin.x as f32) / pixels_per_point,
                                (location.y as f32 - window_origin.y as f32) / pixels_per_point,
                            ];
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
                let size = self.size;
                let needs_resize = match self.scene_mut().render() {
                    Ok(_) => false,
                    Err(wgpu::SurfaceError::Lost) => true,
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        *control_flow = ControlFlow::Exit;
                        false
                    }
                    Err(e) => {
                        eprintln!("{:?}", e);
                        false
                    }
                };
                if needs_resize {
                    self.resize(size);
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

impl<M: From<RadiantSceneMessage> + TryInto<RadiantSceneMessage>, N: RadiantNode>
    RadiantView<M, N>
{
    pub fn on_mouse_down(&mut self, position: [f32; 2]) -> Option<M> {
        let id = pollster::block_on(self.scene_mut().select(position));
        let node_count = self.scene().document().counter;
        self.scene_mut()
            .tool_manager
            .active_tool()
            .on_mouse_down(id, node_count, position)
    }

    pub fn on_mouse_move(&mut self, position: [f32; 2]) -> Option<M> {
        self.scene_mut()
            .tool_manager
            .active_tool()
            .on_mouse_move(position)
    }

    pub fn on_mouse_up(&mut self, position: [f32; 2]) -> Option<M> {
        self.scene_mut()
            .tool_manager
            .active_tool()
            .on_mouse_up(position)
    }
}

impl<M: From<RadiantSceneMessage> + TryInto<RadiantSceneMessage>, N: RadiantNode> View<M, N>
    for RadiantView<M, N>
{
    fn scene(&self) -> RwLockReadGuard<RadiantScene<M, N>> {
        self.scene.read().unwrap()
    }

    fn scene_mut(&mut self) -> RwLockWriteGuard<RadiantScene<M, N>> {
        self.scene.write().unwrap()
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn run_native<
    M: From<RadiantSceneMessage> + TryInto<RadiantSceneMessage> + 'static,
    N: RadiantNode + 'static,
    R: 'static,
>(
    mut runtime: impl Runtime<'static, M, N, R, View = RadiantView<M, N>> + 'static,
    handler: Box<dyn Fn(R)>,
) {
    if let Some(event_loop) = std::mem::replace(&mut runtime.view_mut().event_loop, None) {
        event_loop.run(move |event, _, control_flow| {
            if let Some(message) = runtime.view_mut().handle_event(&event, control_flow) {
                if let Some(response) = runtime.handle_message(message) {
                    handler(response);
                }
            }

            match event {
                RedrawRequested(..) => {
                    let output_frame = std::mem::replace(
                        &mut runtime
                            .view_mut()
                            .scene_mut()
                            .render_manager
                            .current_texture,
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
    M: From<RadiantSceneMessage> + TryInto<RadiantSceneMessage> + 'static,
    N: RadiantNode + 'static,
    R: serde::ser::Serialize + 'static,
>(
    runtime: Arc<RwLock<impl Runtime<'static, M, N, R, View = RadiantView<M, N>> + 'static>>,
    f: js_sys::Function,
) {
    let event_loop;
    {
        let Ok(mut runtime_) = runtime.write() else { return; };
        event_loop = std::mem::replace(&mut runtime_.view_mut().event_loop, None);
    }

    let weak_runtime = Arc::downgrade(&runtime);

    if let Some(event_loop) = event_loop {
        event_loop.spawn(move |event, _, control_flow| {
            if let Some(runtime) = weak_runtime.upgrade() {
                if let Ok(mut runtime) = runtime.write() {
                    if let Some(message) = runtime.view_mut().handle_event(&event, control_flow) {
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
