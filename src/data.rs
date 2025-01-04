use egui_winit::winit::dpi::{PhysicalPosition, PhysicalSize};
use egui_wgpu::wgpu as wgpu;
use crate::camera::Camera;

pub struct ColorSelectorData {
    pub mouse_locked: bool,
    pub mouse_pos: PhysicalPosition<f64>,
    pub toggle_texture: bool,
    pub rd_frame_time: f64,
    pub current_fps: f64,
    pub camera: Camera
}

impl ColorSelectorData {
    pub(crate) fn new() -> Self {
        Self {
            mouse_locked: false,
            mouse_pos: PhysicalPosition::new(0.0, 0.0),
            toggle_texture: true,
            rd_frame_time: 0.0,
            current_fps: 0.0,
            camera: Camera::new()
        }
    }

    pub(crate) fn save_mouse_pos(&mut self, pos: &PhysicalPosition<f64>) {
        self.mouse_pos = pos.clone();
    }
}