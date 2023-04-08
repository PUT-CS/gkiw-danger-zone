use std::fmt::Debug;

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
/// The missile only knows what ID the Enemy it targets has.
/// Each frame it receives a reference to the enemy it targets so it can update its state.
pub struct Missile {
    target: Option<EnemyID>,
    model: Model,
    /// An optional integer representing the number of ticks left until termination.
    /// Set to TERMINATION_TIME by calling `terminate` on a Missile instance.
    termination_timer: Option<u32>,
}

impl Missile {
    pub fn new(target: Option<EnemyID>) -> Self {
        Self {
            target,
            model: Model::new("resources/objects/cockpit/cockpit.obj"),
            termination_timer: None,
        }
    }

    /// Report on what the missile is doing this frame
    /// based on the information from the Enemy reference
    pub fn update(&self, target_info: &Enemy) -> MissileMessage {
        todo!()
    }

    /// Missile is no longer pointing close enough to the Enemy it targets
    pub fn lose_lock(&mut self) {
        assert!(self.target().is_some());
        self.target = None;
    }

    /// Only possible if the missile is not targeting an Enemy,
    /// but one flew in front of it close enough
    pub fn regain_lock(&mut self) {
        assert!(self.target.is_none());
        todo!()
    }

    /// Missile has flown without a target for too long, start the countdown
    pub fn begin_terminate(&mut self) {
        self.termination_timer = Some(TERMINATION_TIME);
    }

    pub fn target(&self) -> Option<EnemyID> {
        self.target
    }
}

impl Debug for Missile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Missile: {:?}, {:?}", self.target, self.termination_timer)
    }
}
