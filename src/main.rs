use anyhow::Context;
use anyhow::Result;
use log::{info, log, warn};
use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

pub fn run() -> Result<()> {
    env_logger::builder().filter_level(log::LevelFilter::Info).init();
    let event_loop = EventLoop::new().context("Error creating the event loop")?;
    let window = WindowBuilder::new().build(&event_loop).context("Error creating the window")?;

    event_loop.run(move |event, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
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
                event: key_event,
                ..
            } => {
                handle_key_event(&key_event);
            }
            _ => {}
        },
        Event::DeviceEvent {
            ref event,
            ..
        } => {
            match event {
                DeviceEvent::MouseMotion {
                    delta,
                } => {
                    info!("Mouse motion: {:?}", delta);
                }
                DeviceEvent::Button {
                    button,
                    state,
                } => {
                    info!("Mouse button: {:?} {:?}", button, state);
                }
                _ => {}
            }
        }
        _ => {}
    }).context("Error in event loop")?;
    Ok(())
}

fn handle_key_event(event: &KeyEvent) {
    info!("Key event: {:?}", event);
}

fn main() -> Result<()> {
    run()
}
