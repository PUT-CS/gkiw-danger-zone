use std::ops::Sub;

use super::flight::aircraft::{self, Aircraft, AircraftKind};
use super::flight::steerable::Steerable;
use super::missile::EnemyID;
use crate::cg::consts::{VEC_RIGHT};
use crate::{gen_ref_getters, DELTA_TIME};
use cgmath::{Point3, Quaternion, Vector3, EuclideanSpace, InnerSpace};
use image::pnm::ArbitraryHeader;
use rand::{thread_rng, Rng};
use vek::{QuadraticBezier3, Vec3};

/// Struct representing an enemy
pub struct Enemy {
    id: u32,
    pub aircraft: Aircraft,
    destroyed: bool,
    start_point: Vec3<f32>,
    end_point: Vec3<f32>,
    bezier: QuadraticBezier3<f32>,
    progress: f32,
}

impl Enemy {
    /// Create a new enemy with the given aircraft kind
    pub fn new(id: EnemyID, kind: AircraftKind) -> Self {
        let aircraft = Aircraft::new(kind);
	let rand1 = thread_rng().gen_range(-30., 30.);
        let rand2 = thread_rng().gen_range(20., 40.);
        let rand3 = thread_rng().gen_range(-30., 30.);
        let mid = {
            // Select a point in front of the launching aircraft to simulate the missile accelerating
            let mid = aircraft.model().position() + aircraft.model().front() * 50.;
            Vec3::from([mid.x, mid.y, mid.z])
        };
        let start_point = Vec3::new(0., 0., 0.);
        let end_point = Vec3::new(rand1, rand2, rand3);
        let points = Vec3::from([start_point, mid, end_point]);

        Self {
            id,
            aircraft,
            destroyed: false,
            start_point,
            end_point,
            bezier: QuadraticBezier3::from(points),
	    progress: 0.,
        }
    }
    pub fn destroy(&mut self) {
        self.destroyed = true
    }
    pub fn aircraft_mut(&mut self) -> &mut Aircraft {
        &mut self.aircraft
    }
    pub fn position(&self) -> Point3<f32> {
        self.aircraft().model().position()
    }
    pub fn id(&self) -> EnemyID {
        self.id
    }
    pub fn fly(&mut self) {
        // Progress along the curve
	if should_change_route(self.aircraft().model().position_vek(), self.end_point) {
	    self.progress = 0.;
	    self.start_point = self.aircraft().model().position_vek();
	    let rand1 = thread_rng().gen_range(-30., 30.);
            let rand2 = thread_rng().gen_range(20., 40.);
            let rand3 = thread_rng().gen_range(-30., 30.);
            let mid = {
		// Select a point in front of the launching aircraft to simulate the missile accelerating
		let mid = self.aircraft.model().position() + self.aircraft.model().front() * 20.;
		Vec3::from([mid.x, mid.y, mid.z])
            };
	    self.end_point = Vec3::new(rand1, rand2, rand3);
	    let points = Vec3::from([self.start_point, mid, self.end_point]);
	    self.bezier = QuadraticBezier3::from(points);
	}
        let t = {
            let bezier = self.bezier;
            let t = 0.001;
            let v1 = (2. * bezier.start) - (4. * bezier.ctrl) + (2. * bezier.end);
            let v2 = (-2. * bezier.start) + (2. * bezier.ctrl);
            let l = unsafe { DELTA_TIME };
            t + (l / (t * v1 + v2).magnitude())
        };
        self.progress += t;

        let new_point = {
            let eval = self.bezier.evaluate(self.progress);
            Vector3::from([eval.x, eval.y, eval.z])
        };

        self.aircraft_mut().model_mut().set_translation(new_point);

        let vec_to_point = vek_to_cgmath(&self.end_point).sub(self.position().to_vec()).normalize();
        let quat = Quaternion::from_arc(*VEC_RIGHT, vec_to_point, None);
        self.aircraft_mut().model_mut().set_orientation(quat);
	self.aircraft_mut().model_mut().yaw(-90.);
    }
}

fn should_change_route(start: Vec3<f32>, end: Vec3<f32>) -> bool{
    let difference = end - start;
    dbg!(&difference);
    if difference[0] < 0.2 && difference[1] < 0.2 && difference[2] < 0.2 {
	return true;
    }
    false
}

fn vek_to_cgmath(v: &Vec3<f32>) -> Vector3<f32> {
    Vector3 { x: v.x, y: v.y, z: v.z }
}

gen_ref_getters! {
    Enemy,
    aircraft -> &Aircraft,
}
