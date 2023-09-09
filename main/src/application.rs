use super::renderer::RenderState;
use radiant_core::{RadiantDocumentNode, RadiantNode, RadiantRectangleNode};
use winit::window::Window;
use winit::{event::*, event_loop::ControlFlow};

pub struct RadiantApp {
    pub document: RadiantDocumentNode,
    pub render_state: Option<RenderState>,
    mouse_position: [f32; 2],
}

impl Default for RadiantApp {
    fn default() -> Self {
        Self {
            document: RadiantDocumentNode::new(),
            render_state: None,
            mouse_position: [0.0, 0.0],
        }
    }
}

impl RadiantApp {
    pub async fn init(&mut self, window: Window) {
        let render_state = RenderState::new(window).await;

        self.render_state = Some(render_state);
    }

    pub fn handle_event(&mut self, event: Event<()>, control_flow: &mut ControlFlow) {
        log::debug!("Event: {:?}", event);
        if let Some(ref mut render_state) = &mut self.render_state {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == render_state.window().id() => {
                    if !render_state.input(event) {
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
                                render_state.resize(*physical_size);
                            }
                            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                                render_state.resize(**new_inner_size);
                            }
                            WindowEvent::MouseInput { state, button, .. } => {
                                let is_pressed = *state == ElementState::Pressed;
                                if button == &MouseButton::Left && is_pressed {
                                    log::info!("Left Mouse Button: {:?}", is_pressed);
                                    if let Some(render_state) = &self.render_state {
                                        let mut node = RadiantRectangleNode::new(
                                            &render_state.device,
                                            &render_state.config,
                                            // self.mouse_position,
                                            [
                                                (self.mouse_position[0]
                                                    / render_state.size.width as f32
                                                    - 0.5)
                                                    * 2.0,
                                                (0.5 - self.mouse_position[1]
                                                    / render_state.size.height as f32)
                                                    * 2.0,
                                            ],
                                        );
                                        if let Some(artboard) = self.document.get_active_artboard()
                                        {
                                            node.set_id(artboard.counter);
                                        }
                                        self.document.add(Box::new(node));
                                        render_state.window().request_redraw();
                                    }
                                }

                                // if button == &MouseButton::Right && is_pressed {
                                //     log::info!("Right Mouse Button: {:?}", is_pressed);
                                //     if let Some(render_state) = &mut self.render_state {
                                //         log::info!("Will select");
                                //         pollster::block_on(render_state.select(&self.document, self.mouse_position));
                                //     }
                                // }
                                // self.is_drag_rotate = is_pressed;
                            }
                            WindowEvent::CursorMoved { position, .. } => {
                                log::debug!("Cursor Moved: {:?}", position);
                                // self.mouse_position = [
                                //     (position.x as f32 / render_state.size.width as f32 - 0.5)
                                //         * 2.0,
                                //     (0.5 - position.y as f32 / render_state.size.height as f32)
                                //         * 2.0,
                                // ];
                                self.mouse_position = [position.x as f32, position.y as f32];
                                if let Some(render_state) = &self.render_state {
                                    log::info!("Will select");
                                    let id = pollster::block_on(
                                        render_state.select(&self.document, self.mouse_position),
                                    );
                                    if id > 0 {
                                        self.document.select(id - 1);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                Event::RedrawRequested(window_id) if window_id == render_state.window().id() => {
                    render_state.update();
                    match render_state.render(&mut self.document) {
                        Ok(_) => {}
                        // Reconfigure the surface if lost
                        Err(wgpu::SurfaceError::Lost) => render_state.resize(render_state.size),
                        // The system is out of memory, we should probably quit
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        // All other errors (Outdated, Timeout) should be resolved by the next frame
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                Event::MainEventsCleared => {
                    // RedrawRequested will only trigger once, unless we manually
                    // request it.
                    render_state.window().request_redraw();
                }
                // Event::DeviceEvent { device_id, event } => {
                //     match event {
                //         DeviceEvent::MouseWheel { delta, .. } => {
                //             let scroll_amount = -match delta {
                //                 // A mouse line is about 1 px.
                //                 MouseScrollDelta::LineDelta(_, scroll) => scroll * 1.0,
                //                 MouseScrollDelta::PixelDelta(PhysicalPosition { y: scroll, .. }) => {
                //                     *scroll as f32
                //                 }
                //             };
                //             camera.add_distance(scroll_amount * self.zoom_speed);
                //             window.request_redraw();
                //         }
                //         DeviceEvent::MouseMotion { delta } => {
                //             if self.is_drag_rotate {
                //                 camera.add_yaw(-delta.0 as f32 * self.rotate_speed);
                //                 camera.add_pitch(delta.1 as f32 * self.rotate_speed);
                //                 window.request_redraw();
                //             }
                //         }
                //         _ => (),
                //     }
                // }
                _ => {}
            }
        }
    }
}
