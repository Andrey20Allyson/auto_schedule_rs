use std::{collections::HashMap, fmt::Display};

use colored::Colorize;

use super::irregularity::{Irregularity, IrregularityPenality};

pub struct TableIntegrity {
    pub irregularities: HashMap<&'static str, Irregularity>,
    pub penality: Option<IrregularityPenality>,
}

impl TableIntegrity {
    pub fn new() -> TableIntegrity {
        TableIntegrity {
            penality: None,
            irregularities: Default::default(),
        }
    }

    pub fn from_irregularities(irregularities: &mut Vec<Irregularity>) -> TableIntegrity {
        let mut integrity = TableIntegrity::new();

        integrity.digest_vec(irregularities);

        integrity
    }

    pub fn digest(&mut self, irregularity: Irregularity) {
        match &mut self.penality {
            Some(self_penality) => self_penality.join(&irregularity.penality),
            None => self.penality = Some(irregularity.penality),
        }

        match self.irregularities.get_mut(irregularity.name) {
            Some(saved_irregularity) => {
                saved_irregularity.join(&irregularity);
            }
            None => {
                self.irregularities.insert(irregularity.name, irregularity);
            }
        };
    }

    pub fn digest_vec(&mut self, irregularities: &mut Vec<Irregularity>) {
        for irregularity in irregularities.drain(0..) {
            self.digest(irregularity)
        }
    }

    pub fn is_under_limit(&self, penality_limit: u64) -> bool {
        match self.penality {
            Some(penality) => penality.is_under_limit(penality_limit),
            None => true,
        }
    }
}

impl Display for TableIntegrity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[ {} ]", "Integrity".red())?;

        for (_, irregularity) in self.irregularities.iter() {
            write!(f, "  {}", irregularity)?;
        }

        Ok(())
    }
}
