use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{extra_day::ExtraDay, extra_table::ExtraDutyTable, worker::Worker};

pub struct ExtraDuty {
    pub workers: HashMap<u64, Rc<Worker>>,
    pub day: Rc<RefCell<ExtraDay>>,
    pub table: Rc<RefCell<ExtraDutyTable>>,
    pub index: usize,
}

impl ExtraDuty {
    pub fn new(index: usize, day: &Rc<RefCell<ExtraDay>>) -> Rc<RefCell<ExtraDuty>> {
        let table = &day.borrow().table;

        Rc::new(RefCell::new(ExtraDuty {
            index,
            workers: HashMap::new(),
            day: Rc::clone(day),
            table: Rc::clone(table),
        }))
    }

    pub fn worker_qnt(&self) -> usize {
        self.workers.len()
    }

    pub fn add(&mut self, worker: &Rc<Worker>) -> () {
        self.workers.insert(worker.id, Rc::clone(worker));
    }
}
