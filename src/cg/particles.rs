use std::{ffi::c_void, ffi::CStr, ptr};

use cgmath::{Vector3, Vector4, Zero, Point3, EuclideanSpace};
use rand::{thread_rng, Rng};

use crate::c_str;
use crate::{game::drawable::Drawable, DELTA_TIME};

use crate::cg::shader::Shader;

#[derive(Clone, Debug)]
pub struct Particle {
    postion: Point3<f32>,
    velocity: Vector3<f32>,
    color: Vector4<f32>,
    life: f32,
}

#[derive(Clone, Debug)]
pub struct ParticleGenerator {
    particles: Vec<Particle>,
    color: Vector4<f32>,
    pub offset: Vector3<f32>,
    last_revived_particle_idx: usize,
    pub vao: u32,
    vbo: u32,
    pub enabled: bool,
}

impl Particle {
    pub fn new(color: Vector4<f32>) -> Self {
        Self {
            postion: Point3::from([0.; 3]),
            velocity: Vector3::zero(),
            color,
            life: 0.,
        }
    }
}

impl Drawable for ParticleGenerator {
    unsafe fn draw(&self, shader: &Shader) {
	dbg!("");
	gl::BlendFunc(gl::SRC_ALPHA, gl::ONE);
	self.particles.iter().for_each(|p|{
	    if p.life > 0. {
		shader.set_vector3(c_str!("offset"), &p.postion.to_vec());
		shader.set_vector4(c_str!("color"), &p.color);
		// there should be texture bind!
		gl::BindVertexArray(self.vao);
		gl::DrawArrays(gl::TRIANGLES, 0, 6);
		gl::BindVertexArray(0); 
	    }
	});
	gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }
    
}

impl ParticleGenerator {
    pub fn new(size: usize, color: Vector4<f32>, offset: Vector3<f32>) -> Self {
        let mut particles = Vec::with_capacity(size);
        particles.resize_with(size, || Particle::new(color));
        let mut generator = Self {
            particles,
            color,
	    offset,
            last_revived_particle_idx: 0,
            vao: u32::MAX,
	    vbo: u32::MAX,
	    enabled: false
        };
	generator.init();
	generator
    }

    pub fn enable(&mut self) {
	self.enabled = true;
    }

    pub fn init(&mut self) {
        let particle_quad = vec![
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
        position: Point3<f32>,
        offset: Vector3<f32>,
        first_dead: usize,
    ) {
	let first_dead = &mut self.particles[first_dead];
        let rand = thread_rng().gen_range(-1., 1.);
        let random = Vector3::new(rand, rand, rand);
        first_dead.postion = position + random + offset;
        first_dead.color = self.color;
        first_dead.life = 1.;
        first_dead.velocity = position.to_vec() * 0.3;
    }

    pub fn update_particles(
        &mut self,
        position: Point3<f32>,
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
