use super::{drawable::Drawable, modeled::Modeled};
use crate::cg::{model::Model, particles::ParticleGenerator};
use cgmath::EuclideanSpace;

pub trait ParticleGeneration: Modeled {
    unsafe fn draw_particles(&mut self, shader: &crate::cg::shader::Shader) {
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE);
        let model = self.particle_generator_mut().model_mut() as *mut Model;
        self.particle_generator_mut()
            .particles
            .iter_mut()
            .for_each(|p| {
                if p.life > 0. {
                    model.as_mut().unwrap().set_translation(p.position.to_vec());
		    model.as_ref().unwrap().draw(&shader);
                }
            });
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA); 
    }

    fn particle_generator(&self) -> &ParticleGenerator;
    fn particle_generator_mut(&mut self) -> &mut ParticleGenerator;
}
