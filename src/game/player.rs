use super::flight::aircraft::{Aircraft, AircraftKind};
use crate::game::drawable::Drawable;
use crate::cg::{
    camera::{Camera, ControlSurfaces, Movement, Movement::*},
    model::Model,
    shader::Shader,
};
use crate::game::flight::steerable::Steerable;

#[derive(Clone, Debug)]
pub struct Player {
    aircraft: Aircraft,
    camera: Camera,
    pub cockpit: Model,
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
            cockpit: Model::new("resources/objects/cockpit/cockpit_old.obj"),
        }
    }
}

impl Drawable for Player {
    unsafe fn draw(&self, shader: &Shader) {
        self.aircraft.model().draw(shader);
    } 
}

impl Player {
    pub fn new(aircraft_kind: AircraftKind) -> Self {
        Player {
            aircraft: Aircraft::new(aircraft_kind),
            camera: Camera::default(),
            kills: 0,
            cockpit: Model::new("resources/objects/cockpit/cockpit.obj"),
        }
    }
    pub fn aircraft_mut(&mut self) -> &mut Aircraft {
        &mut self.aircraft
    }
    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    /// Modify the player's position and camera based on the Controls
    pub fn apply_controls(&mut self, delta_time: f32) {
        let controls = self.aircraft.controls().clone();
        self.camera_mut()
            .pitch(controls.pitch_bias() * delta_time);
        self.camera_mut()
            .yaw(controls.yaw_bias() * delta_time);
        self.camera_mut()
            .roll(controls.roll_bias() * delta_time);
        //self.camera_mut().forward(controls.throttle());
    }

    /// Handle key events meant for player controls.
    pub fn process_key(&mut self, direction: Movement, delta_time: f32) {
        //let velocity = self.camera.movement_speed() * delta_time;
        let velocity = delta_time;
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
                *self.aircraft.controls_mut().throttle_mut() =
                    (self.aircraft.controls().throttle() + 0.0003).clamp(0.1, 1.)
            }
            ThrottleDown => {
                *self.aircraft.controls_mut().throttle_mut() =
                    (self.aircraft.controls().throttle() - 0.0003).clamp(0.1, 1.)
            }
        }
        self.camera.update_view_matrix();
    }
}
