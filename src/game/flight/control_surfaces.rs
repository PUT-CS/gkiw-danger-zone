use crate::cg::camera::ControlSurfaces;

const DECAY: f32 = 0.001;

#[derive(Clone, Debug)]
pub struct Controls {
    pitch_bias: f32,
    yaw_bias: f32,
    roll_bias: f32,
    throttle: f32,
    decay: [bool; 3],
}

impl Default for Controls {
    fn default() -> Self {
        Controls {
            pitch_bias: 0.,
            yaw_bias: 0.,
            roll_bias: 0.,
            throttle: 0.05,
            decay: [true, true, true],
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
    pub fn decay(&self) -> &[bool; 3] {
        &self.decay
    }
    pub fn set_decay(&mut self, surface: ControlSurfaces, b: bool) {
        self.decay[surface as usize] = b;
    }
    pub fn set_all_decays(&mut self, b: bool) {
        self.decay = [b, b, b]
    }
    pub fn apply_pitch_decay(&mut self) {
        self.pitch_bias = if self.pitch_bias.abs() > DECAY {
            round(self.pitch_bias + DECAY * self.pitch_bias.signum() * -1., 3)
        } else {
            0.
        }
    }
    pub fn apply_yaw_decay(&mut self) {
        self.yaw_bias = if self.yaw_bias.abs() > DECAY {
            round(self.yaw_bias + DECAY * self.yaw_bias.signum() * -1., 3)
        } else {
            0.
        }
    }
    pub fn apply_roll_decay(&mut self) {
        self.roll_bias = if self.roll_bias.abs() > DECAY {
            round(self.roll_bias + DECAY * self.roll_bias.signum() * -1., 3)
        } else {
            0.
        }
    }
}

gen_getters! {
    Controls,
    pitch_bias -> f32,
    yaw_bias -> f32,
    roll_bias -> f32,
    throttle -> f32,
}

pub fn round(number: f32, rounding: i32) -> f32 {
    let scale: f32 = 10_f32.powi(rounding);
    (number * scale).round() / scale
}
