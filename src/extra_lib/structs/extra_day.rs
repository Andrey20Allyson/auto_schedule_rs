use std::fmt::Display;

use colored::Colorize;

use super::{
    super::error::ExtraError, extra_config::ExtraConfig, extra_duty::ExtraDuty,
    extra_table::ExtraDutyTable,
};

pub struct ExtraDay {
    pub index: usize,
    pub config: ExtraConfig,
    pub duties: Vec<ExtraDuty>,
}

impl ExtraDay {
    pub fn new(index: usize, table: &ExtraDutyTable) -> ExtraDay {
        let mut day = ExtraDay {
            index,
            config: table.config.clone(),
            duties: Vec::new(),
        };

        for index in 0..table.config.get_duties_per_day() {
            let duty = ExtraDuty::new(index, table, &day);

            day.duties.push(duty);
        }

        day
    }

    pub fn list_duties(&self) -> Vec<&ExtraDuty> {
        self.duties.iter().collect()
    }

    pub fn number_of_duties(&self) -> usize {
        self.config.get_duties_per_day()
    }

    pub fn get_duty(&self, index: usize) -> Result<&ExtraDuty, ExtraError> {
        self.duties
            .get(index)
            .ok_or(ExtraError::CantFindDuty { index: 0 })
    }
}

impl Display for ExtraDay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "[ {} {:0pad$} ] => [",
            "Day".bright_black(),
            self.index.to_string(),
            pad = 2
        )?;

        for duty in self.duties.iter() {
            write!(f, "{}", duty)?;

            if duty.index < self.number_of_duties() - 1 {
                write!(f, ", ")?;
            }
        }

        writeln!(f, "]")?;

        Ok(())
    }
}
