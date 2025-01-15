use anyhow::Context;
use anyhow::Result;
use egui_wgpu::wgpu;
use glam::vec3;
use gltf::accessor::Item;
use gltf::json::accessor::ComponentType;
use gltf::mesh::util::{ReadIndices, ReadTexCoords};
use gltf::{Gltf, Semantic};
use std::primitive;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
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
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<[f32; 3]>() as wgpu::BufferAddress * 2,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

pub fn load_model_woman() -> Result<(Vec<Vertex>, Vec<u16>)> {
    let (gltf, buffers, _) =
        gltf::import("rsc/Woman.gltf").context("File should be in rsc folder")?;
    
    let primitive = gltf
        .meshes()
        .nth(0)
        .context("Should have meshes")?
        .primitives()
        .nth(0)
        .context("Should have primitives")?;

    let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
    let iter_position = reader
        .read_positions()
        .context("Should have valid positions")?;
    let iter_normal = reader.read_normals().context("Should have valid normals")?;
    let iter_uv = match reader.read_tex_coords(0).context("Should have valid uvs")? {
        ReadTexCoords::F32(iter) => iter,
        _ => return Err(anyhow::anyhow!("UVs should be f32")),
    };

    if iter_position.clone().count() != iter_normal.clone().count()
        || iter_position.clone().count() != iter_uv.clone().count()
    {
        return Err(anyhow::anyhow!(
            "Different count of positions, normals and uvs"
        ));
    }

    let mut vertex = Vec::with_capacity(iter_position.clone().count());
    for ((position, normal), uv) in iter_position.zip(iter_normal).zip(iter_uv) {
        vertex.push(Vertex {
            position,
            normal,
            uv,
        });
    }

    let indices: Vec<u16> = match reader.read_indices().context("Should have indices")? {
        ReadIndices::U8(iter) => iter.map(|i| i as u16).collect(),
        ReadIndices::U16(iter) => iter.collect(),
        ReadIndices::U32(iter) => return Err(anyhow::anyhow!("U32 indices not supported")),
    };

    Ok((vertex, indices))
}

pub fn VERTICES() -> Vec<Vertex> {
    vec![
        // front
        Vertex {
            position: [-0.5, -0.5, 0.5],
            normal: vec3(-1.0, -1.0, 1.0).normalize().to_array(),
            uv: [0.0, 1.0],
        },
        Vertex {
            position: [0.5, -0.5, 0.5],
            normal: vec3(1.0, -1.0, 1.0).normalize().to_array(),
            uv: [1.0, 1.0],
        },
        Vertex {
            position: [-0.5, 0.5, 0.5],
            normal: vec3(-1.0, 1.0, 1.0).normalize().to_array(),
            uv: [0.0, 0.0],
        },
        Vertex {
            position: [0.5, 0.5, 0.5],
            normal: vec3(1.0, 1.0, 1.0).normalize().to_array(),
            uv: [1.0, 0.0],
        },
        // back
        Vertex {
            position: [0.5, -0.5, -0.5],
            normal: vec3(1.0, -1.0, -1.0).normalize().to_array(),
            uv: [0.0, 1.0],
        },
        Vertex {
            position: [-0.5, -0.5, -0.5],
            normal: vec3(-1.0, -1.0, -1.0).normalize().to_array(),
            uv: [1.0, 1.0],
        },
        Vertex {
            position: [0.5, 0.5, -0.5],
            normal: vec3(1.0, 1.0, -1.0).normalize().to_array(),
            uv: [0.0, 0.0],
        },
        Vertex {
            position: [-0.5, 0.5, -0.5],
            normal: vec3(-1.0, 1.0, -1.0).normalize().to_array(),
            uv: [1.0, 0.0],
        },
    ]
}
pub const INDICES: &[u16] = &[
    0, 1, 2, 2, 1, 3, //front
    4, 5, 6, 6, 5, 7, //back
    2, 3, 6, 6, 7, 2, //top
    0, 4, 1, 4, 0, 5, //bottom
    3, 1, 4, 4, 6, 3, //right
    0, 2, 5, 5, 2, 7, //left
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_model() {
        let (vertices, indices) = load_model_woman().unwrap();
        assert_eq!(vertices.len(), 100);
    }
}
