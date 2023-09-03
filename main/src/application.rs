use super::renderer::RenderState;
use log::info;
use radiant_core::{RadiantDocumentNode, RadiantRectangleNode};
use winit::window::Window;
use winit::{event::*, event_loop::ControlFlow};

pub struct RadiantApp {
    pub document: RadiantDocumentNode,
    pub render_state: Option<RenderState>,
}

impl Default for RadiantApp {
    fn default() -> Self {
        Self {
            document: RadiantDocumentNode::new(),
            render_state: None,
        }
    }
}

impl RadiantApp {
    pub async fn init(&mut self, window: Window) {
        let render_state = RenderState::new(window).await;

        let node = RadiantRectangleNode::new(&render_state.device, &render_state.config);
        self.document.add(Box::new(node));

        self.render_state = Some(render_state);
    }

    pub fn handle_event(&mut self, event: Event<()>, control_flow: &mut ControlFlow) {
        if let Some(ref mut state) = &mut self.render_state {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == state.window().id() => {
                    if !state.input(event) {
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
                                state.resize(*physical_size);
                            }
                            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                                state.resize(**new_inner_size);
                            }
                            _ => {}
                        }
                    }
                }
                Event::RedrawRequested(window_id) if window_id == state.window().id() => {
                    state.update();
                    match state.render(&self.document) {
                        Ok(_) => {}
                        // Reconfigure the surface if lost
                        Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                        // The system is out of memory, we should probably quit
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        // All other errors (Outdated, Timeout) should be resolved by the next frame
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                Event::MainEventsCleared => {
                    // RedrawRequested will only trigger once, unless we manually
                    // request it.
                    state.window().request_redraw();
                }
                Event::DeviceEvent { device_id, event } => {
                    match event {
                        DeviceEvent::Button {
                            #[cfg(target_os = "macos")]
                                button: 0, // The Left Mouse Button on macos.
                            // This seems like it is a winit bug?
                            #[cfg(not(target_os = "macos"))]
                                button: 1, // The Left Mouse Button on all other platforms.
            
                            state,
                        } => {
                            let is_pressed = state == ElementState::Pressed;
                            // self.is_drag_rotate = is_pressed;
                            println!("Left Mouse Button: {:?}", is_pressed);
                        }
                        DeviceEvent::MouseWheel { delta, .. } => {
                            // let scroll_amount = -match delta {
                            //     // A mouse line is about 1 px.
                            //     MouseScrollDelta::LineDelta(_, scroll) => scroll * 1.0,
                            //     MouseScrollDelta::PixelDelta(PhysicalPosition { y: scroll, .. }) => {
                            //         *scroll as f32
                            //     }
                            // };
                            // camera.add_distance(scroll_amount * self.zoom_speed);
                            // window.request_redraw();
                        }
                        DeviceEvent::MouseMotion { delta } => {
                            // if self.is_drag_rotate {
                            //     camera.add_yaw(-delta.0 as f32 * self.rotate_speed);
                            //     camera.add_pitch(delta.1 as f32 * self.rotate_speed);
                            //     window.request_redraw();
                            // }
                        }
                        _ => (),
                    }
                }
                _ => {}
            }
        }
    }
}
