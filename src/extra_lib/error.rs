#[derive(Debug)]
pub enum ExtraError {
    CantFindDuty { index: usize },
    CantFindDay { index: usize },
}

impl std::error::Error for ExtraError {}

impl std::fmt::Display for ExtraError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExtraError::CantFindDay { index } => write!(f, "Can't find day at index {}", index),
            ExtraError::CantFindDuty { index } => write!(f, "Can't find duty at index {}", index),
        }
    }
}

pub type DynError = Box<dyn std::error::Error>;
