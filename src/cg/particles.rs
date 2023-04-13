use cgmath::{Vector3, Vector4, Zero, Vector2, num_traits::float};
use rand::{thread_rng, Rng};

pub struct Particle {
    postion: Vector3<f32>,
    velocity: Vector3<f32>,
    color: Vector4<f32>,
    life: f32,
}

pub struct ParticleGenerator {
    particles: Vec<Particle>,
    color: Vector4<f32>,
    last_revived_particle_idx: usize,
}

impl Particle {
    pub fn new(color: Vector4<f32>) -> Self {
        Self {
            postion: Vector3::zero(),
            velocity: Vector3::zero(),
            color,
            life: 0.,
        }
    }
}

impl ParticleGenerator {
    pub fn new(size: usize, color: Vector4<f32>) -> Self {
	let mut particles = Vec::with_capacity(size);
	particles.resize_with(size, || Particle::new(color));
        Self {
	    particles,
            color,
	    last_revived_particle_idx: 0,
        }
    }
    
    pub fn first_dead_particle(&self) -> usize {
	let predicate = |(_, p): &(usize, &Particle)| p.life <= 0.;

	if let Some((idx,_)) = self.particles[self.last_revived_particle_idx..].iter().enumerate().find(predicate) {
	    return idx;
	}
	if let Some((idx,_)) = self.particles.iter().enumerate().find(predicate) {
	    return idx;
	}
	0
    }

//     pub fn respawn_particle(&mut self, position: Vector3<f32>, offset: Vector2<f32>) {
// 	let random = thread_rng().gen_range(-50, 50) as f32 / 10.0;
// 	self.particles[self.last_revived_particle_idx];
//     }
 }
