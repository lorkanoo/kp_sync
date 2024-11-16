mod advanced_tab;
mod general_tab;
use crate::addon::Addon;
use nexus::imgui::Ui;

const ERROR_COLOR: [f32; 4] = [0.4, 0.4, 0.4, 1.0];
impl Addon {
    pub fn render_options(&mut self, ui: &Ui) {
        if let Some(_token) = ui.tab_bar("options") {
            if let Some(_token) = ui.tab_item("General") {
                self.render_general_tab(ui);
            }

            if let Some(_token) = ui.tab_item("Advanced") {
                self.render_advanced_tab(ui);
            }
        }
    }
}
