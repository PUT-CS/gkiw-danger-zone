use crate::cg::model::Model;

use super::enemy::Enemy;
pub type EnemyID = u32;
const TERMINATION_TIME: u32 = 30000;

pub enum MissileMessage {
    LostLock,
    Terminated,
    HitEnemy(EnemyID),
}

/// Struct representing a missile fired by the player
pub struct Missile {
    target: Option<EnemyID>,
    model: Model,
    /// An optional integer representing the number of ticks left until termination.
    /// Set to TERMINATION_TIME by calling `terminate` on a Missile instance.
    termination_timer: Option<u32>
}

impl Missile {
    pub fn new(target: Option<EnemyID>) -> Self {
        Missile {
            target: target,
            model: Model::new("resources/objects/cockpit/cockpit.obj"),
            termination_timer: None
        }
    }

    pub fn lose_lock(&mut self) {
        self.target = None.into()
    }
    pub fn terminate(&mut self) {
        self.termination_timer = Some(TERMINATION_TIME);
    }
    pub fn update(&self, target_info: &Enemy) -> MissileMessage {
        todo!()
    }
}
