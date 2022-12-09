mod camera;
mod components;
mod config;
mod entity;
mod graphics;
mod input;
mod physics;
mod resources;
mod state;
mod ui;

use cgmath::prelude::*;
use dotenv::dotenv;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use components::player_movement::PlayerMovement;
use graphics::{material, sorting_layer};
use input::Input;
use physics::collider;

pub async fn run() {
    dotenv().ok();
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut state = state::State::new(&window).await;

    let ball_texture = resources::load_texture(&state.graphics, "ball.png")
        .await
        .unwrap();

    let ball_material =
        material::Material::new(String::from("ball"), &state.graphics, ball_texture);

    let ball_mat_id = state.add_material(ball_material);

    let mut player = entity::Entity::create(
        state.num_entities(),
        cgmath::Vector2::zero(),
        cgmath::Quaternion::zero(),
        ball_mat_id,
        sorting_layer::SortingLayer::Foreground,
        Some(collider::Collider {
            origin: cgmath::Vector2::zero(),
            width: 1.0,
            height: 1.0,
        }),
    );

    player.add_component(Box::new(PlayerMovement { walk_speed: 35.0 }));
    state.add_entity(player);

    state.add_entity(entity::Entity::create(
        state.num_entities() + 1,
        cgmath::Vector2 { x: 3.0, y: 1.0 },
        cgmath::Quaternion::zero(),
        ball_mat_id,
        sorting_layer::SortingLayer::Foreground,
        Some(collider::Collider {
            origin: cgmath::Vector2::zero(),
            width: 1.0,
            height: 1.0,
        }),
    ));

    let mut input = Input::new();

    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            state.update(input.to_read_only());

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
            if !input.handle_event(event) {
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
