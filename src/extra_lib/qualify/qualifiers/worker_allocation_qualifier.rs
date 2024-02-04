use crate::extra_lib::{
    error::DynError,
    qualify::irregularity::IrregularityStack,
    structs::{extra_duty::ExtraDuty, extra_table::ExtraDutyTable},
};

use super::Qualifier;

pub struct WorkerAllocationQualifier {
    pub base_penality: u64,
}

impl WorkerAllocationQualifier {
    pub fn new(base_penality: u64) -> WorkerAllocationQualifier {
        WorkerAllocationQualifier { base_penality }
    }

    pub fn calculate_unassigned_worker_penality(&self, positions_left: u8) -> u64 {
        self.base_penality * (1.4 * positions_left.pow(2) as f32) as u64
    }

    pub fn duty_is_worker_insuficient(&self, duty: &ExtraDuty) -> bool {
        return duty.worker_qnt() < 2;
    }
}

impl Qualifier for WorkerAllocationQualifier {
    fn qualify(
        &self,
        irregularities: &mut IrregularityStack,
        table: &ExtraDutyTable,
    ) -> Result<(), DynError> {
        // let is_worker_insuficient = table
        //     .list_duties()
        //     .into_iter()
        //     .any(move |duty| self.duty_is_worker_insuficient(&duty.borrow()));

        // if is_worker_insuficient == false {
        //     return Ok(());
        // }

        for worker in table.list_workers().iter() {
            if worker.schedule.has_days_off() == false {
                continue;
            }

            println!("{}", worker.name);

            let penality = self.calculate_unassigned_worker_penality(2);

            irregularities.add_penality("Worker hasn't collectly allocated", penality);
        }

        Ok(())
    }
}
