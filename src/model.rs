use crate::texture::Texture;
use anyhow::Context;
use anyhow::Result;
use egui_wgpu::wgpu;
use gltf::mesh::util::{ReadIndices, ReadTexCoords};
use std::path::Path;
use wgpu::BindGroupLayout;
use wgpu::util::DeviceExt;

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

pub struct Model {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    indice_len: usize,
    texture: Texture,
}

impl Model {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, model_path: &Path, texture_path: &Path, texture_bind_group_layout: &BindGroupLayout) -> Result<Self> {
        let (vertices, indices) = load_model(model_path)?;
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices.as_slice()),
            usage: wgpu::BufferUsages::INDEX,
        });
        let indice_len = indices.len();


        let mut texture = Texture::from_path(&device, &queue, texture_path);
        texture.create_texture_group(&device, texture_bind_group_layout);
        
        Ok(Self {
            vertex_buffer,
            index_buffer,
            indice_len,
            texture,
        })
    }
    
    pub fn draw(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.set_bind_group(0, self.texture.get_bind_group(), &[]);
        render_pass.draw_indexed(0..self.indice_len as u32, 0, 0..1);
    }
}

fn load_model(model_path: &Path) -> Result<(Vec<Vertex>, Vec<u16>)> {
    let (gltf, buffers, _) =
        gltf::import(model_path).context("File should be in rsc folder")?;

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
        ReadIndices::U32(_) => return Err(anyhow::anyhow!("U32 indices not supported")),
    };

    Ok((vertex, indices))
}