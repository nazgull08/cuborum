use tracing::info;
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

use crate::renderer::state::State;

struct App {
    state: Option<State>,
}

impl App {
    async fn new(window: Arc<Window>) -> Self {
        let state = State::new(window.clone()).await;
        Self {
            state: Some(state),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.state.is_none() {
            let window = Arc::new(
                event_loop.create_window(Window::default_attributes()).unwrap(),
            );

            let state = pollster::block_on(State::new(window.clone()));
            self.state = Some(state);

            window.request_redraw();
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        if let Some(state) = &mut self.state {
            state.input(&event);
            match event {
                WindowEvent::CloseRequested => {
                    tracing::info!("Close requested, exiting.");
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

pub async fn run() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);

    let window = Arc::new(
        event_loop
            .create_window(Window::default_attributes().with_title("Cuborum MVP"))
            .unwrap(),
    );

    let mut app = App::new(window).await;
    event_loop.run_app(&mut app).unwrap();
}
