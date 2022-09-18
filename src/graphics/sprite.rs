use std::ops::Range;

use super::{material, vertex};

pub struct Sprite {
    pub material_id: usize,
    pub indices: [u16; 6],
    pub vertices: [vertex::Vertex; 4],
}

impl Sprite {
    pub fn new(material_id: usize) -> Self {
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

        let indices = [0, 1, 3, 1, 2, 3];

        Self {
            material_id,
            vertices,
            indices,
        }
    }

    /* TODO: This is a temporary method to facilitate early engine development.
     * This should be removed when sprites/meshes can be loaded from disk instead
     * of being created programmatically. */
    pub fn duplicate(&self) -> Self {
        Self {
            material_id: self.material_id,
            indices: self.indices,
            vertices: self.vertices,
        }
    }
}

pub trait DrawSprite<'a> {
    fn draw_sprite(&mut self, material: &'a material::Material, entity_id: usize);

    fn draw_sprite_instanced(
        &mut self,
        material: &'a material::Material,
        entity_id: usize,
        instances: Range<u32>,
    );
}

impl<'a, 'b> DrawSprite<'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_sprite(&mut self, material: &'b material::Material, entity_id: usize) {
        self.draw_sprite_instanced(material, entity_id, 0..1);
    }

    fn draw_sprite_instanced(
        &mut self,
        material: &'b material::Material,
        entity_id: usize,
        instances: Range<u32>,
    ) {
        let index_start = entity_id as u32 * 6;
        let index_end = index_start + 6;
        self.set_bind_group(0, &material.bind_group, &[]);
        self.draw_indexed(index_start..index_end, 0, instances);
    }
}
