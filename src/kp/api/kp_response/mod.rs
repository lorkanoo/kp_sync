pub mod failure_reason;

use crate::kp::api::kp_response::failure_reason::FailureReason;
use std::fmt;

#[derive(Debug, Clone)]
pub enum KpResponse {
    Unavailable,
    Success,
    InvalidId(String),
    Failure(FailureReason),
}

impl fmt::Display for KpResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KpResponse::Unavailable => write!(f, "not refreshed recently"),
            KpResponse::Success => write!(f, "refresh successful"),
            KpResponse::Failure(reason) => write!(f, "failed ({})", reason),
            KpResponse::InvalidId(kp_id) => {
                write!(f, "invalid config (KP id \"{}\" not found)", kp_id)
            }
        }
    }
}
