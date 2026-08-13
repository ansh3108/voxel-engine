use std::process::exit;

use winit::window;

mod chunk;
mod mesh;

struct State<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>
}

    impl <'a> State<'a> {
        async fn new(window: &'a winit::window::Window) -> Self {
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
    }


fn main() {
    let event_loop = winit::event_loop::EventLoop::new().unwrap(); 
    
    let window = winit::window::WindowBuilder::new()
    .with_title("Voxel Engine")
    .build(&event_loop)
    .unwrap();

    event_loop.run(move |event, elwt| {
        match event {
            winit::event::Event::WindowEvent { 
                event: winit::event::WindowEvent::CloseRequested,
                ..
            } => {
                println!("Closing window!");
                elwt.exit(); 
            }
            _ => ()
        }
    }).unwrap();
}

