use crate::arrow_model::ArrowVertex;
use crate::data::UserDomain;
use wgpu::util::DeviceExt;
use wgpu::{BindGroupLayout, Buffer, Device, RenderPass, RenderPipeline, SurfaceConfiguration};

#[derive(Clone)]
#[derive(PartialEq)]
pub struct ArrowInstance {
    pub model: glam::Mat4,
    pub color: glam::Vec4,
}

impl ArrowInstance {
    fn to_raw(&self) -> ArrowInstanceRaw {
        ArrowInstanceRaw {
            model: self.model.to_cols_array_2d(),
            color: self.color.to_array(),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct ArrowInstanceRaw {
    model: [[f32; 4]; 4],
    color: [f32; 4],
}

impl ArrowInstanceRaw {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<ArrowInstanceRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x4,
                },
                //color
                wgpu::VertexAttribute {
                    offset: size_of::<[f32; 16]>() as wgpu::BufferAddress,
                    shader_location: 9,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

pub struct ArrowRenderer {
    pub render_pipeline: RenderPipeline,

    pub arrow_vertex_buffer: Buffer,
    pub arrow_index_buffer: Buffer,
    pub arrow_indice_len: usize,
    pub arrow_instance_buffer: Buffer,
    pub arrow_instance_len: usize,

    pub line_vertex_buffer: Buffer,
    pub line_index_buffer: Buffer,
    pub line_indice_len: usize,
    pub line_instance_buffer: Buffer,
    pub line_instance_len: usize,
}

impl ArrowRenderer {
    pub fn new(
        device: &Device,
        camera_bind_group_layout: &BindGroupLayout,
        config: &SurfaceConfiguration,
        data: &mut UserDomain,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::include_wgsl!("arrow.wgsl"));

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Arrow Render Pipeline Layout"),
                bind_group_layouts: &[&camera_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Arrow Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: None,
                buffers: &[ArrowVertex::desc(), ArrowInstanceRaw::desc()],
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
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        let arrow_model = crate::arrow_model::get_arrow_model();

        let arrow_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Arrow Vertex Buffer"),
            contents: bytemuck::cast_slice(arrow_model.0.as_slice()),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let arrow_index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Arrow Index Buffer"),
            contents: bytemuck::cast_slice(arrow_model.1.as_slice()),
            usage: wgpu::BufferUsages::INDEX,
        });

        let arrow_instance_data: Vec<ArrowInstanceRaw> =
            data.load_arrow().iter().map(|a| a.to_raw()).collect();

        let arrow_instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Arrow Instance Buffer"),
            contents: bytemuck::cast_slice(&arrow_instance_data),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let line_model = crate::arrow_model::get_line_model();

        let line_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Line Vertex Buffer"),
            contents: bytemuck::cast_slice(line_model.0.as_slice()),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let line_index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Line Index Buffer"),
            contents: bytemuck::cast_slice(line_model.1.as_slice()),
            usage: wgpu::BufferUsages::INDEX,
        });

        let line_instance_data: Vec<ArrowInstanceRaw> =
            data.load_line().iter().map(|a| a.to_raw()).collect();

        let line_instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Line Instance Buffer"),
            contents: bytemuck::cast_slice(&line_instance_data),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self {
            render_pipeline,
            arrow_vertex_buffer,
            arrow_index_buffer,
            arrow_indice_len: arrow_model.1.len(),
            arrow_instance_buffer,
            arrow_instance_len: arrow_instance_data.len(),
            line_vertex_buffer,
            line_index_buffer,
            line_indice_len: line_model.1.len(),
            line_instance_buffer,
            line_instance_len: line_instance_data.len(),
        }
    }

    pub fn render(
        &mut self,
        render_pass: &mut RenderPass,
        camera_bind_group: &wgpu::BindGroup,
        data: &mut UserDomain,
        device: &Device,
    ) {
        
        if data.calculate_arrow() {
            let instance_data: Vec<ArrowInstanceRaw> =
                data.load_arrow().iter().map(|a| a.to_raw()).collect();
            self.arrow_instance_buffer.destroy();
            self.arrow_instance_buffer =
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Arrow Instance Buffer"),
                    contents: bytemuck::cast_slice(&instance_data),
                    usage: wgpu::BufferUsages::VERTEX,
                });
            self.arrow_instance_len = instance_data.len();
        }

        if data.calculate_line(){
            let instance_data: Vec<ArrowInstanceRaw> =
                data.load_line().iter().map(|a| a.to_raw()).collect();
            self.line_instance_buffer.destroy();
            self.line_instance_buffer =
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Line Instance Buffer"),
                    contents: bytemuck::cast_slice(&instance_data),
                    usage: wgpu::BufferUsages::VERTEX,
                });
            self.line_instance_len = instance_data.len();
        }

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, camera_bind_group, &[]);
        if self.arrow_instance_len != 0 {
            render_pass.set_vertex_buffer(0, self.arrow_vertex_buffer.slice(..));
            render_pass
                .set_index_buffer(self.arrow_index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.set_vertex_buffer(1, self.arrow_instance_buffer.slice(..));
            render_pass.draw_indexed(
                0..self.arrow_indice_len as u32,
                0,
                0..self.arrow_instance_len as u32,
            );
        }

        if self.line_instance_len != 0 {
            render_pass.set_vertex_buffer(0, self.line_vertex_buffer.slice(..));
            render_pass
                .set_index_buffer(self.line_index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.set_vertex_buffer(1, self.line_instance_buffer.slice(..));
            render_pass.draw_indexed(
                0..self.line_indice_len as u32,
                0,
                0..self.line_instance_len as u32,
            );
        }
    }
}
