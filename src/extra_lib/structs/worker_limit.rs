use std::collections::HashMap;

#[derive(Debug)]
pub struct WorkerLimit {
    pub default_limit: u8,
    pub places: HashMap<u8, u8>,
}

impl WorkerLimit {
    pub fn new(places: HashMap<u8, u8>) -> WorkerLimit {
        WorkerLimit {
            places,
            ..Default::default()
        }
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
