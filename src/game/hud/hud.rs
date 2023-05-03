use cgmath::{ortho, vec3, vec4, Deg, EuclideanSpace, InnerSpace, Ortho, SquareMatrix, Vector2, Matrix4, Vector3};
use lazy_static::lazy_static;
use std::{
    ffi::CStr,
    ops::{Div, Mul},
};

use crate::{
    c_str,
    cg::{camera::Camera, model::Model},
    game::{drawable::Drawable, enemies::Enemies, flight::steerable::Steerable},
    SCR_HEIGHT, SCR_WIDTH,
};

lazy_static! {
    static ref TARGET_RECTANGLE: Model = {
        let mut model = Model::new("resources/objects/hud/target_rectangle.obj");
        //let mut model = Model::new("resources/objects/cockpit/cockpit.obj");
        model.pitch(90.);
        model.scale(1.).deref().clone()
    };
}

pub struct Hud {
    enabled: bool,
    target_rectangles: Vec<Model>,
}

impl Hud {
    pub fn new() -> Self {
        Self {
            enabled: true,
            target_rectangles: vec![],
        }
    }
    pub fn update(&mut self, enemies: &Enemies, camera: &Camera) {
        self.target_rectangles
            .resize_with(enemies.map.len(), || TARGET_RECTANGLE.clone());
        self.target_rectangles
            .iter_mut()
            .zip(enemies.map.values())
            .for_each(|(rect, enemy)| {
                let projection_view = camera.projection_matrix() * camera.view_matrix();
                let vec_to_enemy = enemy.position() - camera.position();
                let clip_space = projection_view * enemy.position().to_homogeneous();
                let ndc = clip_space / clip_space.w;

                let screen_pos = Vector2::new(
                    ((ndc.x + 1.) / 2.) * SCR_WIDTH as f32,
                    ((ndc.y + 1.) / 2.) * SCR_HEIGHT as f32,
                );

                let new_world_pos = {
                    let inv = projection_view.invert().unwrap();
                    let v = vec4(ndc.x, ndc.y, -1., 1.);
                    //let new_vec = .extend(-1.).extend(1.);
                    let new_pos = inv * v;
                    new_pos
                    //new_pos / new_pos.w
                };

                if ndc.x.abs() < 1.
                    && ndc.y.abs() < 1.
                    && vec_to_enemy.angle(camera.front) < Deg(90.).into()
                {
                    dbg!(ndc);
                    //dbg!(screen_pos);
                    //dbg!(new_world_pos);
                    //dbg!(camera.position());
                }

                let new_pos = Vector3::from((ndc.x, ndc.y, 0.1));
                
                rect.set_translation(new_pos);
                rect.set_scale(0.2);
                //rect.set_translation(enemy.position().to_vec());
                //rect.set_translation(camera.position().to_vec() + camera.front * 10.);
                //rect.set_orientation(camera.orientation_quat());
                //rect.pitch(90.);
                //rect.set_translation(enemy.position().to_vec());
            })
    }
}

impl Drawable for Hud {
    unsafe fn draw(&self, shader: &crate::cg::shader::Shader) {
        // let projection = ortho(
        //     -(SCR_WIDTH as f32) / 2.,
        //     SCR_WIDTH as f32 / 2.,
        //     -(SCR_HEIGHT as f32) / 2.,
        //     SCR_HEIGHT as f32 / 2.,
        //     -1.,
        //     30000.,
        // );
        // let ortho = ortho(0., SCR_WIDTH as f32, 0., SCR_HEIGHT as f32, -1., 1.);
        // shader.set_mat4(c_str!("projection"), &ortho);
        // let aspect = SCR_WIDTH as f32/SCR_HEIGHT as f32;
        // //let ort = ortho(0., SCR_WIDTH as f32, 0., SCR_HEIGHT as f32, -1., 1.);
        // let ort = ortho(-aspect, aspect, -1., 1., -1., 1.);
        // shader.set_mat4(c_str!("projection"), &ort);

        shader.set_mat4(c_str!("view"), &Matrix4::identity());
        shader.set_mat4(c_str!("projection"), &Matrix4::identity());
        gl::Disable(gl::DEPTH_TEST);
        self.target_rectangles.iter().for_each(|r| r.draw(shader));
        gl::Enable(gl::DEPTH_TEST);
    }
}
