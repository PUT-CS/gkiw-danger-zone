use cgmath::{EuclideanSpace, InnerSpace, Point3, Vector3, Vector4, Zero};
use rand::{thread_rng, Rng};

use crate::game::modeled::Modeled;
use crate::{game::drawable::Drawable, DELTA_TIME};

use super::model::Model;

#[derive(Clone, Debug)]
pub struct Particle {
    pub position: Point3<f32>,
    velocity: Vector3<f32>,
    color: Vector4<f32>,
    pub life: f32,
}

#[derive(Clone, Debug)]
pub struct ParticleGenerator {
    pub particles: Vec<Particle>,
    pub color: Vector4<f32>,
    pub offset: f32,
    last_revived_particle_idx: usize,
    pub enabled: bool,
    pub model: Model,
}

impl Particle {
    pub fn new(color: Vector4<f32>) -> Self {
        Self {
            position: Point3::from([0.; 3]),
            velocity: Vector3::zero(),
            color,
            life: 0.,
        }
    }
}

impl Drawable for ParticleGenerator {
    unsafe fn draw(&self, shader: &crate::cg::shader::Shader) {
        self.model().draw(shader);
    }
}

impl Modeled for ParticleGenerator {
    fn model(&self) -> &Model {
        &self.model
    }
    fn model_mut(&mut self) -> &mut Model {
        &mut self.model
    }
}

impl ParticleGenerator {
    pub fn new(size: usize, color: Vector4<f32>, offset: f32) -> Self {
        let mut model = Model::new("resources/objects/particle/particle.obj");
        model.set_scale(0.1);
        let mut particles = Vec::with_capacity(size);
        particles.resize_with(size, || Particle::new(color));
        let generator = Self {
            particles,
            color,
            offset,
            last_revived_particle_idx: 0,
            enabled: true,
            model,
        };
        generator
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn first_dead_particle(&self) -> usize {
        let predicate = |(_, p): &(usize, &Particle)| p.life <= 0.;

        if let Some((idx, _)) = self.particles[self.last_revived_particle_idx..]
            .iter()
            .enumerate()
            .find(predicate)
        {
            return idx;
        }
        if let Some((idx, _)) = self.particles.iter().enumerate().find(predicate) {
            return idx;
        }
        0
    }

    pub fn respawn_particle(
        &mut self,
        position: Point3<f32>,
        first_dead: usize,
        front: Vector3<f32>,
    ) {
        let first_dead = &mut self.particles[first_dead];
        let rand1 = thread_rng().gen_range(-0.1, 0.1);
        let rand2 = thread_rng().gen_range(-0.1, 0.1);
        let rand3 = thread_rng().gen_range(-0.1, 0.1);
        let random = Vector3::new(rand1, rand2, rand3);
        let offset = front * -1. * self.offset;
        first_dead.position = position + random + offset;
        first_dead.color = self.color;
        first_dead.life = thread_rng().gen_range(3., 5.);
        first_dead.velocity = (position.to_vec()).normalize();
    }

    pub fn update_particles(
        &mut self,
        position: Point3<f32>,
        number_new_particles: usize,
        front: Vector3<f32>,
    ) {
        if !self.enabled {
            return;
        }
        for _ in 0..number_new_particles {
            let first_dead = self.first_dead_particle();
            self.respawn_particle(position, first_dead, front);
        }
        self.particles.iter_mut().for_each(|p| {
            let delta_time = unsafe { DELTA_TIME };
            p.life -= delta_time;
            if p.life > 0. {
                p.position -= p.velocity * delta_time;
                p.color.w -= 2.5 * delta_time;
            }
        })
    }
}
