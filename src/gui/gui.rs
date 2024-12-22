use egui::{Align2, Context};
use crate::data::ColorSelectorData;

pub fn GUI(data: &mut ColorSelectorData, ui: &Context) {
    egui::Window::new("Infos")
        .default_open(true)
        .max_width(1000.0)
        .max_height(800.0)
        .default_width(800.0)
        .resizable(true)
        .movable(true)
        .anchor(Align2::LEFT_TOP, [2.0, 2.0])
        .show(&ui, |mut ui| {
            if ui.button("Toggle Texture (Space)").clicked() {
                data.toggle_texture = !data.toggle_texture
            }
            ui.separator();
            ui.label(format!("Bg Color (r:{:.2}, g:{:.2}, b:{:.2})", data.color.r,  data.color.b, data.color.g));
        });
}