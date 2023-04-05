use cgmath::prelude::*;
use cgmath::Vector2;
use cgmath::Vector3;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub tex_coords: Vector2<f32>,
    pub tangent: Vector3<f32>,
    pub bitangent: Vector3<f32>,
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            position: Vector3::zero(),
            normal: Vector3::zero(),
            tex_coords: Vector2::zero(),
            tangent: Vector3::zero(),
            bitangent: Vector3::zero(),
        }
    }
}
