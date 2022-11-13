pub mod material;
pub mod sprite;
pub mod texture;
pub mod vertex;

use wgpu::util::DeviceExt;
use wgpu_glyph::{ab_glyph, GlyphBrush, GlyphBrushBuilder, Section, Text};
use winit::window::Window;

pub struct Graphics {
    camera_bind_group: wgpu::BindGroup,
    camera_buffer: wgpu::Buffer,
    camera_uniform: camera::CameraUniform,
    clear_color: wgpu::Color,
    pub device: wgpu::Device,
    index_buffer: wgpu::Buffer,
    pub queue: wgpu::Queue,
    render_pipeline: wgpu::RenderPipeline,
    staging_belt: wgpu::util::StagingBelt,
    surface: wgpu::Surface,
    surface_config: wgpu::SurfaceConfiguration,
    text_brush: GlyphBrush<()>,
    texture_bind_group_layout: wgpu::BindGroupLayout,
    vertex_buffer: wgpu::Buffer,
}

use super::sprite::DrawSprite;

use crate::{
    camera,
    entity,
    graphics::texture::Texture, // this couldn't be imported within super::{} for some reason?
    resources,
};

const MAX_ENTITIES: usize = 24000;

impl Graphics {
    pub async fn new(window: &Window, camera: &camera::Camera) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            present_mode: wgpu::PresentMode::Fifo,
        };

        surface.configure(&device, &surface_config);

        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            size: std::mem::size_of::<[vertex::RenderVertex; MAX_ENTITIES * 4]>() as u64,
            label: Some("Vertex Buffer"),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let mut indices = Vec::<u32>::new();

        for i in 0..MAX_ENTITIES {
            let new_indices = sprite::Sprite::get_indices()
                .into_iter()
                .map(|idx| idx + (4 * i as u32));

            indices.extend(new_indices);
        }

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices.as_slice()),
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
        });

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });

        let sprite_shader = resources::load_string("shader.wgsl").await.unwrap();
        let sprite_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Sprite Shader"),
            source: wgpu::ShaderSource::Wgsl(sprite_shader.into()),
        });

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&texture_bind_group_layout, &camera_bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = create_render_pipeline(
            sprite_shader,
            layout,
            &[vertex::RenderVertex::desc()],
            &device,
            &surface_config,
        );

        let clear_color = wgpu::Color {
            r: 0.0,
            g: 0.2,
            b: 1.0,
            a: 1.0,
        };

        let mut camera_uniform = camera::CameraUniform::new();
        camera_uniform.update_view_proj(&camera);

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let inconsolata =
            ab_glyph::FontArc::try_from_slice(include_bytes!("Inconsolata-Regular.ttf")).unwrap();

        let text_brush =
            GlyphBrushBuilder::using_font(inconsolata).build(&device, surface_config.format);

        let staging_belt = wgpu::util::StagingBelt::new(1024);

        Graphics {
            camera_bind_group,
            camera_buffer,
            camera_uniform,
            clear_color,
            device,
            index_buffer,
            queue,
            render_pipeline,
            staging_belt,
            surface,
            surface_config,
            texture_bind_group_layout,
            text_brush,
            vertex_buffer,
        }
    }

    pub fn render(
        &mut self,
        entities: &Vec<Option<entity::Entity>>,
        player: &Option<entity::Entity>,
        wall: &Option<entity::Entity>,
        materials: &Vec<material::Material>,
    ) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(self.clear_color),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(1, &self.camera_bind_group, &[]);

        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);

        let mat_id = entities[0].as_ref().unwrap().sprite_mat;
        let material = &materials[mat_id];
        render_pass.draw_sprites(&material, 0..entities.len() as u32);

        if let Some(player) = player {
            let material = &materials[player.sprite_mat];
            render_pass.draw_sprite(&material, player.get_id());
        }

        if let Some(wall) = wall {
            let material = &materials[wall.sprite_mat];
            render_pass.draw_sprite(&material, wall.get_id());
        }

        drop(render_pass);

        self.text_brush.queue(Section {
            screen_position: (30.0, 30.0),
            bounds: (
                self.surface_config.width as f32,
                self.surface_config.height as f32,
            ),
            text: vec![Text::new("Hello wgpu_glyph!")
                .with_color([0.0, 0.0, 0.0, 1.0])
                .with_scale(40.0)],
            ..Section::default()
        });

        self.text_brush.queue(Section {
            screen_position: (30.0, 90.0),
            bounds: (
                self.surface_config.width as f32,
                self.surface_config.height as f32,
            ),
            text: vec![Text::new("Hello wgpu_glyph!")
                .with_color([1.0, 1.0, 1.0, 1.0])
                .with_scale(40.0)],
            ..Section::default()
        });

        self.text_brush
            .draw_queued(
                &self.device,
                &mut self.staging_belt,
                &mut encoder,
                &view,
                self.surface_config.width,
                self.surface_config.height,
            )
            .expect("Draw queued");

        self.staging_belt.finish();
        self.queue.submit([encoder.finish()]);

        output.present();

        self.staging_belt.recall();

        Ok(())
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.surface_config.width = new_size.width;
        self.surface_config.height = new_size.height;
        self.surface.configure(&self.device, &self.surface_config);
    }

    pub fn write_camera(&mut self, camera: &camera::Camera) {
        self.camera_uniform.update_view_proj(camera);

        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );
    }

    pub fn write_entity(&mut self, id: usize, verts: Vec<vertex::RenderVertex>) {
        let offset = std::mem::size_of::<vertex::RenderVertex>() * 4 * id;

        self.queue.write_buffer(
            &self.vertex_buffer,
            offset as wgpu::BufferAddress,
            bytemuck::cast_slice(verts.as_slice()),
        );
    }

    pub fn create_texture_bind_group(
        &self,
        name: &str,
        diffuse_texture: &Texture,
    ) -> wgpu::BindGroup {
        self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                },
            ],
            label: Some(&format!("{name} Bind Group")),
        })
    }
}

fn create_render_pipeline(
    shader: wgpu::ShaderModule,
    layout: wgpu::PipelineLayout,
    v_buffers: &[wgpu::VertexBufferLayout],
    device: &wgpu::Device,
    config: &wgpu::SurfaceConfiguration,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: v_buffers,
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
