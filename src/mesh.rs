use crate::chunk::{DEPTH, HEIGHT, WIDTH};

#[derive(Debug)]
pub enum square {
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back
}

pub fn generate_mesh(chunk: &crate::chunk::Chunk) {
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            for z in 0..DEPTH {
                if chunk.get_block(x, y, z) == 0 {
                    continue;
                } 
                if chunk.is_face_visible(x, y, z, square::Top) {
                    println!("Drawing {:?} face at {}, {}, {}", square::Top, x, y, z);
                }
                if chunk.is_face_visible(x, y, z, square::Bottom) {
                    println!("Drawing {:?} face at {}, {}, {}", square::Bottom, x, y, z);
                }
                if chunk.is_face_visible(x, y, z, square::Left) {
                    println!("Drawing {:?} face at {}, {}, {}", square::Left, x, y, z);
                }
                if chunk.is_face_visible(x, y, z, square::Right) {
                    println!("Drawing {:?} face at {}, {}, {}", square::Right, x, y, z);
                }
                if chunk.is_face_visible(x, y, z, square::Front) {
                    println!("Drawing {:?} face at {}, {}, {}", square::Front, x, y, z);
                }
                if chunk.is_face_visible(x, y, z, square::Back) {
                    println!("Drawing {:?} face at {}, {}, {}", square::Back, x, y, z);
                }
            }
        }
    }
}