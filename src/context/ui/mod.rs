use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct UiContext {
    pub previous_main_id: String,
    pub new_kp_map_search_term: String,
    pub new_retain_map_search_term: String,
    pub errors: Errors,
    pub map_names: HashMap<String, String>,
}

#[derive(Clone, Debug)]
pub struct Errors {
    pub linked_ids: bool,
}

impl UiContext {
    pub fn default() -> Self {
        Self {
            previous_main_id: "".to_string(),
            new_kp_map_search_term: "".to_string(),
            new_retain_map_search_term: "".to_string(),
            errors: Errors::default(),
            map_names: HashMap::new(),
        }
    }
}

impl Errors {
    pub fn default() -> Self {
        Self { linked_ids: false }
    }
}
