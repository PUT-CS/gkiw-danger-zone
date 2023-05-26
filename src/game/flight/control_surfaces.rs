use crate::{cg::camera::ControlSurfaces, gen_getters, DELTA_TIME};

const DECAY: f32 = 50.;

#[derive(Clone, Debug)]
/// Struct describing the mechanical state of the control parameters in the aircraft.
/// values of -1 and 1 indicate maximum flap rotation
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
            throttle: 0.1,
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
    /// set all decay values to a boolean
    pub fn set_all_decays(&mut self, b: bool) {
        self.decay = [b, b, b]
    }
    /// Compute a new value of the aircraft's pitch based on decay and set it as the new pitch bias
    pub fn apply_pitch_decay(&mut self) {
        let delta_time = unsafe { DELTA_TIME };
        self.pitch_bias = if self.pitch_bias.abs() > delta_time {
            round(
                self.pitch_bias + (DECAY * delta_time) * self.pitch_bias.signum() * -1.,
                5,
            )
        } else {
            0.
        }
    }
    /// Compute a new value of the aircraft's yaw based on decay and set it as the new yaw bias
    pub fn apply_yaw_decay(&mut self) {
        let delta_time = unsafe { DELTA_TIME };
        self.yaw_bias = if self.yaw_bias.abs() > delta_time {
            round(
                self.yaw_bias + (DECAY * delta_time) * self.yaw_bias.signum() * -1.,
                5,
            )
        } else {
            0.
        }
    }
    /// Compute a new value of the aircraft's roll based on decay and set it the as the new roll bias
    pub fn apply_roll_decay(&mut self) {
        let delta_time = unsafe { DELTA_TIME };
        self.roll_bias = if self.roll_bias.abs() > delta_time {
            round(
                self.roll_bias + (DECAY * delta_time) * self.roll_bias.signum() * -1.,
                5,
            )
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
