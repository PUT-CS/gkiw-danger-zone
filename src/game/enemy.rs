use super::flight::aircraft::{Aircraft, AircraftKind};
use crate::cg::shader::Shader;
use crate::game::drawable::Drawable;

/// Struct representing an enemy
#[derive(Debug)]
pub struct Enemy {
    id: u32,
    aircraft: Aircraft,
    destroyed: bool,
}

impl Drawable for Enemy {
    unsafe fn draw(&self, shader: &Shader) {
        self.aircraft.model().draw(shader);
    }
    
}

impl Enemy {
    /// Create a new enemy with the given aircraft kind
    pub fn new(kind: AircraftKind, id: u32) -> Self {
        Enemy {
            id,
            aircraft: Aircraft::new(kind),
            destroyed: false,
        }
    }
    pub unsafe fn draw(&self, shader: &Shader) {
        self.aircraft.model().draw(shader);
    }
    pub fn destroy(&mut self) {
        self.destroyed = true
    }
    pub fn aircraft_mut(&mut self) -> &mut Aircraft {
        &mut self.aircraft
    }
}

gen_ref_getters! {
    Enemy,
    id -> &u32,
    aircraft -> &Aircraft,
}
