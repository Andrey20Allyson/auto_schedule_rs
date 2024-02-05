use std::rc::Rc;

use crate::extra_lib::{
    assign::rules::AssignRule,
    error::ExtraError,
    structs::{extra_duty::ExtraDuty, extra_table::ExtraDutyTable, worker::Worker},
    utils::random::{randomize_vec, randomize_vec_ref, Randomizable},
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
        let old_duty_limit = table.config.get_duty_limit();

        let mut unassigned_workers: Vec<_> = workers.iter().collect();
        let mut rand_days = randomize_vec(table.days.iter().collect());

        for current_limit in 1..=3 {
            table.config.set_duty_limit(current_limit);

            randomize_vec_ref(&mut rand_days);

            for day in rand_days.iter() {
                for duty in day.list_duties().rand() {
                    randomize_vec_ref(&mut unassigned_workers);

                    for worker in unassigned_workers.iter() {
                        self.assin_worker(&worker, duty);

                        if duty.is_full() {
                            break;
                        }
                    }
                }

                unassigned_workers = unassigned_workers
                    .into_iter()
                    .filter(|worker| table.limiter.reached_limit(worker) == false)
                    .collect();
            }
        }

        table.config.set_duty_limit(old_duty_limit);

        Ok(())
    }
}
