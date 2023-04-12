use cgmath::Vector3;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref VEC_FRONT: Vector3<f32> = Vector3::unit_z() * -1.;
    pub static ref VEC_UP: Vector3<f32> = Vector3::unit_y();
    pub static ref VEC_RIGHT: Vector3<f32> = Vector3::unit_x();
}
