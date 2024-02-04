use crate::extra_lib::{error::DynError, structs::extra_table::ExtraDutyTable};

use super::{integrity::TableIntegrity, irregularity::IrregularityStack, qualifiers::Qualifier};

type Qualifiers = Vec<Box<dyn Qualifier>>;
pub struct Verifier {
    pub qualifiers: Qualifiers,
}

impl Verifier {
    pub fn new() -> Verifier {
        Verifier {
            qualifiers: Vec::new(),
        }
    }

    pub fn from_qualifiers(qualifiers: &mut Qualifiers) -> Verifier {
        let mut verifier = Verifier::new();

        verifier.qualifiers.append(qualifiers);

        verifier
    }

    pub fn verify(&self, table: &ExtraDutyTable) -> Result<TableIntegrity, DynError> {
        let mut stack = IrregularityStack::new();

        for qualifier in self.qualifiers.iter() {
            qualifier.qualify(&mut stack, table)?;
        }

        Ok(TableIntegrity::from_irregularities(
            &mut stack.irregularities(),
        ))
    }
}
