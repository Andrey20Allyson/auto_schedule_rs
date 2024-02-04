mod duty_limit_rule;

pub use duty_limit_rule::DutyLimitRule;

use crate::extra_lib::structs::{extra_duty::ExtraDuty, worker::Worker};

pub trait AssignRule {
    fn check(&self, worker: &Worker, duty: &ExtraDuty) -> bool;
}
