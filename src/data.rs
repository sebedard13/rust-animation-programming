use egui_winit::winit::dpi::{PhysicalPosition, PhysicalSize};
use egui_wgpu::wgpu as wgpu;

pub struct ColorSelectorData {
    pub mouse_pos: PhysicalPosition<f64>,
    pub color: wgpu::Color,
    pub toggle_texture: bool,
    pub rd_frame_time: f64,
    pub current_fps: f64,
}

impl ColorSelectorData {
    pub(crate) fn new() -> Self {
        Self {
            mouse_pos: PhysicalPosition::new(0.0, 0.0),
            color: wgpu::Color::WHITE,
            toggle_texture: true,
            rd_frame_time: 0.0,
            current_fps: 0.0,
        }
    }

    pub(crate) fn save_mouse_pos(&mut self, pos: &PhysicalPosition<f64>) {
        self.mouse_pos = pos.clone();
    }

    pub(crate) fn calculate_color(&mut self, size: PhysicalSize<u32>) {
        let width = size.width as f64;
        let height = size.height as f64;

        let first_fifth = width / 5.0;
        let second_fifth = 2.0 * first_fifth;
        let third_fifth = 3.0 * first_fifth;
        let fourth_fifth = 4.0 * first_fifth;

        let saturation = ((height - self.mouse_pos.y) / height) * 2.0;

        let color = if self.mouse_pos.x < first_fifth {
            let mut color = wgpu::Color {
                r: 1.0,
                g: self.mouse_pos.x / first_fifth,
                b: 0.0,
                a: 1.0,
            };

            if saturation > 1.0 {
                color.b = (saturation - 1.0);
            } else {
                color.r = color.r * saturation;
                color.g = color.g * saturation;
            }
            color
        } else if self.mouse_pos.x < second_fifth {
            let mut color = wgpu::Color {
                r: 1.0 - (self.mouse_pos.x - first_fifth) / first_fifth,
                g: 1.0,
                b: 0.0,
                a: 1.0,
            };

            if saturation > 1.0 {
                color.b = (saturation - 1.0);
            } else {
                color.r = color.r * saturation;
                color.g = color.g * saturation;
            }
            color
        } else if self.mouse_pos.x < third_fifth {
            let mut color = wgpu::Color {
                r: 0.0,
                g: 1.0,
                b: (self.mouse_pos.x - second_fifth) / first_fifth,
                a: 1.0,
            };

            if saturation > 1.0 {
                color.r = (saturation - 1.0);
            } else {
                color.g = color.g * saturation;
                color.b = color.b * saturation;
            }
            color
        } else if self.mouse_pos.x < fourth_fifth {
            let mut color = wgpu::Color {
                r: 0.0,
                g: 1.0 - (self.mouse_pos.x - third_fifth) / first_fifth,
                b: 1.0,
                a: 1.0,
            };
            if saturation > 1.0 {
                color.r = (saturation - 1.0);
            } else {
                color.g = color.g * saturation;
                color.b = color.b * saturation;
            }
            color
        } else {
            let mut color = wgpu::Color {
                r: (self.mouse_pos.x - fourth_fifth) / first_fifth,
                g: 0.0,
                b: 1.0,
                a: 1.0,
            };
            if saturation > 1.0 {
                color.g = (saturation - 1.0);
            } else {
                color.r = color.r * saturation;
                color.b = color.b * saturation;
            }
            color
        };

        self.color = color;
    }

    fn get_color(&self) -> wgpu::Color {
        self.color
    }
}