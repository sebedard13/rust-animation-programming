use egui::{Align2, Context};
use crate::data::ColorSelectorData;

pub fn gui(data: &mut ColorSelectorData, ui: &Context) {
    egui::Window::new("Infos")
        .default_open(true)
        .default_width(200.0)
        .max_height(800.0)
        .default_width(800.0)
        .resizable(true)
        .movable(true)
        .anchor(Align2::LEFT_TOP, [2.0, 2.0])
        .show(&ui, |ui| {
            let mut new_fps = 0.0;
            if data.rd_frame_time > 0.0 {
                new_fps = 1.0 / data.rd_frame_time;
            }
            data.current_fps = (0.98 *  data.current_fps) +
                (1.0 - 0.98) * new_fps;

            ui.label(format!("FPS {:.2}", data.current_fps));

            ui.separator();
            ui.label("Camera");
            ui.add(egui::Slider::new(&mut data.camera.fovy, 30.0..=120.0).text("Fov"));
            ui.label(format!("Azimuth: {:.2}", data.camera.view_azimuth));
            ui.label(format!("Elevation: {:.2}", data.camera.view_elevation));
            ui.label(format!("Position: ({:.2}, {:.2}, {:.2})", data.camera.position.x, data.camera.position.y, data.camera.position.z));
        });
}