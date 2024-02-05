use std::env::args;

use chrono::Local;
use colored::Colorize;
use extra_lib::{
    assign::assigner::TableAssigner, assign::rules::DutyLimitRule, error::DynError,
    mock::worker::WorkerMocker, structs::extra_table::ExtraDutyTable,
};

use crate::extra_lib::{
    assign::rules::WorkerLimitRule,
    qualify::{
        qualifiers::worker_allocation_qualifier::WorkerAllocationQualifier, verifier::Verifier,
    },
};

pub mod extra_lib;

fn main() -> Result<(), DynError> {
    let table = ExtraDutyTable::new(Default::default());
    let mut mocker = WorkerMocker::new();
    let workers = mocker.vec(27);

    let assigner = TableAssigner::from_rules(&mut vec![
        Box::new(DutyLimitRule::new()),
        Box::new(WorkerLimitRule::new()),
    ]);

    let verifier =
        Verifier::from_qualifiers(&mut vec![Box::new(WorkerAllocationQualifier::new(140))]);

    println!("{}", table.config.get_num_of_days());
    println!("{}", table.config.get_duties_per_day());

    let start_process_time = Local::now().timestamp_millis();

    let args: Vec<_> = args().collect();
    let iterations: i32 = args.get(1).unwrap().parse().unwrap();

    for _ in 0..iterations {
        table.clear();

        assigner.assign_into(&table, &workers)?;

        verifier.verify(&table)?;
    }

    let end_process_time = Local::now().timestamp_millis();

    let verifier =
        Verifier::from_qualifiers(&mut vec![Box::new(WorkerAllocationQualifier::new(140))]);

    let integrity = verifier.verify(&table)?;

    // println!("{}", table);
    println!("{}", integrity);
    println!(
        "Assign Time: {}",
        (end_process_time - start_process_time)
            .to_string()
            .bright_red()
    );

    Ok(())
}
