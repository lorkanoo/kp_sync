use crate::api::kp::kp_path;
use crate::api::kp::kp_response::failure_reason::FailureReason;
use crate::api::kp::kp_response::KpResponse;
use crate::api::{get_sync, print_error_chain};
use ::function_name::named;
use log::{error, warn};
use regex::Regex;
use std::time::Duration;

const DEFAULT_RETRY_FREQUENCY: Duration = Duration::new(5 * 60, 0);

#[named]
pub fn cooldown_request(kp_id: &String) -> KpResponse {
    match get_sync(kp_path(kp_id)) {
        Ok(response) => match response.text() {
            Ok(html) => match extract_duration(html) {
                Some(duration) => KpResponse::Failure(FailureReason::RefreshCooldown(duration)),
                None => default_cooldown_response(),
            },
            _ => {
                warn!("[{}] Could not get html", function_name!());
                default_cooldown_response()
            }
        },
        Err(error) => {
            error!("[{}] Unknown error: {}", function_name!(), error);
            print_error_chain(&error);
            default_cooldown_response()
        }
    }
}

pub fn default_cooldown_response() -> KpResponse {
    KpResponse::Failure(FailureReason::RefreshCooldown(DEFAULT_RETRY_FREQUENCY))
}

fn extract_duration(text: String) -> Option<Duration> {
    let re = Regex::new(r"Time until next refresh available is (\d+) minute").unwrap();
    let added_seconds = 30;
    if let Some(caps) = re.captures(text.as_str()) {
        caps.get(1)?
            .as_str()
            .parse::<u64>()
            .ok()
            .map(|minutes| Duration::new(minutes * 60 + added_seconds, 0))
    } else {
        None
    }
}
