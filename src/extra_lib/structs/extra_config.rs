use std::{cell::RefCell, rc::Rc};

use super::month::Month;

const HOURS_IN_DAY: u64 = 24;

pub struct InnerExtraConfig {
    pub duties_per_day: usize,
    pub duty_duration_time: u64,
    pub duty_limit: usize,
    pub month: Month,
}

impl Default for InnerExtraConfig {
    fn default() -> Self {
        let month = Month::now();

        InnerExtraConfig {
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
        Default::default()
    }

    pub fn from(init: InnerExtraConfig) -> ExtraConfig {
        ExtraConfig {
            inner: Rc::new(RefCell::new(init)),
        }
    }

    pub fn from_month(month: Month) -> ExtraConfig {
        let init = InnerExtraConfig {
            month,
            ..Default::default()
        };

        ExtraConfig::from(init)
    }

    pub fn get_num_of_days(&self) -> usize {
        self.inner.borrow().month.number_of_days() as usize
    }

    pub fn get_duties_per_day(&self) -> usize {
        self.inner.borrow().duties_per_day
    }

    pub fn get_duty_duration_time(&self) -> u64 {
        self.inner.borrow().duty_duration_time
    }

    pub fn get_month(&self) -> Month {
        self.inner.borrow().month
    }

    pub fn get_duty_limit(&self) -> usize {
        self.inner.borrow().duty_limit
    }
}

impl Default for ExtraConfig {
    fn default() -> Self {
        ExtraConfig {
            inner: Default::default(),
        }
    }
}
