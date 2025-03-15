use crate::renderer::state::State;
use winit::event::WindowEvent;

pub fn process_input(state: &mut State, event: &WindowEvent) {
    match event {
        WindowEvent::KeyboardInput { event, .. } => {
            match event.state {
                winit::event::ElementState::Pressed => {
                    state.camera.pressed_keys.insert(event.logical_key.clone());
                }
                winit::event::ElementState::Released => {
                    state.camera.pressed_keys.remove(&event.logical_key);
                }
            }
        }
        WindowEvent::CursorMoved { position, .. } => {
            state.camera.process_mouse(position.x as f32, position.y as f32);
        }
        _ => (),
    }
}
