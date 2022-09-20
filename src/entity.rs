use cgmath;

pub struct Entity {
    position: cgmath::Vector2<f32>,
    rotation: cgmath::Quaternion<f32>,
    pub sprite_mat: usize,
    id: usize,
}

impl Entity {
    pub fn create(
        id: usize,
        position: cgmath::Vector2<f32>,
        rotation: cgmath::Quaternion<f32>,
        sprite_mat: usize,
    ) -> Self {
        Self {
            id,
            position,
            rotation,
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

    pub fn move_by(&mut self, offset: cgmath::Vector2<f32>) {
        self.position += offset;
    }
}
