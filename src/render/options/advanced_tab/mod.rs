use crate::addon::Addon;
use crate::render::options::ERROR_COLOR;
use crate::render::separate_with_spacing;
use nexus::imgui::Ui;
use std::num::ParseIntError;

impl Addon {
    pub fn render_advanced_tab(&mut self, ui: &Ui) {
        ui.text("Maps that schedule refresh to be triggered when non-kp map is loaded: ");
        ui.spacing();
        self.render_kp_maps(ui);
        separate_with_spacing(ui);
        ui.text("Maps that extend scheduled refresh until non-kp map is loaded: ");
        ui.spacing();
        self.render_retain_refresh_maps(ui);
        separate_with_spacing(ui);
        ui.text("Additional information:");
        ui.spacing();
        if let Some(m) = self.context.mumble {
            ui.text(format!("Current map id: {}", m.read_map_id()));
        }
        ui.spacing();
        if ui.button("Browse map ids") {
            if let Err(err) = open::that_detached("https://api.guildwars2.com/v1/map_names.json") {
                log::error!("Failed to open map ids url: {err}");
            }
        }
    }

    fn render_kp_maps(&mut self, ui: &Ui) {
        let mut to_remove = Vec::new();
        if let Some(_t) = ui.begin_table("kp_map_ids", 6) {
            ui.table_next_row();
            for (i, map_id) in self.config.kp_map_ids.iter().enumerate() {
                ui.table_next_column();
                if ui.small_button(format!("-##km{}", map_id)) {
                    to_remove.push(i);
                }
                ui.same_line();
                ui.text(map_id.to_string());
            }
        }
        for map_index in to_remove {
            self.config.kp_map_ids.remove(map_index);
        }

        ui.spacing();
        ui.input_text("Map id##km", &mut self.context.ui.new_map_id)
            .build();
        let new_map_id: Result<u32, ParseIntError> = self.context.ui.new_map_id.parse();
        if new_map_id.is_ok() {
            let id = new_map_id.unwrap();
            if self.map_already_added(&id) {
                ui.text_colored(ERROR_COLOR, "Map already added");
            } else if ui.button("Add") {
                self.config.kp_map_ids.push(id);
            }
        } else if !self.context.ui.new_map_id.is_empty() {
            ui.text_colored(ERROR_COLOR, "Incorrect map id");
        } else {
            ui.text_disabled("Enter id of a map to add")
        }
    }

    fn render_retain_refresh_maps(&mut self, ui: &Ui) {
        let mut to_remove = Vec::new();
        if let Some(_t) = ui.begin_table("retain_refresh_map_ids", 6) {
            ui.table_next_row();
            for (i, map_id) in self.config.retain_refresh_map_ids.iter().enumerate() {
                ui.table_next_column();
                if ui.small_button(format!("-##rrm{}", map_id)) {
                    to_remove.push(i);
                }
                ui.same_line();
                ui.text(map_id.to_string());
            }
        }
        for map_index in to_remove {
            self.config.retain_refresh_map_ids.remove(map_index);
        }

        ui.spacing();
        ui.input_text("Map id##rrm", &mut self.context.ui.new_retain_map_id)
            .build();
        let retain_map_id: Result<u32, ParseIntError> = self.context.ui.new_retain_map_id.parse();
        if retain_map_id.is_ok() {
            let id = retain_map_id.unwrap();
            if self.map_already_added(&id) {
                ui.text_colored(ERROR_COLOR, "Map already added");
            } else if ui.button("Add") {
                self.config.retain_refresh_map_ids.push(id);
            }
        } else if !self.context.ui.new_retain_map_id.is_empty() {
            ui.text_colored(ERROR_COLOR, "Incorrect map id");
        } else {
            ui.text_disabled("Enter id of a map to add")
        }
    }

    fn map_already_added(&mut self, id: &u32) -> bool {
        self.config.kp_map_ids.contains(id) || self.config.retain_refresh_map_ids.contains(id)
    }
}
