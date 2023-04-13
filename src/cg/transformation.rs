use cgmath::{vec3, Vector3};

#[derive(Debug, Clone)]
pub struct Transformation {
    pub translation: Vector3<f32>,
    pub scale: f32,
}

impl Default for Transformation {
    fn default() -> Self {
        Self {
            translation: vec3(0., 0., 0.),
            scale: 1.,
        }
    }
}
