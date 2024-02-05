use std::{cell::RefCell, rc::Rc};

use super::month::Month;

const HOURS_IN_DAY: u64 = 24;

pub struct InnerExtraConfig {
    pub num_of_days: usize,
    pub duties_per_day: usize,
    pub duty_duration_time: u64,
    pub duty_limit: usize,
    pub month: Month,
}

impl Default for InnerExtraConfig {
    fn default() -> Self {
        let month = Month::now();

        InnerExtraConfig {
            num_of_days: month.number_of_days() as usize,
            duties_per_day: 4,
            duty_duration_time: HOURS_IN_DAY / 4,
            duty_limit: 3,
            month,
        }
    }
}

#[derive(Clone)]
pub struct ExtraConfig {
    inner: Rc<RefCell<InnerExtraConfig>>,
}

impl ExtraConfig {
    pub fn new() -> ExtraConfig {
        ExtraConfig {
            inner: Default::default(),
        }
    }

    pub fn from(init: InnerExtraConfig) -> ExtraConfig {
        ExtraConfig {
            inner: Rc::new(RefCell::new(init)),
        }
    }

    pub fn from_month(month: Month) -> ExtraConfig {
        let init = InnerExtraConfig {
            month,
            num_of_days: month.number_of_days() as usize,
            ..Default::default()
        };

        ExtraConfig::from(init)
    }

    pub fn set_num_of_days(&self, num_of_days: usize) {
        self.inner.borrow_mut().num_of_days = num_of_days;
    }

    pub fn get_num_of_days(&self) -> usize {
        self.inner.borrow().num_of_days
    }
}
