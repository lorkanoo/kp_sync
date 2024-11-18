use crate::addon::Addon;
use crate::api::kp::refresh::refresh_kp_thread;
use crate::render::scheduled_refresh_text;
use nexus::imgui::Ui;

impl Addon {
    pub fn render_quick_access(&mut self, ui: &Ui) {
        ui.text(format!(
            "Scheduled refresh: {}",
            scheduled_refresh_text(&self.context.scheduled_refresh)
        ));
        ui.spacing();
        if self.config.valid() && self.context.valid(&self.config.kp_identifiers.main_id) {
            if ui.button(" Refresh ") {
                refresh_kp_thread();
            }
        } else {
            ui.text_disabled("Refresh (configuration is not valid)");
        }
    }
}
