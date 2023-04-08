use crate::game::flight::steerable::Steerable;
use crate::SCR_HEIGHT;
use crate::SCR_WIDTH;
use cgmath;
use cgmath::perspective;
use cgmath::prelude::*;
use cgmath::vec3;
use cgmath::Deg;
use cgmath::Quaternion;
use crate::{gen_getters, gen_ref_getters};
type Point3 = cgmath::Point3<f32>;
type Vector3 = cgmath::Vector3<f32>;
type Matrix4 = cgmath::Matrix4<f32>;

#[derive(PartialEq, Clone, Copy)]
pub enum Movement {
    PitchUp,
    PitchDown,
    YawLeft,
    YawRight,
    RollRight,
    RollLeft,
    ThrottleUp,
    ThrottleDown,
}

pub enum ControlSurfaces {
    Pitch = 0,
    Yaw = 1,
    Roll = 2,
}

const SPEED: f32 = 25.;
const SENSITIVTY: f32 = 0.1;
const ZOOM: f32 = 45.0;

#[derive(Debug, Clone)]
pub struct Camera {
    pub position: Point3,
    pub front: Vector3,
    pub up: Vector3,
    pub right: Vector3,
    view_matrix: Matrix4,
    projection_matrix: Matrix4,
    movement_speed: f32,
    mouse_sensitivity: f32,
    zoom: f32,
}

impl Default for Camera {
    fn default() -> Camera {
        Camera {
            position: Point3::new(0.0, 0.0, 5.0),
            front: vec3(0.0, 0.0, -1.0),
            up: Vector3::unit_y(),
            right: Vector3::unit_x(),
            view_matrix: Matrix4::from_axis_angle(Vector3::unit_z(), Deg(180.0)),
            projection_matrix: perspective(
                Deg(ZOOM),
                SCR_WIDTH as f32 / SCR_HEIGHT as f32,
                0.1,
                30000.0,
            ),
            movement_speed: SPEED,
            mouse_sensitivity: SENSITIVTY,
            zoom: ZOOM,
        }
    }
}

gen_getters! {
    Camera,
    movement_speed -> f32,
    view_matrix -> Matrix4,
}
gen_ref_getters! {
    Camera,
    projection_matrix -> &Matrix4,
    position -> &Point3,
}

impl Steerable for Camera {
    fn pitch(&mut self, amount: f32) {
        let rotation = Quaternion::from_axis_angle(self.right, Deg(amount));
        self.front = (rotation * self.front).normalize();
        self.up = (rotation * self.up).normalize();
        self.update_view_matrix();
    }
    fn yaw(&mut self, amount: f32) {
        let rotation = Quaternion::from_axis_angle(self.up, Deg(amount));
        self.front = (rotation * self.front).normalize();
        self.right = (rotation * self.right).normalize();
        self.update_view_matrix();
    }

    fn roll(&mut self, amount: f32) {
        let rotation = Quaternion::from_axis_angle(self.front, Deg(amount));
        self.right = (rotation * self.right).normalize();
        self.up = (rotation * self.up).normalize();
        self.update_view_matrix();
    }
    fn forward(&mut self, throttle: f32) {
        self.position += self.front * throttle / 100.;
        self.update_view_matrix();
    }
}

impl Camera {
    pub fn update_view_matrix(&mut self) {
        self.view_matrix = Matrix4::look_at(self.position, self.position + self.front, self.up);
    }

    pub fn debug_print(&self) {}

    pub fn process_mouse_movement(&mut self, mut xoffset: f32, mut yoffset: f32) {
        xoffset *= self.mouse_sensitivity;
        yoffset *= self.mouse_sensitivity;
        self.yaw(-xoffset);
        self.pitch(yoffset);

        // compensate for unwanted roll here

        self.update_view_matrix();
        self.debug_print();
    }

    pub fn process_mouse_scroll(&mut self, yoffset: f32) {
        if self.zoom >= 1.0 && self.zoom <= 45.0 {
            self.zoom -= yoffset;
        }
        self.zoom = self.zoom.clamp(1.0, 45.0);
        self.update_view_matrix();
    }
}
