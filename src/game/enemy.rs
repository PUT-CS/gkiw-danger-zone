use cgmath::Point3;
use super::flight::aircraft::{Aircraft, AircraftKind};
use super::missile::EnemyID;
use crate::gen_ref_getters;

/// Struct representing an enemy
pub struct Enemy {
    id: u32,
    pub aircraft: Aircraft,
    destroyed: bool,
}

impl Enemy {
    /// Create a new enemy with the given aircraft kind
    pub fn new(id: EnemyID, kind: AircraftKind) -> Self {
        Self {
            id,
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
    pub fn position(&self) -> Point3<f32> {
        self.aircraft().model().position()
    }
    pub fn id(&self) -> EnemyID {
        self.id
    }
}

gen_ref_getters! {
    Enemy,
    aircraft -> &Aircraft,
}
