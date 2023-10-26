use radiant_scene::{RadiantMessage, RadiantScene, ScreenDescriptor, RadiantNodeType, RadiantRectangleNode, RadiantTessellatable, RadiantComponentProvider, ColorComponent, RadiantTextNode, RadiantImageNode, TransformComponent, RadiantTransformable, SelectionTool};
use serde::{Serialize, Deserialize};
use winit::window::Window;
use winit::{event::*, event_loop::ControlFlow};

pub struct RadiantApp {
    pub window: Window,
    pub size: winit::dpi::PhysicalSize<u32>,

    pub scene: RadiantScene<RadiantMessage, RadiantNodeType>,

    mouse_position: [f32; 2],
    mouse_dragging: bool,

    pub handler: Box<dyn Fn(RadiantResponse)>,
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
            pixels_per_point: window.scale_factor() as f32,
        };

        let scene = RadiantScene::new(config, surface, device, queue, screen_descriptor, SelectionTool::new());

        Self {
            window,
            size,
            scene,
            mouse_position: [0.0, 0.0],
            mouse_dragging: false,
            handler,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.scene.resize([new_size.width, new_size.height]);
    }

    pub fn input(&mut self, _event: &WindowEvent) -> bool {
        false
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
                            if button == &MouseButton::Left {
                                let is_pressed = *state == ElementState::Pressed;
                                if is_pressed {
                                    if self.on_mouse_down(self.mouse_position) {
                                        self.window.request_redraw();
                                    }
                                } else {
                                    if self.on_mouse_up(self.mouse_position) {
                                        self.window.request_redraw();
                                    }
                                }
                                self.mouse_dragging = is_pressed;
                            }
                        }
                        WindowEvent::CursorMoved { position, .. } => {
                            let current_position = [position.x as f32, position.y as f32];
                            // let transform = [
                            //     current_position[0] - self.mouse_position[0],
                            //     current_position[1] - self.mouse_position[1],
                            // ];
                            self.mouse_position = current_position;
                            if self.on_mouse_move(self.mouse_position) {
                                self.window.request_redraw();
                            }
                        }
                        WindowEvent::Touch(Touch {
                            location, phase, ..
                        }) => {
                            let current_position = [location.x as f32, location.y as f32];
                            // let transform = [
                            //     current_position[0] - self.mouse_position[0],
                            //     current_position[1] - self.mouse_position[1],
                            // ];
                            self.mouse_position = current_position;
                            match phase {
                                TouchPhase::Started => {
                                    if self.on_mouse_down(self.mouse_position) {
                                        self.window.request_redraw();
                                    }
                                }
                                TouchPhase::Moved => {
                                    if self.on_mouse_move(self.mouse_position) {
                                        self.window.request_redraw();
                                    }
                                }
                                TouchPhase::Ended => {
                                    if self.on_mouse_up(self.mouse_position) {
                                        self.window.request_redraw();
                                    }
                                }
                                TouchPhase::Cancelled => {
                                    if self.on_mouse_up(self.mouse_position) {
                                        self.window.request_redraw();
                                    }
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

impl RadiantApp {
    pub fn handle_message(&mut self, message: RadiantMessage) -> Option<RadiantResponse> {
        match message {
            RadiantMessage::AddArtboard => {
                self.scene.document.add_artboard();
            }
            RadiantMessage::SelectArtboard(id) => {
                self.scene.document.set_active_artboard(id);
            }
            RadiantMessage::SelectNode(id) => {
                if !self.scene.interaction_manager.is_interaction(id) {
                    self.scene.document.select(id);
                    if let Some(node) = self.scene.document.get_node(id) {
                        self.scene.interaction_manager
                            .enable_interactions(node, &self.scene.screen_descriptor);
                        return Some(RadiantResponse::NodeSelected(node.clone()));
                    } else {
                        self.scene.interaction_manager.disable_interactions();
                    }
                }
            }
            RadiantMessage::AddNode {
                node_type,
                position,
                scale,
            } => {
                let id = self.scene.document.counter;
                let node = match node_type.as_str() {
                    "Rectangle" =>
                        Some(RadiantNodeType::Rectangle(RadiantRectangleNode::new(
                            id,
                            position,
                            scale,
                        ))),
                    _ => None
                };
                if let Some(node) = node {
                    self.scene.add(node);
                    return self.handle_message(RadiantMessage::SelectNode(id));
                }
            }
            RadiantMessage::TransformNode {
                id,
                position,
                scale,
            } => {
                if self.scene.interaction_manager.is_interaction(id) {
                    if let Some(message) = self.scene.interaction_manager.handle_interaction(message) {
                        return self.handle_message(message);
                    }
                } else if let Some(node) = self.scene.document.get_node_mut(id) {
                    if let Some(component) = node.get_component_mut::<TransformComponent>() {
                        component.transform_xy(&position);
                        component.transform_scale(&scale);

                        let response = RadiantResponse::TransformUpdated {
                            id,
                            position: component.get_xy(),
                            scale: component.get_scale(),
                        };

                        node.set_needs_tessellation();
                        self.scene.interaction_manager
                            .update_interactions(node, &self.scene.screen_descriptor);

                        return Some(response);
                    }
                }
            }
            RadiantMessage::SetTransform {
                id,
                position,
                scale,
            } => {
                if let Some(node) = self.scene.document.get_node_mut(id) {
                    if let Some(component) = node.get_component_mut::<TransformComponent>() {
                        component.set_xy(&position);
                        component.set_scale(&scale);
                        node.set_needs_tessellation();

                        self.scene.interaction_manager
                            .update_interactions(node, &self.scene.screen_descriptor);
                    }
                }
            }
            RadiantMessage::SetFillColor { id, fill_color } => {
                if let Some(node) = self.scene.document.get_node_mut(id) {
                    if let Some(component) = node.get_component_mut::<ColorComponent>() {
                        component.set_fill_color(fill_color);
                        node.set_needs_tessellation();
                    }
                }
            }
            RadiantMessage::SetStrokeColor { id, stroke_color } => {
                if let Some(node) = self.scene.document.get_node_mut(id) {
                    if let Some(component) = node.get_component_mut::<ColorComponent>() {
                        component.set_stroke_color(stroke_color);
                        node.set_needs_tessellation();
                    }
                }
            }
            RadiantMessage::SelectTool { id } => {
                self.scene.tool_manager.activate_tool(id);
            }
            RadiantMessage::AddImage { .. } => {
                let image = epaint::ColorImage::new([400, 100], epaint::Color32::RED);
                let texture_handle =
                    self.scene.texture_manager
                        .load_texture("test", image, Default::default());

                let id = self.scene.document.counter;
                let node = RadiantNodeType::Image(RadiantImageNode::new(
                    id,
                    [400.0, 100.0],
                    [100.0, 100.0],
                    texture_handle,
                ));
                self.scene.add(node);
                return self.handle_message(RadiantMessage::SelectNode(id));
            }
            RadiantMessage::AddText { position, .. } => {
                let id = self.scene.document.counter;
                let node =
                    RadiantNodeType::Text(RadiantTextNode::new(id, position, [100.0, 100.0]));
                self.scene.add(node);
                return self.handle_message(RadiantMessage::SelectNode(id));
            }
        }
        None
    }
}

impl RadiantApp {
    pub fn process_message(&mut self, message: RadiantMessage) {
        let response = self.handle_message(message);
        self.handle_response(response);
    }

    fn handle_response(&self, response: Option<RadiantResponse>) {
        if let Some(response) = response {
            (self.handler)(response);
        }
    }
}

impl RadiantApp {
    pub fn on_mouse_down(&mut self, position: [f32; 2]) -> bool {
        let id = pollster::block_on(self.scene.select(position));
        if let Some(message) =
            self.scene.tool_manager
                .active_tool()
                .on_mouse_down(id,  position)
        {
            self.process_message(message);
            true
        } else {
            false
        }
    }

    pub fn on_mouse_move(&mut self, position: [f32; 2]) -> bool {
        if let Some(message) = self
            .scene
            .tool_manager
            .active_tool()
            .on_mouse_move(position)
        {
            self.process_message(message);
            true
        } else {
            false
        }
    }

    pub fn on_mouse_up(&mut self, position: [f32; 2]) -> bool {
        if let Some(message) = self
            .scene
            .tool_manager
            .active_tool()
            .on_mouse_up(position)
        {
            self.process_message(message);
            true
        } else {
            false
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RadiantResponse {
    NodeSelected(RadiantNodeType),
    TransformUpdated {
        id: u64,
        position: [f32; 2],
        scale: [f32; 2],
    },
}
