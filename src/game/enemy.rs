use super::flight::aircraft::{Aircraft, AircraftKind};
use crate::gen_ref_getters;

/// Struct representing an enemy
pub struct Enemy {
    pub aircraft: Aircraft,
    destroyed: bool,
}

impl Enemy {
    /// Create a new enemy with the given aircraft kind
    pub fn new(kind: AircraftKind) -> Self {
        Self {
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

gen_ref_getters! {
    Enemy,
    aircraft -> &Aircraft,
}
