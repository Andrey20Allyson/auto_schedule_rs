use std::{fmt::Display, rc::Rc};

use super::{worker_limit::WorkerLimit, worker_shedule::WorkerSchedule};

#[derive(Debug)]
pub enum Graduation {
    Insp,
    SubInsp,
    Gcm,
}

impl Graduation {
    pub fn is_gcm(&self) -> bool {
        match self {
            Graduation::Gcm => true,
            _ => false,
        }
    }

    pub fn is_sub_insp(&self) -> bool {
        match self {
            Graduation::SubInsp => true,
            _ => false,
        }
    }

    pub fn is_insp(&self) -> bool {
        match self {
            Graduation::Insp => true,
            _ => false,
        }
    }
}

impl Default for Graduation {
    fn default() -> Self {
        Graduation::Gcm
    }
}

impl Display for Graduation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Graduation::Gcm => write!(f, "GCM"),
            Graduation::Insp => write!(f, "INSP"),
            Graduation::SubInsp => write!(f, "SUB-INSP"),
        }
    }
}

#[derive(Debug)]
pub enum Gender {
    Male,
    Female,
}

impl Default for Gender {
    fn default() -> Self {
        Gender::Male
    }
}

impl Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Gender::Male => write!(f, "M"),
            Gender::Female => write!(f, "F"),
        }
    }
}

#[derive(Debug)]
pub struct Worker {
    pub id: u64,
    pub name: String,
    pub gender: Gender,
    pub limit: WorkerLimit,
    pub graduation: Graduation,
    pub schedule: WorkerSchedule,
}

impl Worker {
    pub fn new_rc(
        id: u64,
        name: String,
        schedule: WorkerSchedule,
        limit: WorkerLimit,
    ) -> Rc<Worker> {
        Rc::new(Worker {
            id,
            name,
            limit,
            schedule,
            gender: Default::default(),
            graduation: Default::default(),
        })
    }

    pub fn rc_wrap(self) -> Rc<Worker> {
        Rc::new(self)
    }
}
