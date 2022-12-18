use std::{collections::LinkedList, time::Instant};

use cgmath::{prelude::*, Quaternion};
use wgpu::Color;
use winit::window::Window;

use crate::{
    camera::Camera,
    config::Config,
    entity,
    graphics::{
        material, sorting_layer,
        sprite::{self, Sprite},
        vertex, Graphics,
    },
    input, resources,
    ui::{canvas, ui_vertex::UiRenderVertex},
};

pub struct State {
    pub camera: Camera,
    config: Config,
    delta_time: f64,
    pub input: input::ReadOnlyInput,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub graphics: Graphics,
    materials: Vec<material::Material>,
    entities: Vec<Option<entity::Entity>>,
    instant: Instant,
    last_n_ticks: LinkedList<f64>,
    tick_queue_len: usize,
    ui_canvas: canvas::Canvas,
}

impl State {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        let camera = Camera {
            aspect: 1.8,
            position: (0.0, 0.0, 5.0).into(),
            scale: 6.0,
            znear: 0.1,
            zfar: 100.0,
        };

        let mut graphics = Graphics::new(&window, &camera).await;

        let grass_texture = resources::load_texture(&graphics, "grass.png")
            .await
            .unwrap();

        let grass_material =
            material::Material::new(String::from("grass"), &graphics, grass_texture);

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
                    sorting_layer::SortingLayer::Background,
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

        State {
            camera,
            config: Config::new(),
            delta_time: 0.0,
            graphics,
            entities,
            input: input::ReadOnlyInput::new(),
            size,
            materials,
            instant: Instant::now(),
            last_n_ticks: LinkedList::new(),
            tick_queue_len: 15,
            ui_canvas: canvas::Canvas::new(),
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
        self.delta_time = self.instant.elapsed().as_micros() as f64 / 1_000_000.00;

        if self.delta_time > 0.0 {
            self.last_n_ticks.push_front(1.0 / self.delta_time as f64);
        }

        if self.last_n_ticks.len() > self.tick_queue_len {
            self.last_n_ticks.pop_back();
        }

        let fps: f64 = self.last_n_ticks.iter().sum::<f64>() / self.tick_queue_len as f64;
        let fps = fps.floor();

        self.ui_canvas.root().set_body(&format!("FPS: {fps}"));

        self.instant = Instant::now();

        for i in 0..self.entities.len() {
            if let Some(mut entity) = self.entities[i].take() {
                entity.update(self, self.delta_time);
                self.entities[i] = Some(entity);
            }
        }

        self.graphics.write_camera(&self.camera);

        let verts = [
            vertex::Vertex {
                position: [-0.9, 0.9, 0.0],
                tex_coords: [0.0, 0.0],
            },
            vertex::Vertex {
                position: [-0.9, -0.9, 0.0],
                tex_coords: [0.0, 1.0],
            },
            vertex::Vertex {
                position: [0.9, -0.9, 0.0],
                tex_coords: [1.0, 1.0],
            },
            vertex::Vertex {
                position: [0.9, 0.9, 0.0],
                tex_coords: [1.0, 0.0],
            },
        ];

        let verts = UiRenderVertex::new(&verts, Color::BLACK);
        self.graphics.write_ui_element(0, verts);
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.graphics.render(
            &self.entities,
            &self.materials,
            &mut self.ui_canvas,
            &self.config,
        )
    }

    pub fn add_entity(&mut self, entity: entity::Entity) -> usize {
        let verts = vertex::RenderVertex::new(
            entity.get_position(),
            entity.get_rotation(),
            &sprite::Sprite::get_vertices(),
        );

        self.graphics.write_entity(entity.get_id(), verts);
        self.entities.push(Some(entity));
        self.entities.len() - 1
    }

    pub fn add_material(&mut self, material: material::Material) -> usize {
        self.materials.push(material);
        self.materials.len() - 1
    }

    pub fn num_entities(&self) -> usize {
        self.entities.len()
    }
}
