use std::fmt::Display;

use crate::extra_lib::error::DynError;

use super::year::Year;

#[derive(Debug)]
pub struct InvalidMonthIndexError(u8);

impl std::error::Error for InvalidMonthIndexError {}

impl Display for InvalidMonthIndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Invalid month index: {}", self.0)
    }
}

const FEBRUARY_MONTH_INDEX: u8 = 1;
const NUMBER_OF_MONTHS: u8 = 12;
const DAYS_IN_MONTH: [u8; NUMBER_OF_MONTHS as usize] =
    [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

#[derive(Clone, Copy, Debug)]
pub struct Month {
    pub year: Year,
    pub index: u8,
}

impl Month {
    pub fn new(year: u16, index: u8) -> Result<Month, DynError> {
        if index > NUMBER_OF_MONTHS - 1 {
            return Err(Box::new(InvalidMonthIndexError(index)));
        }

        Ok(Month {
            year: Year::new(year),
            index,
        })
    }

    pub fn len(&self) -> u8 {
        let aditional_day = if self.index == FEBRUARY_MONTH_INDEX && self.year.is_leap() {
            1
        } else {
            0
        };

        DAYS_IN_MONTH[self.index as usize] + aditional_day
    }
}
