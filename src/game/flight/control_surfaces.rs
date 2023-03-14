pub struct Controls {
    pitch_bias: f32,
    yaw_bias: f32,
    roll_bias: f32,
    throttle: f32
}

impl Default for Controls {
    fn default() -> Self {
        Controls {
            pitch_bias: 0.,
            yaw_bias: 0.,
            roll_bias: 0.,
            throttle: 0.
        }
    }
}
