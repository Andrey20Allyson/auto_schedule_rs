use std::fmt::Display;

use colored::Colorize;

#[derive(Clone, Copy)]
pub enum IrregularityPenality {
    Accumulate(u64),
    Invalidate,
}

impl IrregularityPenality {
    pub fn is_under_limit(&self, limit: u64) -> bool {
        match self {
            IrregularityPenality::Accumulate(self_acc) => *self_acc < limit,
            IrregularityPenality::Invalidate => false,
        }
    }

    pub fn join(&mut self, other: &IrregularityPenality) {
        match self {
            IrregularityPenality::Accumulate(self_acc) => match other {
                IrregularityPenality::Accumulate(other_acc) => {
                    *self = IrregularityPenality::Accumulate(*self_acc + other_acc);
                }
                IrregularityPenality::Invalidate => {
                    *self = IrregularityPenality::Invalidate;
                }
            },
            IrregularityPenality::Invalidate => (),
        }
    }
}

pub struct Irregularity {
    pub name: &'static str,
    pub penality: IrregularityPenality,
    pub count: u64,
}

impl Irregularity {
    pub fn acc(name: &'static str, penality: u64) -> Irregularity {
        Irregularity {
            name,
            penality: IrregularityPenality::Accumulate(penality),
            count: 1,
        }
    }

    pub fn invalidate(name: &'static str) -> Irregularity {
        Irregularity {
            name,
            penality: IrregularityPenality::Invalidate,
            count: 1,
        }
    }

    pub fn join(&mut self, other: &Irregularity) {
        self.penality.join(&other.penality);

        self.count += other.count;
    }
}

pub struct IrregularityStack {
    stack: Vec<Irregularity>,
}

impl IrregularityStack {
    pub fn new() -> IrregularityStack {
        IrregularityStack { stack: Vec::new() }
    }

    pub fn add_penality(&mut self, name: &'static str, penality_count: u64) -> &mut Self {
        self.stack.push(Irregularity::acc(name, penality_count));

        self
    }

    pub fn add_invalidation(&mut self, name: &'static str) -> &mut Self {
        self.stack.push(Irregularity::invalidate(name));

        self
    }

    pub fn irregularities(&mut self) -> &mut Vec<Irregularity> {
        &mut self.stack
    }
}

impl Display for Irregularity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.penality {
            IrregularityPenality::Accumulate(penality_acc) => writeln!(
                f,
                "'{}' x {}, Penality: {}",
                self.name.yellow(),
                self.count,
                penality_acc
            )?,
            IrregularityPenality::Invalidate => writeln!(f, "'{}'", self.name.red())?,
        };

        Ok(())
    }
}
