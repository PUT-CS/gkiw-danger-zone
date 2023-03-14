pub struct ControlSurfaces {
    pitch_bias: f32,
    yaw_bias: f32,
    roll_bias: f32,
}

impl Default for ControlSurfaces {
    fn default() -> Self {
        ControlSurfaces {
            pitch_bias: 0.,
            yaw_bias: 0.,
            roll_bias: 0.,
        }
    }
}
