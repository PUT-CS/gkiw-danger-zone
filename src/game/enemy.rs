use super::flight::aircraft::{Aircraft, AircraftKind};
use super::flight::steerable::Steerable;
use super::missile::EnemyID;
use super::terrain::Terrain;
use crate::cg::consts::VEC_RIGHT;
use crate::{gen_ref_getters, DELTA_TIME};
use cgmath::{EuclideanSpace, InnerSpace, Point3, Quaternion, Vector3};
use rand::{thread_rng, Rng};
use std::ops::Sub;
use vek::{QuadraticBezier3, Vec3};

/// Struct representing an enemy
pub struct Enemy {
    id: u32,
    pub aircraft: Aircraft,
    start_point: Vec3<f32>,
    end_point: Vec3<f32>,
    bezier: QuadraticBezier3<f32>,
    progress: f32,
}

impl Enemy {
    /// Create a new enemy with the given aircraft kind
    pub fn new(id: EnemyID, kind: AircraftKind) -> Self {
        let aircraft = Aircraft::new(kind);
        let random_mid = thread_rng().gen_range(10., 20.);
        let random_length = thread_rng().gen_range(30., 40.);
        let mid = {
            // Select a point in front of the launching aircraft to simulate the missile accelerating
            let mid = aircraft.model().position() + aircraft.model().front() * random_mid;
            Vec3::from([mid.x, mid.y, mid.z])
        };
        let start_point = aircraft.model().position_vek();
        let end_point = cgmath_to_vek(
            &(aircraft.model().position() + aircraft.model().front() * random_length).to_vec(),
        ) + Vec3::new(random_length, 0., random_length);
        let points = Vec3::from([start_point, mid, end_point]);

        Self {
            id,
            aircraft,
            start_point,
            end_point,
            bezier: QuadraticBezier3::from(points),
            progress: 0.,
        }
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
    pub fn fly(&mut self, terrain: &Terrain) {
        // Progress along the curve

        // The enemy has arrived at their destination and we should select a new one.
        if at_destination(self.aircraft().model().position_vek(), self.end_point) {
            self.progress = 0.;
            self.start_point = self.aircraft().model().position_vek();
            let random_mid = thread_rng().gen_range(100., 200.);
            let random_x = thread_rng().gen_range(terrain.bounds.x.start, terrain.bounds.x.end);
            let random_z = thread_rng().gen_range(terrain.bounds.z.start, terrain.bounds.z.end);
            let rand_height_offset = thread_rng().gen_range(40., 250.);
            let rand_coord = Vec3::<f32>::from([
                random_x as f32,
                terrain.height_at(&(random_x, random_z).into()) + rand_height_offset,
                random_z as f32,
            ]);
            let mid = {
                // Select a point in front of the aircraft to simulate turning
                let mid =
                    self.aircraft.model().position() + self.aircraft.model().front() * random_mid;
                Vec3::from([mid.x, mid.y, mid.z])
            };
            self.end_point = rand_coord;
            let points = Vec3::from([self.start_point, mid, self.end_point]);
            self.bezier = QuadraticBezier3::from(points);
        } else if !in_world_bounds(self.aircraft().model().position_vek(), terrain) {
            self.progress = 0.;
            self.start_point = self.aircraft().model().position_vek();
            let random_mid_distance = thread_rng().gen_range(30., 200.);
            let new_x = thread_rng().gen_range(-40., 40.);
            let new_y = thread_rng().gen_range(10., 20.);
            let new_z = thread_rng().gen_range(-40., 40.);

            let mid = {
                // Select a point in front of the launching aircraft to simulate the missile accelerating
                let mid = self.aircraft.model().position()
                    + self.aircraft.model().front() * random_mid_distance;
                Vec3::from([mid.x, mid.y, mid.z])
            };
            self.end_point = Vec3::new(new_x, new_y, new_z);
            let points = Vec3::from([self.start_point, mid, self.end_point]);
            self.bezier = QuadraticBezier3::from(points);
        };

        let t = {
            let bezier = self.bezier;
            let t = 0.0005;
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

        let vec_to_point = vek_to_cgmath(&self.end_point)
            .sub(self.position().to_vec())
            .normalize();

        let quat = Quaternion::from_arc(*VEC_RIGHT, vec_to_point, None);
        self.aircraft_mut().model_mut().set_orientation(quat);
        self.aircraft_mut().model_mut().yaw(-90.);
    }
}

fn in_world_bounds(pos: Vec3<f32>, terrain: &Terrain) -> bool {
    terrain.bounds.x.contains(&(pos.x as i32)) && terrain.bounds.z.contains(&(pos.z as i32))
}

fn at_destination(start: Vec3<f32>, end: Vec3<f32>) -> bool {
    let difference = end - start;
    difference[0].abs() < 0.2 && difference[1].abs() < 0.2 && difference[2].abs() < 0.2
}

fn vek_to_cgmath(v: &Vec3<f32>) -> Vector3<f32> {
    Vector3 {
        x: v.x,
        y: v.y,
        z: v.z,
    }
}

fn cgmath_to_vek(v: &Vector3<f32>) -> Vec3<f32> {
    Vec3 {
        x: v.x,
        y: v.y,
        z: v.z,
    }
}

gen_ref_getters! {
    Enemy,
    aircraft -> &Aircraft,
}
