use std::process::exit;

mod chunk;
mod mesh;

fn main() {
    let event_loop = winit::event_loop::EventLoop::new().unwrap(); 
    
    let window = winit::window::WindowBuilder::new()
    .with_title("Voxel Engine")
    .build(&event_loop)
    .unwrap();

    event_loop.run(move |event, elwt| {
        match event {
            winit::event::Event::WindowEvent { 
                event: winit::event::WindowEvent::CloseRequested,
                ..
            } => {
                println!("Closing window!");
                elwt.exit(); 
            }
            _ => ()
        }
    }).unwrap();
}