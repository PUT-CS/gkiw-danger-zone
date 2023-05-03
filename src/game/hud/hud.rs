use crate::{
    c_str,
    cg::{camera::Camera, model::Model},
    game::{drawable::Drawable, enemies::Enemies, flight::steerable::Steerable},
    GLFW_TIME,
};
use cgmath::{Deg, InnerSpace, Matrix4, MetricSpace, SquareMatrix, Vector3};
use lazy_static::lazy_static;
use std::{ffi::CStr, ops::Div};

lazy_static! {
    static ref TARGET_RECTANGLE: Model = {
        let mut model = Model::new("resources/objects/hud/target_rectangle.obj");
        model.pitch(90.);
        model.scale(1.).deref().clone()
    };
}

const UPDATE_INTERVAL: f64 = 0.1;

pub struct Hud {
    enabled: bool,
    target_rectangles: Vec<Model>,
    last_update_time: f64,
}

impl Hud {
    pub fn new() -> Self {
        Self {
            enabled: true,
            target_rectangles: vec![],
            last_update_time: 0.,
        }
    }

    pub fn update(&mut self, enemies: &Enemies, camera: &Camera) {
        if self.last_update_time + UPDATE_INTERVAL > unsafe { GLFW_TIME } || !self.enabled {
            return;
        }

        self.target_rectangles
            .resize_with(enemies.map.len(), || TARGET_RECTANGLE.clone());
        self.target_rectangles
            .iter_mut()
            .zip(enemies.map.values())
            .for_each(|(rect, enemy)| {
                let ndc = {
                    let clip_space = camera.projection_matrix()
                        * camera.view_matrix()
                        * enemy.position().to_homogeneous();
                    clip_space / clip_space.w
                };

                let vec_to_enemy = (enemy.position() - camera.position()).normalize();
                // If player is facing the enemy
                if vec_to_enemy.angle(camera.front) < Deg(100.).into() {
                    let new_pos = Vector3::from((ndc.x, ndc.y, 0.1));
                    let distance_to_enemy = camera.position().distance(enemy.position());
                    let scale = 1.0.div(distance_to_enemy).clamp(0.06, 0.3);
                    rect.set_translation(new_pos);
                    rect.set_scale(scale);
                } else {
                    rect.set_scale(0.);
                }
            });
        self.last_update_time = unsafe { GLFW_TIME };
    }
}

impl Drawable for Hud {
    unsafe fn draw(&self, shader: &crate::cg::shader::Shader) {
        shader.set_mat4(c_str!("view"), &Matrix4::identity());
        shader.set_mat4(c_str!("projection"), &Matrix4::identity());
        gl::Disable(gl::DEPTH_TEST);
        self.target_rectangles.iter().for_each(|r| r.draw(shader));
        gl::Enable(gl::DEPTH_TEST);
    }
}
