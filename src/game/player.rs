use log::{info, warn};

use super::flight::aircraft::{Aircraft, AircraftKind};
use crate::cg::{
    camera::{Camera, Movement, Movement::*},
    shader::Shader,
};
use crate::game::flight::steerable::Steerable;

pub struct Player {
    aircraft: Aircraft,
    camera: Camera,
    kills: u32,
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
        }
    }
}

impl Player {
    pub fn new(aircraft_kind: AircraftKind) -> Self {
        Player {
            aircraft: Aircraft::new(aircraft_kind),
            camera: Camera::default(),
            kills: 0,
        }
    }
    pub unsafe fn draw(&self, shader: &Shader) {
        self.aircraft.model().draw(shader);
    }
    pub fn aircraft_mut(&mut self) -> &mut Aircraft {
        &mut self.aircraft
    }
    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    /// Modify the player's position based on the Controls
    pub fn apply_controls(&mut self) {
        let throttle = self.aircraft.controls().throttle();
        self.camera_mut().forward(throttle)
    }

    pub fn process_key(&mut self, direction: Movement, delta_time: f32) {
        let velocity = self.camera.movement_speed() * delta_time;
        if direction == PitchUp {
            self.aircraft_mut().pitch(velocity);
            self.camera_mut().pitch(velocity);
        }
        if direction == PitchDown {
            self.aircraft_mut().pitch(-velocity);
            self.camera_mut().pitch(-velocity);
        }
        if direction == YawLeft {
            self.aircraft_mut().yaw(velocity);
            self.camera_mut().yaw(velocity)
        }
        if direction == YawRight {
            self.aircraft_mut().yaw(-velocity);
            self.camera_mut().yaw(-velocity)
        }
        if direction == RollRight {
            self.aircraft_mut().roll(0.1);
            self.camera.roll(0.1);
        }
        if direction == RollLeft {
            self.aircraft_mut().roll(-0.1);
            self.camera.roll(-0.1);
        }
        dbg!(&self.aircraft().controls());
        self.camera.update_view_matrix();
    }
}
