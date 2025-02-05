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

use std::sync::Arc;
use crate::state::State;
use anyhow::Context;
use anyhow::Result;
use egui_winit::winit::dpi::{PhysicalSize, Size};
use egui_winit::winit::window::{Window, WindowAttributes, WindowId};
use egui_winit::winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
};
use egui_winit::winit::application::ApplicationHandler;
use egui_winit::winit::event_loop::{ActiveEventLoop, ControlFlow};
use log::{error, warn};

#[derive(Default)]
pub struct App {
    pub window: Option<Arc<Window>>,
    pub state: Option<State>,
}
impl ApplicationHandler for App  {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = WindowAttributes::default()
            .with_title("Rust animation programming")
            .with_inner_size(Size::Physical(PhysicalSize::new(800, 600)));
        match event_loop.create_window(window_attributes){
            Ok(window) => {
                let rc_window = Arc::new(window);
                self.window = Some(rc_window.clone());
                let state = pollster::block_on(State::new(rc_window));
                self.state = Some(state);
            }
            Err(_) => {}
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        let (window, state) = match (&self.window, &mut self.state) {
            (Some(window), Some(state)) => (window, state),
            _ => return,
        };
        
        if window.id() != window_id {
            return;
        }
        
        if !state.input(&event) {
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
                } => event_loop.exit(),
                WindowEvent::RedrawRequested => {
                    let elapsed = state.time_prev.elapsed();
                    state.time_prev = std::time::Instant::now();
                    state.update(elapsed);
                    
                    match state.render() {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                            state.resize(state.size)
                        }
                        Err(wgpu::SurfaceError::OutOfMemory) => {
                            error!("OutOfMemory");
                            event_loop.exit();
                        }
                        Err(wgpu::SurfaceError::Timeout) => {
                            warn!("Surface timeout")
                        }
                    }
                }
                WindowEvent::Resized(physical_size) => {
                    state.resize(physical_size);
                }
                _ => {}
            }
        } else {
           window.request_redraw()
        }
    }

    fn device_event(&mut self, _event_loop: &ActiveEventLoop, _device_id: DeviceId, event: DeviceEvent) {
        let (window, state) = match (&self.window, &mut self.state) {
            (Some(window), Some(state)) => (window, state),
            _ => return,
        };
        if state.raw_input(&event) {
            window.request_redraw()
        }
    }
}

fn main() -> Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .filter(Some("wgpu_core"), log::LevelFilter::Warn)
        .init();
    let event_loop = EventLoop::new().context("Error creating the event loop")?;
    event_loop.set_control_flow(ControlFlow::Poll);
   
    let mut app = App::default();
    event_loop.run_app(&mut app).context("Error running the app")?;
    
    Ok(())
}
