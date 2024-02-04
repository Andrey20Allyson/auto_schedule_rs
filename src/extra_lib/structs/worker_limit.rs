use std::collections::HashMap;

use super::extra_place::ExtraPlaceId;

#[derive(Debug)]
pub struct WorkerLimit {
    pub default_limit: u8,
    pub places: HashMap<ExtraPlaceId, u8>,
}

impl WorkerLimit {
    pub fn new(places: HashMap<u8, u8>) -> WorkerLimit {
        WorkerLimit {
            places,
            ..Default::default()
        }
    }

    pub fn of(&self, place_id: &ExtraPlaceId) -> u8 {
        *self.places.get(place_id).unwrap_or(&self.default_limit)
    }
}

impl Default for WorkerLimit {
    fn default() -> Self {
        WorkerLimit {
            default_limit: 10,
            places: Default::default(),
        }
    }
}
