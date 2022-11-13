use anyhow::*;
use image::GenericImageView;

use super::Graphics;

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
}

impl Texture {
    pub fn from_bytes(graphics: &Graphics, bytes: &[u8], label: &str) -> Result<Self> {
        let img = image::load_from_memory(bytes)?;
        Self::from_image(graphics, &img, label)
    }

    pub fn from_image(graphics: &Graphics, img: &image::DynamicImage, label: &str) -> Result<Self> {
        let buffer = img.to_rgba8();
        let dimensions = img.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let (texture, view) = graphics.create_texture(label, size, buffer);

        Ok(Self { texture, view })
    }
}
