use super::{control_surfaces::Controls, spec::AircraftSpec, steerable::Steerable};
use crate::{cg::{camera::ControlSurfaces, model::Model}, gen_ref_getters};
use cgmath::{Vector3, Deg};
use lazy_static::lazy_static;
use log::info;
use std::collections::HashMap;
use AircraftKind::*;

const MAX_PITCH_BIAS: f32 = 0.25;
const MAX_YAW_BIAS: f32 = 0.20;
const MAX_ROLL_BIAS: f32 = 0.70;

/// Struct representing an aircraft which can be steered and displayed
#[derive(Clone, Debug)]
pub struct Aircraft {
    model: Model,
    spec: AircraftSpec,
    controls: Controls,
    kind: AircraftKind,
}

gen_ref_getters! {
    Aircraft,
    model -> &Model,
    spec -> &AircraftSpec,
    kind -> &AircraftKind,
    controls -> &Controls,
}

#[derive(Hash, PartialEq, Eq, Debug)]
/// Defines aircraft models available
#[derive(Clone)]
pub enum AircraftKind {
    Mig21,
    F16,
}

lazy_static! {
    static ref BLUEPRINTS: HashMap<AircraftKind, AircraftSpec> =
        HashMap::from([(Mig21, AircraftSpec::new([0.0003, 0.0005, 0.0005]))]);
    static ref MODEL_PATHS: HashMap<AircraftKind, &'static str> =
        HashMap::from([(Mig21, "resources/objects/mig21/mig21.obj")]);
}

impl Aircraft {
    pub fn new(kind: AircraftKind) -> Self {
        info!("Creating new Aircraft of kind : {kind:?}");
        let mut model = Model::new(MODEL_PATHS.get(&kind).expect("Path not found for kind"));
        //model.yaw(1.);
        Aircraft {
            model,
            spec: BLUEPRINTS
                .get(&kind)
                .expect("Blueprint not found for kind")
                .to_owned(),
            kind,
            controls: Controls::default(),
        }
    }
    pub fn controls_mut(&mut self) -> &mut Controls {
        &mut self.controls
    }
    pub fn model_mut(&mut self) -> &mut Model {
        &mut self.model
    }

    pub fn set_decay(&mut self, c: ControlSurfaces, b: bool) {
        self.controls_mut().set_decay(c, b);
    }

    /// Mutate the control parameters of the aircraft by making all the flap
    /// biases closer to zero if that particular surface was not used by the player in this frame
    pub fn apply_decay(&mut self) {
        if self.controls().decay()[ControlSurfaces::Pitch as usize] {
            self.controls_mut().apply_pitch_decay()
        }
        if self.controls().decay()[ControlSurfaces::Yaw as usize] {
            self.controls_mut().apply_yaw_decay()
        }
        if self.controls().decay()[ControlSurfaces::Roll as usize] {
            self.controls_mut().apply_roll_decay()
        }
    }
}
/// This implementation handles mutating the control parameters of the aircraft.
/// It does not modify the actual model of the plane, only sets values
/// which are later used to calculate the actual rotation of the model
impl Steerable for Aircraft {
    /// Mutate the pitch flaps bias
    fn pitch(&mut self, amount: f32) {
        *self.controls_mut().pitch_bias_mut() = (self.controls().pitch_bias()
            + self.spec().pitch_rate() * amount.signum())
        .clamp(-MAX_PITCH_BIAS, MAX_PITCH_BIAS);
    }
    /// Mutate the yaw flaps bias
    fn yaw(&mut self, amount: f32) {
        *self.controls_mut().yaw_bias_mut() = (self.controls().yaw_bias()
            + self.spec().yaw_rate() * amount.signum())
        .clamp(-MAX_YAW_BIAS, MAX_YAW_BIAS);
    }
    /// Mutate the roll flaps bias
    fn roll(&mut self, amount: f32) {
        *self.controls_mut().roll_bias_mut() = (self.controls().roll_bias()
            + self.spec().roll_rate() * amount.signum())
        .clamp(-MAX_ROLL_BIAS, MAX_ROLL_BIAS);
    }
    /// Mutate the throttle
    fn forward(&mut self, amount: f32) {
        *self.controls_mut().throttle_mut() += amount
    }
}
