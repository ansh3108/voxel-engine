use wgpu::TextureSampleType::Depth;

use crate::chunk::{Chunk, DEPTH, HEIGHT, WIDTH};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex {
    pub const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Face {
    Top, Bottom, Left, Right, Front, Back,
}

impl Face {
    pub const ALL: [Face; 6] = [
        Face::Top, Face::Bottom, Face::Left, 
        Face::Right, Face::Front, Face::Back
    ];
}

fn get_face_vertices(face: Face, x: usize, y: usize, z: usize, w: usize) -> [Vertex; 6] {
    let fx = x as f32;
    let fy = y as f32;
    let fz = z as f32;
    let fw = w as f32;

let color = match face {
        Face::Top => [0.08, 0.94, 0.58],    
        Face::Bottom => [0.03, 0.03, 0.04], 
        Face::Left => [0.12, 0.12, 0.14],   
        Face::Right => [0.08, 0.08, 0.10],  
        Face::Front => [0.10, 0.10, 0.12],  
        Face::Back => [0.06, 0.06, 0.08],   
    };

    match face {
        Face::Front => [
            Vertex { position: [fx, fy, fz + 1.0], color },
            Vertex { position: [fx + fw, fy, fz + 1.0], color }, 
            Vertex { position: [fx + fw, fy + 1.0, fz + 1.0], color }, 
            Vertex { position: [fx, fy, fz + 1.0], color },
            Vertex { position: [fx + fw, fy + 1.0, fz + 1.0], color }, 
            Vertex { position: [fx, fy + 1.0, fz + 1.0], color },
        ],
        Face::Back => [
            Vertex { position: [fx + fw, fy, fz], color }, 
            Vertex { position: [fx, fy, fz], color },
            Vertex { position: [fx, fy + 1.0, fz], color },
            Vertex { position: [fx + fw, fy, fz], color }, 
            Vertex { position: [fx, fy + 1.0, fz], color },
            Vertex { position: [fx + fw, fy + 1.0, fz], color }, 
        ],
        Face::Left => [
            Vertex { position: [fx, fy, fz], color },
            Vertex { position: [fx, fy, fz + 1.0], color },
            Vertex { position: [fx, fy + 1.0, fz + 1.0], color },
            Vertex { position: [fx, fy, fz], color },
            Vertex { position: [fx, fy + 1.0, fz + 1.0], color },
            Vertex { position: [fx, fy + 1.0, fz], color },
        ],
        Face::Right => [
            Vertex { position: [fx + fw, fy, fz + 1.0], color },
            Vertex { position: [fx + fw, fy, fz], color },
            Vertex { position: [fx + fw, fy + 1.0, fz], color },
            Vertex { position: [fx + fw, fy, fz + 1.0], color },
            Vertex { position: [fx + fw, fy + 1.0, fz], color },
            Vertex { position: [fx + fw, fy + 1.0, fz + 1.0], color },
        ],
        Face::Top => [
            Vertex { position: [fx, fy + 1.0, fz + 1.0], color },
            Vertex { position: [fx + fw, fy + 1.0, fz + 1.0], color }, 
            Vertex { position: [fx + fw, fy + 1.0, fz], color }, 
            Vertex { position: [fx, fy + 1.0, fz + 1.0], color },
            Vertex { position: [fx + fw, fy + 1.0, fz], color }, 
            Vertex { position: [fx, fy + 1.0, fz], color },
        ],
        Face::Bottom => [
            Vertex { position: [fx, fy, fz], color },
            Vertex { position: [fx + fw, fy, fz], color }, 
            Vertex { position: [fx + fw, fy, fz + 1.0], color },
            Vertex { position: [fx, fy, fz], color },
            Vertex { position: [fx + fw, fy, fz + 1.0], color }, 
            Vertex { position: [fx, fy, fz + 1.0], color },
        ],
    }
}

pub fn generate_mesh(chunk: &Chunk) -> Vec<Vertex> {
    let mut vertices = Vec::new();

    for face in Face::ALL{
        for y in 0..HEIGHT{
            for z in 0..DEPTH {
                let mut run_start_x = 0;
                let mut run_length = 0;

                for x in 0..WIDTH {
                    let is_solid = chunk.get_block(x, y, z) != 0;
                    let is_visible = is_solid && chunk.is_face_visible(x, y, z, face);

                    if is_visible {
                        if run_length == 0 {
                            run_start_x = x;
                        }
                        run_length += 1;
                    }

                    if (!is_visible || x == WIDTH -1) && run_length > 0{
                        vertices.extend_from_slice(&get_face_vertices(face, run_start_x, y, z, run_length));
                        run_length = 0;
                    }
                }
            }
        }
    }
    vertices
}