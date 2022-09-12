use std::time::Instant;

use cgmath::{prelude::*, Quaternion};
use wgpu::util::DeviceExt;
use winit::{event::*, window::Window};

use crate::{
    camera, entity,
    graphics::{
        material,
        sprite::{DrawSprite, Sprite},
        vertex,
    },
    resources,
};

pub struct State {
    camera: camera::Camera,
    camera_uniform: camera::CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    clear_color: wgpu::Color,
    surface_config: wgpu::SurfaceConfiguration,
    pub device: wgpu::Device,
    pub texture_bind_group_layout: wgpu::BindGroupLayout,
    is_down_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
    is_up_pressed: bool,
    pub player: Option<entity::Entity>,
    pub queue: wgpu::Queue,
    render_pipeline: wgpu::RenderPipeline,
    pub size: winit::dpi::PhysicalSize<u32>,

    surface: wgpu::Surface,

    materials: Vec<material::Material>,
    entities: Vec<entity::Entity>,

    instant: Instant,
}

impl State {
    pub async fn new(window: &Window) -> Self {
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
            present_mode: wgpu::PresentMode::Fifo,
        };

        surface.configure(&device, &surface_config);

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

        let camera = camera::Camera {
            aspect: 1.8,
            position: (0.0, 0.0, 5.0).into(),
            scale: 6.0,
            znear: 0.1,
            zfar: 100.0,
        };

        let mut camera_uniform = camera::CameraUniform::new();
        camera_uniform.update_view_proj(&camera);

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
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

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let clear_color = wgpu::Color {
            r: 0.0,
            g: 0.2,
            b: 1.0,
            a: 1.0,
        };

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

        const NUM_INSTANCES_PER_ROW: u32 = 100;

        let grass_texture = resources::load_texture("grass.png", &device, &queue)
            .await
            .unwrap();

        let grass_material = material::Material::new(
            String::from("grass"),
            &device,
            &texture_bind_group_layout,
            grass_texture,
        );

        let materials = vec![grass_material];

        let grass_sprite = Sprite::new(String::from("grass"), 0, &device);

        let mut entities = Vec::new();

        for y in 0..NUM_INSTANCES_PER_ROW {
            for x in 0..NUM_INSTANCES_PER_ROW {
                let entity = entity::Entity::create(
                    cgmath::Vector2 {
                        x: x as f32,
                        y: y as f32,
                    },
                    Quaternion::zero(),
                    grass_sprite.duplicate(&device),
                    &queue,
                );

                entities.push(entity);
            }
        }

        Self {
            camera,
            camera_bind_group,
            camera_buffer,
            camera_uniform,
            clear_color,
            surface_config,
            device,
            entities,
            is_down_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
            is_up_pressed: false,
            queue,
            render_pipeline,
            size,
            player: None,
            texture_bind_group_layout,
            surface,
            materials,
            instant: Instant::now(),
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.camera.resize(new_size.width, new_size.height);
            self.size = new_size;
            self.surface_config.width = new_size.width;
            self.surface_config.height = new_size.height;
            self.surface.configure(&self.device, &self.surface_config);
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(keycode),
                        ..
                    },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;

                match keycode {
                    VirtualKeyCode::W | VirtualKeyCode::Up => {
                        self.is_up_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::A | VirtualKeyCode::Left => {
                        self.is_left_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::S | VirtualKeyCode::Down => {
                        self.is_down_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::D | VirtualKeyCode::Right => {
                        self.is_right_pressed = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    pub fn update(&mut self) {
        let fps = 1_000_000 / self.instant.elapsed().as_micros();
        println!("FPS: {}", fps);

        self.instant = Instant::now();

        let mut movement = cgmath::Vector2::<f32>::zero();

        if self.is_left_pressed {
            movement.x = -0.07;
        } else if self.is_right_pressed {
            movement.x = 0.07;
        }

        if self.is_up_pressed {
            movement.y = 0.07;
        } else if self.is_down_pressed {
            movement.y = -0.07;
        }

        if let Some(player) = &mut self.player {
            player.move_by(movement, &self.queue);
            self.camera.set_position(player.get_position());
        }

        self.camera_uniform.update_view_proj(&self.camera);
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
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

        for entity in &self.entities {
            let material = &self.materials[entity.sprite.material_id];

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.draw_sprite(&entity.sprite, &material, &self.camera_bind_group);
        }

        if let Some(player) = &self.player {
            let sprite = &player.sprite;
            let material = &self.materials[player.sprite.material_id];

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.draw_sprite(sprite, &material, &self.camera_bind_group);
        }

        drop(render_pass);

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn add_material(&mut self, material: material::Material) -> usize {
        self.materials.push(material);
        self.materials.len() - 1
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
