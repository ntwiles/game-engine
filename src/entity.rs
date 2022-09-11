use cgmath;

pub struct Entity {
    pub position: cgmath::Vector3<f32>,
    pub rotation: cgmath::Quaternion<f32>,
    pub sprite_id: usize,
}
