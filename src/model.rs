use egui_wgpu::{wgpu, ScreenDescriptor};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct Vertex {
    pub(crate) position: [f32; 3],
    pub(crate) uv: [f32; 2],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

pub const VERTICES: &[Vertex] = &[
    // front
    Vertex {
        position: [-0.5, -0.5, 0.5],
        uv: [0.0, 1.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.5],
        uv: [1.0, 1.0],
    },
    Vertex {
        position: [-0.5, 0.5, 0.5],
        uv: [0.0, 0.0],
    },
    Vertex {
        position: [0.5, 0.5, 0.5],
        uv: [1.0, 0.0],
    },
    // back
    Vertex {
        position: [0.5, -0.5, -0.5],
        uv: [0.0, 1.0],
    },
    Vertex {
        position: [-0.5, -0.5, -0.5],
        uv: [1.0, 1.0],
    },
    Vertex {
        position: [0.5, 0.5, -0.5],
        uv: [0.0, 0.0],
    },
    Vertex {
        position: [-0.5, 0.5, -0.5],
        uv: [1.0, 0.0],
    },
];
pub const INDICES: &[u16] = &[
    0, 1, 2, 2, 1, 3, //front
    4, 5, 6, 6, 5, 7, //back
    2, 3, 6, 6, 7, 2, //top
    0, 4, 1, 4, 0, 5, //bottom
    3, 1, 4, 4, 6, 3, //right
    0, 2, 5, 5, 2, 7, //left
];
