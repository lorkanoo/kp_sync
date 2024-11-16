#[derive(Clone, Debug)]
pub struct UiContext {
    pub previous_main_id: String,
    pub new_map_id: String,
    pub new_retain_map_id: String,
    pub errors: Errors,
}

#[derive(Clone, Debug)]
pub struct Errors {
    pub linked_ids: bool,
}

impl UiContext {
    pub fn default() -> Self {
        Self {
            previous_main_id: "".to_string(),
            new_map_id: "".to_string(),
            new_retain_map_id: "".to_string(),
            errors: Errors::default(),
        }
    }
}

impl Errors {
    pub fn default() -> Self {
        Self { linked_ids: false }
    }
}
