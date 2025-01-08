use egui::{Align2, Context};
use crate::data::UserDomain;

pub fn gui(user_domain: &mut UserDomain, ui: &Context) {
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
            if user_domain.rd_frame_time > 0.0 {
                new_fps = 1.0 / user_domain.rd_frame_time;
            }
            user_domain.current_fps = (0.98 *  user_domain.current_fps) +
                (1.0 - 0.98) * new_fps;

            ui.label(format!("FPS {:.2}", user_domain.current_fps));

            ui.separator();
            ui.label("Camera");
            ui.add(egui::Slider::new(&mut user_domain.camera.fovy, 30.0..=120.0).text("Fov"));
            ui.label(format!("Azimuth: {:.2}", user_domain.camera.view_azimuth));
            ui.label(format!("Elevation: {:.2}", user_domain.camera.view_elevation));
            ui.label(format!("Position: ({:.2}, {:.2}, {:.2})", user_domain.camera.position.x, user_domain.camera.position.y, user_domain.camera.position.z));
        });
}