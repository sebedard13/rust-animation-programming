use glam::{Mat4, Vec3};

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Mat4 = Mat4::from_cols_array(&[
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0]
);

pub struct Camera {
    pub position: Vec3,
    pub view_direction: Vec3,
    pub up: Vec3,

    pub view_azimuth: f64,
    pub view_elevation: f64,

    pub move_front_back: f32,
    pub move_left_right: f32,
    pub move_up_down: f32,

    pub fovy: f32,
    znear: f32,
    zfar: f32,
    pub aspect: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: Vec3::new(0.0, 4.0, 6.0),
            view_direction: Vec3::ZERO,
            up: Vec3::Y,
            view_azimuth: 0.0,
            view_elevation: -26.56,

            fovy: 90.0,
            znear: 0.1,
            zfar: 100.0,

            move_front_back: 0.0,
            move_left_right: 0.0,
            move_up_down: 0.0,
            aspect: 1.0,
        }
    }

    pub fn update_vectors(&mut self) {
        if self.view_azimuth > 360.0 {
            self.view_azimuth -= 360.0;
        } else if self.view_azimuth < 0.0 {
            self.view_azimuth += 360.0;
        }

        if self.view_elevation > 89.0 {
            self.view_elevation = 89.0;
        } else if self.view_elevation < -89.0 {
            self.view_elevation = -89.0;
        }

        let azimuth = self.view_azimuth.to_radians();
        let elevation = self.view_elevation.to_radians();

        let cos_azimuth = azimuth.cos();
        let sin_azimuth = azimuth.sin();
        let cos_elevation = elevation.cos();
        let sin_elevation = elevation.sin();

        self.view_direction = Vec3::new(
            (sin_azimuth * cos_elevation) as f32,
            sin_elevation as f32,
            (-cos_azimuth * cos_elevation) as f32,
        )
        .normalize();
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        let proj = Mat4::perspective_rh(self.fovy.to_radians(), self.aspect, self.znear, self.zfar);
        OPENGL_TO_WGPU_MATRIX * proj * Mat4::look_at_rh(self.position, self.position + self.view_direction, self.up)
    }

    const MOVE_SPEED: f32 = 0.1;

    pub fn move_update(&mut self) {
        if self.move_front_back != 0.0 {
            self.position += self.view_direction * self.move_front_back * Self::MOVE_SPEED;
        }
        if self.move_left_right != 0.0 {
            self.position += self.view_direction.cross(self.up).normalize() * self.move_left_right * Self::MOVE_SPEED;
        }
        if self.move_up_down != 0.0 {
            self.position += self.up * self.move_up_down * Self::MOVE_SPEED;
        }
    }

    pub fn reset(&mut self) {
        self.position = Vec3::new(0.0, 1.0, 2.0);
        self.view_azimuth = 0.0;
        self.view_elevation = -26.56;
        self.update_vectors();
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraMatBuffer {
    pub mat: [[f32; 4]; 4],
}

impl CameraMatBuffer {
    pub fn new() -> Self {
        Self {
            mat: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    pub fn update(&mut self, camera: &Camera) {
        self.mat = camera.get_view_matrix().to_cols_array_2d();
    }
}
