use crate::{model::Vertex, texture::Texture};
mod nodes_tree;



pub struct Modelv2 {
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
    texture: Texture,

    vertex_buffer: Option<wgpu::Buffer>,
    index_buffer: Option<wgpu::Buffer>,
}