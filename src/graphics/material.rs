use super::texture;

pub struct Material {
    pub name: String,
    pub diffuse_texture: texture::Texture,
    pub bind_group: wgpu::BindGroup,
}

impl Material {
    pub fn new(name: String, device: &wgpu::Device, layout: &wgpu::BindGroupLayout, diffuse_texture: texture::Texture) -> Self {
        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                    }
                ],
                label: Some(&format!("{name} Bind Group")),
            }
        );

        Self {
            name,
            bind_group,
            diffuse_texture
        }
    }
}