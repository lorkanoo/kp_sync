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
        log::warn!("[{}] refresh is already in progress", function_name!())
    }
    addon.context.kp_response = Pending;
    refresh_thread();
}

#[named]
fn refresh_thread() {
    thread::spawn(|| {
        log::debug!("[{}] started", function_name!());
        let kp_id = Addon::lock().config.kp_id.clone();
        let retry_frequency = Addon::lock().config.retry_frequency;

        let kp_response = refresh_kill_proof(kp_id);
        match kp_response {
            KpResponse::Success => {
                Addon::lock().config.last_refresh_date = Some(Local::now());
            }
            KpResponse::Failure(FailureReason::RefreshCooldown) => {
                Addon::lock().context.scheduled_refresh =
                    Some(ScheduledRefresh::OnTime(Local::now().add(retry_frequency)));
                log::debug!(
                    "[{}] Failed to refresh, retrying in {:?}",
                    function_name!(),
                    retry_frequency
                );
            }
            _ => {}
        }

        Addon::lock().context.kp_response = kp_response;
        log::info!("[{}] refresh status updated", function_name!());
    });
}
