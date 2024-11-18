mod cooldown;
pub mod kp_response;
pub mod linked_ids;
pub mod refresh;

const KP_URL: &str = "https://killproof.me";

fn kp_path(kp_id: &String) -> String {
    format!("{}/proof/{}", KP_URL, kp_id)
}
