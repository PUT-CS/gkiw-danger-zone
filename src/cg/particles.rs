use std::{ffi::c_void, ptr};

use cgmath::{Vector3, Vector4, Zero};
use rand::{thread_rng, Rng};

use crate::{game::drawable::Drawable, DELTA_TIME};

use crate::cg::shader::Shader;

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
    pub vao: u32,
    vbo: u32,
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

impl Drawable for ParticleGenerator {
    unsafe fn draw(&self, shader: &Shader) {
	// gl::BlendFunc(gl::SRC_ALPHA, gl::ONE);
	// shader.use_program();
	// self.particles.iter_mut().for_each(|p|{
	//     if p.life > 0. {
	// 	shader.set_mat3(c_str!("offset"), p.postion);
		
	//     }
	// })
    }
}

impl ParticleGenerator {
    pub fn new(size: usize, color: Vector4<f32>) -> Self {
        let mut particles = Vec::with_capacity(size);
        particles.resize_with(size, || Particle::new(color));
        let mut generator = Self {
            particles,
            color,
            last_revived_particle_idx: 0,
            vao: u32::MAX,
	    vbo: u32::MAX,
        };
	generator.init();
	generator
    }

    pub fn init(&mut self) {
        let mut particle_quad = vec![
            0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0,
            1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 0.0,
        ];
        unsafe {
	    let data: *const c_void;
	    data = particle_quad.as_ptr() as *const c_void;
            gl::GenVertexArrays(1, &mut self.vao);
	    gl::GenBuffers(1, &mut self.vbo);
	    gl::BindVertexArray(self.vao);
	    //fill mesh buffer
	    gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
	    gl::BufferData(gl::ARRAY_BUFFER, 96, data, gl::STATIC_DRAW);
	    //set mesh attributes
	    gl::EnableVertexAttribArray(0);
	    gl::VertexAttribPointer(0, 4, gl::FLOAT, gl::FALSE, 16, ptr::null() as *const c_void);
	    gl::BindVertexArray(0);
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
        self.particles.iter_mut().for_each(|p| {
            let delta_time = unsafe { DELTA_TIME };
            p.life -= delta_time;
            if p.life > 0. {
                p.postion -= p.velocity * delta_time;
                p.color.w -= 2.5 * delta_time;
            }
        })
    }
}
