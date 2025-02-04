mod camera;
mod data;
mod gui;
mod state;
mod texture;
mod vertex;
mod model;
mod hermite_spline;
mod color;
mod basic_object;
mod light;
mod utils_glam;

use crate::state::State;
use anyhow::Context;
use anyhow::Result;
use egui_winit::winit::dpi::{PhysicalSize, Size};
use egui_winit::winit::window::WindowAttributes;
use egui_winit::winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
};
use log::{error, warn};

use egui_wgpu::wgpu;

#[allow(deprecated)]
pub async fn run() -> Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .filter(Some("wgpu_core"), log::LevelFilter::Warn)
        .init();
    let event_loop = EventLoop::new().context("Error creating the event loop")?;
    let window_attributes = WindowAttributes::default()
        .with_title("Rust animation programming")
        .with_inner_size(Size::Physical(PhysicalSize::new(800, 600)));
    let window = event_loop.create_window(window_attributes)?;

    let mut state = State::new(&window).await;

    let mut time_prev = std::time::Instant::now();

   
    event_loop
        .run(|event, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !state.input(event) {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            event:
                                KeyEvent {
                                    state: ElementState::Pressed,
                                    physical_key: PhysicalKey::Code(KeyCode::Escape),
                                    ..
                                },
                            ..
                        } => control_flow.exit(),
                        WindowEvent::RedrawRequested => {
                            match state.render() {
                                Ok(_) => {}
                                // Reconfigure the surface if it's lost or outdated
                                Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                                    state.resize(state.size)
                                }
                                // The system is out of memory, we should probably quit
                                Err(wgpu::SurfaceError::OutOfMemory) => {
                                    error!("OutOfMemory");
                                    control_flow.exit();
                                }

                                // This happens when the a frame takes too long to present
                                Err(wgpu::SurfaceError::Timeout) => {
                                    warn!("Surface timeout")
                                }
                            }
                            let frame_duration = std::time::Instant::now() - time_prev;
                            time_prev = std::time::Instant::now();
                            state.data.rd_frame_time = frame_duration.as_secs_f64();
                        }
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        }
                        _ => {}
                    }
                } else {
                    window.request_redraw();
                }
            }
            Event::DeviceEvent { ref event, .. } => {
                if state.raw_input(&event) {
                    window.request_redraw();
                }
            },
            _ => {}
        })
        .context("Error in event loop")?;
    Ok(())
}
fn main() -> Result<()> {
    pollster::block_on(run())
}
