pub mod kp_response;
pub mod linked_ids;
pub mod refresh;

use crate::addon::Addon;
use crate::api::kp::kp_response::failure_reason::FailureReason;
use crate::api::kp::kp_response::KpResponse;
use crate::api::kp::linked_ids::fetch_linked_ids;
use crate::api::kp::refresh::refresh_kill_proof;
use crate::context::scheduled_refresh::ScheduledRefresh;
use chrono::Local;
use function_name::named;
use log::{debug, info, warn};
use std::ops::Add;
use std::thread;

const KP_URL: &str = "https://killproof.me";

fn kp_path(kp_id: &String) -> String {
    format!("{}/proof/{}", KP_URL, kp_id)
}

#[named]
pub fn refresh_kp_thread() {
    Addon::threads().push(thread::spawn(|| {
        debug!("[{}] started", function_name!());

        if Addon::lock().context.refresh_in_progress {
            warn!("[{}] refresh is already in progress", function_name!());
            return;
        }
        Addon::lock().context.refresh_in_progress = true;

        if !Addon::lock().config.valid() {
            warn!("[{}] addon configuration is not valid", function_name!());
            return;
        }
        let kp_id = Addon::lock().config.kp_identifiers.main_id.clone();
        Addon::lock().context.linked_kp_responses = vec![];

        let main_kp_response = refresh_kill_proof(&kp_id, true);
        handle_main_kp_response(main_kp_response);

        let linked_ids = Addon::lock().config.kp_identifiers.linked_ids.clone();

        if let Some(linked_ids) = linked_ids {
            let mut kp_responses: Vec<(String, KpResponse)> = Vec::new();
            for linked_id in linked_ids {
                debug!(
                    "[{}] Updating KP for linked id: {}",
                    function_name!(),
                    linked_id
                );
                let kp_response = refresh_kill_proof(&linked_id, false);
                debug!("[{}] Linked kp response: {}", function_name!(), kp_response);
                kp_responses.push((linked_id, kp_response));
            }
            Addon::lock().context.linked_kp_responses = kp_responses;
        }
        info!("[{}] refresh status updated", function_name!());
        Addon::lock().context.refresh_in_progress = false;
    }));
}

#[named]
fn handle_main_kp_response(main_kp_response: KpResponse) {
    let mut addon = Addon::lock();
    match main_kp_response {
        KpResponse::Success => {
            addon.config.last_refresh_date = Some(Local::now());
            addon.context.scheduled_refresh = None;
        }
        KpResponse::Failure(FailureReason::RefreshCooldown(_duration)) => {
            addon.context.scheduled_refresh =
                Some(ScheduledRefresh::OnTime(Local::now().add(_duration)));
            debug!(
                "[{}] Failed to refresh, retrying in {:?}s",
                function_name!(),
                _duration.as_secs()
            );
        }
        KpResponse::InvalidId(_) => {
            addon.context.scheduled_refresh = None;
            addon.config.kp_identifiers.linked_ids = None;
        }
        _ => {}
    }
    addon.context.main_kp_response = main_kp_response;
}

#[named]
pub fn fetch_linked_ids_thread() {
    Addon::threads().push(thread::spawn(|| {
        debug!("[{}] started", function_name!());
        if !Addon::lock().config.valid() {
            warn!("[{}] addon configuration is not valid", function_name!());
            return;
        }
        let kp_id = Addon::lock().config.kp_identifiers.main_id.clone();
        let ids = fetch_linked_ids(&kp_id);
        let mut addon = Addon::lock();
        if ids.is_empty() {
            addon.config.kp_identifiers.linked_ids = None;
            addon.context.ui.errors.linked_ids = true;
        } else {
            addon.config.kp_identifiers.linked_ids = Some(ids);
        }
    }));
}
