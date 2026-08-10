pub const WIDTH: usize= 16;
pub const HEIGHT: usize = 16;
pub const DEPTH: usize = 16;

pub const VOLUME: usize= WIDTH * HEIGHT * DEPTH;

struct Chunk {
    blocks: int,
}