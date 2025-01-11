
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct ArrowVertex {
    pub(crate) position: [f32; 3],
}

impl ArrowVertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<ArrowVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x3,
            }],
        }
    }
}

pub fn get_arrow_model() -> (Vec<ArrowVertex>, Vec<u16>) {
    use lyon::math::point;
    use lyon::path::Path;
    use lyon::tessellation::*;

    let mut builder = Path::builder();
    builder.begin(point(0.0, 0.0));
    builder.line_to(point(0.0, 1.0));
    builder.line_to(point(0.2, 0.8));
    builder.line_to(point(0.0, 1.0));
    builder.line_to(point(-0.2, 0.8));
    builder.line_to(point(0.0, 1.0));
    builder.end(true);


    let mut tessellator = StrokeTessellator::new();
    let mut geometry: VertexBuffers<ArrowVertex, u16> = VertexBuffers::new();
    {
        tessellator.tessellate_path(
            &builder.build(),
            &StrokeOptions::default().with_line_width(0.05).with_line_join(LineJoin::Bevel),
            &mut BuffersBuilder::new(&mut geometry, |pos: StrokeVertex| ArrowVertex{ position: [pos.position().x, pos.position().y, 0.0]}),
        ).unwrap();
    }
    
    let z_vertices: Vec<ArrowVertex> = geometry.vertices.iter().map(|v| ArrowVertex{position: [0.0, v.position[1], v.position[0]]}).collect();
    let z_indices: Vec<u16> = geometry.indices.iter().map(|i| i + geometry.vertices.len() as u16).collect();
    
    let merged_vertices = geometry.vertices.iter().chain(z_vertices.iter()).cloned().collect();
    let merged_indices = geometry.indices.iter().chain(z_indices.iter()).cloned().collect();
    
     (merged_vertices, merged_indices)
}

pub fn get_line_model() -> (Vec<ArrowVertex>, Vec<u16>) {
    use lyon::math::point;
    use lyon::path::Path;
    use lyon::tessellation::*;

    let mut builder = Path::builder();
    builder.begin(point(0.0, 0.0));
    builder.line_to(point(0.0, 1.0));
    builder.end(false);

    let mut tessellator = StrokeTessellator::new();
    let mut geometry: VertexBuffers<ArrowVertex, u16> = VertexBuffers::new();
    {
        tessellator.tessellate_path(
            &builder.build(),
            &StrokeOptions::default().with_line_width(0.05).with_line_join(LineJoin::Bevel),
            &mut BuffersBuilder::new(&mut geometry, |pos: StrokeVertex| ArrowVertex{ position: [pos.position().x, pos.position().y, 0.0]}),
        ).unwrap();
    }
    
    let z_vertices: Vec<ArrowVertex> = geometry.vertices.iter().map(|v| ArrowVertex{position: [0.0, v.position[1], v.position[0]]}).collect();
    let z_indices: Vec<u16> = geometry.indices.iter().map(|i| i + geometry.vertices.len() as u16).collect();
    
    let merged_vertices = geometry.vertices.iter().chain(z_vertices.iter()).cloned().collect();
    let merged_indices = geometry.indices.iter().chain(z_indices.iter()).cloned().collect();
    
     (merged_vertices, merged_indices)
}
