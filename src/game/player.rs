use crate::cg::{camera::Camera, shader::Shader};
use super::flight::aircraft::{Aircraft, AircraftKind};

pub struct Player {
    aircraft: Aircraft,
    camera: Camera,
    kills: u32,
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
    pub fn aircraft(&self) -> &Aircraft {
        &self.aircraft
    }
    pub fn camera(&self) -> &Camera {
        &self.camera
    }
    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }
    pub fn kills(&self) -> u32 {
        self.kills
    }
}
