use std::rc::Rc;

use crate::extra_lib::structs::{month::Month, worker::Worker, worker_shedule::WorkerSchedule};

pub struct WorkerMocker {
    id: u64,
}

impl WorkerMocker {
    pub fn new() -> WorkerMocker {
        WorkerMocker { id: 0 }
    }

    pub fn mock(&mut self, name: Option<String>) -> Worker {
        let id = self.id;
        let name = name.unwrap_or(format!("John Due #{}", id));
        self.id += 1;

        Worker {
            id,
            name,
            schedule: WorkerSchedule::new(Month::new(2024, 1).unwrap(), false),
            limit: Default::default(),
            gender: Default::default(),
            graduation: Default::default(),
        }
    }

    pub fn mock_rc(&mut self, name: Option<String>) -> Rc<Worker> {
        self.mock(name).rc_wrap()
    }

    pub fn vec(&mut self, size: usize) -> Vec<Rc<Worker>> {
        let mut vec: Vec<Rc<Worker>> = Vec::new();

        for _ in 0..size {
            vec.push(self.mock_rc(None));
        }

        vec
    }
}
