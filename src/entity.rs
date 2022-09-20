use std::mem;

use cgmath;

use crate::graphics::{sprite, vertex};

pub struct Entity {
    position: cgmath::Vector2<f32>,
    rotation: cgmath::Quaternion<f32>,
    pub sprite_mat: usize,
    pub id: usize,
}

impl Entity {
    pub fn create(
        id: usize,
        position: cgmath::Vector2<f32>,
        rotation: cgmath::Quaternion<f32>,
        sprite_mat: usize,
        queue: &wgpu::Queue,
        index_buffer: &wgpu::Buffer,
        vertex_buffer: &wgpu::Buffer,
    ) -> Self {
        let verts = vertex::RenderVertex::new(position, rotation, &sprite::Sprite::get_vertices());

        let offset = mem::size_of::<vertex::RenderVertex>() * verts.len() * id;

        queue.write_buffer(
            &vertex_buffer,
            offset as wgpu::BufferAddress,
            bytemuck::cast_slice(verts.as_slice()),
        );

        let data = sprite::Sprite::get_indices()
            .iter()
            .map(|i| i + (4 * id as u16))
            .collect::<Vec<_>>();

        let offset = mem::size_of::<u16>() * 6 * id;

        queue.write_buffer(
            &index_buffer,
            offset as wgpu::BufferAddress,
            bytemuck::cast_slice(data.as_slice()),
        );

        Self {
            id,
            position,
            rotation,
            sprite_mat,
        }
    }

    pub fn get_position(&self) -> cgmath::Vector2<f32> {
        self.position
    }

    pub fn move_by(
        &mut self,
        offset: cgmath::Vector2<f32>,
        queue: &wgpu::Queue,
        vertex_buffer: &wgpu::Buffer,
    ) {
        self.position += offset;

        // TODO: Should we write to the buffer after every move, or once as its own step before render?
        let verts = vertex::RenderVertex::new(
            self.position,
            self.rotation,
            &sprite::Sprite::get_vertices(),
        );

        let offset = std::mem::size_of::<vertex::RenderVertex>() * 4 * self.id;

        queue.write_buffer(
            &vertex_buffer,
            offset as wgpu::BufferAddress,
            bytemuck::cast_slice(verts.as_slice()),
        );
    }
}
