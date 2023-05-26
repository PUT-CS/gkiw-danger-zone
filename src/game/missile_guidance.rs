use super::missile::EnemyID;
use vek::QuadraticBezier3;

/// Number of frames after which a missile without a target gets deleted
const TERMINATION_TIME: u32 = 5000;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GuidanceData {
    pub target_id: EnemyID,
    pub bezier: QuadraticBezier3<f32>,
    pub progress: f32,
}

type TerminationTimer = u32;
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GuidanceStatus {
    /// Contains an integer representing the number of ticks left until termination.
    None(TerminationTimer),
    /// Contains data necessary for guidance
    Active(GuidanceData),
}

impl GuidanceStatus {
    pub fn none() -> Self {
        GuidanceStatus::None(TERMINATION_TIME)
    }
    pub fn new(target_id: EnemyID, bezier: QuadraticBezier3<f32>) -> Self {
        Self::Active(GuidanceData {
            target_id,
            bezier,
            progress: 0.,
        })
    }
}
