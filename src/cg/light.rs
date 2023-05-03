use cgmath::{Vector3, Point3};

#[derive(Clone, Debug)]
pub struct Material {
    diffuse: Vector3<f32>,
    specular: Vector3<f32>,
    shininess: f32,
}


#[derive(Clone, Debug)]
pub struct DirLight {
    position: Point3<f32>,
    ambient: Vector3<f32>,
    diffuse: Vector3<f32>,
    specular: Vector3<f32>,
}


#[derive(Clone, Debug)]
pub struct PointLight {
    position: Point3<f32>,
    constant: f32,
    linear: f32,
    quadratic: f32,
    ambient: Vector3<f32>,
    diffuse: Vector3<f32>,
    specular: Vector3<f32>,
}

