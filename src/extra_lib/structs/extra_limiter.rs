use std::collections::HashMap;

use super::{
    extra_place::{ExtraPlaceHolder, ExtraPlaceId},
    worker::Worker,
};

pub struct ExtraLimiter {
    pub current_place: ExtraPlaceHolder,
    places: HashMap<ExtraPlaceId, HashMap<u64, u8>>,
}

impl ExtraLimiter {
    pub fn new(current_place: ExtraPlaceHolder) -> Self {
        Self {
            current_place,
            ..Default::default()
        }
    }

    pub fn current_counter(&self) -> Option<&HashMap<u64, u8>> {
        let place_id = self.current_place.get();

        self.places.get(&place_id)
    }

    pub fn positions_of(&self, worker_id: &u64) -> u8 {
        let counter = self.current_counter();

        match counter {
            Some(counter) => match counter.get(worker_id) {
                Some(positions) => *positions,
                None => 0,
            },
            None => 0,
        }
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
