use crate::game::flight::steerable::Steerable;
use crate::gen_ref_getters;
use crate::SCR_HEIGHT;
use crate::SCR_WIDTH;
use cgmath;
use cgmath::Point2;
use cgmath::perspective;
use cgmath::prelude::*;
use cgmath::vec3;
use cgmath::Deg;
use cgmath::Matrix3;
use cgmath::Quaternion;
use num::ToPrimitive;
use vek::Vec3;

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

const SENSITIVTY: f32 = 0.1;
const ZOOM: f32 = 45.0;

#[derive(Debug, Clone)]
pub struct Camera {
    pub position: Point3,
    pub front: Vector3,
    pub up: Vector3,
    pub right: Vector3,
    projection_matrix: Matrix4,
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
            projection_matrix: perspective(
                Deg(ZOOM),
                SCR_WIDTH as f32 / SCR_HEIGHT as f32,
                0.1,
                30000.0,
            ),
            mouse_sensitivity: SENSITIVTY,
            zoom: ZOOM,
        }
    }
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
    }
    fn yaw(&mut self, amount: f32) {
        let rotation = Quaternion::from_axis_angle(self.up, Deg(amount));
        self.front = (rotation * self.front).normalize();
        self.right = (rotation * self.right).normalize();
    }
    fn roll(&mut self, amount: f32) {
        let rotation = Quaternion::from_axis_angle(self.front, Deg(amount));
        self.right = (rotation * self.right).normalize();
        self.up = (rotation * self.up).normalize();
    }
    fn forward(&mut self, throttle: f32) {
        self.position += self.front * throttle;
    }
}

impl Camera {
    pub fn view_matrix(&self) -> Matrix4 {
        Matrix4::look_at(self.position, self.position + self.front, self.up)
    }

    pub fn orientation_quat(&self) -> Quaternion<f32> {
        let m = self.view_matrix().invert().expect("Invertible view matrix");
        let rot = Matrix3::from([
            [m.x.x, m.x.y, m.x.z],
            [m.y.x, m.y.y, m.y.z],
            [m.z.x, m.z.y, m.z.z],
        ]);
        Quaternion::from(rot)
    }

    pub fn process_mouse_movement(&mut self, mut xoffset: f32, mut yoffset: f32) {
        xoffset *= self.mouse_sensitivity;
        yoffset *= self.mouse_sensitivity;
        self.yaw(-xoffset);
        self.pitch(yoffset);
    }

    pub fn process_mouse_scroll(&mut self, yoffset: f32) {
        if self.zoom >= 1.0 && self.zoom <= 45.0 {
            self.zoom -= yoffset;
        }
        self.zoom = self.zoom.clamp(1.0, 45.0);
    }

    pub fn altitude(&self) -> f32 {
        self.position().y
    }

    pub fn xz_ints(&self) -> Point2<i32> {
        let p = self.position().map(|s| s.to_i32().unwrap());
        (p.x, p.z).into()
    }

    pub fn position_vek(&self) -> Vec3<f32> {
        let p = &self.position;
        Vec3::from([p.x, p.y, p.z])
    }
}
