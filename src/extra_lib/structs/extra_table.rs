use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

use crate::extra_lib::{error::ExtraError, utils};

use colored::Colorize;

use super::{
    extra_day::ExtraDay, extra_duty::ExtraDuty, extra_limiter::ExtraLimiter,
    extra_place::ExtraPlaceHolder, worker::Worker,
};

pub struct ExtraDutyTable {
    pub days: Vec<Rc<RefCell<ExtraDay>>>,
    pub width: usize,
    pub limiter: ExtraLimiter,
    pub current_place: ExtraPlaceHolder,
    pub duty_limit: usize,
    pub day_size: usize,
}

impl ExtraDutyTable {
    pub fn new(width: usize, day_size: usize) -> Rc<RefCell<ExtraDutyTable>> {
        let current_place = ExtraPlaceHolder::default();

        let table = ExtraDutyTable {
            limiter: ExtraLimiter::new(current_place.clone()),
            days: Vec::new(),
            current_place,
            duty_limit: 3,
            day_size,
            width,
        };

        let shared_table = Rc::new(RefCell::new(table));

        for i in 0..width {
            let day = ExtraDay::new(i, &shared_table);

            shared_table.borrow_mut().days.push(day);
        }

        shared_table
    }

    pub fn list_duties(&self) -> Vec<Rc<RefCell<ExtraDuty>>> {
        let mut duties = Vec::<Rc<RefCell<ExtraDuty>>>::new();

        for day in self.days.iter() {
            for duty in day.borrow().duties.iter() {
                duties.push(Rc::clone(&duty));
            }
        }

        duties
    }

    pub fn list_workers(&self) -> Vec<Rc<Worker>> {
        let mut worker_map = HashMap::<u64, Rc<Worker>>::new();

        for day in self.days.iter() {
            for duty in day.borrow().duties.iter() {
                for (key, worker) in duty.borrow().workers.iter() {
                    worker_map.insert(*key, Rc::clone(worker));
                }
            }
        }

        worker_map.drain().map(|(_, worker)| worker).collect()
    }

    pub fn get_day(&self, index: usize) -> Result<&Rc<RefCell<ExtraDay>>, ExtraError> {
        self.days
            .get(index)
            .ok_or(ExtraError::CantFindDay { index })
    }

    pub fn rand(&self) -> Vec<Rc<RefCell<ExtraDay>>> {
        utils::random::randomize_vec(self.days.clone())
    }
}

impl Display for ExtraDutyTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "======== [ {} ] ========", "Table".bright_red())?;

        for day in self.days.iter() {
            write!(f, " ")?;

            day.borrow().fmt(f)?;
        }

        Ok(())
    }
}
