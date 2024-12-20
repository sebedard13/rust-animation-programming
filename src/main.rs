mod state;

use crate::state::State;
use anyhow::Context;
use anyhow::Result;
use log::{error, info, log, warn};
use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

pub async fn run() -> Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    let event_loop = EventLoop::new().context("Error creating the event loop")?;
    let window = WindowBuilder::new()
        .build(&event_loop)
        .context("Error creating the window")?;

    let mut state = State::new(&window).await;

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
                            state.window().request_redraw();

                            state.update();
                            match state.render() {
                                Ok(_) => {}
                                // Reconfigure the surface if it's lost or outdated
                                Err(
                                    wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated,
                                ) => state.resize(state.size),
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

                        }
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        }
                        _ => {}
                    }
                };
            }
            _ => {}
        })
        .context("Error in event loop")?;
    Ok(())
}
fn main() -> Result<()> {
    pollster::block_on(run())
}
