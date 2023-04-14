use cgmath::{num_traits::float, Vector2, Vector3, Vector4, Zero};
use rand::{thread_rng, Rng};
// change all 0.1 to delta time when available!!!!

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
        position: Vector3<f32>,
        offset: Vector3<f32>,
        first_dead: usize,
    ) {
        let rand = thread_rng().gen_range(-50, 50) as f32 / 10.0;
        let random = Vector3::new(rand, rand, rand);
        self.particles[first_dead].postion = position + random + offset;
        self.particles[first_dead].color = self.color;
        self.particles[first_dead].life = 1.;
        self.particles[first_dead].velocity = position * 0.3;
    }
    
    pub fn update_particles(
        &mut self,
        position: Vector3<f32>,
        offset: Vector3<f32>,
        number_new_particles: usize,
    ) {
        for _ in 0..number_new_particles {
            let first_dead = self.first_dead_particle();
            self.respawn_particle(position, offset, first_dead);
        }
	self.particles.iter_mut().for_each(|p|{
	    p.life -= 0.1;
	    if p.life > 0. {
		p.postion -= p.velocity * 0.1;
		p.color.w -= 2.5 * 0.1; 
	    }
	})
    }
}
