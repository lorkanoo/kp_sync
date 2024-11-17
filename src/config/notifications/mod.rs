use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notifications {
    pub notify_success: bool,
    pub notify_retry: bool,
    pub notify_failure: bool,
    pub notify_failure_linked: bool,
}

impl Notifications {
    pub fn default() -> Self {
        Self {
            notify_success: true,
            notify_retry: true,
            notify_failure: false,
            notify_failure_linked: false,
        }
    }
}
