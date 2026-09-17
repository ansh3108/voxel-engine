use crate::mesh::Face;
use rayon::prelude::*;

pub const WIDTH: usize = 16;
pub const HEIGHT: usize = 16;
pub const DEPTH: usize = 16;
pub const VOLUME: usize = WIDTH * HEIGHT * DEPTH;

pub struct Chunk {
    pub blocks: [u8; VOLUME],
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            blocks: [0; VOLUME],
        }
    }

    fn get_index(x: usize, y: usize, z: usize) -> usize {
        x + (y * WIDTH) + (z * WIDTH * HEIGHT)
    }

    fn in_bounds(x: usize, y: usize, z: usize) -> bool {
        x < WIDTH && y < HEIGHT && z < DEPTH 
    }

    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block_id: u8) {
        if Self::in_bounds(x, y, z) {
            let index = Self::get_index(x, y, z);
            self.blocks[index] = block_id;
        }
    }

    pub fn get_block(&self, x: usize, y: usize, z: usize) -> u8 {
        if Self::in_bounds(x, y, z) {
            let index = Self::get_index(x, y, z);
            self.blocks[index]
        } else {
            0
        }
    }

    pub fn is_face_visible(&self, x: usize, y: usize, z: usize, face: Face) -> bool {
        match face {
            Face::Left => {
                if x == 0 { true } else { self.get_block(x-1, y, z) == 0 }
            }
            Face::Right => self.get_block(x+1, y, z) == 0,
            Face::Bottom => {
                if y == 0 { true } else { self.get_block(x, y-1, z) == 0 }
            }
            Face::Top => self.get_block(x, y+1, z) == 0,
            Face::Back => {
                if z == 0 { true } else { self.get_block(x, y, z-1) == 0 }
            }
            Face::Front => self.get_block(x, y, z+1) == 0,
        }   

    }
        pub fn generate(&mut self){
            self.blocks.par_iter_mut().enumerate().for_each(|(index, block) | {
                let x = index % WIDTH;
                let y = (index/WIDTH) % HEIGHT;
                let z = index / (WIDTH*HEIGHT);

                let fx = x as f32;
                let fz = z as f32;

                let wave_height = ((fx / 3.0).sin() * 2.5 + (fz/3.0).cos() * 2.5) as i32 + 5;

                if (y as i32) < wave_height {
                    *block = 1;
                } else if x == 8 && z == 8 && y < 14 {
                    *block = 1;
                } else {
                    *block = 0;
                }
            });
        }
}
