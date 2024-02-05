use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

use colored::Colorize;

use super::{
    extra_config::ExtraConfig, extra_day::ExtraDay, extra_limiter::ExtraLimiter,
    extra_table::ExtraDutyTable, worker::Worker,
};

pub struct ExtraDuty {
    worker_map: RefCell<HashMap<u64, Rc<Worker>>>,
    pub limiter: ExtraLimiter,
    pub config: ExtraConfig,
    pub day_index: usize,
    pub index: usize,
}

impl ExtraDuty {
    pub fn new(index: usize, table: &ExtraDutyTable, day: &ExtraDay) -> ExtraDuty {
        ExtraDuty {
            index,
            day_index: day.index,
            worker_map: Default::default(),
            config: table.config.clone(),
            limiter: table.limiter.clone(),
        }
    }

    pub fn worker_qnt(&self) -> usize {
        self.worker_map.borrow().len()
    }

    pub fn list_workers(&self) -> Vec<Rc<Worker>> {
        let mut workers = Vec::new();

        for (_, worker) in self.worker_map.borrow().iter() {
            workers.push(Rc::clone(worker));
        }

        workers
    }

    pub fn is_full(&self) -> bool {
        self.worker_qnt() >= self.config.get_duty_limit()
    }

    pub fn clear(&self) {
        for (_, worker) in self.worker_map.borrow().iter() {
            self.limiter.decrement(worker.id);
        }

        self.worker_map.borrow_mut().clear();
    }

    pub fn add(&self, worker: &Rc<Worker>) -> () {
        let inserted = self
            .worker_map
            .borrow_mut()
            .insert(worker.id, Rc::clone(worker))
            .is_none();

        if inserted == false {
            return;
        }

        self.limiter.increment(worker.id);
    }
}

impl Display for ExtraDuty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let worker_qnt = self.worker_qnt();
        let message = worker_qnt.to_string();

        let colored_message = if worker_qnt == 0 {
            message.bright_cyan()
        } else if worker_qnt < 2 {
            message.bright_red()
        } else if worker_qnt == 2 {
            message.bright_green()
        } else {
            message.yellow()
        };

        write!(f, "{}", colored_message)?;

        Ok(())
    }
}
