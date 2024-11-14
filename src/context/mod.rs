pub mod scheduled_refresh;

use crate::context::scheduled_refresh::ScheduledRefresh;
use crate::kp::api::kp_response::KpResponse;
use nexus::data_link::get_mumble_link;
use nexus::data_link::mumble::MumblePtr;

#[derive(Debug, Clone)]
pub struct Context {
    pub main_kp_response: KpResponse,
    pub linked_kp_responses: Vec<(String, KpResponse)>,
    pub mumble: Option<MumblePtr>,
    pub on_kp_map: bool,
    pub scheduled_refresh: Option<ScheduledRefresh>,
    pub show_linked_ids_err: bool,
    pub previous_main_id: String,
    pub run_background_thread: bool,
    pub refresh_in_progress: bool,
}

impl Context {
    pub fn default() -> Self {
        Self {
            main_kp_response: KpResponse::Unavailable,
            linked_kp_responses: vec![],
            mumble: get_mumble_link(),
            on_kp_map: false,
            scheduled_refresh: None,
            show_linked_ids_err: false,
            previous_main_id: "".to_string(),
            run_background_thread: true,
            refresh_in_progress: false,
        }
    }
}
