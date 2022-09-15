use super::vertex;

pub struct Mesh {
    pub name: String,
    pub indices: Vec<u16>,
    pub verts: Vec<vertex::Vertex>,
    pub num_elements: u32,
}

impl Mesh {
    pub fn new(name: String, verts: &[vertex::Vertex], indices: &[u16]) -> Self {
        let verts = verts.to_vec();
        let indices = indices.to_vec();
        let num_elements = indices.len() as u32;

        Self {
            name,
            indices,
            verts,
            num_elements,
        }
    }

    /* TODO: This is a temporary method to facilitate early engine development.
     * This should be removed when sprites/meshes can be loaded from disk instead
     * of being created programmatically. */
    pub fn duplicate(&self) -> Self {
        Self {
            name: self.name.clone(),
            indices: self.indices.clone(),
            verts: self.verts.clone(),
            num_elements: self.num_elements,
        }
    }
}
