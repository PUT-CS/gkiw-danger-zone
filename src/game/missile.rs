use super::drawable::Drawable;
use crate::game::flight::steerable::Steerable;
use crate::{cg::camera::Camera, cg::model::Model};
use cgmath::{Deg, EuclideanSpace, InnerSpace, Matrix3, Matrix4, Quaternion, SquareMatrix};
use log::warn;
use std::fmt::Debug;

pub type EnemyID = u32;

/// Number of frames after which a missile without a target gets deleted
const TERMINATION_TIME: u32 = 5000;

pub enum MissileMessage {
    LostLock,
    Terminated,
    None,
    HitEnemy(EnemyID),
}
 
/// Struct representing a missile fired by the player
/// The missile only knows what ID the Enemy it targets has.
/// Each frame it receives a reference to the enemy it targets so it can update its state.
pub struct Missile {
    target: Option<EnemyID>,
    pub model: Model,
    /// An optional integer representing the number of ticks left until termination.
    /// Set to TERMINATION_TIME by calling `terminate` on a Missile instance.
    pub termination_timer: Option<u32>,
}

impl Missile {
    /// Create a new missile.
    /// Uses player's position to spawn the missile at the right coordinates.
    pub fn new(camera: &Camera, target: Option<EnemyID>) -> Self {
        let mut model = Model::new("resources/objects/cockpit/cockpit.obj");

        let m = camera.view_matrix().invert().expect("Invertible view matrix");
        let rot = Matrix3::from([
            [m.x.x, m.x.y, m.x.z],
            [m.y.x, m.y.y, m.y.z],
            [m.z.x, m.z.y, m.z.z],
        ]);
        let quat = Quaternion::from(rot);
        model.apply_quaternion(quat);

        let pos = camera.position().to_vec();
        model.translate(pos);

        Self {
            target,
            model,
            termination_timer: None,
        }
    }

    /// Report on what the missile is doing this frame
    /// based on the information from the Enemy reference
    //pub fn update(&mut self, target_info: &Enemy) -> MissileMessage {
    pub fn update(&mut self) -> MissileMessage {
        self.model.forward(0.1);
        
        // Missile is already in termination countdown
        if let Some(timer) = self.termination_timer {
            if timer == 0 {
                warn!("Terminated a missile!");
                return MissileMessage::Terminated;
            }
            self.termination_timer = Some(timer - 1);
            return MissileMessage::None;
        }

        // Missile has just lost the target
        if self.target.is_none() {
            warn!("Starting termination");
            self.begin_terminate();
            return MissileMessage::None;
        }
        
        MissileMessage::None
    }

    /// Missile is no longer pointing close enough to the Enemy it targets
    pub fn lose_lock(&mut self) {
        assert!(self.target().is_some());
        assert!(self.termination_timer.is_none());
        self.target = None;
    }

    /// Only possible if the missile is not targeting an Enemy,
    /// but one flew in front of it close enough
    pub fn regain_lock(&mut self) {
        assert!(matches!(self.termination_timer, Some(_)));
        assert!(self.target.is_none());
        todo!()
    }

    /// Missile has flown without a target for too long, start the countdown
    pub fn begin_terminate(&mut self) {
        assert!(self.target.is_none());
        assert!(self.termination_timer.is_none());
        self.termination_timer = Some(TERMINATION_TIME);
    }

    pub fn target(&self) -> Option<EnemyID> {
        self.target
    }
}

impl Debug for Missile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Missile: {:?}, {:?}",
            self.target, self.termination_timer
        )
    }
}

impl Drawable for Missile {
    unsafe fn draw(&self, shader: &crate::cg::shader::Shader) {
        self.model.draw(shader);
    }
}
