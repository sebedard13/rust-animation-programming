use crate::data::UserDomain;
use egui::{Align2, ComboBox, Context, Slider};

pub fn gui(user_domain: &mut UserDomain, ui: &Context) {
    egui::Window::new("Infos")
        .default_open(true)
        .default_width(200.0)
        .max_height(800.0)
        .default_width(500.0)
        .resizable(true)
        .movable(true)
        .anchor(Align2::LEFT_TOP, [2.0, 2.0])
        .show(&ui, |ui| {
            ui.label(format!("FPS {:.2}", user_domain.current_fps));

            ui.separator();
            ui.collapsing("Camera", |ui| {
                ui.add(Slider::new(&mut user_domain.camera.fovy, 30.0..=120.0).text("Fov"));
                ui.label(format!("Azimuth: {:.2}", user_domain.camera.view_azimuth));
                ui.label(format!("Elevation: {:.2}", user_domain.camera.view_elevation));
                ui.label(format!(
                    "Position: ({:.2}, {:.2}, {:.2})",
                    user_domain.camera.position.x, user_domain.camera.position.y, user_domain.camera.position.z
                ));
            });

            ui.separator();
            ui.collapsing("Model", |ui| {
                ui.checkbox(&mut user_domain.draw_world_coordinates, "Draw World Coordinates");
                ui.checkbox(&mut user_domain.draw_model_coordinates, "Draw Model Coordinates");
                ui.add(
                    Slider::new(&mut user_domain.scale, 0.001..=100.0)
                        .text("Scale")
                        .step_by(0.01),
                );
                if ui.button("Reset Animation").clicked() {
                    user_domain.reset_animation();
                }
                ui.label("Start rotation");
                ui.horizontal(|ui| {
                    ui.add(Slider::new(&mut user_domain.start_rotation.x, -180.0..=180.0));
                    ui.add(Slider::new(&mut user_domain.start_rotation.y, -180.0..=180.0));
                    ui.add(Slider::new(&mut user_domain.start_rotation.z, -180.0..=180.0));
                });
                ui.label("End rotation");
                ui.horizontal(|ui| {
                    ui.add(Slider::new(&mut user_domain.end_rotation.x, -180.0..=180.0));
                    ui.add(Slider::new(&mut user_domain.end_rotation.y, -180.0..=180.0));
                    ui.add(Slider::new(&mut user_domain.end_rotation.z, -180.0..=180.0));
                });

                ui.checkbox(&mut user_domain.draw_spline, "Draw Spline");
                ui.label("Start pos");
                ui.horizontal(|ui| {
                    ui.add(Slider::new(&mut user_domain.start_pos.x, -10.0..=10.0));
                    ui.add(Slider::new(&mut user_domain.start_pos.y, -10.0..=10.0));
                    ui.add(Slider::new(&mut user_domain.start_pos.z, -10.0..=10.0));
                });
                ui.label("Start tangent");
                ui.horizontal(|ui| {
                    ui.add(Slider::new(&mut user_domain.start_tangent.x, -10.0..=10.0));
                    ui.add(Slider::new(&mut user_domain.start_tangent.y, -10.0..=10.0));
                    ui.add(Slider::new(&mut user_domain.start_tangent.z, -10.0..=10.0));
                });
                ui.label("End pos");
                ui.horizontal(|ui| {
                    ui.add(Slider::new(&mut user_domain.end_pos.x, -10.0..=10.0));
                    ui.add(Slider::new(&mut user_domain.end_pos.y, -10.0..=10.0));
                    ui.add(Slider::new(&mut user_domain.end_pos.z, -10.0..=10.0));
                });
                ui.label("End tangent");
                ui.horizontal(|ui| {
                    ui.add(Slider::new(&mut user_domain.end_tangent.x, -10.0..=10.0));
                    ui.add(Slider::new(&mut user_domain.end_tangent.y, -10.0..=10.0));
                    ui.add(Slider::new(&mut user_domain.end_tangent.z, -10.0..=10.0));
                });
            });

            ui.collapsing("Animation", |ui| {
                ui.checkbox(&mut user_domain.double_quat_joints_render, "Double Quat Joints");

                ComboBox::from_label("Animation")
                    .selected_text(format!("{:?}", user_domain.animations[user_domain.selected_animation]))
                    .show_ui(ui, |ui| {
                        for i in 0..user_domain.animations.len() {
                            ui.selectable_value(&mut user_domain.selected_animation, i, &user_domain.animations[i]);
                        }
                    });

                ui.add(
                    Slider::new(&mut user_domain.speed, 0.0..=1.0)
                        .text("Speed")
                        .step_by(0.01),
                );
                ui.add(Slider::new(&mut user_domain.interpolation, 0.0..=1.0).text("Interpolation"));
                if ui.button("Reset Animation").clicked() {
                    user_domain.reset_animation();
                }
            });

            ui.collapsing("Light", |ui| {
                ui.label("Position");
                ui.horizontal(|ui| {
                    ui.add(Slider::new(&mut user_domain.light_pos.x, -10.0..=10.0));
                    ui.add(Slider::new(&mut user_domain.light_pos.y, -10.0..=10.0));
                    ui.add(Slider::new(&mut user_domain.light_pos.z, -10.0..=10.0));
                });
                ui.label("Color");
                ui.horizontal(|ui| {
                    ui.add(Slider::new(&mut user_domain.light_color.x, 0.0..=1.0));
                    ui.add(Slider::new(&mut user_domain.light_color.y, 0.0..=1.0));
                    ui.add(Slider::new(&mut user_domain.light_color.z, 0.0..=1.0));
                });
            });
        });
}
