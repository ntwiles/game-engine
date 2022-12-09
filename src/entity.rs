use cgmath;

use crate::{components, graphics::sorting_layer, physics::collider, state::State};

pub struct Entity {
    id: usize,
    pub collider: Option<collider::Collider>,
    pub components: Vec<Option<Box<dyn components::Component>>>,
    position: cgmath::Vector2<f32>,
    rotation: cgmath::Quaternion<f32>,
    sorting_layer: sorting_layer::SortingLayer,
    pub sprite_mat: usize,
}

impl Entity {
    pub fn create(
        id: usize,
        position: cgmath::Vector2<f32>,
        rotation: cgmath::Quaternion<f32>,
        sprite_mat: usize,
        sorting_layer: sorting_layer::SortingLayer,
        collider: Option<collider::Collider>,
    ) -> Self {
        Self {
            collider,
            components: Vec::new(),
            id,
            position,
            rotation,
            sorting_layer,
            sprite_mat,
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_position(&self) -> cgmath::Vector2<f32> {
        self.position
    }

    pub fn get_rotation(&self) -> cgmath::Quaternion<f32> {
        self.rotation
    }

    pub fn get_sorting_layer(&self) -> sorting_layer::SortingLayer {
        self.sorting_layer
    }

    pub fn move_by(&mut self, offset: cgmath::Vector2<f32>) {
        self.position += offset;
    }

    pub fn add_component(&mut self, component: Box<dyn components::Component>) {
        self.components.push(Some(component));
    }

    pub fn update(&mut self, state: &mut State, delta_time: f64) {
        for i in 0..self.components.len() {
            if let Some(component) = self.components[i].take() {
                component.update(self, state, delta_time);
                self.components[i] = Some(component);
            }
        }
    }
}
