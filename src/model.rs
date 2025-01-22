use crate::texture::Texture;
use anyhow::Context;
use anyhow::Result;
use egui_wgpu::wgpu;
use gltf::mesh::util::{ReadIndices, ReadTexCoords};
use std::path::Path;
use gltf::image::Format;
use wgpu::BindGroupLayout;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
    pub affected_joints: [u16; 4],
    pub joints_weights: [f32; 4],
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
                    offset: (size_of::<[f32; 3]>() * 2) as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: (size_of::<[f32; 3]>() * 2 + size_of::<[f32; 2]>()) as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Uint16x4,
                },
                wgpu::VertexAttribute {
                    offset: (size_of::<[f32; 3]>() * 2 + size_of::<[f32; 2]>() + size_of::<[u16; 4]>()) as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

pub struct Model {
    vertex_buffer: wgpu::Buffer,
    indices_buffer: wgpu::Buffer,
    indices_len: usize,
    texture: Texture,
}

impl Model {
    pub fn from_gltf(device: &wgpu::Device, queue: &wgpu::Queue, model_path: &Path, texture_bind_group_layout: &BindGroupLayout) -> Result<Self> {
        let (vertices, indices, image) = load_model(model_path)?;
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


        let mut texture = Texture::from_bytes(&device, &queue, image.0.as_slice(),  image.1,  image.2);
        texture.create_texture_group(&device, texture_bind_group_layout);

        Ok(Self {
            vertex_buffer,
            indices_buffer: index_buffer,
            indices_len: indice_len,
            texture,
        })
    }
    
    pub fn draw(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.indices_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.set_bind_group(0, self.texture.get_bind_group(), &[]);
        render_pass.draw_indexed(0..self.indices_len as u32, 0, 0..1);
    }
}

fn load_model(model_path: &Path) -> Result<(Vec<Vertex>, Vec<u16>, (Vec<u8>, u32, u32))> {
    let (gltf, buffers, images) =
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
            affected_joints: [0, 0, 0, 0],
            joints_weights: [0.0, 0.0, 0.0, 0.0],
        });
    }

    let indices: Vec<u16> = match reader.read_indices().context("Should have indices")? {
        ReadIndices::U8(iter) => iter.map(|i| i as u16).collect(),
        ReadIndices::U16(iter) => iter.collect(),
        ReadIndices::U32(_) => return Err(anyhow::anyhow!("U32 indices not supported")),
    };
    
    if images.len() == 0 {
        return Ok((vertex, indices, (Vec::new(), 0, 0)));
    }
    
    let index_image_color = gltf.materials().nth(0).context("Should have materials")?.pbr_metallic_roughness().base_color_texture().unwrap().texture().index();
    let image_rgb = images[index_image_color].pixels.clone();

    match images[index_image_color].format{

        Format::R8G8B8 => {
            let image_rgba = image_rgb.chunks_exact(3).map(|chunk| {
                let mut chunk = chunk.to_vec();
                chunk.push(255);
                chunk
            }).flatten().collect();
            return  Ok((vertex, indices, (image_rgba, images[0].width, images[0].height)));
        }
        Format::R8G8B8A8 => {
            return  Ok((vertex, indices, (image_rgb, images[0].width, images[0].height)));
        }
        Format::R16 => unimplemented!(),
        Format::R16G16 => unimplemented!(),
        Format::R16G16B16 => unimplemented!(),
        Format::R16G16B16A16 => unimplemented!(),
        Format::R32G32B32FLOAT => unimplemented!(),
        Format::R32G32B32A32FLOAT => unimplemented!(),
        Format::R8 => unimplemented!(),
        Format::R8G8 => unimplemented!(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_load_model() {
        let path = PathBuf::from("rsc").join("duck").join("glTF").join("Duck.gltf");
        let (_, _, _) = load_model(&path).unwrap();
    
    }
}