mod state;

use crate::state::State;
use anyhow::Context;
use anyhow::Result;
use log::{info, log, warn};
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
                        WindowEvent::KeyboardInput {
                            event: key_event, ..
                        } => {
                            handle_key_event(&key_event);
                        }
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        }
                        _ => {}
                    }
                };
            }
            Event::DeviceEvent { ref event, .. } => match event {
                DeviceEvent::MouseMotion { delta } => {
                    info!("Mouse motion: {:?}", delta);
                }
                DeviceEvent::Button { button, state } => {
                    info!("Mouse button: {:?} {:?}", button, state);
                }
                _ => {}
            },
            _ => {}
        })
        .context("Error in event loop")?;
    Ok(())
}

fn handle_key_event(event: &KeyEvent) {
    info!("Key event: {:?}", event);
}

fn main() -> Result<()> {
    pollster::block_on(run())
}
