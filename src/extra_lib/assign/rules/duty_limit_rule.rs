use crate::extra_lib::structs::{extra_duty::ExtraDuty, worker::Worker};

use super::AssignRule;

pub struct DutyLimitRule {
    pub id: u8,
}

impl DutyLimitRule {
    pub fn new() -> DutyLimitRule {
        DutyLimitRule { id: 0x00 }
    }
}

impl AssignRule for DutyLimitRule {
    fn check(&self, _worker: &Worker, duty: &ExtraDuty) -> bool {
        duty.worker_qnt() < duty.table.borrow().duty_limit
    }
}
