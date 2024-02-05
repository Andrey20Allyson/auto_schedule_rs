use super::month::Month;

#[derive(Clone, Copy, Debug)]
pub enum DayRestriction {
    None,
    Ordinary,
    License,
}

pub struct RestrictionEntry {
    pub day: usize,
    pub restriction: DayRestriction,
}

impl RestrictionEntry {
    pub fn new(day: usize, restriction: DayRestriction) -> RestrictionEntry {
        RestrictionEntry { day, restriction }
    }
}

#[derive(Debug)]
pub struct WorkerSchedule {
    pub month: Month,
    pub is_daily_worker: bool,
    num_of_days_off: u8,
    days: Vec<DayRestriction>,
}

impl WorkerSchedule {
    pub fn new(month: Month, is_daily_worker: bool) -> WorkerSchedule {
        let mut schedule = WorkerSchedule {
            days: Vec::new(),
            is_daily_worker,
            num_of_days_off: month.number_of_days(),
            month,
        };

        for _ in 0..schedule.month.number_of_days() {
            schedule.days.push(DayRestriction::None);
        }

        schedule
    }

    pub fn num_of_days_off(&self) -> u8 {
        self.num_of_days_off
    }

    pub fn has_days_off(&self) -> bool {
        self.num_of_days_off > 0
    }

    pub fn set_restriction(&mut self) {}

    pub fn len(&self) -> usize {
        self.days.len()
    }
}
