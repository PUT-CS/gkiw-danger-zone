use super::{
    drawable::Drawable,
    enemy::Enemy,
    flight::steerable::Steerable,
    game::{ID_GENERATOR, TARGET_ENEMIES},
    missile::EnemyID,
};
use crate::game::flight::aircraft::AircraftKind::*;
use crate::game::id_gen::IDKind;
use log::warn;
use rand::{thread_rng, Rng};
use std::collections::HashMap;

pub struct Enemies {
    pub map: HashMap<EnemyID, Enemy>,
}

impl Default for Enemies {
    fn default() -> Self {
        let mut e = Self {
            map: HashMap::with_capacity(TARGET_ENEMIES),
        };
        e.respawn_enemies();
        e.randomize_positions();
        e
    }
}

impl Enemies {
    pub fn respawned_enemies(&mut self) -> Option<HashMap<EnemyID, Enemy>> {
        let diff = TARGET_ENEMIES - self.map.len();
        if diff == 0 {
            return None;
        }
        warn!("Respawning {diff} enemies");
        let mut id_gen = ID_GENERATOR.lock().expect("Lock IDGenerator mutex");
        Some(
            (0..diff)
                .map(|_| (id_gen.get_new_id_of(IDKind::Enemy), Enemy::new(Mig21)))
                .collect(),
        )
    }

    pub fn respawn_enemies(&mut self) {
        if let Some(enemies) = self.respawned_enemies() {
            self.map.extend(enemies);
        }
    }

    pub fn randomize_positions(&mut self) {
        assert!(!self.map.is_empty());
        self.map.values_mut().for_each(|e| {
            // TEMPORARY
            let a = e.aircraft_mut().model_mut();
            let amount = thread_rng().gen_range(-360., 360.);
            a.roll(amount);
            a.yaw(amount);
            a.pitch(amount);
        });
    }

    pub fn get_by_id(&self, id: EnemyID) -> Option<&Enemy> {
        self.map.get(&id)
    }

    pub fn get_mut_by_id(&mut self, id: EnemyID) -> Option<&mut Enemy> {
        self.map.get_mut(&id)
    }
}

impl Drawable for Enemies {
    unsafe fn draw(&self, shader: &crate::cg::shader::Shader) {
        self.map.values().for_each(|e| {
            e.draw(shader);
        });
    }
}
