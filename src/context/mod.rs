use crate::kp::api::KpResponse;
use chrono::{DateTime, Local};
use nexus::data_link::get_mumble_link;
use nexus::data_link::mumble::MumblePtr;

#[derive(Debug, Clone)]
pub struct Context {
    pub kp_response: KpResponse,
    pub mumble: Option<MumblePtr>,
    pub on_kp_map: bool,
    pub scheduled_refresh: Option<ScheduledRefresh>,
}

#[derive(Clone, Debug)]
pub enum ScheduledRefresh {
    OnKPMapExit,
    OnTime(DateTime<Local>),
}

impl Context {
    pub fn default() -> Self {
        Self {
            kp_response: KpResponse::Unavailable,
            mumble: get_mumble_link(),
            on_kp_map: false,
            scheduled_refresh: None,
        }
    }
}
