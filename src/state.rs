use std::{collections::LinkedList, time::Instant};

use cgmath::{prelude::*, Quaternion};
use winit::{event::*, window::Window};

use crate::{
    camera, entity,
    graphics::{material, sprite::Sprite, Graphics},
    resources,
};

pub struct State {
    camera: camera::Camera,
    is_down_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
    is_up_pressed: bool,
    pub player: Option<entity::Entity>,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub graphics: Graphics,

    materials: Vec<material::Material>,

    entities: Vec<entity::Entity>,

    instant: Instant,
    last_n_ticks: LinkedList<u16>,
    tick_queue_len: usize,
}

impl State {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        let camera = camera::Camera {
            aspect: 1.8,
            position: (0.0, 0.0, 5.0).into(),
            scale: 6.0,
            znear: 0.1,
            zfar: 100.0,
        };

        let graphics = Graphics::new(window, &camera).await;

        let grass_texture = resources::load_texture("grass.png", &graphics.device, &graphics.queue)
            .await
            .unwrap();

        let grass_material = material::Material::new(
            String::from("grass"),
            &graphics.device,
            &graphics.texture_bind_group_layout,
            grass_texture,
        );

        let materials = vec![grass_material];

        let grass_sprite = Sprite::new(0);

        let mut entities = Vec::new();

        const NUM_INSTANCES_PER_ROW: u32 = 100;

        for y in 0..NUM_INSTANCES_PER_ROW {
            for x in 0..NUM_INSTANCES_PER_ROW {
                let entity = entity::Entity::create(
                    entities.len(),
                    cgmath::Vector2 {
                        x: x as f32,
                        y: y as f32,
                    },
                    Quaternion::zero(),
                    grass_sprite.duplicate(),
                    &graphics.queue,
                    &graphics.index_buffer,
                    &graphics.vertex_buffer,
                );

                entities.push(entity);
            }
        }

        Self {
            camera,
            graphics,
            entities,
            is_down_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
            is_up_pressed: false,
            size,
            player: None,
            materials,

            instant: Instant::now(),
            last_n_ticks: LinkedList::new(),
            tick_queue_len: 15,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.camera.resize(new_size.width, new_size.height);
            self.size = new_size;
            self.graphics.resize(new_size);
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(keycode),
                        ..
                    },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;

                match keycode {
                    VirtualKeyCode::W | VirtualKeyCode::Up => {
                        self.is_up_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::A | VirtualKeyCode::Left => {
                        self.is_left_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::S | VirtualKeyCode::Down => {
                        self.is_down_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::D | VirtualKeyCode::Right => {
                        self.is_right_pressed = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    pub fn update(&mut self) {
        let mut movement = cgmath::Vector2::<f32>::zero();

        if self.is_left_pressed {
            movement.x = -0.07;
        } else if self.is_right_pressed {
            movement.x = 0.07;
        }

        if self.is_up_pressed {
            movement.y = 0.07;
        } else if self.is_down_pressed {
            movement.y = -0.07;
        }

        if let Some(player) = &mut self.player {
            player.move_by(movement, &self.graphics.queue, &self.graphics.vertex_buffer);
            self.camera.set_position(player.get_position());
        }

        self.graphics.update_camera(&self.camera);
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.last_n_ticks
            .push_front((1_000_000 / self.instant.elapsed().as_micros()) as u16);

        if self.last_n_ticks.len() > self.tick_queue_len {
            self.last_n_ticks.pop_back();
        }

        let fps = self.last_n_ticks.iter().sum::<u16>() / self.last_n_ticks.len() as u16;
        println!("FPS: {}", fps);

        self.instant = Instant::now();

        self.graphics
            .render(&self.entities, &self.player, &self.materials)
    }

    pub fn add_material(&mut self, material: material::Material) -> usize {
        self.materials.push(material);
        self.materials.len() - 1
    }

    pub fn num_entities(&self) -> usize {
        self.entities.len()
    }
}
