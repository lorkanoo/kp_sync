use log::error;
use reqwest::Error;
use serde::de::StdError;

pub mod gw2;
pub mod kp;

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
