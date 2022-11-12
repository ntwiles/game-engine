use std::{collections::LinkedList, time::Instant};

use cgmath::{prelude::*, Quaternion};
use winit::window::Window;

use crate::{
    camera, entity,
    graphics::{material, sprite, vertex, Graphics},
    input, resources,
};

pub struct State {
    pub camera: camera::Camera,
    pub input: input::ReadOnlyInput,
    pub player: Option<entity::Entity>,
    pub wall: Option<entity::Entity>,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub graphics: Graphics,

    materials: Vec<material::Material>,
    entities: Vec<Option<entity::Entity>>,

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

        let mut graphics = Graphics::new(window, &camera).await;

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
                    0,
                    None,
                );

                let verts = vertex::RenderVertex::new(
                    entity.get_position(),
                    entity.get_rotation(),
                    &sprite::Sprite::get_vertices(),
                );

                graphics.write_entity(entity.get_id(), verts);

                entities.push(Some(entity));
            }
        }

        let input = input::ReadOnlyInput::new();

        Self {
            camera,
            graphics,
            entities,
            input,
            size,
            player: None,
            wall: None,
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

    pub fn update(&mut self, input: input::ReadOnlyInput) {
        self.input = input;

        for i in 0..self.entities.len() {
            if let Some(mut entity) = self.entities[i].take() {
                entity.update(self);
                self.entities[i] = Some(entity);
            }
        }

        if let Some(mut player) = self.player.take() {
            player.update(self);
            self.player = Some(player);
        }

        self.graphics.write_camera(&self.camera);
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.last_n_ticks
            .push_front((1_000_000 / self.instant.elapsed().as_micros()) as u16);

        if self.last_n_ticks.len() > self.tick_queue_len {
            self.last_n_ticks.pop_back();
        }

        let fps = self.last_n_ticks.iter().sum::<u16>() / self.tick_queue_len as u16;
        println!("FPS: {}", fps);

        self.instant = Instant::now();

        self.graphics
            .render(&self.entities, &self.player, &self.wall, &self.materials)
    }

    pub fn add_material(&mut self, material: material::Material) -> usize {
        self.materials.push(material);
        self.materials.len() - 1
    }

    pub fn num_entities(&self) -> usize {
        self.entities.len()
    }
}
