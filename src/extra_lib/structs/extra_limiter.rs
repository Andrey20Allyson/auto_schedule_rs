use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{
    extra_place::{ExtraPlaceHolder, ExtraPlaceId},
    worker::Worker,
};

#[derive(Clone)]
pub struct ExtraLimiter {
    pub current_place: ExtraPlaceHolder,
    places: Rc<RefCell<HashMap<ExtraPlaceId, Rc<RefCell<HashMap<u64, u8>>>>>>,
}

impl ExtraLimiter {
    pub fn new(current_place: ExtraPlaceHolder) -> Self {
        Self {
            current_place,
            ..Default::default()
        }
    }

    pub fn current_counter(&self) -> Rc<RefCell<HashMap<u64, u8>>> {
        let place_id = self.current_place.get();

        let mut counters = self.places.borrow_mut();

        match counters.get(&place_id) {
            Some(counter) => Rc::clone(counter),
            None => {
                let new_counter = Default::default();

                counters.insert(place_id, Rc::clone(&new_counter));

                new_counter
            }
        }
    }

    pub fn positions_of(&self, worker_id: &u64) -> u8 {
        match self.current_counter().borrow().get(worker_id) {
            Some(positions) => *positions,
            None => 0,
        }
    }

    pub fn set_positions_of(&self, worker_id: u64, value: u8) {
        let current_counter = self.current_counter();

        current_counter.borrow_mut().insert(worker_id, value);
    }

    pub fn increment(&self, worker_id: u64) -> u8 {
        let new_value = self.positions_of(&worker_id) + 1;

        self.set_positions_of(worker_id, new_value);

        new_value
    }

    pub fn decrement(&self, worker_id: u64) -> u8 {
        let new_value = self.positions_of(&worker_id) - 1;

        self.set_positions_of(worker_id, new_value);

        new_value
    }

    pub fn clear(&self) {
        self.current_counter().borrow_mut().clear();
    }

    pub fn positions_left_of(&self, worker: &Worker) -> u8 {
        let place_id = self.current_place.get();

        worker
            .limit
            .of(&place_id)
            .checked_sub(self.positions_of(&worker.id))
            .unwrap_or(0)
    }

    pub fn is_unassigned(&self, worker_id: &u64) -> bool {
        self.positions_of(worker_id) <= 0
    }

    pub fn has_positions_left(&self, worker: &Worker) -> bool {
        self.positions_left_of(worker) > 0
    }

    pub fn reached_limit(&self, worker: &Worker) -> bool {
        self.positions_left_of(worker) <= 0
    }
}

impl Default for ExtraLimiter {
    fn default() -> Self {
        ExtraLimiter {
            current_place: Default::default(),
            places: Default::default(),
        }
    }
}
