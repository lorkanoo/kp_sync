use crate::addon::Addon;
use crate::context::ScheduledRefresh;
use crate::kp::api::KpResponse::Pending;
use crate::kp::api::{refresh_kill_proof, FailureReason, KpResponse};
use chrono::Local;
use function_name::named;
use std::ops::Add;
use std::thread;

pub mod api;

#[named]
pub fn refresh(addon: &mut Addon) {
    log::info!("[{}] started", function_name!());
    if matches!(addon.context.kp_response, Pending) {
        log::warn!("[{}] refresh is already in progress", function_name!());
    } else if !addon.config.valid() {
        log::warn!("[{}] addon configuration is not valid", function_name!());
    } else {
        addon.context.kp_response = Pending;
        refresh_thread();
    }
}

#[named]
fn refresh_thread() {
    thread::spawn(|| {
        log::debug!("[{}] started", function_name!());
        let kp_id = Addon::lock().config.kp_id.clone();

        let kp_response = refresh_kill_proof(kp_id);
        match kp_response {
            KpResponse::Success => {
                Addon::lock().config.last_refresh_date = Some(Local::now());
                Addon::lock().context.scheduled_refresh = None;
            }
            KpResponse::Failure(FailureReason::RefreshCooldown(_duration)) => {
                Addon::lock().context.scheduled_refresh =
                    Some(ScheduledRefresh::OnTime(Local::now().add(_duration)));
                log::debug!(
                    "[{}] Failed to refresh, retrying in {:?}s",
                    function_name!(),
                    _duration.as_secs()
                );
            }
            KpResponse::InvalidId(_) => Addon::lock().context.scheduled_refresh = None,
            _ => {}
        }

        Addon::lock().context.kp_response = kp_response;
        log::info!("[{}] refresh status updated", function_name!());
    });
}
