use std::fmt;
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum FailureReason {
    NotFound,
    NotAccessible,
    RefreshCooldown(Duration),
    Unknown,
}

impl fmt::Display for FailureReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FailureReason::NotFound => write!(f, "not found"),
            FailureReason::NotAccessible => write!(f, "not accessible"),
            FailureReason::RefreshCooldown(_) => write!(f, "refreshed too recently"),
            FailureReason::Unknown => write!(f, "unknown error occurred"),
        }
    }
}
