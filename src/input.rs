//! Input Handling

use std::collections::HashMap;
use winit::{ElementState, VirtualKeyCode};

pub struct InputMap {
    keys: HashMap<VirtualKeyCode, ElementState>,
    mouse1: ElementState,
    mouse_delta: glm::Vec2,
}

impl InputMap {
    pub fn new() -> Self {
        InputMap {
            keys: HashMap::new(),
            mouse1: ElementState::Released,
            mouse_delta: glm::vec2(0.0, 0.0),
        }
    }

    pub fn update_keys(&mut self, key: VirtualKeyCode, state: ElementState) {
        self.keys.insert(key, state);
    }

    pub fn update_mouse1(&mut self, state: ElementState) {
        self.mouse1 = state;
    }

    pub fn update_mouse_motion(&mut self, (dx, dy): (f64, f64)) {
        if self.mouse1 == ElementState::Pressed {
            self.mouse_delta[0] += dx as f32;
            self.mouse_delta[1] += dy as f32;
        }
    }

    pub fn is_pressed(&self, key: VirtualKeyCode) -> bool {
        self.keys.get(&key) == Some(&ElementState::Pressed)
    }

    pub fn mouse_delta(&self) -> glm::Vec2 {
        self.mouse_delta
    }

    pub fn reset_delta(&mut self) {
        self.mouse_delta = glm::vec2(0.0, 0.0);
    }
}
