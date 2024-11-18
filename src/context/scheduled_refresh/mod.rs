use crate::render::countdown_str;
use chrono::{DateTime, Local};
use std::fmt;

#[derive(Clone, Debug)]
pub enum ScheduledRefresh {
    OnNormalMapEnter,
    OnTime(DateTime<Local>),
}

impl fmt::Display for ScheduledRefresh {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScheduledRefresh::OnNormalMapEnter => write!(f, "on normal map enter"),
            ScheduledRefresh::OnTime(time) => {
                let delta = time.signed_duration_since(Local::now());
                write!(f, "in {}", countdown_str(delta))
            }
        }
    }
}
