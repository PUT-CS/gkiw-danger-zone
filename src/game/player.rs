use crate::game::flight::steerable::Steerable;
use crate::cg::{camera::{Camera, Movement ,Movement::*}, shader::Shader};
use super::flight::aircraft::{Aircraft, AircraftKind};

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

impl Player {
    pub fn new(aircraft_kind: AircraftKind) -> Self {
        Player {
            aircraft: Aircraft::new(aircraft_kind),
            camera: Camera::default(),
            kills: 0
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
            self.camera_mut().pitch(velocity);
        }
        if direction == PitchDown {
            self.camera_mut().pitch(-velocity);
        }
        if direction == YawLeft {
            self.camera_mut().yaw(velocity / 2.)
        }
        if direction == YawRight {
            self.camera_mut().yaw(-velocity / 2.)
        }
        if direction == RollRight {
            self.camera.roll(0.1);
        }
        if direction == RollLeft {
            self.camera.roll(-0.1);
        }
        self.camera.update_view_matrix();
    }
    
}
