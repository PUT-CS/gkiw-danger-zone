use crate::Shader;

pub trait Drawable{
    unsafe fn draw(&self, shader: &Shader);
}
