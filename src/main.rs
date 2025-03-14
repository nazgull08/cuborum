use tracing::{info};
use tracing_subscriber::{fmt, EnvFilter};
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

mod renderer;
use renderer::state::State;

#[derive(Default)]
struct App {
    state: Option<State>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop.create_window(Window::default_attributes()).unwrap(),
        );

        let state = pollster::block_on(State::new(window.clone()));
        self.state = Some(state);

        window.request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        if let Some(state) = &mut self.state {
            match event {
                WindowEvent::CloseRequested => {
                    info!("Close requested, exiting.");
                    event_loop.exit();
                }
                WindowEvent::RedrawRequested => {
                    state.render();
                    state.get_window().request_redraw();
                }
                WindowEvent::Resized(size) => {
                    state.resize(size);
                }
                _ => (),
            }
        }
    }
}

fn main() {
    fmt().with_env_filter(EnvFilter::from_default_env()).init();
    info!("Cuborum MVP started.");

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
