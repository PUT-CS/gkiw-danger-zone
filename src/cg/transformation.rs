use cgmath::{Matrix4, Vector3};

pub struct Transfromation {
    translation: Vector3<f32>,
    scale: f32,
    rotation: Matrix4<f32>,
}
