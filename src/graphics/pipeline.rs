use crate::{resources, ui::ui_vertex};

use super::vertex;

fn new(
    shader: wgpu::ShaderModule,
    layout: wgpu::PipelineLayout,
    device: &wgpu::Device,
    config: &wgpu::SurfaceConfiguration,
    label: &str,
    buffer_layout: wgpu::VertexBufferLayout,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(label),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[buffer_layout],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    })
}

pub async fn create_sprite_render_pipeline(
    device: &wgpu::Device,
    config: &wgpu::SurfaceConfiguration,
    bind_group_layouts: &[&wgpu::BindGroupLayout],
) -> wgpu::RenderPipeline {
    let sprite_shader = resources::load_string("sprite.wgsl").await.unwrap();
    let sprite_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Sprite Shader"),
        source: wgpu::ShaderSource::Wgsl(sprite_shader.into()),
    });

    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Sprite Render Pipeline Layout"),
        bind_group_layouts,
        push_constant_ranges: &[],
    });

    new(
        sprite_shader,
        layout,
        device,
        config,
        "Sprite Render Pipeline",
        vertex::RenderVertex::desc(),
    )
}

// TODO: Generalize this with the above function.
pub async fn create_ui_render_pipeline(
    device: &wgpu::Device,
    config: &wgpu::SurfaceConfiguration,
    bind_group_layouts: &[&wgpu::BindGroupLayout],
) -> wgpu::RenderPipeline {
    let sprite_shader = resources::load_string("color.wgsl").await.unwrap();
    let sprite_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Primitive Shader"),
        source: wgpu::ShaderSource::Wgsl(sprite_shader.into()),
    });

    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("UI Render Pipeline Layout"),
        bind_group_layouts,
        push_constant_ranges: &[],
    });

    new(
        sprite_shader,
        layout,
        device,
        config,
        "UI Render Pipeline",
        ui_vertex::UiRenderVertex::desc(),
    )
}
