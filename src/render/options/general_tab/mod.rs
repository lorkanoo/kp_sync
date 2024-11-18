use crate::addon::Addon;
use crate::api::kp::kp_response::KpResponse;
use crate::api::kp::linked_ids::fetch_linked_ids_thread;
use crate::api::kp::refresh::refresh_kp_thread;
use crate::render::options::ERROR_COLOR;
use crate::render::{scheduled_refresh_text, separate_with_spacing, table_rows};
use nexus::imgui::Ui;

impl Addon {
    pub fn render_general_tab(&mut self, ui: &Ui) {
        self.render_status_table(ui);
        separate_with_spacing(ui);
        ui.input_text(
            "Kill proof id / account name",
            &mut self.config.kp_identifiers.main_id,
        )
        .build();

        if self.kp_id_changed() {
            self.on_kp_id_change();
            self.context.ui.previous_main_id = self.config.kp_identifiers.main_id.clone();
        }

        if self.config.valid() {
            if self.context.valid(&self.config.kp_identifiers.main_id) {
                self.render_on_valid_state(ui);
            }
        } else if let KpResponse::InvalidId(invalid_id) = &self.context.main_kp_response {
            if invalid_id.eq(&self.config.kp_identifiers.main_id) {
                ui.text_colored(ERROR_COLOR, "KP Id not found. Enter different value.");
            }
        } else {
            ui.text_colored(
                ERROR_COLOR,
                "Enter a valid id, for example: \"xAd8\" or \"jennah.1234\" ",
            );
        }
    }

    fn kp_id_changed(&mut self) -> bool {
        self.config.kp_identifiers.main_id != self.context.ui.previous_main_id
    }

    fn on_kp_id_change(&mut self) {
        self.config.kp_identifiers.linked_ids = None;
        self.config.last_refresh_date = None;
        self.context.ui.errors.linked_ids = false;
        self.context.scheduled_refresh = None;
        self.context.linked_kp_responses.clear();
    }

    fn render_on_valid_state(&mut self, ui: &Ui) {
        if ui.button("Refresh") {
            refresh_kp_thread();
        }
        let mut checkbox_checked = self.config.kp_identifiers.linked_ids.is_some();
        ui.checkbox("Refresh linked accounts", &mut checkbox_checked);

        if checkbox_checked {
            if self.config.kp_identifiers.linked_ids.is_none() {
                self.context.ui.errors.linked_ids = false;
                self.config.kp_identifiers.linked_ids = Some(Vec::new());
                fetch_linked_ids_thread();
            }
        } else {
            self.config.kp_identifiers.linked_ids = None;
            self.context.linked_kp_responses.clear();
        }
        self.render_linked_ids(ui);
    }

    fn render_status_table(&mut self, ui: &Ui) {
        if let Some(_t) = ui.begin_table("status", 2) {
            ui.table_next_row();
            table_rows(
                ui,
                vec![
                    ("Current status".to_string(), self.current_status_text()),
                    (
                        "Scheduled refresh".to_string(),
                        scheduled_refresh_text(&self.context.scheduled_refresh),
                    ),
                    (
                        "Last successful refresh".to_string(),
                        self.last_refresh_text(),
                    ),
                ],
            );
        }
    }

    fn current_status_text(&mut self) -> String {
        if self.context.refresh_in_progress {
            "refresh is in progress..".to_string()
        } else if self.config.valid() {
            self.context.main_kp_response.to_string()
        } else {
            "invalid config (KP id format invalid)".to_string()
        }
    }

    fn last_refresh_text(&mut self) -> String {
        match self.config.last_refresh_date {
            Some(last_refresh) => last_refresh.format("%Y-%m-%d %H:%M").to_string(),
            None => "unavailable".to_string(),
        }
    }

    fn render_linked_ids(&mut self, ui: &Ui) {
        if self.context.ui.errors.linked_ids {
            ui.text_colored(ERROR_COLOR, "Linked accounts not found");
        } else if let Some(ids) = &self.config.kp_identifiers.linked_ids {
            if ids.is_empty() {
                ui.text("Loading..");
            } else {
                ui.text("Linked accounts:");
                for id in ids {
                    ui.text(format!("- {}", id));
                }
            }

            if self.context.refresh_in_progress {
                ui.text("Loading..");
            } else {
                for response in &self.context.linked_kp_responses {
                    let kp_id = &response.0;
                    let kp_response = &response.1;
                    match kp_response {
                        KpResponse::Success => {}
                        _ => ui.text_colored(
                            ERROR_COLOR,
                            format!("Could not refresh {}: {}", kp_id, kp_response),
                        ),
                    }
                }
            }
        }
    }
}
