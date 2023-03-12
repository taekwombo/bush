use gluty::winit::dpi::{PhysicalPosition, PhysicalSize};
use gluty::winit::event::*;
use std::time::Instant;

#[derive(Debug)]
pub enum MovementAxis {
    X,
    Z,
}

pub struct InputState {
    pub movement_timestamp: Option<Instant>,
    pub cursor_position: Option<PhysicalPosition<f32>>,
    pub mouse: Option<MouseButton>,
    pub size: PhysicalSize<f32>,
}

impl InputState {
    pub fn new(size: PhysicalSize<u32>) -> Self {
        Self {
            size: PhysicalSize::new(size.width as f32, size.height as f32),
            movement_timestamp: None,
            cursor_position: None,
            mouse: None,
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

    pub fn key_release(&mut self) {
        self.movement_timestamp = None;
    }

    pub fn key_movement(&mut self, keycode: &VirtualKeyCode) -> Option<(MovementAxis, f32)> {
        if self.movement_timestamp.is_none() {
            self.movement_timestamp.replace(Instant::now());
            return None;
        }

        let elapsed = self
            .movement_timestamp
            .as_ref()
            .unwrap()
            .elapsed()
            .as_secs_f32();
        self.movement_timestamp.replace(Instant::now());

        Some(match keycode {
            VirtualKeyCode::W => (MovementAxis::Z, elapsed * 1.0),
            VirtualKeyCode::S => (MovementAxis::Z, elapsed * -1.0),
            VirtualKeyCode::A => (MovementAxis::X, elapsed * -1.0),
            VirtualKeyCode::D => (MovementAxis::X, elapsed * 1.0),
            _ => unreachable!(),
        })
    }
}
