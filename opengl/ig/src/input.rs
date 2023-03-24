use gluty::winit::dpi::{PhysicalPosition, PhysicalSize};
use gluty::winit::event::*;

#[derive(Debug)]
pub struct InputState {
    pub cursor_position: Option<PhysicalPosition<f32>>,
    pub mouse: Option<MouseButton>,
    pub size: PhysicalSize<f32>,
    pub alt: bool,
    pub ctrl: bool,
    pub shift: bool,
}

impl InputState {
    pub fn new(size: PhysicalSize<u32>) -> Self {
        Self {
            size: PhysicalSize::new(size.width as f32, size.height as f32),
            cursor_position: None,
            mouse: None,
            alt: false,
            ctrl: false,
            shift: false,
        }
    }

    pub fn mouse_click(&mut self, state: &ElementState, button: &MouseButton) {
        if *state == ElementState::Released {
            let _ = self.mouse.take();
            let _ = self.cursor_position.take();
            return;
        }

        match button {
            MouseButton::Left | MouseButton::Right => {
                self.mouse.replace(*button);
            }
            _ => (),
        };
    }

    pub fn cursor_move(&mut self, pos: &PhysicalPosition<f64>) -> Option<(f32, f32)> {
        if self.cursor_position.is_none() {
            self.cursor_position
                .replace(PhysicalPosition::new(pos.x as f32, pos.y as f32));

            return None;
        }

        let prev = self.cursor_position.as_ref().unwrap();
        let x = pos.x as f32;
        let y = pos.y as f32;

        let delta_x = (x - prev.x) / (self.size.width / 100.0);
        let delta_y = (y - prev.y) / (self.size.height / -100.0);

        self.cursor_position.replace(PhysicalPosition::new(x, y));

        Some((delta_x, delta_y))
    }

    pub fn modifiers(&mut self, state: &ElementState, keycode: &VirtualKeyCode) {
        match keycode {
            VirtualKeyCode::LControl | VirtualKeyCode::RControl => {
                self.ctrl = *state == ElementState::Pressed;
            }
            VirtualKeyCode::LAlt | VirtualKeyCode::RAlt => {
                self.alt = *state == ElementState::Pressed;
            }
            VirtualKeyCode::LShift | VirtualKeyCode::RShift => {
                self.shift = *state == ElementState::Pressed;
            }
            _ => (),
        }
    }
}
