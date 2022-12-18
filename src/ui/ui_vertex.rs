use crate::graphics::vertex::Vertex;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct UiRenderVertex {
    pub position: [f32; 3],
    pub color: [f32; 3], // TODO: Support alpha channel and maybe 64-bit components?
}

impl UiRenderVertex {
    pub fn new(verts: &[Vertex], color: wgpu::Color) -> Vec<Self> {
        verts
            .iter()
            .map(|v| Self {
                position: v.position,
                color: [color.r as f32, color.g as f32, color.b as f32],
            })
            .collect::<Vec<_>>()
    }

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}
