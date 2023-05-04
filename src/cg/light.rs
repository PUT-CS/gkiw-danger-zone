use cgmath::{Point3, Vector3};

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Material {
    pub diffuse: Vector3<f32>,
    pub specular: Vector3<f32>,
    pub shininess: f32,
}

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
    position: Point3<f32>,
    constant: f32,
    linear: f32,
    quadratic: f32,
    ambient: Vector3<f32>,
    diffuse: Vector3<f32>,
    specular: Vector3<f32>,
}

// values for material were used to imitate chrome
impl Material {
    pub fn new() -> Self {
        Self {
            diffuse: Vector3::new(0.4, 0.4, 0.4),
            specular: Vector3::new(0.774597, 0.774597, 0.774597),
            shininess: 0.6,
        }
    }
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
    fn new() -> Self {
        Self {
            position: Point3::new(0., 0., 0.),
            constant: 1.,
            linear: 0.09,
            quadratic: 0.032,
            ambient: Vector3::new(0.05, 0.05, 0.05),
            diffuse: Vector3::new(0.4, 0.4, 0.4),
            specular: Vector3::new(0.5, 0.5, 0.5),
        }
    }
}
