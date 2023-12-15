use once_cell::sync::Lazy;
use radiantkit::{
    RadiantMessage, RadiantPathNode, RadiantRectangleNode, RadiantResponse, RadiantRuntime,
    RadiantSceneMessage, RadiantTextNode, Runtime, View, RadiantTextMessage,
};
use uuid::Uuid;
use std::iter;
use winit::event::Event::RedrawRequested;

use egui::{FontDefinitions, Id};
use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
use egui_winit_platform::{Platform, PlatformDescriptor};

static NODE_1: Lazy<Uuid> = Lazy::new(|| Uuid::new_v4());
static NODE_2: Lazy<Uuid> = Lazy::new(|| Uuid::new_v4());
static NODE_3: Lazy<Uuid> = Lazy::new(|| Uuid::new_v4());

struct RadiantKitAppController {
    pending_messages: Vec<RadiantMessage>,
    text: String,
}

impl RadiantKitAppController {
    fn new() -> Self {
        Self {
            pending_messages: Vec::new(),
            text: "Hello".to_string(),
        }
    }
}

impl RadiantKitAppController {
    fn update(&mut self, ctx: &egui::Context) {
        let frame = egui::Frame::none().fill(egui::Color32::TRANSPARENT);
        egui::TopBottomPanel::top(Id::new("top"))
            .frame(frame)
            .show(ctx, |ui| {
                ui.heading("Radiant App");
                if ui.button("Select").clicked() {
                    self.pending_messages
                        .push(RadiantSceneMessage::SelectTool { id: 0 }.into());
                }
                if ui.button("Rect").clicked() {
                    self.pending_messages
                        .push(RadiantSceneMessage::SelectTool { id: 1 }.into());
                }
                if ui.button("Load Image").clicked() {
                    self.pending_messages.push(RadiantMessage::AddImage {
                        name: "".to_string(),
                        path: "https://i.imgur.com/XbLP6ux.png".to_string(),
                    });
                }
                if ui.button("Add Text").clicked() {
                    self.pending_messages.push(RadiantMessage::AddText {
                        text: "Hello".to_string(),
                        position: [200.0, 200.0],
                    });
                }
                if ui.text_edit_singleline(&mut self.text).changed() {
                    self.pending_messages.push(RadiantMessage::TextMessage(
                        RadiantTextMessage::SetText {
                            id: *NODE_3,
                            text: self.text.clone(),
                        },
                    ));
                }
                #[cfg(feature = "video")]
                if ui.button("Load Video").clicked() {
                    self.pending_messages.push(RadiantMessage::AddVideo {
                        name: "".to_string(),
                        path: "".to_string(), // Add video path here
                    });
                }
                #[cfg(feature = "video")]
                if ui.button("Play Video").clicked() {
                    self.pending_messages
                        .push(RadiantMessage::PlayVideo { id: 4 });
                }
                ui.add_space(10.0);
            });
    }
}

async fn run() {
    let env = env_logger::Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    let handler: Box<dyn Fn(RadiantResponse)> = Box::new(move |response: RadiantResponse| {
        log::info!("Response: {:?}", response);
    });

    let mut runtime = RadiantRuntime::new(2, None).await;
    runtime.add(RadiantRectangleNode::new(*NODE_1, [200.0, 200.0], [200.0, 200.0]).into());
    runtime.add(RadiantPathNode::new(*NODE_2, [400.0, 400.0]).into());
    runtime
        .add(RadiantTextNode::new(*NODE_3, String::from("Hello"), [300.0, 300.0], [200.0, 200.0]).into());

    let size = runtime.view.window.inner_size();
    let scale_factor = runtime.view.window.scale_factor();

    let mut platform = Platform::new(PlatformDescriptor {
        physical_width: size.width as u32,
        physical_height: size.height as u32,
        scale_factor,
        font_definitions: FontDefinitions::default(),
        style: Default::default(),
    });

    let mut egui_rpass;
    {
        let scene = runtime.view.scene_mut();
        egui_rpass = RenderPass::new(
            &scene.render_manager.device,
            scene.render_manager.config.format,
            1,
        );
    }
    let mut demo_app = RadiantKitAppController::new();

    if let Some(event_loop) = std::mem::replace(&mut runtime.view.event_loop, None) {
        event_loop.run(move |event, _, control_flow| {
            if demo_app.pending_messages.len() > 0 {
                for message in demo_app.pending_messages.drain(..) {
                    if let Some(response) = runtime.handle_message(message) {
                        handler(response);
                    }
                }
            }

            platform.handle_event(&event);

            if !platform.captures_event(&event) {
                if let Some(message) = runtime.view.handle_event(&event, control_flow) {
                    if let Some(response) = runtime.handle_message(message) {
                        log::info!("Response: {:?}", response);
                    }
                }
            }

            match event {
                RedrawRequested(..) => {
                    platform.begin_frame();

                    demo_app.update(&platform.context());

                    let full_output = platform.end_frame(Some(&mut runtime.view.window));
                    let paint_jobs = platform.context().tessellate(full_output.shapes);

                    let scene = &mut runtime.scene_mut();

                    let output_frame =
                        std::mem::replace(&mut scene.render_manager.current_texture, None);
                    let output_frame = output_frame.unwrap();

                    let output_view = scene.render_manager.current_view.as_ref().unwrap();

                    // Upload all resources for the GPU.
                    let screen_descriptor = ScreenDescriptor {
                        physical_width: scene.render_manager.config.width,
                        physical_height: scene.render_manager.config.height,
                        scale_factor: scale_factor as f32,
                    };
                    let tdelta: egui::TexturesDelta = full_output.textures_delta;
                    egui_rpass
                        .add_textures(
                            &scene.render_manager.device,
                            &scene.render_manager.queue,
                            &tdelta,
                        )
                        .expect("add texture ok");
                    egui_rpass.update_buffers(
                        &scene.render_manager.device,
                        &scene.render_manager.queue,
                        &paint_jobs,
                        &screen_descriptor,
                    );

                    let mut encoder = scene.render_manager.device.create_command_encoder(
                        &wgpu::CommandEncoderDescriptor {
                            label: Some("encoder"),
                        },
                    );
                    // Record all render passes.
                    egui_rpass
                        .execute(
                            &mut encoder,
                            &output_view,
                            &paint_jobs,
                            &screen_descriptor,
                            None,
                            // Some(wgpu::Color::BLACK),
                        )
                        .unwrap();
                    // Submit the commands.
                    scene
                        .render_manager
                        .queue
                        .submit(iter::once(encoder.finish()));

                    // Redraw egui
                    output_frame.present();

                    egui_rpass
                        .remove_textures(tdelta)
                        .expect("remove texture ok");
                }
                _ => (),
            }
        });
    }
}

#[tokio::main]
async fn main() {
    pollster::block_on(run());
}
