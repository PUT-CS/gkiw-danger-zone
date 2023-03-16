#[derive(Clone, Debug)]
pub struct Controls {
    pitch_bias: f32,
    yaw_bias: f32,
    roll_bias: f32,
    throttle: f32,
}

impl Default for Controls {
    fn default() -> Self {
        Controls {
            pitch_bias: 0.,
            yaw_bias: 0.,
            roll_bias: 0.,
            throttle: 0.05,
        }
    }
}

impl Controls {
    pub fn pitch_bias_mut(&mut self) -> &mut f32 {
        &mut self.pitch_bias
    }
    pub fn yaw_bias_mut(&mut self) -> &mut f32 {
        &mut self.yaw_bias
    }
    pub fn roll_bias_mut(&mut self) -> &mut f32 {
        &mut self.roll_bias
    }
    pub fn throttle_mut(&mut self) -> &mut f32 {
        &mut self.throttle
    }
}

gen_getters! {
    Controls,
    pitch_bias -> f32,
    yaw_bias -> f32,
    roll_bias -> f32,
    throttle -> f32,
}
