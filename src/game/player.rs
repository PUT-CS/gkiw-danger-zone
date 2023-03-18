use std::sync::Arc;

use super::flight::aircraft::{Aircraft, AircraftKind};
use crate::cg::{
    camera::{Camera, ControlSurfaces, Movement, Movement::*},
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

    /// Modify the player's position and camera based on the Controls
    pub fn apply_controls(&mut self) {
        let steering_constant = 0.1;
        let throttle = self.aircraft.controls().throttle();
        let controls = self.aircraft.controls().clone();
        self.camera_mut().forward(throttle);
        self.camera_mut()
            .pitch(controls.pitch_bias() * steering_constant / 2.);
        self.camera_mut()
            .yaw(controls.yaw_bias() * steering_constant / 7.);
        self.camera_mut()
            .roll(controls.roll_bias() * steering_constant * 1.5);
    }

    pub fn process_key(&mut self, direction: Movement, delta_time: f32) {
        let velocity = self.camera.movement_speed() * delta_time;
        if direction == PitchUp {
            self.aircraft_mut().pitch(velocity);
            self.aircraft_mut()
                .controls_mut()
                .set_decay(ControlSurfaces::Pitch, false);
        }
        if direction == PitchDown {
            self.aircraft_mut().pitch(-velocity);
            self.aircraft_mut()
                .controls_mut()
                .set_decay(ControlSurfaces::Pitch, false);
        }
        if direction == YawLeft {
            self.aircraft_mut().yaw(velocity);
            self.aircraft_mut()
                .controls_mut()
                .set_decay(ControlSurfaces::Yaw, false);
        }
        if direction == YawRight {
            self.aircraft_mut().yaw(-velocity);
            self.aircraft_mut()
                .controls_mut()
                .set_decay(ControlSurfaces::Yaw, false);
        }
        if direction == RollRight {
            self.aircraft_mut().roll(0.1);
            self.aircraft_mut()
                .controls_mut()
                .set_decay(ControlSurfaces::Roll, false);
        }
        if direction == RollLeft {
            self.aircraft_mut().roll(-0.1);
            self.aircraft_mut()
                .controls_mut()
                .set_decay(ControlSurfaces::Roll, false);
        }
        if direction == ThrottleUp {
            *self.aircraft.controls_mut().throttle_mut() =
                (self.aircraft.controls().throttle() + 0.0003).clamp(0.1, 1.)
        }
        if direction == ThrottleDown {
            *self.aircraft.controls_mut().throttle_mut() =
                (self.aircraft.controls().throttle() - 0.0003).clamp(0.1, 1.)
        }
        dbg!(&self.aircraft().controls());
        self.camera.update_view_matrix();
    }
}
