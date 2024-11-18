use crate::addon::Addon;
use crate::api::kp::kp_path;
use crate::api::kp::kp_response::KpResponse;
use crate::api::kp::refresh::request::refresh_kp_request;
use crate::api::{get_sync, print_error_chain};
use ::function_name::named;
use log::{debug, error, warn};
use nexus::alert::send_alert;
use select::document::Document;
use select::predicate::{Class, Name};
use std::thread;

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

#[named]
pub fn refresh_linked_kp(linked_id: &String) -> KpResponse {
    debug!(
        "[{}] Updating KP for linked id: {}",
        function_name!(),
        linked_id
    );
    let kp_response = refresh_kp_request(linked_id, false);
    debug!("[{}] Linked kp response: {}", function_name!(), kp_response);
    if Addon::lock().config.notifications.notify_failure_linked
        && matches!(kp_response, KpResponse::Failure(_))
    {
        send_alert(format!(
            "Linked Killproof account {} could not be refreshed",
            linked_id
        ));
    }
    kp_response
}

#[named]
pub fn fetch_linked_ids(kp_id: &String) -> Vec<String> {
    match get_sync(kp_path(kp_id)) {
        Ok(response) => match response.text() {
            Ok(html) => extract_linked_ids(html),
            _ => {
                warn!("[{}] Could not get html", function_name!());
                vec![]
            }
        },
        Err(error) => {
            error!("[{}] Unknown error: {}", function_name!(), error);
            print_error_chain(&error);
            vec![]
        }
    }
}

#[named]
fn extract_linked_ids(html: String) -> Vec<String> {
    let mut linked_accounts = Vec::new();
    let document = Document::from(html.as_str());
    let link = document.find(Class("fa-link")).next();
    if let Some(link) = link {
        let paragraph = link.parent();
        for a_tag in paragraph.unwrap().find(Name("a")) {
            linked_accounts.push(a_tag.text());
        }
    }
    debug!(
        "[{}] Extracted linked ids: {:?}",
        function_name!(),
        linked_accounts
    );
    linked_accounts
}
