use std::ops::Range;

use super::{material, mesh};

const VERTICES: &[mesh::Vertex] = &[
    mesh::Vertex { position: [-0.5, 0.5, 0.0], tex_coords: [0.0, 0.0], },
    mesh::Vertex { position: [-0.5, -0.5, 0.0], tex_coords: [0.0, 1.0], },
    mesh::Vertex { position: [0.5, -0.5, 0.0], tex_coords: [1.0, 1.0], }, 
    mesh::Vertex { position: [0.5, 0.5, 0.0], tex_coords: [1.0, 0.0], }, 
];

const INDICES: &[u16] = &[
    0, 1, 3,
    1, 2, 3
];

pub struct Sprite {
    pub material: material::Material,
    pub mesh: mesh::Mesh
}

impl Sprite {
    pub fn new(name: String, material: material::Material, device: &wgpu::Device) -> Self {
        let mesh = mesh::Mesh::new(name, device, VERTICES, INDICES);

        Self {
            material,
            mesh
        }
    }
}

pub trait DrawSprite<'a> {
    fn draw_mesh(
        &mut self, 
        mesh: &'a mesh::Mesh, 
        material: &'a material::Material, 
        camera_bind_group: &'a wgpu::BindGroup
    );

    fn draw_mesh_instanced(
        &mut self,
        mesh: &'a mesh::Mesh,
        material: &'a material::Material,
        instances: Range<u32>,
        camera_bind_group: &'a wgpu::BindGroup,
    );
}

impl<'a, 'b> DrawSprite<'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_mesh(
        &mut self, 
        mesh: &'b mesh::Mesh, 
        material: &'b material::Material,         
        camera_bind_group: &'b wgpu::BindGroup,
) {
        self.draw_mesh_instanced(mesh, material, 0..1, camera_bind_group);
    }

    fn draw_mesh_instanced(
        &mut self,
        mesh: &'b mesh::Mesh,
        material: &'b material::Material,
        instances: Range<u32>,
        camera_bind_group: &'b wgpu::BindGroup,
    ){        
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.set_bind_group(0, &material.bind_group, &[]);
        self.set_bind_group(1, camera_bind_group, &[]);
        self.draw_indexed(0..mesh.num_elements, 0, instances);
    }
}
