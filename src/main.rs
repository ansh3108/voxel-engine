mod chunk;
mod mesh;

use winit::keyboard::{KeyCode, PhysicalKey};
use wgpu::{naga::AddressSpace::WorkGroup, util::DeviceExt};

pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

struct Camera {
    eye: cgmath::Point3<f32>,
    target: cgmath::Point3<f32>,
    up: cgmath::Vector3<f32>,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Camera {
    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
        OPENGL_TO_WGPU_MATRIX * proj * view
    }
}

struct CameraController {
    speed: f32,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
}

impl CameraController {
    fn new(speed: f32) -> Self {
        Self {
            speed,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
        }
    }

    fn process_events(&mut self, event: &winit::event::WindowEvent) -> bool {
        match event {
            winit::event::WindowEvent::KeyboardInput {
                event: winit::event::KeyEvent {
                    physical_key: PhysicalKey::Code(keycode),
                    state,
                    ..
                },
                ..
            } => {
                let is_pressed = *state == winit::event::ElementState::Pressed;
                match keycode {
                    KeyCode::KeyW | KeyCode::ArrowUp => { self.is_forward_pressed = is_pressed; true }
                    KeyCode::KeyA | KeyCode::ArrowLeft => { self.is_left_pressed = is_pressed; true }
                    KeyCode::KeyS | KeyCode::ArrowDown => { self.is_backward_pressed = is_pressed; true }
                    KeyCode::KeyD | KeyCode::ArrowRight => { self.is_right_pressed = is_pressed; true }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    fn update_camera(&self, camera: &mut Camera) {
        use cgmath::InnerSpace;
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();

        if self.is_forward_pressed {
            camera.eye += forward_norm * self.speed;
            camera.target += forward_norm * self.speed;
        }
        if self.is_backward_pressed {
            camera.eye -= forward_norm * self.speed;
            camera.target -= forward_norm * self.speed;
        }

        let right = forward_norm.cross(camera.up);

        if self.is_right_pressed {
            camera.eye += right * self.speed;
            camera.target += right * self.speed;
        }
        if self.is_left_pressed {
            camera.eye -= right * self.speed;
            camera.target -= right * self.speed;
        }
    }
}

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
    Vertex { position: [-0.5, -0.5,  0.5], color: [1.0, 0.0, 0.0] },
    Vertex { position: [ 0.5, -0.5,  0.5], color: [1.0, 0.0, 0.0] },
    Vertex { position: [ 0.5,  0.5,  0.5], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-0.5, -0.5,  0.5], color: [1.0, 0.0, 0.0] },
    Vertex { position: [ 0.5,  0.5,  0.5], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-0.5,  0.5,  0.5], color: [1.0, 0.0, 0.0] },

    Vertex { position: [-0.5, -0.5, -0.5], color: [0.0, 1.0, 0.0] },
    Vertex { position: [-0.5,  0.5, -0.5], color: [0.0, 1.0, 0.0] },
    Vertex { position: [ 0.5,  0.5, -0.5], color: [0.0, 1.0, 0.0] },
    Vertex { position: [-0.5, -0.5, -0.5], color: [0.0, 1.0, 0.0] },
    Vertex { position: [ 0.5,  0.5, -0.5], color: [0.0, 1.0, 0.0] },
    Vertex { position: [ 0.5, -0.5, -0.5], color: [0.0, 1.0, 0.0] },

    Vertex { position: [-0.5,  0.5, -0.5], color: [0.0, 0.0, 1.0] },
    Vertex { position: [-0.5,  0.5,  0.5], color: [0.0, 0.0, 1.0] },
    Vertex { position: [ 0.5,  0.5,  0.5], color: [0.0, 0.0, 1.0] },
    Vertex { position: [-0.5,  0.5, -0.5], color: [0.0, 0.0, 1.0] },
    Vertex { position: [ 0.5,  0.5,  0.5], color: [0.0, 0.0, 1.0] },
    Vertex { position: [ 0.5,  0.5, -0.5], color: [0.0, 0.0, 1.0] },

    Vertex { position: [-0.5, -0.5, -0.5], color: [1.0, 1.0, 0.0] },
    Vertex { position: [ 0.5, -0.5, -0.5], color: [1.0, 1.0, 0.0] },
    Vertex { position: [ 0.5, -0.5,  0.5], color: [1.0, 1.0, 0.0] },
    Vertex { position: [-0.5, -0.5, -0.5], color: [1.0, 1.0, 0.0] },
    Vertex { position: [ 0.5, -0.5,  0.5], color: [1.0, 1.0, 0.0] },
    Vertex { position: [-0.5, -0.5,  0.5], color: [1.0, 1.0, 0.0] },

    Vertex { position: [ 0.5, -0.5, -0.5], color: [1.0, 0.0, 1.0] },
    Vertex { position: [ 0.5,  0.5, -0.5], color: [1.0, 0.0, 1.0] },
    Vertex { position: [ 0.5,  0.5,  0.5], color: [1.0, 0.0, 1.0] },
    Vertex { position: [ 0.5, -0.5, -0.5], color: [1.0, 0.0, 1.0] },
    Vertex { position: [ 0.5,  0.5,  0.5], color: [1.0, 0.0, 1.0] },
    Vertex { position: [ 0.5, -0.5,  0.5], color: [1.0, 0.0, 1.0] },

    Vertex { position: [-0.5, -0.5, -0.5], color: [0.0, 1.0, 1.0] },
    Vertex { position: [-0.5, -0.5,  0.5], color: [0.0, 1.0, 1.0] },
    Vertex { position: [-0.5,  0.5,  0.5], color: [0.0, 1.0, 1.0] },
    Vertex { position: [-0.5, -0.5, -0.5], color: [0.0, 1.0, 1.0] },
    Vertex { position: [-0.5,  0.5,  0.5], color: [0.0, 1.0, 1.0] },
    Vertex { position: [-0.5,  0.5, -0.5], color: [0.0, 1.0, 1.0] },
];

struct State {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    num_vertices: u32,
    camera_bind_group: wgpu::BindGroup,
    depth_texture_view: wgpu::TextureView,
    camera: Camera,
    camera_controller: CameraController,
    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
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

