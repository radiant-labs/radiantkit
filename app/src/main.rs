use radiant_main::{RadiantApp, RadiantMessage, RadiantResponse, RadiantTool};
use std::iter;
use winit::event::Event::RedrawRequested;
use winit::{event_loop::EventLoop, window::WindowBuilder};

use egui::FontDefinitions;
use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
use egui_winit_platform::{Platform, PlatformDescriptor};

async fn run() {
    let env = env_logger::Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let size = window.inner_size();
    let scale_factor = window.scale_factor();

    let handler: Box<dyn Fn(RadiantResponse)> = Box::new(move |response: RadiantResponse| {
        println!("Response: {:?}", response);
    });

    let mut app = RadiantApp::new(window, handler).await;
    app.handle_message(RadiantMessage::SelectTool(RadiantTool::Rectangle));

    let mut platform = Platform::new(PlatformDescriptor {
        physical_width: size.width as u32,
        physical_height: size.height as u32,
        scale_factor,
        font_definitions: FontDefinitions::default(),
        style: Default::default(),
    });

    let mut egui_rpass = RenderPass::new(&app.scene.device, app.scene.config.format, 1);
    let mut demo_app = egui_demo_lib::DemoWindows::default();

    event_loop.run(move |event, _, control_flow| {
        platform.handle_event(&event);

        if let Some(response) = app.handle_event(&event, control_flow) {
            println!("Response: {:?}", response);
        }

        match event {
            RedrawRequested(..) => {
                let output_frame = std::mem::replace(&mut app.scene.current_texture, None);
                let output_frame = output_frame.unwrap();

                let output_view = app.scene.current_view.as_ref().unwrap();

                platform.begin_frame();

                demo_app.ui(&platform.context());

                let full_output = platform.end_frame(Some(&app.window));
                let paint_jobs = platform.context().tessellate(full_output.shapes);

                // Upload all resources for the GPU.
                let screen_descriptor = ScreenDescriptor {
                    physical_width: app.scene.config.width,
                    physical_height: app.scene.config.height,
                    scale_factor: scale_factor as f32,
                };
                let tdelta: egui::TexturesDelta = full_output.textures_delta;
                egui_rpass
                    .add_textures(&app.scene.device, &app.scene.queue, &tdelta)
                    .expect("add texture ok");
                egui_rpass.update_buffers(
                    &app.scene.device,
                    &app.scene.queue,
                    &paint_jobs,
                    &screen_descriptor,
                );

                let mut encoder =
                    app.scene
                        .device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                            label: Some("encoder"),
                        });
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
                app.scene.queue.submit(iter::once(encoder.finish()));

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

fn main() {
    pollster::block_on(run());
}
