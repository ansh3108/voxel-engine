mod chunk;
mod mesh;

fn main() {
    let mut my_chunk = chunk::Chunk::new();
    my_chunk.set_block(5, 5, 5, 1);

    mesh::generate_mesh(&my_chunk);
}
