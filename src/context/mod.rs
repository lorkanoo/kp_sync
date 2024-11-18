pub mod scheduled_refresh;
mod ui;

use crate::addon::Addon;
use crate::api::kp::kp_response::KpResponse;
use crate::context::scheduled_refresh::ScheduledRefresh;
use crate::context::ui::UiContext;
use nexus::data_link::get_mumble_link;
use nexus::data_link::mumble::MumblePtr;
use std::sync::MutexGuard;

#[derive(Debug, Clone)]
pub struct Context {
    pub main_kp_response: KpResponse,
    pub linked_kp_responses: Vec<(String, KpResponse)>,
    pub mumble: Option<MumblePtr>,
    pub scheduled_refresh: Option<ScheduledRefresh>,
    pub on_kp_map: bool,
    pub run_background_thread: bool,
    pub refresh_in_progress: bool,
    pub ui: UiContext,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            main_kp_response: KpResponse::Unavailable,
            linked_kp_responses: vec![],
            mumble: get_mumble_link(),
            on_kp_map: false,
            scheduled_refresh: None,
            run_background_thread: true,
            refresh_in_progress: false,
            ui: UiContext::default(),
        }
    }
}
impl Context {
    pub fn valid(&mut self, main_kp_id: &str) -> bool {
        match &self.main_kp_response {
            KpResponse::InvalidId(invalid_id) => invalid_id != main_kp_id,
            _ => true,
        }
    }
}

pub fn init_context(addon: &mut MutexGuard<Addon>) {
    addon.context.ui.previous_main_id = addon.config.kp_identifiers.main_id.clone();
}
