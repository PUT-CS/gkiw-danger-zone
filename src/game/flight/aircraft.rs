use super::{control_surfaces::Controls, spec::AircraftSpec, steerable::Steerable};
use crate::cg::model::Model;
pub use paste::paste;
use std::collections::HashMap;
use AircraftKind::*;

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

#[derive(Hash, PartialEq, Eq)]
pub enum AircraftKind {
    Mig21,
}

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
            controls: Controls::default(),
        }
    }
    pub fn controls_mut(&mut self) -> &mut Controls {
        &mut self.controls
    }
}

impl Steerable for Aircraft {
    // modify Controls property according to AircraftSpec
    fn pitch(&mut self, amount: f32) {
        todo!()
    }
    fn roll(&mut self, amount: f32) {
        todo!()
    }
    fn yaw(&mut self, amount: f32) {
        todo!()
    }
    fn forward(&mut self, amount: f32) {
        todo!()
    }
}
