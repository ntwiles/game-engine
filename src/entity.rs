use cgmath;

use crate::graphics::{sprite, vertex};

pub struct Entity {
    position: cgmath::Vector2<f32>,
    rotation: cgmath::Quaternion<f32>,
    pub sprite: sprite::Sprite,
}

impl Entity {
    pub fn create(
        position: cgmath::Vector2<f32>,
        rotation: cgmath::Quaternion<f32>,
        sprite: sprite::Sprite,
        queue: &wgpu::Queue,
    ) -> Self {
        let verts = vertex::RenderVertex::new(position, rotation, &sprite.mesh.verts);

        queue.write_buffer(
            &sprite.mesh.vertex_buffer,
            0,
            bytemuck::cast_slice(verts.as_slice()),
        );

        Self {
            position,
            rotation,
            sprite,
        }
    }

    pub fn get_position(&self) -> cgmath::Vector2<f32> {
        self.position
    }

    pub fn move_by(&mut self, offset: cgmath::Vector2<f32>, queue: &wgpu::Queue) {
        self.position += offset;

        let verts =
            vertex::RenderVertex::new(self.position, self.rotation, &self.sprite.mesh.verts);

        queue.write_buffer(
            &self.sprite.mesh.vertex_buffer,
            0,
            bytemuck::cast_slice(verts.as_slice()),
        );
    }
}
