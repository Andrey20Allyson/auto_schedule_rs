use std::rc::Rc;

use crate::extra_lib::{
    assign::rules::AssignRule,
    error::ExtraError,
    structs::{extra_duty::ExtraDuty, extra_table::ExtraDutyTable, worker::Worker},
    utils::random::Randomizable,
};

pub struct TableAssigner {
    rules: Vec<Box<dyn AssignRule>>,
}

impl TableAssigner {
    pub fn new() -> TableAssigner {
        TableAssigner { rules: Vec::new() }
    }

    pub fn from_rules(rules: &mut Vec<Box<dyn AssignRule>>) -> TableAssigner {
        let mut assigner = Self::new();

        assigner.rules.append(rules);

        assigner
    }

    pub fn can_assing(&self, worker: &Worker, duty: &ExtraDuty) -> bool {
        self.rules.iter().all(|rule| rule.check(worker, duty))
    }

    pub fn assin_worker(&self, worker: &Rc<Worker>, duty: &ExtraDuty) -> bool {
        if self.can_assing(worker, duty) == false {
            return false;
        };

        duty.add(worker);

        return true;
    }

    pub fn assign_into(
        &self,
        table: &ExtraDutyTable,
        workers: &Vec<Rc<Worker>>,
    ) -> Result<(), ExtraError> {
        for day in table.list_days().rand() {
            for duty in day.list_duties().rand() {
                for worker in workers.rand() {
                    self.assin_worker(&worker, duty);
                }
            }
        }

        Ok(())
    }
}
