use cgmath::{vec3, InnerSpace, Rotation};
use itertools::Itertools;
use log::{error, warn};
use super::enemies::Enemies;
use super::flight::aircraft::{Aircraft, AircraftKind};
use super::missile::EnemyID;
use crate::audio::sound::SoundID;
use crate::game::flight::steerable::Steerable;
use crate::gen_ref_getters;
use crate::{
    cg::{
        camera::{Camera, ControlSurfaces, Movement, Movement::*},
        consts::VEC_FRONT,
        model::Model,
    },
    DELTA_TIME,
};

#[derive(Debug)]
pub struct Player {
    aircraft: Aircraft,
    camera: Camera,
    pub cockpit: Model,
    kills: u32,
    pub guns_sound: SoundID,
}

gen_ref_getters! {
    Player,
    aircraft -> &Aircraft,
    camera -> &Camera,
    kills -> &u32,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            aircraft: Aircraft::new(AircraftKind::Mig21),
            camera: Camera::default(),
            kills: 0,
            cockpit: Model::new("resources/objects/cockpit/cockpit_old.obj"),
            guns_sound: SoundID::MAX,
        }
    }
}

impl Player {
    pub fn new(aircraft_kind: AircraftKind) -> Self {
        Player {
            aircraft: Aircraft::new(aircraft_kind),
            camera: Camera::default(),
            kills: 0,
            cockpit: Model::new("resources/objects/cockpit/cockpit.obj"),
            guns_sound: SoundID::MAX,
        }
    }
    pub fn aircraft_mut(&mut self) -> &mut Aircraft {
        &mut self.aircraft
    }
    pub fn cockpit_mut(&mut self) -> &mut Model {
        &mut self.cockpit
    }
    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    /// Modify the player's position and camera based on the Controls
    pub fn apply_controls(&mut self) {
        let delta_time = unsafe { DELTA_TIME };
        let c = self.aircraft.controls().clone();
        self.camera_mut().pitch(c.pitch_bias() * delta_time);
        self.camera_mut().yaw(c.yaw_bias() * delta_time);
        self.camera_mut().roll(c.roll_bias() * delta_time);
        self.camera_mut().forward(c.throttle() * delta_time);

        let model = self.aircraft_mut().model_mut();

        model.pitch(c.pitch_bias() * delta_time);
        model.yaw(c.yaw_bias() * delta_time);
        model.roll(c.roll_bias() * delta_time);
        model.forward(c.throttle() * delta_time);

        //Third person camera (not looking really good now)
        // self.camera.position = self.aircraft().model().position()
        //     + (self
        //         .aircraft()
        //         .model()
        //         .orientation
        //         .rotate_vector(*VEC_FRONT - vec3(-0.05, -0.5, -5.0)))
    }

    /// Check if the player aims their nose at an enemy, triggering a missile lock
    /// countdown on one of them (lock not implemented yet)
    pub fn targeted_enemy_id_nth(&self, enemies: &Enemies, n: usize) -> Option<EnemyID> {
        if let Some(enemies) = self.targetable_enemies(enemies) {
            if enemies.len() <= n {
                warn!("Requested enemy {n}, but there are only {}. Returning the last enemy available", enemies.len());
            }
            let id = enemies.get(n).unwrap_or_else(|| enemies.last().unwrap());
            return Some(*id)
        }
        None
    }

    fn targetable_enemies(&self, enemies: &Enemies) -> Option<Vec<EnemyID>> {
        let player_front = self.camera().front;
        let player_position = self.camera().position;
        
        let targeted = enemies
            .map
            .iter()
            .map(|tuple| {
                let enemy = tuple.1;
                let pos = enemy.aircraft().model().position();
                let direction = (pos - player_position).normalize();
                let deg = direction.angle(player_front).0.to_degrees();
                (tuple.0, (deg, enemy))
            })
            .filter(|&(_, (deg, _))| deg < 20.)
            .sorted_by(|t1, t2| t1.1 .0.partial_cmp(&t2.1 .0).unwrap())
            .map(|(id, _)| *id)
            .collect_vec();
        return if targeted.is_empty() {
            None
        } else {
            Some(targeted)
        }
    }

    /// Handle key events meant for player controls.
    pub fn process_key(&mut self, direction: Movement) {
        let velocity = unsafe { DELTA_TIME };
        match direction {
            PitchUp => {
                self.aircraft_mut().pitch(velocity);
                self.aircraft_mut().set_decay(ControlSurfaces::Pitch, false);
            }
            PitchDown => {
                self.aircraft_mut().pitch(-velocity);
                self.aircraft_mut().set_decay(ControlSurfaces::Pitch, false);
            }
            YawLeft => {
                self.aircraft_mut().yaw(velocity);
                self.aircraft_mut().set_decay(ControlSurfaces::Yaw, false);
            }
            YawRight => {
                self.aircraft_mut().yaw(-velocity);
                self.aircraft_mut().set_decay(ControlSurfaces::Yaw, false);
            }
            RollLeft => {
                self.aircraft_mut().roll(-velocity);
                self.aircraft_mut().set_decay(ControlSurfaces::Roll, false);
            }
            RollRight => {
                self.aircraft_mut().roll(velocity);
                self.aircraft_mut().set_decay(ControlSurfaces::Roll, false);
            }
            ThrottleUp => {
                self.aircraft_mut().throttle_up();
            }
            ThrottleDown => {
                self.aircraft_mut().throttle_down();
            }
        }
    }
}
