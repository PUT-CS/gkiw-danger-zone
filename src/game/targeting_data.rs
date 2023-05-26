use super::missile::EnemyID;

const LOCKING_TIME: f64 = 4.0;

#[derive(Debug)]
pub struct TargetingData {
    pub target_id: EnemyID,
    pub left_until_lock: f64
}

impl TargetingData {
    pub fn new(id: EnemyID) -> Self {
        Self { target_id: id, left_until_lock: LOCKING_TIME }
    }
}
