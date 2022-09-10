mod camera;
mod graphics;
mod resources;
mod state;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder},
};

use graphics::{material, sprite};

pub async fn run() {
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut state = state::State::new(&window).await;

    let grass_texture = resources::load_texture("grass.png", &state.device, &state.queue).await.unwrap();

    let material = material::Material::new(
        String::from("grass"), 
        &state.device, 
        &state.texture_bind_group_layout, 
        grass_texture
    );
    
    state.sprite = Some(sprite::Sprite::new(String::from("grass"), material, &state.device));

    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            state.update();

            match state.render() {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::MainEventsCleared => window.request_redraw(),
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => if !state.input(event) { match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => *control_flow = ControlFlow::Exit,
            WindowEvent::Resized(physical_size) => state.resize(*physical_size),
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => state.resize(**new_inner_size),
            _ => {}
        }},
        _ => {}
    });
}