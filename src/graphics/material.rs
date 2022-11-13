use super::{texture, Graphics};

pub struct Material {
    pub name: String,
    pub diffuse_texture: texture::Texture,
    pub bind_group: wgpu::BindGroup,
}

impl Material {
    pub fn new(name: String, graphics: &Graphics, diffuse_texture: texture::Texture) -> Self {
        let bind_group = graphics.create_texture_bind_group(&name, &diffuse_texture);

        Self {
            name,
            bind_group,
            diffuse_texture,
        }
    }
}
