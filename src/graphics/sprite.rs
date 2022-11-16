use std::ops::Range;

use super::{material, vertex};

pub struct Sprite;

impl Sprite {
    pub fn get_indices() -> [u32; 6] {
        [0, 1, 3, 1, 2, 3]
    }

    pub fn get_vertices() -> [vertex::Vertex; 4] {
        [
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
        ]
    }
}

pub trait DrawSprite<'a> {
    fn draw_sprite(&mut self, material: &'a material::Material, entity_id: usize);
    fn draw_sprites(&mut self, material: &'a material::Material, range: Range<u32>);
}

impl<'a, 'b> DrawSprite<'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_sprite(&mut self, material: &'b material::Material, entity_id: usize) {
        let index_start = entity_id as u32 * 6;
        let index_end = index_start + 6;
        self.set_bind_group(0, &material.bind_group, &[]);
        self.draw_indexed(index_start..index_end, 0, 0..1);
    }

    fn draw_sprites(&mut self, material: &'b material::Material, range: Range<u32>) {
        self.set_bind_group(0, &material.bind_group, &[]);
        self.draw_indexed(range, 0, 0..1);
    }
}
