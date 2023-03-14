#![allow(dead_code)]

use super::{spec::AircraftSpec, steerable::Steerable};
use crate::cg::model::Model;
pub use paste::paste;
use std::collections::HashMap;

macro_rules! gen_ref_getters {
    {$t:ty, $($field:ident -> $type:ty,)+} => {
        impl $t {
            $(
                pub fn $field(&self) -> $type {
                    &self.$field
                }
            )+
        }
    };
}

pub struct Aircraft {
    model: Model,
    spec: AircraftSpec,
    control_surfaces: ControlSurfaces,
    kind: AircraftKind,
}

gen_ref_getters! {
    Aircraft,
    model -> &Model,
    spec -> &AircraftSpec,
    kind -> &AircraftKind,
}

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

#[derive(Hash, PartialEq, Eq)]
pub enum AircraftKind {
    Mig21,
}
use AircraftKind::*;

use lazy_static::lazy_static;
lazy_static! {
    static ref BLUEPRINTS: HashMap<AircraftKind, AircraftSpec> =
        HashMap::from([(Mig21, AircraftSpec::new([0.1, 0.1, 0.1]))]);
    static ref MODEL_PATHS: HashMap<AircraftKind, &'static str> =
        HashMap::from([(Mig21, "resources/objects/mig21/mig21.obj")]);
}

impl Aircraft {
    pub fn new(kind: AircraftKind) -> Self {
        Aircraft {
            model: Model::new(MODEL_PATHS.get(&kind).expect("Path not found for kind")),
            spec: BLUEPRINTS
                .get(&kind)
                .expect("Blueprint not found for kind")
                .to_owned(),
            kind,
            control_surfaces: ControlSurfaces::default(),
        }
    }
}

impl Steerable for Aircraft {
    fn pitch(&mut self, amount: f32) {
        todo!()
    }
    fn roll(&mut self, amount: f32) {
        todo!()
    }
    fn yaw(&mut self, amount: f32) {
        todo!()
    }
    fn throttle_up(&mut self, amount: f32) {
        todo!()
    }
    fn throttle_down(&mut self, amount: f32) {
        todo!()
    }
}
