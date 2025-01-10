use crate::arrow_renderer::ArrowInstance;
use crate::camera::Camera;
use egui_winit::winit::dpi::PhysicalPosition;
use glam::{EulerRot, Mat4, Quat};
use crate::hermite_spline::hermite_spline;

pub struct UserDomain {
    pub mouse_locked: bool,
    pub mouse_pos: PhysicalPosition<f64>,
    pub rd_frame_time: f64,
    pub current_fps: f64,
    pub camera: Camera,

    pub arrow3d: Vec<Mat4>,
    pub reload_arrow: bool,

    pub draw_world_coordinates: bool,
    pub draw_model_coordinates: bool,

    pub interpolation: f32,
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

    pub fn reset_animation(&mut self) {
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

    pub fn calculate_arrow(&mut self) {
        let mut arrow3d: Vec<Mat4> = Vec::new();
        if self.draw_world_coordinates {
            arrow3d.push(Mat4::IDENTITY);
        }
        if self.draw_model_coordinates {
            arrow3d.push(Mat4::from_rotation_translation(
                Quat::from_euler(
                    EulerRot::ZYX,
                    self.start_rotation.z.to_radians(),
                    self.start_rotation.y.to_radians(),
                    self.start_rotation.x.to_radians(),
                ),
                self.start_pos,
            ));

            arrow3d.push(Mat4::from_rotation_translation(
                Quat::from_euler(
                    EulerRot::ZYX,
                    self.end_rotation.z.to_radians(),
                    self.end_rotation.y.to_radians(),
                    self.end_rotation.x.to_radians(),
                ),
                self.end_pos,
            ));
        }

        if (self.arrow3d == arrow3d) {
            self.reload_arrow = false;
        } else {
            self.arrow3d = arrow3d;
            self.reload_arrow = true;
        }
    }

    pub fn calculate_model_matrix(&self) -> Mat4 {
        let start_rotaton =  Quat::from_euler(
            EulerRot::ZYX,
            self.start_rotation.z.to_radians(),
            self.start_rotation.y.to_radians(),
            self.start_rotation.x.to_radians(),
        );
        let end_rotaton =  Quat::from_euler(
            EulerRot::ZYX,
            self.end_rotation.z.to_radians(),
            self.end_rotation.y.to_radians(),
            self.end_rotation.x.to_radians(),
        );

        let rotation = start_rotaton.slerp(end_rotaton, self.interpolation);
        
        let pos = hermite_spline(self.interpolation, self.start_pos, self.start_tangent, self.end_tangent, self.end_pos);
        
        Mat4::from_rotation_translation(rotation, pos)
    }

    pub fn load_arrow(&mut self) -> Vec<ArrowInstance> {
        let mut arrows = Vec::new();
        for arrow in self.arrow3d.iter() {
            arrows.push(ArrowInstance {
                model: *arrow * Mat4::IDENTITY,
                color: glam::Vec4::new(0.0, 1.0, 0.0, 1.0),
            });
            arrows.push(ArrowInstance {
                model: *arrow
                    * Mat4::from_rotation_translation(
                        glam::Quat::from_rotation_x(std::f32::consts::PI / 2.0),
                        glam::Vec3::new(0.0, 0.0, 0.0),
                    ),
                color: glam::Vec4::new(0.0, 0.0, 1.0, 1.0),
            });
            arrows.push(ArrowInstance {
                model: *arrow
                    * Mat4::from_rotation_translation(
                        glam::Quat::from_rotation_z(-std::f32::consts::PI / 2.0),
                        glam::Vec3::new(0.0, 0.0, 0.0),
                    ),
                color: glam::Vec4::new(1.0, 0.0, 0.0, 1.0),
            });
        }

        self.reload_arrow = false;
        arrows
    }

    pub fn load_line(&self) -> Vec<ArrowInstance> {
        let mut arrows = Vec::new();
        arrows.push(ArrowInstance {
            model: Mat4::from_scale(glam::Vec3::new(1.0, 5.0, 1.00)),
            color: glam::Vec4::new(1.0, 1.0, 1.0, 1.0),
        });
        arrows
    }
}
