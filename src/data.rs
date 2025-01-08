use egui_winit::winit::dpi::{PhysicalPosition};
use glam::Mat4;
use crate::arrow_renderer::ArrowInstance;
use crate::camera::Camera;



pub struct UserDomain {
    pub mouse_locked: bool,
    pub mouse_pos: PhysicalPosition<f64>,
    pub rd_frame_time: f64,
    pub current_fps: f64,
    pub camera: Camera,
    
    pub arrow3d: Vec<Mat4>,
    pub arrow: Vec<ArrowInstance>,
    pub reload_arrow: bool,
    
    pub draw_world_coordinates: bool,
    pub draw_model_coordinates: bool,
    
    pub interpolation : f32,
    pub start_rotation: glam::Vec3,
    pub end_rotation: glam::Vec3,
    
    pub draw_spline: bool,
    pub start_pos: glam::Vec3,
    pub start_tangent: glam::Vec3,
    pub end_pos: glam::Vec3,
    pub end_tangent: glam::Vec3,
}

impl UserDomain {
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
            
            interpolation: 0.0,
            
            draw_world_coordinates: true,
            draw_model_coordinates: true,
            start_rotation: glam::Vec3::new(0.0, 0.0, 0.0),
            end_rotation: glam::Vec3::new(0.0, 0.0, 0.0),
            
            draw_spline: true,
            start_pos: glam::Vec3::new(-4.0, 1.0, -2.0),
            start_tangent: glam::Vec3::new(-10.0, -8.0, 0.0),
            end_pos: glam::Vec3::new(4.0, 2.0, -2.0),
            end_tangent: glam::Vec3::new(-6.0, 5.0, -6.0),
            
        }
    }
    
    pub fn reset_animation(&mut self){
        self.interpolation = 0.0;
        self.draw_world_coordinates = true;
        self.draw_model_coordinates = true;
        self.start_rotation = glam::Vec3::new(0.0, 0.0, 0.0);
        self.end_rotation = glam::Vec3::new(0.0, 0.0, 0.0);
        self.draw_spline = true;
        self.start_pos = glam::Vec3::new(-4.0, 1.0, -2.0);
        self.start_tangent = glam::Vec3::new(-10.0, -8.0, 0.0);
        self.end_pos = glam::Vec3::new(4.0, 2.0, -2.0);
        self.end_tangent = glam::Vec3::new(-6.0, 5.0, -6.0);
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