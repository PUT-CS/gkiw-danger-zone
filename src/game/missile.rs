use crate::cg::model::Model;

use super::enemy::Enemy;

type EnemyID = u32;
const TERMINATION_TIME: u32 = 30000;

pub enum MissileMessage {
    LostLock,
    Terminated,
    HitEnemy(EnemyID),
}

/// Struct representing a missile fired by the player
pub struct Missile<'a> {
    target: Option<&'a Enemy>,
    model: Model,
    /// An optional integer representing the number of ticks left until termination.
    /// Set to TERMINATION_TIME by calling `terminate` on a Missile instance.
    termination_timer: Option<u32>
}

impl<'a> Missile<'a> {
    pub fn new(target:Option<&'a Enemy>) -> Self {
        Missile {
            target: target,
            model: Model::new("resources/objects/cockpit/cockpit.obj"),
            termination_timer: None
        }
    }

    pub fn lose_lock(&mut self) {
        self.target = None
    }
    pub fn terminate(&mut self) {
        self.termination_timer = Some(TERMINATION_TIME);
    }
    pub fn update(&self) -> MissileMessage {
        todo!()
    }
}
