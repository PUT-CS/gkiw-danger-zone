use super::{control_surfaces::Controls, spec::AircraftSpec, steerable::Steerable};
use crate::{
    c_str,
    cg::{camera::{ControlSurfaces, Camera}, model::Model, particles::ParticleGenerator, light::PointLight},
    game::{
        drawable::Drawable, guns::Guns, modeled::Modeled, particle_generation::ParticleGeneration, game::TARGET_ENEMIES,
    },
    gen_ref_getters, DELTA_TIME,
};
use cgmath::{Deg, EuclideanSpace, Quaternion, Vector3, Vector4};
use lazy_static::lazy_static;
use log::info;
use std::collections::HashMap;
use std::ffi::CStr;
use AircraftKind::*;

const MAX_PITCH_BIAS: f32 = 35.;
const MAX_YAW_BIAS: f32 = 20.;
const MAX_ROLL_BIAS: f32 = 70.;

/// Struct representing an aircraft which can be steered and displayed
#[derive(Debug)]
pub struct Aircraft {
    model: Model,
    spec: AircraftSpec,
    controls: Controls,
    kind: AircraftKind,
    particle_generator: ParticleGenerator,
    guns: Guns,
}

gen_ref_getters! {
    Aircraft,
    model -> &Model,
    spec -> &AircraftSpec,
    kind -> &AircraftKind,
    controls -> &Controls,
    guns -> &Guns,
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
        HashMap::from([(Mig21, AircraftSpec::new([0.03, 0.05, 0.05]))]);
    static ref MODEL_PATHS: HashMap<AircraftKind, &'static str> =
        HashMap::from([(Mig21, "resources/objects/mig21/mig21.obj")]);
}

impl ParticleGeneration for Aircraft {
    fn particle_generator(&self) -> &ParticleGenerator {
        &self.particle_generator
    }
    fn particle_generator_mut(&mut self) -> &mut ParticleGenerator {
        &mut self.particle_generator
    }
}

impl Drawable for Aircraft {
    unsafe fn draw(&self, shader: &crate::cg::shader::Shader) {
	self.model().draw(shader);
    }
}

impl Modeled for Aircraft {
    fn model(&self) -> &Model {
        &self.model
    }
    fn model_mut(&mut self) -> &mut Model {
        &mut self.model
    }
}

impl Aircraft {
    pub fn new(kind: AircraftKind) -> Self {
        info!("Creating new Aircraft of kind : {kind:?}");
        let model = Model::new(MODEL_PATHS.get(&kind).expect("Path not found for kind"));
        let mut particle_generator = ParticleGenerator::new(1500, Vector4::new(1., 0., 0., 1.), 2.);
        //particle_generator.disable();
        Aircraft {
            model,
            spec: BLUEPRINTS
                .get(&kind)
                .expect("Blueprint not found for kind")
                .to_owned(),
            controls: Controls::default(),
            kind,
            particle_generator,
            guns: Guns::new(),
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

    pub fn fire_guns(&mut self, camera: &Camera) {
        self.guns.fire(camera)
    }

    pub fn guns_mut(&mut self) -> &mut Guns {
        &mut self.guns
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

    pub fn throttle_up(&mut self) {
        let delta_time = unsafe { DELTA_TIME };
        *self.controls_mut().throttle_mut() =
            (self.controls().throttle() + delta_time).clamp(10., 1000.)
    }

    pub fn throttle_down(&mut self) {
        let delta_time = unsafe { DELTA_TIME };
        *self.controls_mut().throttle_mut() =
            (self.controls().throttle() - delta_time).clamp(10., 1000.)
    }
}
/// This implementation handles mutating the control parameters of the aircraft.
/// It does not modify the actual model of the plane, only sets values
/// which are later used to calculate the actual rotation of the model
const STEERING_SENSITIVITY: f32 = 1500.;
impl Steerable for Aircraft {
    /// Mutate the pitch flaps bias
    fn pitch(&mut self, amount: f32) {
        *self.controls_mut().pitch_bias_mut() = (self.controls().pitch_bias()
            + self.spec().pitch_rate() * amount.signum() * amount.abs() * STEERING_SENSITIVITY)
            .clamp(-MAX_PITCH_BIAS, MAX_PITCH_BIAS);
    }
    /// Mutate the yaw flaps bias
    fn yaw(&mut self, amount: f32) {
        *self.controls_mut().yaw_bias_mut() = (self.controls().yaw_bias()
            + self.spec().yaw_rate() * amount.signum() * amount.abs() * STEERING_SENSITIVITY)
            .clamp(-MAX_YAW_BIAS, MAX_YAW_BIAS);
    }
    /// Mutate the roll flaps bias
    fn roll(&mut self, amount: f32) {
        *self.controls_mut().roll_bias_mut() = (self.controls().roll_bias()
            + self.spec().roll_rate() * amount.signum() * amount.abs() * STEERING_SENSITIVITY)
            .clamp(-MAX_ROLL_BIAS, MAX_ROLL_BIAS);
    }
    /// Mutate the throttle
    fn forward(&mut self, amount: f32) {            // TEMPORARY
        *self.controls_mut().throttle_mut() += amount * 500. * STEERING_SENSITIVITY
    }
}
