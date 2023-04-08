pub trait Drawable {
    unsafe fn draw(&self, shader: &crate::cg::shader::Shader);
}
