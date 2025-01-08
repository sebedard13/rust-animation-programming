use crate::camera::{Camera, CameraMatBuffer};
use crate::data::UserDomain;
use crate::gui;
use crate::gui::EguiRenderer;
use crate::model::{Vertex, INDICES, VERTICES};
use crate::texture::Texture;
use egui_wgpu::wgpu::util::DeviceExt;
use egui_wgpu::wgpu::Adapter;
use egui_wgpu::{wgpu, ScreenDescriptor};
use egui_winit::winit::dpi::PhysicalSize;
use egui_winit::winit::event::{
    DeviceEvent, ElementState, KeyEvent, MouseButton, WindowEvent,
};
use egui_winit::winit::keyboard::{KeyCode, PhysicalKey};
use egui_winit::winit::window::Window;
use crate::arrow_renderer::ArrowRenderer;

pub struct State<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub(crate) size: PhysicalSize<u32>,
    // The window must be declared after the surface so
    // it gets dropped after it as the surface contains
    // unsafe references to the window's resources.
    window: &'a Window,

    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,

    egui_renderer: EguiRenderer,

    //Data
    pub data: UserDomain,
    crate_tex: Texture,

    pub camera_mat_buffer: CameraMatBuffer,
    pub camera_buffer: wgpu::Buffer,
    pub camera_bind_group: wgpu::BindGroup,
    
    pub arrow_renderer: ArrowRenderer,
}

impl<'a> State<'a> {
    // Creating some of the wgpu types requires async code
    pub(crate) async fn new(window: &'a Window) -> State<'a> {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(window).unwrap();

        let adapter: Adapter = instance
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
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    label: None,
                    memory_hints: Default::default(),
                },
                None, // Trace path
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result in all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

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

        let mut crate_tex = Texture::from_file(&device, &queue, include_bytes!("crate.png"));
        crate_tex.create_texture_group(&device, &texture_bind_group_layout);

        let camera_mat_buffer = CameraMatBuffer::new();

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_mat_buffer]),
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

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout, &camera_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: None,
                buffers: &[Vertex::desc()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: None,
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let egui_renderer = EguiRenderer::new(&device, config.format, None, 1, &window);

        let mut data = UserDomain::new();
        data.camera.aspect = (size.width as f32) / (size.height as f32);
        data.camera.update_vectors();
        
        let arrow_renderer = ArrowRenderer::new(&device, &camera_bind_group_layout, &config, &mut data);
        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            data,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            crate_tex,
            egui_renderer,
            camera_mat_buffer,
            camera_buffer,
            camera_bind_group,
            arrow_renderer
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub(crate) fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.data.camera.aspect = (new_size.width as f32)/ (new_size.height as f32);
        }
    }

    pub(crate) fn input(&mut self, event: &WindowEvent) -> bool {
        let event_resp = self.egui_renderer.handle_input(self.window, event);
        if event_resp.1 {
            self.window.request_redraw();
        }
        if event_resp.0 {
            return true;
        }

        match event {
            WindowEvent::KeyboardInput {
                event: key_event, ..
            } => {
                handle_wasd_input(key_event, &mut self.data.camera);
                false
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.data.save_mouse_pos(position);
                true
            }
            WindowEvent::MouseInput { button, .. } if *button == MouseButton::Left => true,
            WindowEvent::MouseInput { button,  .. } if *button == MouseButton::Right => {
                self.data.mouse_locked = !self.data.mouse_locked;
                true
            }
            _ => false,
        }
    }

    pub fn raw_input(&mut self, event: &DeviceEvent) -> bool {
        match event {
            DeviceEvent::MouseMotion { delta } => {
                if self.data.mouse_locked {
                    self.data.camera.view_azimuth += delta.0 * 0.1;
                    self.data.camera.view_elevation -= delta.1 * 0.1;
                    self.data.camera.update_vectors();
                    return true;
                }
                false
            }
            _ => false,
        }
    }

    pub(crate) fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.data.camera.move_update();
        self.camera_mat_buffer.update(&self.data.camera);
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_mat_buffer]),
        );
        self.window.set_cursor_visible(!self.data.mouse_locked);

        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, self.crate_tex.get_bind_group(), &[]);
            render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..INDICES.len() as u32, 0, 0..1);
        
            self.arrow_renderer.render(&mut render_pass, &self.camera_bind_group, &mut self.data);
        }

        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [self.config.width, self.config.height],
            pixels_per_point: self.window().scale_factor() as f32,
        };

        self.egui_renderer.draw(
            &self.device,
            &self.queue,
            &mut encoder,
            &self.window,
            &view,
            screen_descriptor,
            |ui| gui::gui(&mut self.data, ui),
        );

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

fn handle_wasd_input(event: &KeyEvent, camera: &mut Camera) {
    match event {
        KeyEvent {
            state: ElementState::Pressed,
            physical_key: PhysicalKey::Code(KeyCode::KeyW),
            ..
        } => camera.move_front_back = 1.0,

        KeyEvent {
            state: ElementState::Released,
            physical_key: PhysicalKey::Code(KeyCode::KeyW),
            ..
        } => camera.move_front_back = 0.0,
        KeyEvent {
            state: ElementState::Pressed,
            physical_key: PhysicalKey::Code(KeyCode::KeyS),
            ..
        } => camera.move_front_back = -1.0,
        KeyEvent {
            state: ElementState::Released,
            physical_key: PhysicalKey::Code(KeyCode::KeyS),
            ..
        } => camera.move_front_back = 0.0,
        KeyEvent {
            state: ElementState::Pressed,
            physical_key: PhysicalKey::Code(KeyCode::KeyA),
            ..
        } => camera.move_left_right = -1.0,
        KeyEvent {
            state: ElementState::Released,
            physical_key: PhysicalKey::Code(KeyCode::KeyA),
            ..
        } => camera.move_left_right = 0.0,
        KeyEvent {
            state: ElementState::Pressed,
            physical_key: PhysicalKey::Code(KeyCode::KeyD),
            ..
        } => camera.move_left_right = 1.0,
        KeyEvent {
            state: ElementState::Released,
            physical_key: PhysicalKey::Code(KeyCode::KeyD),
            ..
        } => camera.move_left_right = 0.0,
        KeyEvent {
            state: ElementState::Pressed,
            physical_key: PhysicalKey::Code(KeyCode::KeyQ),
            ..
        } => camera.move_up_down = -1.0,
        KeyEvent {
            state: ElementState::Released,
            physical_key: PhysicalKey::Code(KeyCode::KeyQ),
            ..
        } => camera.move_up_down = 0.0,
        KeyEvent {
            state: ElementState::Pressed,
            physical_key: PhysicalKey::Code(KeyCode::KeyE),
            ..
        } => camera.move_up_down = 1.0,
        KeyEvent {
            state: ElementState::Released,
            physical_key: PhysicalKey::Code(KeyCode::KeyE),
            ..
        } => camera.move_up_down = 0.0,
        KeyEvent {
            state: ElementState::Pressed,
            physical_key: PhysicalKey::Code(KeyCode::KeyR),
            ..
        } => camera.reset(),
        _ => {}
    }
}