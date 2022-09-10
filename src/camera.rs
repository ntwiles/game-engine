
#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

pub struct Camera {
    pub aspect: f32,
    pub position: cgmath::Point3<f32>,
    pub scale: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.position, (self.position.x, self.position.y, 0.0).into(), cgmath::Vector3::unit_y());
        
        let proj = cgmath::ortho(
            -self.scale * self.aspect, 
            self.scale * self.aspect, 
            -self.scale, 
            self.scale, 
            self.znear, 
            self.zfar
        );

        return OPENGL_TO_WGPU_MATRIX  * proj * view;
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}

// TODO: Remove this controller.
pub struct CameraController {}

impl CameraController {
    pub fn update_camera(&self, camera: &mut Camera, target: cgmath::Vector3<f32>) {
        camera.position.x = target.x;
        camera.position.y = target.y;
    }
}
