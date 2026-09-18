# voxel-engine

A high-performance, systems-level rendering engine built from absolute scratch in Rust using `wgpu`.

This project operates as a technical sandbox to demonstrate low-level memory safety, deterministic execution, and aggressive computational optimization. Built without high-level game engines, it interfaces directly with the GPU through strict ABI boundaries. The architecture explicitly mirrors the performance requirements of high-throughput blockchain runtimes (like Solana's Sealevel), focusing on parallel state execution, zero-copy memory mapping, and payload reduction.

## Usage

You need Rust installed (edition 2024). Then:

```
cargo run
```

A window opens with a flat 16x16 chunk of blocks and a small pillar in the center. Move the camera with WASD or arrow keys.

## How it works

The engine is split into a few modules:

- **chunk** -- A 16x16x16 voxel grid stored as a flat `[u8; 4096]` array. Each entry is a block ID (0 = air). Handles bounds checking and face visibility queries for neighboring blocks.
- **mesh** -- Takes a chunk and produces a vertex buffer. Iterates over every non-air block, checks each of the 6 faces, and only emits triangles for faces that border air (basic occlusion culling). Each face gets a distinct color based on its direction.
- **shader.wgsl** -- A straightforward vertex/fragment shader. The vertex shader transforms positions by a camera view-projection matrix; the fragment shader outputs the interpolated per-vertex color.
- **main** -- Sets up wgpu (surface, device, queue, render pipeline with depth testing), creates the chunk, generates the mesh, and runs the winit event loop.

## Dependencies

| Crate | What it does |
|-------|-------------|
| wgpu 0.19 | GPU abstraction (Vulkan/Metal/DX12/WebGPU) |
| winit 0.29 | Window creation and input handling |
| cgmath 0.18 | Linear algebra for camera math |
| bytemuck 1.25 | Safe casting of structs to byte slices for GPU buffers |
| pollster 1.0 | Blocks on async (used to initialize wgpu) |

## Project structure

```
src/
  main.rs        -- Entry point, wgpu setup, event loop, camera
  chunk.rs       -- Chunk data structure and block queries
  mesh.rs        -- Mesh generation from chunk data
  shader.wgsl    -- WGSL vertex and fragment shaders
```

## Current limitations

- Single chunk only, no world made of multiple chunks
- No textures, just flat colors per face direction
- Camera can only strafe, no mouse look or vertical movement
- No block placement or removal at runtime
- Mesh is generated once at startup, not rebuilt on changes