            let mut chunk = crate::chunk::Chunk::new();
            for x in 0..16 {
                for z in 0..16 {
                    chunk.set_block(x, 0, z, 1);
                }
            }
            chunk.set_block(8, 1, 8, 1);
            chunk.set_block(8, 2, 8, 1);

            let vertices = crate::mesh::generate_mesh(&chunk);
            let num_vertices = vertices.len() as u32;

            let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Chunk Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

            let camera = Camera {
            eye: cgmath::Point3::new(8.0_f32, 12.0_f32, 25.0_f32),
            target: cgmath::Point3::new(8.0_f32, 0.0_f32, 8.0_f32),
            up: cgmath::Vector3::unit_y(),
            aspect: size.width as f32 / size.height as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        };

        let camera_controller = CameraController::new(0.2); 

        let mut camera_uniform = CameraUniform {
            view_proj: camera.build_view_projection_matrix().into(),
        };

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, 
        });

            let opengl_to_wgpu = cgmath::Matrix4::new(
                1.0_f32, 0.0_f32, 0.0_f32, 0.0_f32,
                0.0_f32, 1.0_f32, 0.0_f32, 0.0_f32,
                0.0_f32, 0.0_f32, 0.5_f32, 0.0_f32,
                0.0_f32, 0.0_f32, 0.5_f32, 1.0_f32,
            );

        let mut camera_uniform = CameraUniform {
            view_proj: camera.build_view_projection_matrix().into(),
        };

            

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

            let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
                label: Some("Depth Texture"),
                size: wgpu::Extent3d { 
                    width: config.width, 
                    height: config.height, 
                    depth_or_array_layers: 1, 
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Depth32Float,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            });

            let depth_texture_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

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
                    buffers: &[crate::mesh::Vertex::desc()] 
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
                depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less, 
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
                }),

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
                depth_texture_view,
                num_vertices,
                camera,
                camera_controller,
                camera_buffer,
                camera_uniform
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
                
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0), 
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw(0..self.num_vertices, 0..1);
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

                let depth_texture = self.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("Depth Texture"),
                size: wgpu::Extent3d {
                    width: self.config.width,
                    height: self.config.height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Depth32Float,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            });
                self.depth_texture_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());
            }
        }

        pub fn input(&mut self, event: &winit::event::WindowEvent) -> bool {
            self.camera_controller.process_events(event)
        }

        pub fn update(&mut self) {
            self.camera_controller.update_camera(&mut self.camera);
            self.camera_uniform.view_proj = self.camera.build_view_projection_matrix().into();
            self.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[self.camera_uniform]));
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
            if !state.input(&event){
            match event {
                winit::event::WindowEvent::Resized(physical_size) => {
                    state.resize(physical_size);
                }

                winit::event::WindowEvent::CloseRequested => {
                    println!("Closing window!");
                    elwt.exit();
                }
                winit::event::WindowEvent::RedrawRequested => {
                    state.update();
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
    }
        _ => ()
    }
}).unwrap();
}

