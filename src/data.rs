use egui_winit::winit::dpi::{PhysicalPosition};
use glam::Mat4;
use crate::arrow_renderer::ArrowInstance;
use crate::camera::Camera;



pub struct ColorSelectorData {
    pub mouse_locked: bool,
    pub mouse_pos: PhysicalPosition<f64>,
    pub rd_frame_time: f64,
    pub current_fps: f64,
    pub camera: Camera,
    pub arrow3d: Vec<Mat4>,
    pub arrow: Vec<ArrowInstance>,
    pub reload_arrow: bool,
}

impl ColorSelectorData {
    pub(crate) fn new() -> Self {
        Self {
            mouse_locked: false,
            mouse_pos: PhysicalPosition::new(0.0, 0.0),
            rd_frame_time: 0.0,
            current_fps: 0.0,
            camera: Camera::new(),
            arrow3d: vec![Mat4::IDENTITY],
            arrow: vec![],
            reload_arrow: true,
        }
    }

    pub(crate) fn save_mouse_pos(&mut self, pos: &PhysicalPosition<f64>) {
        self.mouse_pos = pos.clone();
    }
    
    pub fn load_arrow(&mut self) -> Vec<ArrowInstance> {
        let mut arrows = Vec::new();
        for arrow in self.arrow3d.iter() {
            arrows.push(ArrowInstance {
                model: *arrow*Mat4::IDENTITY,
                color: glam::Vec4::new(0.0, 1.0, 0.0, 1.0),
            });
            arrows.push(ArrowInstance {
                model: *arrow*Mat4::from_rotation_translation(
                    glam::Quat::from_rotation_x(std::f32::consts::PI / 2.0),
                    glam::Vec3::new(0.0, 0.0, 0.0),
                ),
                color: glam::Vec4::new(0.0, 0.0, 1.0, 1.0),
            });
            arrows.push(ArrowInstance {
                model: *arrow*Mat4::from_rotation_translation(
                    glam::Quat::from_rotation_z(-std::f32::consts::PI / 2.0),
                    glam::Vec3::new(0.0, 0.0, 0.0),
                ),
                color: glam::Vec4::new(1.0, 0.0, 0.0, 1.0),
            });
        }
        arrows.append(&mut (self.arrow.clone()));
        
        self.reload_arrow = false;
        arrows
    }
}