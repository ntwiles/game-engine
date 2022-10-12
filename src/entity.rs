use cgmath;

use crate::{components, physics::collider, state::State};

pub struct Entity {
    position: cgmath::Vector2<f32>,
    rotation: cgmath::Quaternion<f32>,
    pub sprite_mat: usize,
    id: usize,
    pub collider: Option<collider::Collider>,
    pub components: Vec<Option<Box<dyn components::Component>>>,
}

impl Entity {
    pub fn create(
        id: usize,
        position: cgmath::Vector2<f32>,
        rotation: cgmath::Quaternion<f32>,
        sprite_mat: usize,
        collider: Option<collider::Collider>,
    ) -> Self {
        Self {
            id,
            position,
            rotation,
            sprite_mat,
            collider,
            components: Vec::new(),
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

    pub fn move_by(&mut self, offset: cgmath::Vector2<f32>) {
        self.position += offset;
    }

    pub fn add_component(&mut self, component: Box<dyn components::Component>) {
        self.components.push(Some(component));
    }

    pub fn update(&mut self, state: &mut State) {
        for i in 0..self.components.len() {
            if let Some(component) = self.components[i].take() {
                component.update(self, state);
                self.components[i] = Some(component);
            }
        }
    }
}
