use std::path::{self, PathBuf};

use wgpu_glyph::ab_glyph;

use crate::graphics::{texture, Graphics};

pub struct Resource {}

impl Resource {
    pub fn build_path(sub_path: Option<&str>, file_name: &str) -> PathBuf {
        let path = path::Path::new(env!("OUT_DIR")).join("res");

        let path = if let Some(sub_path) = sub_path {
            path.join(sub_path)
        } else {
            path
        };

        path.join(file_name)
    }

    pub async fn load_bytes(sub_path: Option<&str>, file_name: &str) -> anyhow::Result<Vec<u8>> {
        let path = Self::build_path(sub_path, file_name);
        Ok(std::fs::read(path)?)
    }

    /**
     * TODO: This is being used for shaders (and maybe other future data types) and so is kept top-
     * level. There shouldn't be top-level data, shaders and other data types should live in their
     * own sub paths.
     */

    pub async fn load_string(file_name: &str) -> anyhow::Result<String> {
        let path = Self::build_path(None, file_name);
        Ok(std::fs::read_to_string(path)?)
    }

    pub async fn load_texture(
        graphics: &Graphics,
        file_name: &str,
    ) -> anyhow::Result<texture::Texture> {
        let data = Self::load_bytes(Some("textures"), file_name).await?;
        texture::Texture::from_bytes(graphics, &data, file_name)
    }

    pub async fn load_font(file_name: &str) -> anyhow::Result<ab_glyph::FontArc> {
        let data = Self::load_bytes(Some("fonts"), file_name).await?;
        Ok(ab_glyph::FontArc::try_from_vec(data)?)
    }
}
