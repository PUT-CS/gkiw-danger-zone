use crate::cg::particles::ParticleGenerator;
use crate::c_str;
use std::ffi::CStr;
use cgmath::EuclideanSpace;
use super::{drawable::Drawable, modeled::Modeled};

pub trait ParticleGeneration:Modeled {
    unsafe fn draw_particles(&mut self, shader: &crate::cg::shader::Shader) {
	gl::BlendFunc(gl::SRC_ALPHA, gl::ONE);
	self.particle_generator().particles.iter().for_each(|p| {
	    if p.life > 0. {
		shader.set_vector3(c_str!("offset"), &self.model().position().to_vec());
		shader.set_vector4(c_str!("color"), &self.particle_generator().color);
		self.particle_generator().model.draw(&shader);
	    } 
	});
	gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }
    
    fn particle_generator(& self) -> &ParticleGenerator;
    fn particle_generator_mut(&mut self) -> &mut ParticleGenerator;
}
