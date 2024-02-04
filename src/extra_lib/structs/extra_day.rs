use std::{cell::RefCell, fmt::Display, rc::Rc};

use colored::Colorize;

use crate::extra_lib::utils;

use super::{super::error::ExtraError, extra_duty::ExtraDuty, extra_table::ExtraDutyTable};

pub struct ExtraDay {
    pub table: Rc<RefCell<ExtraDutyTable>>,
    pub index: usize,
    pub duties: Vec<Rc<RefCell<ExtraDuty>>>,
    size: usize,
}

impl ExtraDay {
    pub fn new(index: usize, table: &Rc<RefCell<ExtraDutyTable>>) -> Rc<RefCell<ExtraDay>> {
        let size = table.borrow().day_size;

        let day = Rc::new(RefCell::new(ExtraDay {
            index,
            duties: Vec::new(),
            table: Rc::clone(table),
            size,
        }));

        for index in 0..size {
            let duty = ExtraDuty::new(index, &day);

            day.borrow_mut().duties.push(duty);
        }

        day
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn get_duty(&self, index: usize) -> Result<&Rc<RefCell<ExtraDuty>>, ExtraError> {
        self.duties
            .get(index)
            .ok_or(ExtraError::CantFindDuty { index: 0 })
    }

    pub fn rand(&self) -> Vec<Rc<RefCell<ExtraDuty>>> {
        utils::random::randomize_vec(self.duties.clone())
    }
}

impl Display for ExtraDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[ {} {:0pad$} ] => [",
            "Day".bright_black(),
            self.index.to_string(),
            pad = 2
        )?;

        for duty in self.duties.iter() {
            let duty = duty.borrow();
            write!(f, "{}", duty.worker_qnt().to_string().bright_green())?;

            if duty.index < self.size() - 1 {
                write!(f, ", ")?;
            }
        }

        writeln!(f, "]")?;

        Ok(())
    }
}
