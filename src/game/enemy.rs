use log::warn;

use super::flight::aircraft::{Aircraft, AircraftKind};
use crate::cg::shader::Shader;
use crate::game::drawable::Drawable;
use crate::{c_str, gen_ref_getters};
use std::ffi::CStr;
use std::fmt::Debug;

/// Struct representing an enemy
pub struct Enemy {
    aircraft: Aircraft,
    destroyed: bool,
}

impl Drawable for Enemy {
    unsafe fn draw(&self, shader: &Shader) {
        shader.set_mat4(c_str!("model"), &self.aircraft().model().model_matrix());
        //println!("Drawing with matrix: {:?}", self.aircraft().model().model_matrix());
        self.aircraft.model().draw(shader);
    }
}

impl Enemy {
    /// Create a new enemy with the given aircraft kind
    pub fn new(kind: AircraftKind) -> Self {
        Enemy {
            aircraft: Aircraft::new(kind),
            destroyed: false,
        }
    }
    pub fn destroy(&mut self) {
        self.destroyed = true
    }
    pub fn aircraft_mut(&mut self) -> &mut Aircraft {
        &mut self.aircraft
    }
}

impl Debug for Enemy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("{}", self.destroyed);
        Ok(())
    }
}

gen_ref_getters! {
    Enemy,
    aircraft -> &Aircraft,
}
