pub mod scheduled_refresh;
mod ui;
mod clipboard;

use crate::addon::Addon;
use crate::api::kp::kp_response::KpResponse;
use crate::context::scheduled_refresh::ScheduledRefresh;
use crate::context::ui::UiContext;
use nexus::data_link::get_mumble_link;
use nexus::data_link::mumble::MumblePtr;
use std::sync::MutexGuard;
use chrono::{DateTime, Local};
use nexus::data_link::rtapi::read_rtapi;
use crate::context::clipboard::CustomClipboard;
use nexus::rtapi::data::RealTimeData;

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
    pub detected_account_name: String,
    pub clipboard: CustomClipboard,
    pub rtapi: Option<RealTimeData>,
    pub last_config_save_date: DateTime<Local>,
    pub last_refresh_daemon_tick_date: DateTime<Local>,
    pub first_map_tick: bool,
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
            ui: Default::default(),
            detected_account_name: "".to_string(),
            clipboard: CustomClipboard::default(),
            rtapi: None,
            last_config_save_date: Local::now(),
            last_refresh_daemon_tick_date: Local::now(),
            first_map_tick: true,
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
    pub unsafe fn update_rtapi(&mut self) {
        if let Some(rtapi) = read_rtapi() {
            if rtapi.game_build != 0 {
                self.rtapi = Some(rtapi);
            } else {
                self.rtapi = None
            }
        }
    }
}

pub fn init_context(addon: &mut MutexGuard<Addon>) {
    addon.context.ui.previous_main_id = addon.config.kp_identifiers.main_id.clone();
}
