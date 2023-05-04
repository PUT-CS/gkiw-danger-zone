use crate::{
    c_str,
    cg::{camera::Camera, model::Model},
    game::{drawable::Drawable, enemies::Enemies, flight::steerable::Steerable, missile::EnemyID, targeting_data::{TargetingData, self}},
    GLFW_TIME,
};
use cgmath::{vec3, Deg, InnerSpace, Matrix4, MetricSpace, SquareMatrix, Vector3};
use lazy_static::{__Deref, lazy_static};
use log::warn;
use std::{ffi::CStr, ops::Div};

lazy_static! {
    static ref TARGET_RECTANGLE: Model = {
        let mut model = Model::new("resources/objects/hud/target_rectangle.obj");
        model.pitch(90.);
        model.scale(1.).deref().clone()
    };
    static ref TARGET_CIRCLE: Model = {
        let mut model = Model::new("resources/objects/hud/target_circle.obj");
        model.yaw(90.);
        model.scale(2.).deref().clone()
    };
}

const UPDATE_INTERVAL: f64 = 0.1;

pub struct Hud {
    enabled: bool,
    target_rectangles: Vec<Model>,
    target_circle: Model,
    last_update_time: f64,
}

impl Hud {
    pub fn new() -> Self {
        Self {
            enabled: true,
            target_rectangles: vec![],
            target_circle: TARGET_CIRCLE.clone(),
            last_update_time: 0.,
        }
    }

    pub fn update(&mut self, camera: &Camera, enemies: &Enemies, targeting_data: &Option<TargetingData>) {
        if self.last_update_time + UPDATE_INTERVAL > unsafe { GLFW_TIME } || !self.enabled {
            return;
        }

        let target_id = {
            if let Some(data) = targeting_data {
                Some(data.target_id)
            } else {
                self.target_circle.set_scale(0.);
                None
            }
        };

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
                    rect.set_translation(new_pos).set_scale(scale);
                    if target_id == Some(enemy.id()) {
                        warn!("UPDATING TARGET");
                        self.target_circle
                            .set_translation(new_pos)
                            .set_scale(scale * 2.);
                    }
                } else {
                    rect.set_scale(0.);
                    self.target_circle.set_scale(0.);
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
        self.target_circle.draw(shader);
        gl::Enable(gl::DEPTH_TEST);
    }
}
