use super::{drawable::Drawable, enemies::Enemies, flight::steerable::Steerable, missile::EnemyID};
use crate::{
    cg::{camera::Camera, model::Model},
    DELTA_TIME, GLFW_TIME,
};
use cgmath::{Deg, EuclideanSpace, MetricSpace, Quaternion, Rotation3, Vector3};
use itertools::Itertools;
use log::info;
use rand::{thread_rng, Rng};

const BULLET_SPEED: f32 = 1000.;
const BULLET_TERMINATION_TIME: f64 = 3.;
const BULLET_COOLDOWN: f64 = 0.02;

#[derive(Debug)]
pub struct Bullet {
    pub model: Model,
    /// Time at which the bullet should disappear.
    /// Calculated at initialization to be the current
    /// game time + `BULLET_TERMINATION_TIME`
    pub termination_time: f64,
}

#[derive(Debug)]
pub struct Guns {
    bullets: Vec<Bullet>,
    last_fire_time: f64,
    pub firing: bool,
}

impl Guns {
    pub fn new() -> Self {
        Self {
            bullets: Vec::with_capacity(1000),
            last_fire_time: 0.,
            firing: false,
        }
    }

    pub fn fire(&mut self, camera: &Camera) {
        let time = unsafe { GLFW_TIME };
        if self.last_fire_time + BULLET_COOLDOWN > time {
            return;
        }
        let position = camera.position().to_vec() + camera.right * 2.5 + camera.up * -1.5;
        let rand_quat = {
            let rands: (Deg<f32>, Deg<f32>, Deg<f32>) = (0..3)
                .map(|_| Deg(thread_rng().gen_range(-0.3, 0.3)))
                .collect_tuple()
                .unwrap();
            Quaternion::from_angle_x(rands.0)
                * Quaternion::from_angle_y(rands.1)
                * Quaternion::from_angle_z(rands.2)
        };
        let orientation = camera.orientation_quat() * rand_quat;
        self.bullets.push(Bullet::new(position, orientation));
        self.last_fire_time = time;
        self.firing = true;
    }

    pub fn stop_firing(&mut self) {
        self.firing = false
    }

    pub fn update(&mut self) {
        self.bullets.iter_mut().for_each(|b| {
            b.update();
        });
        self.bullets
            .retain(|b| b.termination_time > unsafe { GLFW_TIME });
    }

    pub fn check_collisions(&self, enemies: &Enemies) -> Option<Vec<EnemyID>> {
        let mut hit_enemies = Vec::with_capacity(enemies.map.len());
        for enemy in enemies.map.values() {
            for bullet in &self.bullets {
                if enemy.position().distance(bullet.model.position()) < 2. {
                    info!("Hit enemy {}", enemy.id());
                    hit_enemies.push(enemy.id());
                }
            }
        }
        if !hit_enemies.is_empty() {
            Some(hit_enemies)
        } else {
            None
        }
    }
}

impl Drawable for Guns {
    unsafe fn draw(&self, shader: &crate::cg::shader::Shader) {
        self.bullets.iter().for_each(|b| b.model.draw(shader));
    }
}

impl Bullet {
    pub fn new(position: Vector3<f32>, orientation: Quaternion<f32>) -> Self {
        let mut model = Model::new("resources/objects/bullet/bullet.obj");
        model.set_translation(position);
        model.set_orientation(orientation);
        Self {
            model,
            termination_time: unsafe { GLFW_TIME + BULLET_TERMINATION_TIME },
        }
    }
    fn update(&mut self) {
        let delta_time = unsafe { DELTA_TIME };
        self.model.forward(BULLET_SPEED * delta_time);
    }
}
