use crate::api::kp::kp_path;
use crate::api::{get_sync, print_error_chain};
use ::function_name::named;
use log::{debug, error, warn};
use select::document::Document;
use select::predicate::{Class, Name};

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