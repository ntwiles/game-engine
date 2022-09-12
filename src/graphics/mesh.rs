use cgmath::prelude::*;
use wgpu::util::DeviceExt;

use super::vertex;

pub struct Mesh {
    pub name: String,
    pub indices: Vec<u16>,
    pub index_buffer: wgpu::Buffer,
    pub verts: Vec<vertex::Vertex>,
    pub vertex_buffer: wgpu::Buffer,
    pub num_elements: u32,
}

impl Mesh {
    pub fn new(
        name: String,
        device: &wgpu::Device,
        verts: &[vertex::Vertex],
        indices: &[u16],
    ) -> Self {
        let position = cgmath::Vector3::zero();
        let rotation = cgmath::Quaternion::zero();

        let vertex_buffer_data = vertex::RenderVertex::new(position, rotation, verts);

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{:?} Vertex Buffer", name)),
            contents: bytemuck::cast_slice(vertex_buffer_data.as_slice()),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{:?} Index Buffer", name)),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let verts = verts.to_vec();
        let indices = indices.to_vec();
        let num_elements = indices.len() as u32;

        Self {
            name,
            indices,
            index_buffer,
            verts,
            vertex_buffer,
            num_elements,
        }
    }

    /* TODO: This is a temporary method to facilitate early engine development.
     * This should be removed when sprites/meshes can be loaded from disk instead
     * of being created programmatically. */
    pub fn duplicate(&self, device: &wgpu::Device) -> Self {
        let position = cgmath::Vector3::zero();
        let rotation = cgmath::Quaternion::zero();

        let vertex_buffer_data =
            vertex::RenderVertex::new(position, rotation, self.verts.as_slice());

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{:?} Vertex Buffer", self.name)),
            contents: bytemuck::cast_slice(vertex_buffer_data.as_slice()),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{:?} Index Buffer", self.name)),
            contents: bytemuck::cast_slice(self.indices.as_slice()),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            name: self.name.clone(),
            indices: self.indices.clone(),
            index_buffer,
            verts: self.verts.clone(),
            vertex_buffer,
            num_elements: self.num_elements,
        }
    }
}
