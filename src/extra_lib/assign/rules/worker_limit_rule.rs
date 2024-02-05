use crate::extra_lib::structs::{extra_duty::ExtraDuty, worker::Worker};

use super::AssignRule;

pub struct WorkerLimitRule {
    pub id: u8,
}

impl WorkerLimitRule {
    pub fn new() -> WorkerLimitRule {
        WorkerLimitRule { id: 2 }
    }
}

impl AssignRule for WorkerLimitRule {
    fn check(&self, worker: &Worker, duty: &ExtraDuty) -> bool {
        duty.limiter.has_positions_left(worker)
    }
}
