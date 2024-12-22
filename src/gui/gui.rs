use egui::{Align2, Context};
use crate::data::ColorSelectorData;

pub fn GUI(data: &mut ColorSelectorData, ui: &Context) {
    egui::Window::new("Infos")
        .default_open(true)
        .default_width(200.0)
        .max_height(800.0)
        .default_width(800.0)
        .resizable(true)
        .movable(true)
        .anchor(Align2::LEFT_TOP, [2.0, 2.0])
        .show(&ui, |mut ui| {
            let mut newFPS = 0.0;
            if data.rd_frame_time > 0.0 {
                newFPS = 1.0 / data.rd_frame_time;
            }
            data.current_fps = (0.98 *  data.current_fps) +
                (1.0 - 0.98) * newFPS;

            ui.label(format!("FPS {:.2}", data.current_fps));
            ui.separator();
            if ui.button("Toggle Texture (Space)").clicked() {
                data.toggle_texture = !data.toggle_texture
            }
            ui.separator();
            ui.label(format!("Bg Color (r:{:.2}, g:{:.2}, b:{:.2})", data.color.r,  data.color.b, data.color.g));
        });
}