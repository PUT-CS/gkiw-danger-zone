use super::{
    drawable::Drawable,
    enemy::Enemy,
    game::{ID_GENERATOR, TARGET_ENEMIES},
    missile::EnemyID, flight::steerable::Steerable,
};
use crate::game::flight::aircraft::AircraftKind::*;
use crate::game::id_gen::IDKind;
use log::warn;
use rand::{thread_rng, Rng};
use std::collections::HashMap;

pub struct Enemies {
    pub map: HashMap<EnemyID, Enemy>,
}

impl Enemies {
    pub fn new() -> Enemies {
        let mut e = Enemies {
            map: HashMap::with_capacity(TARGET_ENEMIES),
        };
        e.respawn_enemies();
        e.randomize_positions();
        e
    }
    pub fn respawned_enemies(&mut self) -> HashMap<EnemyID, Enemy> {
        let diff = TARGET_ENEMIES - self.map.len();
        if diff > 0 {
            warn!("Respawning {diff} enemies");
        }
        let mut id_gen = ID_GENERATOR.lock().expect("Lock IDGenerator mutex");
        (0..diff)
            .map(|_| (id_gen.get_new_id_of(IDKind::Enemy), Enemy::new(Mig21)))
            .collect()
    }
    pub fn respawn_enemies(&mut self) {
        let new_enemies = self.respawned_enemies();
        self.map.extend(new_enemies);
    }
    pub fn randomize_positions(&mut self) {
        assert!(!self.map.is_empty());
        self.map.values_mut().for_each(|e| {
            let a = e.aircraft_mut().model_mut();
            let amount = thread_rng().gen_range(0., 10000.);
            a.roll(amount);
            a.pitch(amount);
            a.yaw(amount);
            a.forward(5.0);
        })
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
        self.map.values().for_each(|e| {e.draw(shader);});
    }
}

