use log::info;
use winit::dpi::PhysicalPosition;
use winit::event::{DeviceEvent, Event, KeyEvent, MouseButton, WindowEvent};
use winit::window::Window;

struct ColorSelectorData {
    mouse_pos: PhysicalPosition<f64>,
    color: wgpu::Color,
}

impl ColorSelectorData {
    fn new() -> Self {
        Self {
            mouse_pos: PhysicalPosition::new(0.0, 0.0),
            color: wgpu::Color::WHITE,
        }
    }

    fn save_mouse_pos(&mut self, pos: &PhysicalPosition<f64>) {
        self.mouse_pos = pos.clone();
    }

    fn calculate_color(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        let width = size.width as f64;
        let height = size.height as f64;

        let first_fifth = width / 5.0;
        let second_fifth = 2.0 * first_fifth;
        let third_fifth = 3.0 * first_fifth;
        let fourth_fifth = 4.0 * first_fifth;

        let saturation = ((height - self.mouse_pos.y) / height) * 2.0;

        let color = if self.mouse_pos.x < first_fifth {
            let mut color = wgpu::Color {
                r: 1.0,
                g: self.mouse_pos.x / first_fifth,
                b: 0.0,
                a: 1.0,
            };

            if saturation > 1.0 {
                color.b = (saturation - 1.0);
            } else {
                color.r = color.r * saturation;
                color.g = color.g * saturation;
            }
            color
        } else if self.mouse_pos.x < second_fifth {
            let mut color = wgpu::Color {
                r: 1.0 - (self.mouse_pos.x - first_fifth) / first_fifth,
                g: 1.0,
                b: 0.0,
                a: 1.0,
            };

            if saturation > 1.0 {
                color.b = (saturation - 1.0);
            } else {
                color.r = color.r * saturation;
                color.g = color.g * saturation;
            }
            color
        } else if self.mouse_pos.x < third_fifth {
            let mut color = wgpu::Color {
                r: 0.0,
                g: 1.0,
                b: (self.mouse_pos.x - second_fifth) / first_fifth,
                a: 1.0,
            };

            if saturation > 1.0 {
                color.r = (saturation - 1.0);
            } else {
                color.g = color.g * saturation;
                color.b = color.b * saturation;
            }
            color
        } else if self.mouse_pos.x < fourth_fifth {
            let mut color = wgpu::Color {
                r: 0.0,
                g: 1.0 - (self.mouse_pos.x - third_fifth) / first_fifth,
                b: 1.0,
                a: 1.0,
            };
            if saturation > 1.0 {
                color.r = (saturation - 1.0);
            } else {
                color.g = color.g * saturation;
                color.b = color.b * saturation;
            }
            color
        } else {
            let mut color = wgpu::Color {
                r: (self.mouse_pos.x - fourth_fifth) / first_fifth,
                g: 0.0,
                b: 1.0,
                a: 1.0,
            };
            if saturation > 1.0 {
                color.g = (saturation - 1.0);
            } else {
                color.r = color.r * saturation;
                color.b = color.b * saturation;
            }
            color
        };

        self.color = color;
    }

    fn get_color(&self) -> wgpu::Color {
        self.color
    }
}

pub struct State<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub(crate) size: winit::dpi::PhysicalSize<u32>,
    // The window must be declared after the surface so
    // it gets dropped after it as the surface contains
    // unsafe references to the window's resources.
    window: &'a Window,

    //Data
    data: ColorSelectorData,
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

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            data: ColorSelectorData::new(),
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub(crate) fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub(crate) fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                event: key_event, ..
            } => {
                handle_key_event(&key_event);
                true
            }
            WindowEvent::CursorMoved { position, .. } => {
                info!("Mouse motion: {:?}", position);
                self.data.save_mouse_pos(position);
                self.data.calculate_color(self.size);
                true
            }
            WindowEvent::MouseInput { button, state, .. } if *button == MouseButton::Left => {
                info!("Mouse button: {:?} {:?}", button, state);


                true
            }
            _ => false,
        }
    }

    pub(crate) fn update(&mut self) {}

    pub(crate) fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        //https://sotrh.github.io/learn-wgpu/beginner/tutorial2-surface/#render
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
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.data.color),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

fn handle_key_event(event: &KeyEvent) {
    info!("Key event: {:?}", event);
}
