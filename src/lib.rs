mod camera;
mod entity;
mod graphics;
mod resources;
mod state;

use cgmath::prelude::*;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use graphics::{material, sprite, vertex};

pub async fn run() {
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut state = state::State::new(&window).await;

    let dude_texture =
        resources::load_texture("dude.png", &state.graphics.device, &state.graphics.queue)
            .await
            .unwrap();

    let dude_material = material::Material::new(
        String::from("grass"),
        &state.graphics.device,
        &state.graphics.texture_bind_group_layout,
        dude_texture,
    );

    let material_id = state.add_material(dude_material);

    let player = entity::Entity::create(
        state.num_entities(),
        cgmath::Vector2::zero(),
        cgmath::Quaternion::zero(),
        material_id,
    );

    let verts = vertex::RenderVertex::new(
        player.get_position(),
        player.get_rotation(),
        &sprite::Sprite::get_vertices(),
    );

    state.graphics.write_entity(player.id, verts);

    state.player = Some(player);

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
        } if window_id == window.id() => {
            if !state.input(event) {
                match event {
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
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size)
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    });
}
