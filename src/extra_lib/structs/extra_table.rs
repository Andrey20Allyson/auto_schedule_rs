use std::{collections::HashMap, fmt::Display, rc::Rc};

use crate::extra_lib::error::ExtraError;

use colored::Colorize;

use super::{
    extra_config::ExtraConfig, extra_day::ExtraDay, extra_duty::ExtraDuty,
    extra_limiter::ExtraLimiter, extra_place::ExtraPlaceHolder, worker::Worker,
};

pub struct ExtraDutyTable {
    pub days: Vec<ExtraDay>,
    pub config: ExtraConfig,
    pub limiter: ExtraLimiter,
    pub current_place: ExtraPlaceHolder,
}

impl ExtraDutyTable {
    pub fn new(config: ExtraConfig) -> ExtraDutyTable {
        let current_place = ExtraPlaceHolder::default();

        let mut table = ExtraDutyTable {
            limiter: ExtraLimiter::new(current_place.clone()),
            days: Vec::new(),
            current_place,
            config,
        };

        for i in 0..table.config.get_num_of_days() {
            let day = ExtraDay::new(i, &table);

            table.days.push(day);
        }

        table
    }

    pub fn load_days(&mut self) {}

    pub fn list_days(&self) -> Vec<&ExtraDay> {
        self.days.iter().collect()
    }

    pub fn list_duties(&self) -> Vec<&ExtraDuty> {
        self.days.iter().flat_map(|day| day.duties.iter()).collect()
    }

    pub fn list_workers(&self) -> Vec<Rc<Worker>> {
        let mut worker_map = HashMap::<u64, Rc<Worker>>::new();

        for day in self.days.iter() {
            for duty in day.duties.iter() {
                for worker in duty.list_workers() {
                    worker_map.insert(worker.id, worker);
                }
            }
        }

        worker_map.drain().map(|(_, worker)| worker).collect()
    }

    pub fn clear(&self) {
        for day in self.days.iter() {
            for duty in day.duties.iter() {
                duty.clear();
            }
        }

        self.limiter.clear();
    }

    pub fn get_day(&self, index: usize) -> Result<&ExtraDay, ExtraError> {
        self.days
            .get(index)
            .ok_or(ExtraError::CantFindDay { index })
    }
}

impl Display for ExtraDutyTable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "======== [ {} ] ========", "Table".bright_red())?;

        for day in self.days.iter() {
            write!(f, " ")?;

            day.fmt(f)?;
        }

        Ok(())
    }
}
