use super::{drawable::Drawable, enemy::Enemy, missile_guidance::GuidanceStatus, modeled::Modeled};
use crate::{
    cg::{camera::Camera, consts::VEC_RIGHT, model::Model, particles::ParticleGenerator},
    game::{flight::steerable::Steerable, particle_generation::ParticleGeneration},
    DELTA_TIME,
};
use cgmath::{EuclideanSpace, InnerSpace, MetricSpace, Point3, Quaternion, Vector3, Vector4};
use log::info;
use vek::{QuadraticBezier3, Vec3};

pub type EnemyID = u32;

pub enum MissileMessage {
    HitEnemy(EnemyID),
    BeganTermination,
}

/// Struct representing a missile fired by the player
/// The missile only knows what ID the Enemy it targets has.
/// Each frame it receives a reference to the enemy it targets so it can update its state.
pub struct Missile {
    pub model: Model,
    pub guidance: GuidanceStatus,
    particle_generator: ParticleGenerator,
}

impl Missile {
    /// Create a new missile.
    /// Uses player's position to spawn the missile at the right coordinates.
    pub fn new(camera: &Camera, target: Option<&Enemy>) -> Self {
        let mut model = Model::new("resources/objects/missile/missile.obj");
        let particle_generator = ParticleGenerator::new(1500, Vector4::new(1., 0., 0., 1.), 2.);

        model.apply_quaternion(camera.orientation_quat());

        let pos = camera.position().to_vec() + camera.up * -0.5;
        model.set_translation(pos);

        let guidance = if let Some(enemy) = target {
            let target = enemy.id();
            let start = model.position_vek();
            let end = enemy.aircraft().model().position_vek();
            let mid = {
                // Select a point in front of the launching aircraft to simulate the missile accelerating
                let mid = model.position() + camera.front * 50.;
                Vec3::from([mid.x, mid.y, mid.z])
            };
            let points = Vec3::from([start, mid, end]);
            let bezier = QuadraticBezier3::from(points);
            GuidanceStatus::new(target, bezier)
        } else {
            GuidanceStatus::none()
        };

        Self {
            model,
            guidance,
            particle_generator,
        }
    }

    /// Report on what the missile is doing this frame
    /// based on the information from the Enemy reference
    pub fn update(&mut self, enemy: Option<&Enemy>) -> Option<MissileMessage> {
        match (enemy, self.guidance) {
            (Some(e), GuidanceStatus::Active(_)) => {
                self.try_hit_target(e).or_else(|| self.guide_towards(e))
            }
            (None, GuidanceStatus::Active(_)) => self.begin_terminate(),
            (_, GuidanceStatus::None(timer)) => self.termination_countdown(timer),
        }
    }

    /// See if the missile should hit, return A message containing the enemy ID if it did.
    fn try_hit_target(&mut self, target: &Enemy) -> Option<MissileMessage> {
        if self.position().distance(target.position()) < 2. {
            info!("MISSILE HIT");
            self.guidance = GuidanceStatus::none();
            return Some(MissileMessage::HitEnemy(target.id()));
        }
        None
    }

    /// Move the missile towards its target along the bezier curve contained in GuidanceData
    fn guide_towards(&mut self, target: &Enemy) -> Option<MissileMessage> {
        assert!(matches!(self.guidance, GuidanceStatus::Active(_)));

        let guidance_data = if let GuidanceStatus::Active(data) = &mut self.guidance {
            data
        } else {
            unreachable!()
        };

        // Progress along the curve
        let t = {
            let bezier = guidance_data.bezier;
            let t = 0.001;
            let v1 = (2. * bezier.start) - (4. * bezier.ctrl) + (2. * bezier.end);
            let v2 = (-2. * bezier.start) + (2. * bezier.ctrl);
            let l = unsafe { DELTA_TIME };
            t + (l / (t * v1 + v2).magnitude())
        };
        guidance_data.progress += t;

        let new_point = {
            let eval = guidance_data.bezier.evaluate(guidance_data.progress);
            Vector3::from([eval.x, eval.y, eval.z])
        };

        guidance_data.bezier.end = target.aircraft().model().position_vek();
        self.model.set_translation(new_point);

        let vec_to_enemy = (target.position() - self.position()).normalize();
        let quat = Quaternion::from_arc(*VEC_RIGHT, vec_to_enemy, None);
        self.model.set_orientation(quat);
        self.model.yaw(-90.);
        None
    }

    /// Missile has flown without a target for too long, start the countdown
    pub fn begin_terminate(&mut self) -> Option<MissileMessage> {
        self.guidance = GuidanceStatus::none();
        Some(MissileMessage::BeganTermination)
    }

    /// Decrement the termination countdown
    fn termination_countdown(&mut self, timer: u32) -> Option<MissileMessage> {
        self.model.forward(100. * unsafe { DELTA_TIME });
        self.guidance = GuidanceStatus::None(timer - 1);
        None
    }

    pub fn target(&self) -> Option<EnemyID> {
        match &self.guidance {
            GuidanceStatus::None(_) => None,
            GuidanceStatus::Active(data) => Some(data.target_id),
        }
    }

    pub fn position(&self) -> Point3<f32> {
        self.model.position()
    }
}

impl Drawable for Missile {
    unsafe fn draw(&self, shader: &crate::cg::shader::Shader) {
        self.model.draw(shader);
    }
}

impl Modeled for Missile {
    fn model(&self) -> &Model {
        &self.model
    }
    fn model_mut(&mut self) -> &mut Model {
        &mut self.model
    }
}

impl ParticleGeneration for Missile {
    fn particle_generator(&self) -> &ParticleGenerator {
        &self.particle_generator
    }
    fn particle_generator_mut(&mut self) -> &mut ParticleGenerator {
        &mut self.particle_generator
    }
}
