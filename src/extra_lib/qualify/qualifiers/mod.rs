pub mod worker_allocation_qualifier;

use crate::extra_lib::{error::DynError, structs::extra_table::ExtraDutyTable};

use super::irregularity::IrregularityStack;

pub trait Qualifier {
    fn qualify(
        &self,
        irregularities: &mut IrregularityStack,
        table: &ExtraDutyTable,
    ) -> Result<(), DynError>;
}
