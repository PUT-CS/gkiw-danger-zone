use cgmath::{ortho, EuclideanSpace, Matrix4, Ortho, SquareMatrix, Vector4, Vector3, Vector2};
use lazy_static::{__Deref, lazy_static};
use std::ffi::CStr;

use crate::{
    c_str,
    cg::model::Model,
    game::{drawable::Drawable, enemies::Enemies, flight::steerable::Steerable},
    SCR_HEIGHT, SCR_WIDTH,
};

lazy_static! {
    static ref TARGET_RECTANGLE: Model = {
        let mut model = Model::new("resources/objects/hud/target_rectangle.obj");
        //let mut model = Model::new("resources/objects/cockpit/cockpit.obj");
        model.pitch(90.);
        model.scale(10.).deref().clone()
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
    pub fn update(&mut self, enemies: &Enemies, projection_matrix: Matrix4<f32>, view_matrix: Matrix4<f32>) {
        self.target_rectangles
            .resize_with(enemies.map.len(), || TARGET_RECTANGLE.clone());
        self.target_rectangles
            .iter_mut()
            .zip(enemies.map.values())
            .for_each(|(rect, enemy)| {
                let enemy_position = enemy.position();
                // clip space
                let c = projection_matrix * (view_matrix * Vector4::from(enemy_position.to_homogeneous()));
                let ndc = Vector3::from((c.x/c.w, c.y/c.w, c.z/c.w));
                let final_pos = (Vector2::from((ndc.x + 1., ndc.y + 1.)) / 2.) * 1000.;
                let x = Vector2::from((
                    (ndc.x + 1. / 2.) * 1000. + 0.,
                    (ndc.y + 1. / 2.) * 1000. + 0.,
                )) / 1000.2;
                dbg!(x);
                //vec2( ((ndcSpacePos.x + 1.0) / 2.0) * viewSize.x + viewOffset.x, ((1.0 - ndcSpacePos.y) / 2.0) * viewSize.y + viewOffset.y )
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
        //     1.,
        // );
        // shader.set_mat4(c_str!("view"), &projection);
        self.target_rectangles.iter().for_each(|r| r.draw(shader))
    }
}
