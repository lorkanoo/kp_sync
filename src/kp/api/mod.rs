pub mod kp_response;
pub mod linked_ids;
pub mod refresh;

use log::error;
use reqwest::{self, Error};
use std::error::Error as StdError;

const KP_URL: &str = "https://killproof.me";

fn print_error_chain(error: &dyn StdError) {
    error!("Error: {}", error);
    let mut source = error.source();
    while let Some(err) = source {
        error!("Caused by: {}", err);
        source = err.source();
    }
}

fn get_sync(url: String) -> Result<reqwest::blocking::Response, Error> {
    let client = reqwest::blocking::Client::builder()
        .danger_accept_invalid_hostnames(true)
        .use_rustls_tls()
        .build()
        .expect("error");
    client.get(url).send()
}

fn kp_path(kp_id: &String) -> String {
    format!("{}/proof/{}", KP_URL, kp_id)
}
