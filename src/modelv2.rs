use std::path::Path;
use anyhow::{Context, Result};
use gltf::image::Format;
use gltf::mesh::util::{ReadIndices, ReadJoints, ReadTexCoords, ReadWeights};
use wgpu::{BindGroup, BindGroupLayout, Device, Queue};
use wgpu::util::DeviceExt;
use crate::model::Vertex;
use crate::modelv2::nodes_tree::{create_nodes_tree_from_joints, NodeTree};
use crate::texture::Texture;

mod nodes_tree;

#[derive(Default)]
pub struct ImageData {
    pub data_rgba: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

impl ImageData {
    pub fn new(data_rgba: Vec<u8>, width: u32, height: u32) -> Self {
        Self {
            data_rgba,
            width,
            height,
        }
    }

}

pub struct Modelv2 {
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
    texture: ImageData,
    nodes_tree: NodeTree,

    vertices_buffer: Option<wgpu::Buffer>,
    indices_buffer: Option<wgpu::Buffer>,
    texture_buffer: Option<Texture>,
    joints_buffer: Option<BindGroup>,
}

impl Modelv2 {
    pub fn load_woman() ->  Result<Self> {
        let model_path = &Path::new("rsc").join("Woman.gltf");
        let (gltf, buffers, images) = gltf::import(model_path).context("File should be in rsc folder")?;

        let scene = gltf.default_scene().or_else(|| gltf.scenes().nth(0)).context("Should have a scene")?;
        let mesh_node = scene.nodes().nth(0).context("Should have a node")?.children().nth(0).context("Should have a child")?;

        let skin = mesh_node.skin().context("Should have a skin")?;
        let joints: Vec<usize> = skin.joints().map(|joint| joint.index()).collect();
        let nodes = gltf.nodes().collect::<Vec<gltf::Node>>();
        let inverse_bind_matrices = skin.reader(|buffer| Some(&buffers[buffer.index()])).read_inverse_bind_matrices().context("Should have inverse bind matrices")?;
        let inverse_bind_matrices: Vec<glam::Mat4> = inverse_bind_matrices.map(|m| {
            glam::Mat4::from_cols_array_2d(&m)
        }).collect();
        let nodes_tree = create_nodes_tree_from_joints(joints, nodes, inverse_bind_matrices);

        let mesh = mesh_node.mesh().context("Should have a mesh")?;
        let primitive = mesh.primitives().nth(0).context("Should have a primitive")?;
        let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
        let positions: Vec<[f32; 3]> = reader.read_positions().context("Should have positions")?.collect();
        let normals: Vec<[f32; 3]> = reader.read_normals().context("Should have normals")?.collect();
        let uvs: Vec<[f32; 2]> = match reader.read_tex_coords(0).context("Should have uvs")? {
            ReadTexCoords::F32(iter) => iter.collect(),
            _ => return Err(anyhow::anyhow!("UVs should be f32")),
        };

        if positions.len() != normals.len() || positions.len() != uvs.len() {
            return Err(anyhow::anyhow!("Positions, normals and uvs should have the same length"));
        }
        
        let affected_joints: Option<Vec<[u32; 4]>> = {
            let affected_joints = reader.read_joints(0);
            if let Some(affected_joints) = affected_joints {
                match affected_joints{
                    ReadJoints::U8(joints) => Some(joints.map(|j: [u8;4]| j.map(|i| i as u32)).collect()),
                    ReadJoints::U16(joints) => Some(joints.map(|j: [u16;4]| j.map(|i| i as u32)).collect()),
                }
            } else {
                None
            }
        };
        
        let joints_weights:  Option<Vec<[f32; 4]>> = {
            let joints_weights = reader.read_weights(0);
            if let Some(joints_weights) = joints_weights {
                match joints_weights{
                    ReadWeights::U8(weight) => Some(weight.map(|w: [u8;4]| w.map(|i| (i as f32)/255.0)).collect()),
                    ReadWeights::U16(weight) => Some(weight.map(|w: [u16;4]| w.map(|i| (i as f32)/65535.0)).collect()),
                    ReadWeights::F32(weight) => Some(weight.collect()),
                }
            } else {
                None
            }
        };

        let mut vertices = Vec::with_capacity(positions.len());
        for i in 0..positions.len() {
            let affected_joints = match &affected_joints {
                Some(affected_joints) => affected_joints[i],
                None => [0, 0, 0, 0],
            };
            let joints_weights = match &joints_weights {
                Some(joints_weights) => joints_weights[i],
                None => [0.0, 0.0, 0.0, 0.0],
            };
            vertices.push(Vertex {
                position: positions[i],
                normal: normals[i],
                uv: uvs[i],
                affected_joints,
                joints_weights
            });
        }

        let indices: Vec<u16> = match reader.read_indices().context("Should have indices")? {
            ReadIndices::U8(iter) => iter.map(|i| i as u16).collect(),
            ReadIndices::U16(iter) => iter.collect(),
            _ => { return Err(anyhow::anyhow!("Indices should be u8 or u16")); }
        };

        let material = primitive.material();
        let image = material.pbr_metallic_roughness().base_color_texture().context("Should have a base color texture")?.texture().source();
        let image_data = &images[image.index()];
        let image_rgba: Vec<u8> = match image_data.format {
            Format::R8G8B8 => {
                image_data.pixels.chunks_exact(3).map(|chunk| {
                    let mut chunk = chunk.to_vec();
                    chunk.push(255);
                    chunk
                }).flatten().collect()
            },
            Format::R8G8B8A8 => image_data.pixels.clone(),
            _ => { return Err(anyhow::anyhow!("Image format should be R8G8B8")); }
        };
        
        let texture = ImageData::new(image_rgba, image_data.width, image_data.height);
        

        Ok(Self{
            vertices,
            indices,
            texture,
            nodes_tree,
            vertices_buffer: None,
            indices_buffer: None,
            texture_buffer: None,
            joints_buffer: None,
        })
    }
    
