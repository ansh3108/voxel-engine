use std::process::exit;

use winit::{event, window};

mod chunk;
mod mesh;

struct State {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>
}

    impl State {
        async fn new(window: std::sync::Arc<winit::window::Window>) -> Self {
            let size = window.inner_size();
            
            let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());

            let surface = instance.create_surface(window).unwrap();

            let adapter = instance.request_adapter(
                &wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::default(),
                    compatible_surface: Some(&surface),
                    force_fallback_adapter: false,
                },
            ).await.unwrap();

            let (device, queue) = adapter.request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            ).await.unwrap();
            
            let config = surface.get_default_config(&adapter, size.width, size.height).unwrap();

            surface.configure(&device, &config);

            Self {
                surface,
                device,
                queue,
                config,
                size
            }
        }

        fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
            let output = self.surface.get_current_texture()?;
            let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

            let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

            {
                let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.1, g: 0.2, b: 0.5, a: 1.0 }),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });
            }

            self.queue.submit(std::iter::once(encoder.finish()));
            output.present();
            Ok(())
        }

        fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
            if new_size.width > 0 && new_size.height > 0 {
                self.size = new_size;
                self.config.width = new_size.width;
                self.config.height = new_size.height;
                self.surface.configure(&self.device, &self.config);
            }
        }


    }


fn main() {
    let event_loop = winit::event_loop::EventLoop::new().unwrap(); 
    
    let window = std::sync::Arc::new(
        winit::window::WindowBuilder::new()
            .with_title("Voxel Enging")
            .build(&event_loop)
            .unwrap()
    );

    let mut state = pollster::block_on(State::new(window.clone()));

    event_loop.run(move |event, elwt| {
    match event {
        winit::event::Event::WindowEvent { event, ..} => {
            match event {
                winit::event::WindowEvent::Resized(physical_size) => {
                    state.resize(physical_size);
                }

                winit::event::WindowEvent::CloseRequested => {
                    println!("Closing window!");
                    elwt.exit();
                }
                winit::event::WindowEvent::RedrawRequested => {
                    match state.render() {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Outdated) => state.resize(state.size),
                        Err(e) => eprintln!("{:?}", e),
                    }
                    window.request_redraw();
                }
                _ => {}
            }
        }
        _ => ()
    }
}).unwrap();
}

