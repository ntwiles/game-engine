pub mod material;
pub mod pipeline;
pub mod sorting_layer;
pub mod sprite;
pub mod texture;
pub mod vertex;

use std::collections::HashMap;

use image::{ImageBuffer, Rgba};
use wgpu::{util::DeviceExt, Sampler, TextureView};
use wgpu_glyph::{GlyphBrush, GlyphBrushBuilder, Section, Text};
use winit::window::Window;

use crate::{camera, config::Config, entity, resources, ui::canvas::Canvas};

use self::pipeline::create_sprite_render_pipeline;
use self::sprite::DrawSprite;
use self::texture::Texture;

pub struct Graphics {
    camera_bind_group: wgpu::BindGroup,
    camera_buffer: wgpu::Buffer,
    camera_uniform: camera::CameraUniform,
    clear_color: wgpu::Color,
    device: wgpu::Device,
    index_buffer: wgpu::Buffer,
    queue: wgpu::Queue,
    render_pipeline: wgpu::RenderPipeline,
    staging_belt: wgpu::util::StagingBelt,
    surface: wgpu::Surface,
    surface_config: wgpu::SurfaceConfiguration,
    text_brush: GlyphBrush<()>,
    texture_bind_group_layout: wgpu::BindGroupLayout,
    texture_sampler: Sampler,
    vertex_buffer: wgpu::Buffer,
}

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

        let render_pipeline = create_sprite_render_pipeline(
            &device,
            &surface_config,
            &[&texture_bind_group_layout, &camera_bind_group_layout],
        )
        .await;

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

        let inconsolata = resources::load_font("Inconsolata-Regular.ttf")
            .await
            .unwrap();

        let text_brush =
            GlyphBrushBuilder::using_font(inconsolata).build(&device, surface_config.format);

        let texture_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Graphics {
            camera_bind_group,
            camera_buffer,
            camera_uniform,
            clear_color,
            device,
            index_buffer,
            queue,
            render_pipeline,
            staging_belt: wgpu::util::StagingBelt::new(1024),
            surface,
            surface_config,
            texture_bind_group_layout,
            texture_sampler,
            text_brush,
            vertex_buffer,
        }
    }

    pub fn render(
        &mut self,
        entities: &Vec<Option<entity::Entity>>,
        materials: &Vec<material::Material>,
        ui_canvas: &mut Canvas,
        config: &Config,
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

        let grouped = group_by_sorting_layer(entities);

        if let Some(background) = grouped.get(&sorting_layer::SortingLayer::Background) {
            for (entity_id, sprite_mat_id) in background {
                let material = &materials[*sprite_mat_id];
                render_pass.draw_sprite(&material, *entity_id);
            }
        }

        if let Some(foreground) = grouped.get(&sorting_layer::SortingLayer::Foreground) {
            for (entity_id, sprite_mat_id) in foreground {
                let material = &materials[*sprite_mat_id];
                render_pass.draw_sprite(&material, *entity_id);
            }
        }

        drop(render_pass);

        if config.developer_mode() {
            let text = ui_canvas.root().body();

            self.text_brush.queue(Section {
                screen_position: (32.0, 32.0),
                bounds: (
                    self.surface_config.width as f32,
                    self.surface_config.height as f32,
                ),
                text: vec![Text::new(&text)
                    .with_color([0.0, 0.0, 0.0, 1.0])
                    .with_scale(20.0)],
                ..Section::default()
            });

            self.text_brush.queue(Section {
                screen_position: (30.0, 30.0),
                bounds: (
                    self.surface_config.width as f32,
                    self.surface_config.height as f32,
                ),
                text: vec![Text::new(&text)
                    .with_color([1.0, 1.0, 1.0, 1.0])
                    .with_scale(20.0)],
                ..Section::default()
            });
        }

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

    pub fn create_texture(
        &self,
        label: &str,
        size: wgpu::Extent3d,
        image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    ) -> (wgpu::Texture, TextureView) {
        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        });

        self.queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &image,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * size.width),
                rows_per_image: std::num::NonZeroU32::new(size.height),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        (texture, view)
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
                    resource: wgpu::BindingResource::Sampler(&self.texture_sampler),
                },
            ],
            label: Some(&format!("{name} Bind Group")),
        })
    }
}

fn group_by_sorting_layer(
    entities: &Vec<Option<entity::Entity>>,
) -> HashMap<sorting_layer::SortingLayer, Vec<(usize, usize)>> {
    entities
        .into_iter()
        .fold(HashMap::new(), |mut acc, entity| {
            if let Some(entity) = entity {
                let layer = entity.get_sorting_layer();
                let value = (entity.get_id(), entity.sprite_mat);

                match acc.get_mut(&layer) {
                    Some(layer_vec) => layer_vec.push(value),
                    None => {
                        let mut layer_vec = Vec::new();
                        layer_vec.push(value);
                        acc.insert(layer, layer_vec);
                    }
                }
            }

            acc
        })
}
