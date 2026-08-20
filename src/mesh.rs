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

fn get_face_vertices(face: Face, x: usize, y: usize, z: usize) -> [Vertex; 6] {
    let fx = x as f32;
    let fy = y as f32;
    let fz = z as f32;

    let color = match face {
        Face::Top => [0.0, 0.8, 0.0],    
        Face::Bottom => [0.0, 0.3, 0.0], 
        Face::Left => [0.6, 0.4, 0.2],   
        Face::Right => [0.5, 0.3, 0.1],  
        Face::Front => [0.7, 0.5, 0.3],  
        Face::Back => [0.4, 0.2, 0.1],   
    };

    match face {
        Face::Front => [
            Vertex { position: [fx, fy, fz + 1.0], color },
            Vertex { position: [fx + 1.0, fy, fz + 1.0], color },
            Vertex { position: [fx + 1.0, fy + 1.0, fz + 1.0], color },
            Vertex { position: [fx, fy, fz + 1.0], color },
            Vertex { position: [fx + 1.0, fy + 1.0, fz + 1.0], color },
            Vertex { position: [fx, fy + 1.0, fz + 1.0], color },
        ],
        Face::Back => [
            Vertex { position: [fx + 1.0, fy, fz], color },
            Vertex { position: [fx, fy, fz], color },
            Vertex { position: [fx, fy + 1.0, fz], color },
            Vertex { position: [fx + 1.0, fy, fz], color },
            Vertex { position: [fx, fy + 1.0, fz], color },
            Vertex { position: [fx + 1.0, fy + 1.0, fz], color },
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
            Vertex { position: [fx + 1.0, fy, fz + 1.0], color },
            Vertex { position: [fx + 1.0, fy, fz], color },
            Vertex { position: [fx + 1.0, fy + 1.0, fz], color },
            Vertex { position: [fx + 1.0, fy, fz + 1.0], color },
            Vertex { position: [fx + 1.0, fy + 1.0, fz], color },
            Vertex { position: [fx + 1.0, fy + 1.0, fz + 1.0], color },
        ],
        Face::Top => [
            Vertex { position: [fx, fy + 1.0, fz + 1.0], color },
            Vertex { position: [fx + 1.0, fy + 1.0, fz + 1.0], color },
            Vertex { position: [fx + 1.0, fy + 1.0, fz], color },
            Vertex { position: [fx, fy + 1.0, fz + 1.0], color },
            Vertex { position: [fx + 1.0, fy + 1.0, fz], color },
            Vertex { position: [fx, fy + 1.0, fz], color },
        ],
        Face::Bottom => [
            Vertex { position: [fx, fy, fz], color },
            Vertex { position: [fx + 1.0, fy, fz], color },
            Vertex { position: [fx + 1.0, fy, fz + 1.0], color },
            Vertex { position: [fx, fy, fz], color },
            Vertex { position: [fx + 1.0, fy, fz + 1.0], color },
            Vertex { position: [fx, fy, fz + 1.0], color },
        ],
    }
}

pub fn generate_mesh(chunk: &Chunk) -> Vec<Vertex> {
    let mut vertices = Vec::new();
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            for z in 0..DEPTH {
                if chunk.get_block(x, y, z) == 0 {
                    continue;
                } 
                for face in Face::ALL {
                    if chunk.is_face_visible(x, y, z, face) {
                        vertices.extend_from_slice(&get_face_vertices(face, x, y, z));
                    }
                }
            }
        }
    }
    vertices
}
