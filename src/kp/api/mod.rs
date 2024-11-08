use ::function_name::named;
use log::error;
use reqwest::{self, Certificate, Error, StatusCode};
use std::error::Error as StdError;

#[derive(Debug, Clone)]
pub enum FailureReason {
    NotFound,
    NotAccessible,
    RefreshCooldown,
    Unknown,
}

#[derive(Debug, Clone)]
pub enum KpResponse {
    Pending,
    Unavailable,
    Success,
    Failure(FailureReason),
}

const KP_URL: &str = "https://killproof.me";

#[named]
pub fn refresh_kill_proof(kp_id: String) -> KpResponse {
    let path = refresh_path(kp_id);

    match get_sync(path) {
        Ok(response) => match response.status() {
            StatusCode::OK => KpResponse::Success,
            StatusCode::FORBIDDEN => KpResponse::Failure(FailureReason::NotAccessible),
            StatusCode::NOT_FOUND => KpResponse::Failure(FailureReason::NotFound),
            StatusCode::NOT_MODIFIED => KpResponse::Failure(FailureReason::RefreshCooldown),
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

fn refresh_path(kp_id: String) -> String {
    format!("{}/proof/{}/refresh", KP_URL, kp_id)
}
