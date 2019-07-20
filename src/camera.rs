//! Camera

use crate::{glm, input::InputMap};
use winit::*;

#[derive(Debug, Clone)]
pub struct Camera {
    move_speed: f32,
    rotation_speed: f32,
    position: glm::Vec3,
    rotation: glm::Vec3,
}

impl Camera {
    pub fn new(position: glm::Vec3, rotation: glm::Vec3) -> Self {
        Camera { position, rotation, move_speed: 1.0, rotation_speed: 1.0 }
    }

    pub fn set_move_speed(&mut self, speed: f32) {
        self.move_speed = speed;
    }

    pub fn set_rotation_speed(&mut self, speed: f32) {
        self.rotation_speed = speed;
    }

    pub fn update(&mut self, dt: f32, input: &InputMap) {
        let mut direction = glm::vec3(0.0, 0.0, 0.0);

        if input.is_pressed(VirtualKeyCode::W) {
            direction.z -= 1.0;
        }
        if input.is_pressed(VirtualKeyCode::S) {
            direction.z += 1.0;
        }
        if input.is_pressed(VirtualKeyCode::D) {
            direction.x += 1.0;
        }
        if input.is_pressed(VirtualKeyCode::A) {
            direction.x -= 1.0;
        }

        let rotation = input.mouse_delta();

        self.position += self.view_dir(direction) * self.move_speed * dt;
        self.rotation.x += rotation[1] * self.rotation_speed * 0.001;
        self.rotation.y += rotation[0] * self.rotation_speed * 0.001;
    }

    pub fn view_dir(&self, direction: glm::Vec3) -> glm::Vec3 {
        glm::rotate_z_vec3(
            &glm::rotate_y_vec3(
                &glm::rotate_x_vec3(&direction, self.rotation.x),
                self.rotation.y,
            ),
            self.rotation.z,
        )
    }

    pub fn view(&self) -> glm::Mat4 {
        glm::look_at(
            &self.position,
            &(self.position + self.view_dir(glm::vec3(0.0, 0.0, -1.0))),
            &glm::vec3(0.0, 1.0, 0.0),
        )
    }
}