    pub fn load_on_gpu(&mut self, device: &Device, queue: &Queue, texture_bind_group_layout: &BindGroupLayout, joints_bind_group_layout: &BindGroupLayout) {
        let joints = self.nodes_tree.get_joints_double_quat();
        let joints:Vec<[[f32;4];2]> = joints.iter().map(|j| [[j[0].x, j[0].y, j[0].z, j[0].w], [j[1].x, j[1].y, j[1].z, j[1].w]]).collect();
        
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&self.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(self.indices.as_slice()),
            usage: wgpu::BufferUsages::INDEX,
        });
        
        let mut texture_buffer = Texture::from_bytes(device, queue, &self.texture.data_rgba, self.texture.width, self.texture.height);
        texture_buffer.create_texture_group(device, texture_bind_group_layout);
        
        let joints_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Joints Buffer"),
            contents: bytemuck::cast_slice(joints.as_slice()),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });
        
        let joints_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: joints_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: joints_buffer.as_entire_binding(),
                },
            ],
            label: Some("joints_bind_group"),
        });
        
    
        self.vertices_buffer = Some(vertex_buffer);
        self.indices_buffer = Some(index_buffer);
        self.texture_buffer = Some(texture_buffer);
        self.joints_buffer = Some(joints_bind_group);
    }

    pub fn draw(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_vertex_buffer(0, self.vertices_buffer.as_ref().unwrap().slice(..));
        render_pass.set_index_buffer(self.indices_buffer.as_ref().unwrap().slice(..), wgpu::IndexFormat::Uint16);
        render_pass.set_bind_group(0, self.texture_buffer.as_ref().unwrap().get_bind_group(), &[]);
        render_pass.set_bind_group(3, self.joints_buffer.as_ref().unwrap(), &[]);
        render_pass.draw_indexed(0..self.indices.len() as u32, 0, 0..1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load(){
        let model = Modelv2::load_woman();
        model.unwrap();
    }
}