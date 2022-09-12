use std::ops::Range;

use super::{material, mesh, sprite, vertex};

pub struct Sprite {
    pub material_id: usize,
    pub mesh: mesh::Mesh,
}

impl Sprite {
    pub fn new(name: String, material_id: usize, device: &wgpu::Device) -> Self {
        let vertices = [
            vertex::Vertex {
                position: [-0.5, 0.5, 0.0],
                tex_coords: [0.0, 0.0],
            },
            vertex::Vertex {
                position: [-0.5, -0.5, 0.0],
                tex_coords: [0.0, 1.0],
            },
            vertex::Vertex {
                position: [0.5, -0.5, 0.0],
                tex_coords: [1.0, 1.0],
            },
            vertex::Vertex {
                position: [0.5, 0.5, 0.0],
                tex_coords: [1.0, 0.0],
            },
        ];

        let indices: &[u16] = &[0, 1, 3, 1, 2, 3];

        let mesh = mesh::Mesh::new(name, device, &vertices, indices);

        Self { material_id, mesh }
    }

    /* TODO: This is a temporary method to facilitate early engine development.
     * This should be removed when sprites/meshes can be loaded from disk instead
     * of being created programmatically. */
    pub fn duplicate(&self, device: &wgpu::Device) -> Self {
        Self {
            material_id: self.material_id,
            mesh: self.mesh.duplicate(device),
        }
    }
}

pub trait DrawSprite<'a> {
    fn draw_sprite(
        &mut self,
        sprite: &'a sprite::Sprite,
        material: &'a material::Material,
        camera_bind_group: &'a wgpu::BindGroup,
    );

    fn draw_sprite_instanced(
        &mut self,
        sprite: &'a sprite::Sprite,
        material: &'a material::Material,
        instances: Range<u32>,
        camera_bind_group: &'a wgpu::BindGroup,
    );
}

impl<'a, 'b> DrawSprite<'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_sprite(
        &mut self,
        sprite: &'b sprite::Sprite,
        material: &'b material::Material,
        camera_bind_group: &'b wgpu::BindGroup,
    ) {
        self.draw_sprite_instanced(
            sprite,
            material,
            0..sprite.mesh.num_elements,
            camera_bind_group,
        );
    }

    fn draw_sprite_instanced(
        &mut self,
        sprite: &'b sprite::Sprite,
        material: &'b material::Material,
        instances: Range<u32>,
        camera_bind_group: &'b wgpu::BindGroup,
    ) {
        // Define the mesh.
        self.set_vertex_buffer(0, sprite.mesh.vertex_buffer.slice(..));
        self.set_index_buffer(
            sprite.mesh.index_buffer.slice(..),
            wgpu::IndexFormat::Uint16,
        );

        self.set_bind_group(0, &material.bind_group, &[]);
        self.set_bind_group(1, camera_bind_group, &[]);

        self.draw_indexed(0..sprite.mesh.num_elements, 0, instances);
    }
}
