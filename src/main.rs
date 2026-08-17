mod chunk;
mod mesh;

use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] = 
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

const VERTICES: &[Vertex] = &[
    Vertex { position: [0.0, 0.5, 0.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [0.5, -0.5, 0.0], color: [1.0, 0.0, 1.0] },
];

struct State {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
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

            let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

            let vertex_buffer = device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(VERTICES),
                    usage: wgpu::BufferUsages::VERTEX,
                }
            );

            let camera_matrix = cgmath::perspective(cgmath::Deg(45.0_f32), size.width as f32 / size.height as f32, 0.1_f32, 100.0_f32)
            * cgmath::Matrix4::look_at_rh(
                cgmath::Point3::new(0.0_f32, 1.0_f32, 2.0_f32), 
                cgmath::Point3::new(0.0_f32, 0.0_f32, 0.0_f32), 
                cgmath::Vector3::unit_y(),
            );

            let opengl_to_wgpu = cgmath::Matrix4::new(
                1.0_f32, 0.0_f32, 0.0_f32, 0.0_f32,
                0.0_f32, 1.0_f32, 0.0_f32, 0.0_f32,
                0.0_f32, 0.0_f32, 0.5_f32, 0.0_f32,
                0.0_f32, 0.0_f32, 0.5_f32, 1.0_f32,
            );

            let camera_uniform = CameraUniform {
                view_proj: (opengl_to_wgpu * camera_matrix).into(),
            };

            let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

            let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer { 
                        ty: wgpu::BufferBindingType::Uniform, 
                        has_dynamic_offset: false, 
                        min_binding_size: None, 
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });

            let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor{
                layout: &camera_bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                }],
                label: Some("camera_bind_group"),
            });

            let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&camera_bind_group_layout],
                push_constant_ranges: &[],
            });

            let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{
                label: Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState { 
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[Vertex::desc()] 
                },
            
            
            
                fragment: Some(wgpu::FragmentState{
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState { format: config.format, blend: Some(wgpu::BlendState::REPLACE), write_mask: wgpu::ColorWrites::ALL, })],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
                });

            surface.configure(&device, &config);

            Self {
                surface,
                device,
                queue,
                config,
                size,
                render_pipeline,
                vertex_buffer,
                camera_bind_group,
            }
        }

        fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
            let output = self.surface.get_current_texture()?;
            let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

            let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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

                render_pass.set_pipeline(&self.render_pipeline);
                render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
                render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
                render_pass.draw(0..VERTICES.len() as u32, 0..1);
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
            .with_title("Voxel Engine")
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

