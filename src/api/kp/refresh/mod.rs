pub mod request;

use crate::addon::Addon;
use crate::api::kp::kp_response::failure_reason::FailureReason;
use crate::api::kp::kp_response::KpResponse;
use crate::api::kp::linked_ids::refresh_linked_kp;
use crate::api::kp::refresh::request::refresh_kp_request;
use crate::context::scheduled_refresh::ScheduledRefresh;
use crate::render::countdown_str;
use ::function_name::named;
use chrono::{Local, TimeDelta};
use log::{debug, info, warn};
use nexus::alert::send_alert;
use std::ops::Add;
use std::sync::MutexGuard;
use std::thread;
use std::time::Duration;

#[named]
fn cant_start_refresh() -> bool {
    if !Addon::lock().config.valid() {
        warn!("[{}] addon configuration is not valid", function_name!());
        return true;
    }
    if Addon::lock().context.refresh_in_progress {
        warn!("[{}] refresh is already in progress", function_name!());
        return true;
    }
    false
}

#[named]
pub fn refresh_kp_thread() {
    Addon::threads().push(thread::spawn(|| {
        debug!("[{}] started", function_name!());
        if cant_start_refresh() {
            return;
        }
        Addon::lock().context.refresh_in_progress = true;

        let kp_id = Addon::lock().config.kp_identifiers.main_id.clone();
        Addon::lock().context.linked_kp_responses = vec![];

        let main_kp_response = refresh_kp_request(&kp_id, true);
        handle_main_kp_response(main_kp_response);

        let linked_ids = Addon::lock().config.kp_identifiers.linked_ids.clone();

        if let Some(linked_ids) = linked_ids {
            let mut kp_responses: Vec<(String, KpResponse)> = Vec::new();
            for linked_id in linked_ids {
                let kp_response = refresh_linked_kp(&linked_id);
                kp_responses.push((linked_id, kp_response));
            }
            Addon::lock().context.linked_kp_responses = kp_responses;
        }
        info!("[{}] refresh status updated", function_name!());
        Addon::lock().context.refresh_in_progress = false;
    }));
}
fn handle_main_kp_response(main_kp_response: KpResponse) {
    let mut addon = Addon::lock();
    match main_kp_response {
        KpResponse::Success => handle_success_kp_response(&mut addon),
        KpResponse::Failure(FailureReason::RefreshCooldown(duration)) => {
            handle_failure_cooldown_kp_response(&mut addon, duration)
        }
        KpResponse::InvalidId(_) => handle_invalid_id_kp_response(&mut addon),
        _ => {
            if addon.config.notifications.notify_failure {
                send_alert("Killproof could not be refreshed due to unknown error");
            }
        }
    }
    addon.context.main_kp_response = main_kp_response;
}

fn handle_invalid_id_kp_response(addon: &mut MutexGuard<Addon>) {
    addon.context.scheduled_refresh = None;
    addon.config.kp_identifiers.linked_ids = None;
    if addon.config.notifications.notify_failure {
        send_alert("Killproof could not be refreshed due to invalid configuration");
    }
}

fn handle_success_kp_response(addon: &mut MutexGuard<Addon>) {
    addon.config.last_refresh_date = Some(Local::now());
    addon.context.scheduled_refresh = None;
    if addon.config.notifications.notify_success {
        send_alert("Killproof refreshed successfully");
    }
}

#[named]
fn handle_failure_cooldown_kp_response(addon: &mut MutexGuard<Addon>, duration: Duration) {
    addon.context.scheduled_refresh = Some(ScheduledRefresh::OnTime(Local::now().add(duration)));
    debug!(
        "[{}] Failed to refresh, retrying in {:?}s",
        function_name!(),
        duration.as_secs()
    );
    if addon.config.notifications.notify_retry {
        send_alert(format!(
            "Killproof could not be refreshed, retrying in {}",
            countdown_str(TimeDelta::seconds(duration.as_secs() as i64))
        ));
    }
}
