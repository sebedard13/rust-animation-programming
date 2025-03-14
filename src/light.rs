use glam::Vec3;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightBuffer {
    pub position: [f32; 3],
    _padding1: f32,
    pub color: [f32; 3],
    _padding2: f32,
}

impl LightBuffer {
    pub fn new(position: &Vec3, color: &Vec3) -> Self {
        Self {
            position: [position.x, position.y, position.z],
            _padding1: 0.0,
            color: [color.x, color.y, color.z],
            _padding2: 0.0,
        }
    }
}
