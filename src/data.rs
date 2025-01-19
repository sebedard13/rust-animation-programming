use crate::basic_object::renderer::BasicObjectInstance;
use crate::camera::Camera;
use crate::hermite_spline::hermite_spline;
use egui_winit::winit::dpi::PhysicalPosition;
use glam::{EulerRot, Mat4, Quat, Vec3};

pub struct UserDomain {
    pub mouse_locked: bool,
    pub mouse_pos: PhysicalPosition<f64>,
    pub rd_frame_time: f64,
    pub current_fps: f64,
    pub camera: Camera,

    pub arrow3d: Vec<Mat4>,
    pub lines: Vec<BasicObjectInstance>,

    pub draw_world_coordinates: bool,
    pub draw_model_coordinates: bool,

    pub interpolation: f32,
    pub start_rotation: Vec3,
    pub end_rotation: Vec3,

    pub draw_spline: bool,

    pub start_pos: Vec3,
    pub start_tangent: Vec3,
    pub end_pos: Vec3,
    pub end_tangent: Vec3,
    
    pub scale: f32,

    pub light_pos: Vec3,
    pub light_color: Vec3,
}

impl UserDomain {
    pub(crate) fn new() -> Self {
        Self {
            mouse_locked: false,
            mouse_pos: PhysicalPosition::new(0.0, 0.0),
            rd_frame_time: 0.0,
            current_fps: 0.0,
            camera: Camera::new(),
            arrow3d: Vec::new(),
            lines: Vec::new(),

            interpolation: 0.0,

            draw_world_coordinates: true,
            draw_model_coordinates: true,
            start_rotation: Vec3::new(0.0, 0.0, 0.0),
            end_rotation: Vec3::new(0.0, 0.0, 0.0),

            draw_spline: true,
            start_pos: Vec3::new(-4.0, 1.0, -2.0),
            start_tangent: Vec3::new(-10.0, -8.0, 8.0),
            end_pos: Vec3::new(4.0, 2.0, -2.0),
            end_tangent: Vec3::new(-6.0, 5.0, -6.0),
            
            scale: 1.0,

            light_pos: Vec3::new(4.0, 5.0, -3.0),
            light_color: Vec3::new(0.5, 0.5, 0.5),
        }
    }

    pub fn reset_animation(&mut self) {
        self.interpolation = 0.0;
        self.draw_world_coordinates = true;
        self.draw_model_coordinates = true;
        self.start_rotation = Vec3::new(0.0, 0.0, 0.0);
        self.end_rotation = Vec3::new(0.0, 0.0, 0.0);
        self.draw_spline = true;
        self.start_pos = Vec3::new(-4.0, 1.0, -2.0);
        self.start_tangent = Vec3::new(-10.0, -8.0, 8.0);
        self.end_pos = Vec3::new(4.0, 2.0, -2.0);
        self.end_tangent = Vec3::new(-6.0, 5.0, -6.0);
        
        self.scale = 1.0;
        
        self.light_pos = Vec3::new(4.0, 5.0, -3.0);
        self.light_color = Vec3::new(0.5, 0.5, 0.5);
    }

    pub(crate) fn save_mouse_pos(&mut self, pos: &PhysicalPosition<f64>) {
        self.mouse_pos = pos.clone();
    }

    pub fn calculate_arrow(&mut self) -> bool {
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

        if self.arrow3d == arrow3d {
            false
        } else {
            self.arrow3d = arrow3d;
            true
        }
    }

    pub fn calculate_model_matrix(&self) -> Mat4 {
        let start_rotaton = Quat::from_euler(
            EulerRot::ZYX,
            self.start_rotation.z.to_radians(),
            self.start_rotation.y.to_radians(),
            self.start_rotation.x.to_radians(),
        );
        let end_rotaton = Quat::from_euler(
            EulerRot::ZYX,
            self.end_rotation.z.to_radians(),
            self.end_rotation.y.to_radians(),
            self.end_rotation.x.to_radians(),
        );

        let rotation = start_rotaton.slerp(end_rotaton, self.interpolation);

        let pos = hermite_spline(
            self.interpolation,
            self.start_pos,
            self.start_tangent,
            self.end_tangent,
            self.end_pos,
        );

        Mat4::from_scale_rotation_translation(Vec3::new(self.scale,self.scale,self.scale),rotation, pos)
    }

    pub fn load_arrow(&mut self) -> Vec<BasicObjectInstance> {
        let mut arrows = Vec::new();
        for arrow in self.arrow3d.iter() {
            arrows.push(BasicObjectInstance {
                model: *arrow * Mat4::IDENTITY,
                color: glam::Vec4::new(0.0, 1.0, 0.0, 1.0),
            });
            arrows.push(BasicObjectInstance {
                model: *arrow
                    * Mat4::from_rotation_translation(
                        Quat::from_rotation_x(std::f32::consts::PI / 2.0),
                        Vec3::new(0.0, 0.0, 0.0),
                    ),
                color: glam::Vec4::new(0.0, 0.0, 1.0, 1.0),
            });
            arrows.push(BasicObjectInstance {
                model: *arrow
                    * Mat4::from_rotation_translation(
                        Quat::from_rotation_z(-std::f32::consts::PI / 2.0),
                        Vec3::new(0.0, 0.0, 0.0),
                    ),
                color: glam::Vec4::new(1.0, 0.0, 0.0, 1.0),
            });
        }

        arrows
    }

    pub fn calculate_line(&mut self) -> bool {
        let l1 = Self::create_line_mat_instance(self.start_pos, self.start_tangent+ self.start_pos);
        let l2 = Self::create_line_mat_instance(self.end_pos, self.end_tangent+ self.end_pos);

        let mut lines = vec![
            BasicObjectInstance {
                model: l1,
                color: glam::Vec4::new(0.0, 0.0, 0.0, 1.0),
            },
            BasicObjectInstance {
                model: l2,
                color: glam::Vec4::new(0.0, 0.0, 0.0, 1.0),
            },
        ];
        let nb_lines_for_spline = 25;

        for i in 0..=(nb_lines_for_spline - 1) {
            let t0 = i as f32 / (nb_lines_for_spline as f32);
            let t1 = (i + 1) as f32 / (nb_lines_for_spline as f32);
            let p0 = hermite_spline(
                t0,
                self.start_pos,
                self.start_tangent,
                self.end_tangent,
                self.end_pos,
            );
            let p1 = hermite_spline(
                t1,
                self.start_pos,
                self.start_tangent,
                self.end_tangent,
                self.end_pos,
            );
            let l = Self::create_line_mat_instance(p0, p1);
            lines.push(BasicObjectInstance {
                model: l,
                color: glam::Vec4::new(1.0, 1.0, 1.0, 1.0),
            });
        }

        if self.lines == lines {
            false
        } else {
            self.lines = lines;
            true
        }
    }

    pub fn load_line(&self) -> Vec<BasicObjectInstance> {
        self.lines.clone()
    }

    pub fn create_line_mat_instance(from: Vec3, to: Vec3) -> Mat4 {
        let dir = to - from;
        let len = dir.length();
        let dir = dir.normalize();
        let up = Vec3::NEG_Y;
        let axis = dir.cross(up).normalize();
        let angle = dir.dot(up).acos();
        let rotation =
            Mat4::from_rotation_translation(Quat::from_axis_angle(axis, angle), Vec3::ZERO);
        let scale = Mat4::from_scale(Vec3::new(1.0, len, 1.0));
        let translation = Mat4::from_translation(from);
        let flip_y = Mat4::from_scale(Vec3::new(1.0, -1.0, 1.0));
        translation * flip_y * rotation * scale
    }
}
