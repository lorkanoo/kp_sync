use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KillproofIdentifiers {
    pub main_id: String,
    pub linked_ids: Option<Vec<String>>,
}

impl KillproofIdentifiers {
    pub fn default() -> Self {
        Self {
            main_id: "".to_string(),
            linked_ids: None,
        }
    }
}
