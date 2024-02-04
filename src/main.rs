use extra_lib::{
    assign::assigner::TableAssigner, assign::rules::DutyLimitRule, error::DynError,
    mock::worker::WorkerMocker, structs::extra_table::ExtraDutyTable,
};

use crate::extra_lib::qualify::{
    qualifiers::worker_allocation_qualifier::WorkerAllocationQualifier, verifier::Verifier,
};

pub mod extra_lib;

fn main() -> Result<(), DynError> {
    let table = ExtraDutyTable::new(30, 4);
    let mut mocker = WorkerMocker::new();
    let workers = mocker.vec(27);

    let assigner = TableAssigner::from_rules(&mut vec![Box::new(DutyLimitRule::new())]);
    assigner.assign_into(&table.borrow(), &workers)?;

    let verifier =
        Verifier::from_qualifiers(&mut vec![Box::new(WorkerAllocationQualifier::new(140))]);
    let integrity = verifier.verify(&table.borrow())?;

    println!("{}", table.borrow());
    println!("{}", integrity);

    Ok(())
}
