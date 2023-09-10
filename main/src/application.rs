use super::renderer::RenderState;
use radiant_core::{RadiantDocumentNode, RadiantNodeType, RadiantRectangleNode};
use winit::window::Window;
use winit::{event::*, event_loop::ControlFlow};

pub struct RadiantApp {
    pub document: RadiantDocumentNode,
    pub render_state: RenderState,
    mouse_position: [f32; 2],
}

impl RadiantApp {
    pub async fn new(window: Window) -> Self {
        let document = RadiantDocumentNode::new();
        let render_state = RenderState::new(window).await;
        let mouse_position = [0.0, 0.0];

        Self {
            document,
            render_state,
            mouse_position,
        }
    }

    pub fn handle_event(&mut self, event: Event<()>, control_flow: &mut ControlFlow) {
        log::debug!("Event: {:?}", event);
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == self.render_state.window().id() => {
                if !self.render_state.input(event) {
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
                            self.render_state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            self.render_state.resize(**new_inner_size);
                        }
                        WindowEvent::MouseInput { state, button, .. } => {
                            let is_pressed = *state == ElementState::Pressed;
                            if button == &MouseButton::Left && is_pressed {
                                let node = RadiantRectangleNode::new(
                                    self.document.counter,
                                    &self.render_state.device,
                                    &self.render_state.config,
                                    [
                                        (self.mouse_position[0]
                                            / self.render_state.size.width as f32
                                            - 0.5)
                                            * 2.0,
                                        (0.5 - self.mouse_position[1]
                                            / self.render_state.size.height as f32)
                                            * 2.0,
                                    ],
                                );
                                self.document.add(RadiantNodeType::Rectangle(node));
                                self.render_state.window().request_redraw();
                            }

                            // if button == &MouseButton::Right && is_pressed {
                            //     log::info!("Right Mouse Button: {:?}", is_pressed);
                            //     if let Some(render_state) = &mut self.render_state {
                            //         log::info!("Will select");
                            //         pollster::block_on(render_state.select(&self.document, self.mouse_position));
                            //     }
                            // }
                        }
                        WindowEvent::CursorMoved { position, .. } => {
                            log::debug!("Cursor Moved: {:?}", position);

                            self.mouse_position = [position.x as f32, position.y as f32];
                            let id = pollster::block_on(
                                self.render_state.select(&self.document, self.mouse_position),
                            );
                            if id > 0 {
                                self.document.select(id - 1);
                            }
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == self.render_state.window().id() => {
                self.render_state.update();
                match self.render_state.render(&mut self.document) {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => self.render_state.resize(self.render_state.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                self.render_state.window().request_redraw();
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
