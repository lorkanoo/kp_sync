use crate::addon::Addon;
use crate::context::ScheduledRefresh;
use crate::kp::api::{FailureReason, KpResponse};
use crate::kp::refresh;
use crate::render::{separate_with_spacing, table_rows};
use chrono::Local;
use function_name::named;
use nexus::imgui::Ui;
use std::fmt;

const ERROR_COLOR: [f32; 4] = [0.4, 0.4, 0.4, 1.0];
impl Addon {
    #[named]
    pub fn render_options(&mut self, ui: &Ui) {
        self.render_status_table(ui);
        separate_with_spacing(ui);
        ui.input_text("Kill proof id / account name", &mut self.config.kp_id)
            .build();
        self.error_text(ui);
        if ui.button("Refresh") {
            refresh(self);
        }
    }

    fn error_text(&mut self, ui: &Ui) {
        if !self.config.valid() {
            ui.text_colored(
                ERROR_COLOR,
                "Enter valid id, for example: \"xAd8\" or \"jennah.1234\" ",
            );
        } else {
            ui.text("");
        }
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
                        self.scheduled_refresh_text(),
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
        if self.config.valid() {
            self.context.kp_response.to_string()
        } else {
            "invalid configuration".to_string()
        }
    }

    fn last_refresh_text(&mut self) -> String {
        match self.config.last_refresh_date {
            Some(last_refresh) => last_refresh.format("%Y-%m-%d %H:%M").to_string(),
            None => "unavailable".to_string(),
        }
    }

    fn scheduled_refresh_text(&mut self) -> String {
        self.context
            .scheduled_refresh
            .as_ref()
            .map_or_else(|| "unavailable".to_string(), |refresh| refresh.to_string())
    }
}

impl fmt::Display for ScheduledRefresh {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScheduledRefresh::OnKPMapExit => write!(f, "On raid/strike exit"),
            ScheduledRefresh::OnTime(time) => {
                let delta = time.signed_duration_since(Local::now());
                if delta.num_seconds() > 0 {
                    write!(f, "in {}s", delta.num_seconds())
                } else {
                    write!(f, "starts soon..")
                }
            }
        }
    }
}

impl fmt::Display for KpResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KpResponse::Pending => write!(f, "refresh is in progress.."),
            KpResponse::Unavailable => write!(f, "not refreshed recently"),
            KpResponse::Success => write!(f, "refresh successful"),
            KpResponse::Failure(reason) => write!(f, "failed ({})", reason),
        }
    }
}

impl fmt::Display for FailureReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FailureReason::NotFound => write!(f, "not found"),
            FailureReason::NotAccessible => write!(f, "not accessible"),
            FailureReason::RefreshCooldown => write!(f, "refreshed too recently"),
            FailureReason::Unknown => write!(f, "unknown error"),
        }
    }
}
