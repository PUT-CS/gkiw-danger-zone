#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use self::Movement::*;
use cgmath;
use cgmath::frustum;
use cgmath::perspective;
use cgmath::prelude::*;
use cgmath::vec3;
use cgmath::Deg;
use cgmath::Quaternion;
type Point3 = cgmath::Point3<f32>;
type Vector3 = cgmath::Vector3<f32>;
type Matrix4 = cgmath::Matrix4<f32>;

#[derive(PartialEq, Clone, Copy)]
pub enum Movement {
    Forward,
    Backward,
    Left,
    Right,
    RollRight,
    RollLeft,
}

const SPEED: f32 = 25.;
const SENSITIVTY: f32 = 0.1;
const ZOOM: f32 = 45.0;

pub struct Camera {
    pub position: Point3,
    pub front: Vector3,
    pub up: Vector3,
    pub right: Vector3,
    pub view_matrix: Matrix4,
    pub projection_matrix: Matrix4,
    pub movement_speed: f32,
    pub mouse_sensitivity: f32,
    pub zoom: f32,
}

impl Default for Camera {
    fn default() -> Camera {
        Camera {
            position: Point3::new(0.0, 0.0, 0.0),
            front: vec3(0.0, 0.0, -1.0),
            up: Vector3::unit_y(),
            right: Vector3::unit_x(),
            view_matrix: Matrix4::from_axis_angle(Vector3::unit_z(), Deg(180.0)),
            projection_matrix: perspective(Deg(45.0), 1.0, 0.1, 100.0),
            movement_speed: SPEED,
            mouse_sensitivity: SENSITIVTY,
            zoom: ZOOM,
        }
    }
}

impl Camera {
    /// Returns the view matrix calculated using Eular Angles and the LookAt Matrix
    pub fn get_view_matrix(&self) -> Matrix4 {
        self.view_matrix
    }

    fn update_view_matrix(&mut self) {
        self.view_matrix = Matrix4::look_at(self.position, self.position + self.front, self.up);
        //self.projection_matrix = self.projection_matrix * self.view_matrix;
    }

    pub fn rotate(&mut self, rotation: Quaternion<f32>) {
        self.front = (rotation * self.front).normalize();
        self.right = (rotation * self.right).normalize();
        self.up = (rotation * self.up).normalize();
        self.update_view_matrix();
    }

    pub fn pitch(&mut self, amount: f32) {
        let rotation = Quaternion::from_axis_angle(self.right, Deg(amount));
        self.front = (rotation * self.front).normalize();
        if self.right.cross(self.front).dot(self.up) < 0.0 {
            self.up *= -1.0;
        }
        self.update_view_matrix();
    }

    pub fn yaw(&mut self, amount: f32) {
        let rotation = Quaternion::from_axis_angle(self.up, Deg(amount));
        self.front = (rotation * self.front).normalize();
        self.right = (rotation * self.right).normalize();
        self.update_view_matrix();
    }

    pub fn roll(&mut self, amount: f32) {
        let rotation = Quaternion::from_axis_angle(self.front, Deg(amount));
        self.right = (rotation * self.right).normalize();
        self.up = (rotation * self.up).normalize();
        self.update_view_matrix();
    }

    /// Processes input received from any keyboard-like input system. Accepts input parameter in the form of camera defined ENUM (to abstract it from windowing systems)
    pub fn process_keyboard_input(&mut self, direction: Movement, deltaTime: f32) {
        let velocity = self.movement_speed * deltaTime;
        if direction == Forward {
            self.position += self.front * velocity;
        }
        if direction == Backward {
            self.position += -(self.front * velocity);
        }
        if direction == Left {
            self.position += -(self.right * velocity);
        }
        if direction == Right {
            self.position += self.right * velocity;
        }
        if direction == RollRight {
            self.roll(0.1);
        }
        if direction == RollLeft {
            self.roll(-0.1);
        }
        self.debug_print();
        self.update_view_matrix();
    }

    pub fn debug_print(&self) {}

    pub fn process_mouse_movement(&mut self, mut xoffset: f32, mut yoffset: f32) {
        xoffset *= self.mouse_sensitivity;
        yoffset *= self.mouse_sensitivity;
        self.pitch(yoffset);
        self.yaw(-xoffset);
        self.update_view_matrix();
    }

    pub fn process_mouse_scroll(&mut self, yoffset: f32) {
        if self.zoom >= 1.0 && self.zoom <= 45.0 {
            self.zoom -= yoffset;
        }
        self.zoom = self.zoom.clamp(1.0, 45.0);
        self.update_view_matrix();
    }
}
