use ::function_name::named;
use log::{error, warn};
use regex::Regex;
use reqwest::{self, Certificate, Error, StatusCode};
use std::error::Error as StdError;
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum FailureReason {
    NotFound,
    NotAccessible,
    RefreshCooldown(Duration),
    Unknown,
}

#[derive(Debug, Clone)]
pub enum KpResponse {
    Pending,
    Unavailable,
    Success,
    InvalidId(String),
    Failure(FailureReason),
}

const KP_URL: &str = "https://killproof.me";
const DEFAULT_RETRY_FREQUENCY: Duration = Duration::new(5 * 60, 0);

#[named]
pub fn refresh_kill_proof(kp_id: String) -> KpResponse {
    let path = refresh_path(&kp_id);

    match get_sync(path) {
        Ok(response) => match response.status() {
            StatusCode::OK => match response.text() {
                Ok(html) => {
                    if html.contains(r#"{"status":"ok","min":0}"#) {
                        KpResponse::Success
                    } else {
                        KpResponse::InvalidId(kp_id)
                    }
                }
                Err(_) => KpResponse::Failure(FailureReason::Unknown),
            },
            StatusCode::FORBIDDEN => KpResponse::Failure(FailureReason::NotAccessible),
            StatusCode::NOT_FOUND => KpResponse::Failure(FailureReason::NotFound),
            StatusCode::NOT_MODIFIED => cooldown_response(&kp_id),
            _ => {
                error!(
                    "[{}] Unknown status: {}",
                    function_name!(),
                    response.status()
                );
                KpResponse::Failure(FailureReason::Unknown)
            }
        },
        Err(error) => {
            error!("[{}] Unknown error: {}", function_name!(), error);
            print_error_chain(&error);
            KpResponse::Failure(FailureReason::Unknown)
        }
    }
}

#[named]
fn cooldown_response(kp_id: &String) -> KpResponse {
    match get_sync(cooldown_path(kp_id)) {
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

fn default_cooldown_response() -> KpResponse {
    KpResponse::Failure(FailureReason::RefreshCooldown(DEFAULT_RETRY_FREQUENCY))
}

fn extract_duration(text: String) -> Option<Duration> {
    let re = Regex::new(r"Time until next refresh available is (\d+) minutes").unwrap();
    let buffer = 30;
    if let Some(caps) = re.captures(text.as_str()) {
        caps.get(1)?
            .as_str()
            .parse::<u64>()
            .ok()
            .map(|minutes| Duration::new(minutes * 60 + buffer, 0))
    } else {
        None
    }
}

fn print_error_chain(error: &dyn StdError) {
    error!("Error: {}", error);
    let mut source = error.source();
    while let Some(err) = source {
        error!("Caused by: {}", err);
        source = err.source();
    }
}

fn get_sync(url: String) -> Result<reqwest::blocking::Response, Error> {
    let cert = Certificate::from_pem(include_bytes!("cert/killproof.pem"))
        .expect("Failed to load certificate");
    let client = reqwest::blocking::Client::builder()
        .use_rustls_tls()
        .add_root_certificate(cert)
        .build()
        .expect("error");
    client.get(url).send()
}

fn refresh_path(kp_id: &String) -> String {
    format!("{}/proof/{}/refresh", KP_URL, kp_id)
}

fn cooldown_path(kp_id: &String) -> String {
    format!("{}/proof/{}", KP_URL, kp_id)
}
