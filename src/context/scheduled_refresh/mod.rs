use chrono::{DateTime, Local};
use std::fmt;

#[derive(Clone, Debug)]
pub enum ScheduledRefresh {
    OnKPMapExit,
    OnTime(DateTime<Local>),
}

impl fmt::Display for ScheduledRefresh {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScheduledRefresh::OnKPMapExit => write!(f, "On raid/strike exit"),
            ScheduledRefresh::OnTime(time) => {
                let delta = time.signed_duration_since(Local::now());
                if delta.num_minutes() > 0 {
                    write!(f, "in {} minutes", delta.num_minutes() + 1)
                } else {
                    let seconds = delta.num_seconds();
                    if seconds > 0 {
                        write!(
                            f,
                            "in {} second{}",
                            seconds,
                            if seconds > 1 { "s" } else { "" }
                        )
                    } else {
                        write!(f, "starts soon..")
                    }
                }
            }
        }
    }
}
