#[derive(Clone, Copy)]
pub struct AircraftSpec {
    pitch_rate: f32,
    yaw_rate: f32,
    roll_rate: f32,
}

impl AircraftSpec {
    pub const fn new(v: [f32; 3]) -> Self {
        AircraftSpec {
            pitch_rate: v[0],
            yaw_rate: v[1],
            roll_rate: v[2],
        }
    }
    pub fn pitch_rate(&self) -> f32 {
        self.pitch_rate
    }
    pub fn yaw_rate(&self) -> f32 {
        self.yaw_rate
    }
    pub fn roll_rate(&self) -> f32 {
        self.roll_rate
    }
}
