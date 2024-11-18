use crate::api::kp::cooldown::{cooldown_request, default_cooldown_response};
use crate::api::kp::kp_response::failure_reason::FailureReason;
use crate::api::kp::kp_response::KpResponse;
use crate::api::kp::KP_URL;
use crate::api::{get_sync, print_error_chain};
use ::function_name::named;
use log::error;
use reqwest::blocking::Response;
use reqwest::{self, StatusCode};

#[named]
pub fn refresh_kp_request(kp_id: &String, fetch_cooldown: bool) -> KpResponse {
    let path = refresh_path(kp_id);

    match get_sync(path) {
        Ok(response) => match response.status() {
            StatusCode::OK => handle_ok_http_response(kp_id, response),
            StatusCode::FORBIDDEN => KpResponse::Failure(FailureReason::NotAccessible),
            StatusCode::NOT_FOUND => KpResponse::Failure(FailureReason::NotFound),
            StatusCode::NOT_MODIFIED => handle_not_modified_http_response(kp_id, fetch_cooldown),
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

fn handle_ok_http_response(kp_id: &str, response: Response) -> KpResponse {
    match response.text() {
        Ok(html) => {
            if html.contains(r#"status":"ok"#) {
                KpResponse::Success
            } else {
                KpResponse::InvalidId(kp_id.to_string())
            }
        }
        Err(_) => KpResponse::Failure(FailureReason::Unknown),
    }
}

fn handle_not_modified_http_response(kp_id: &String, fetch_cooldown: bool) -> KpResponse {
    if fetch_cooldown {
        cooldown_request(kp_id)
    } else {
        default_cooldown_response()
    }
}

fn refresh_path(kp_id: &String) -> String {
    format!("{}/proof/{}/refresh", KP_URL, kp_id)
}
