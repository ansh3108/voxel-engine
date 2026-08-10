pub const WIDTH: usize= 16;
pub const HEIGHT: usize = 16;
pub const DEPTH: usize = 16;

pub const VOLUME: usize= WIDTH * HEIGHT * DEPTH;

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_get_set() {
        let mut chunk = Chunk::new();

        chunk.set_block(15, 15, 15, 1);

        assert_eq!(chunk.get_block(15, 15, 15), 1);

        assert_eq!(chunk.get_block(100, 100, 100), 0);
    }
} 