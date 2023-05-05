use cgmath::{Point3, Vector3};

#[derive(Clone, Debug)]
#[repr(C)]
pub struct DirectionalLight {
    pub direction: Vector3<f32>,
    pub ambient: Vector3<f32>,
    pub diffuse: Vector3<f32>,
    pub specular: Vector3<f32>,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct PointLight {
    pub position: Point3<f32>,
    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,
    pub ambient: Vector3<f32>,
    pub diffuse: Vector3<f32>,
    pub specular: Vector3<f32>,
}

impl DirectionalLight {
    pub fn new(direction: Vector3<f32>) -> Self {
        Self {
            direction,
            ambient: Vector3::new(0.6,0.6, 0.6),
            diffuse: Vector3::new(0.4, 0.4, 0.4),
            specular: Vector3::new(0.5, 0.5, 0.5),
        }
    }
}

impl PointLight {
    pub fn new(position: Point3<f32>) -> Self {
        Self {
            position,
            constant: 1.,
            linear: 0.09,
            quadratic: 0.032,
            ambient: Vector3::new(0.2, 0.2, 0.2),
            diffuse: Vector3::new(0.4, 1.0, 0.4),
            specular: Vector3::new(0.5, 0.5, 0.5),
        }
    }
}
