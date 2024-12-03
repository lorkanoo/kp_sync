use crate::addon::Addon;
use crate::render::options::ERROR_COLOR;
use nexus::imgui::{TreeNodeFlags, Ui};

impl Addon {
    pub fn render_advanced_tab(&mut self, ui: &Ui) {
        self.render_notification_options(ui);
        if ui.collapsing_header(
            "Configuration##kp",
            TreeNodeFlags::SPAN_AVAIL_WIDTH | TreeNodeFlags::DEFAULT_OPEN,
        ) {
            ui.spacing();
            ui.checkbox(
                "Enable scheduling on map load",
                &mut self.config.scheduling_on_map_enter_enabled,
            );
            ui.new_line();
            if self.config.scheduling_on_map_enter_enabled {
                ui.text("Maps that schedule refresh to be triggered when non-kp map is loaded: ");
                ui.spacing();
                self.render_kp_maps(ui);
                ui.new_line();
                ui.text("Maps that extend scheduled refresh until non-kp map is loaded: ");
                ui.spacing();
                self.render_retain_refresh_maps(ui);
            }
            ui.new_line();
        }
        if ui.collapsing_header(
            "Additional information##kp",
            TreeNodeFlags::SPAN_AVAIL_WIDTH,
        ) {
            if let Some(m) = self.context.mumble {
                ui.text(format!("Current map id: {}", m.read_map_id()));
            }
            ui.new_line();
        }
    }

    fn render_notification_options(&mut self, ui: &Ui) {
        if ui.collapsing_header(
            "Notifications##kp",
            TreeNodeFlags::SPAN_AVAIL_WIDTH | TreeNodeFlags::DEFAULT_OPEN,
        ) {
            let notifications = &mut self.config.notifications;
            ui.checkbox(
                "Notify on successful refresh",
                &mut notifications.notify_success,
            );
            ui.checkbox("Notify on scheduled retry", &mut notifications.notify_retry);
            ui.checkbox(
                "Notify on failed refresh",
                &mut notifications.notify_failure,
            );
            ui.checkbox(
                "Notify on failed linked account refresh",
                &mut self.config.notifications.notify_failure_linked,
            );
            ui.new_line();
        }
    }

    fn render_kp_maps(&mut self, ui: &Ui) {
        let mut to_remove = Vec::new();
        if let Some(_t) = ui.begin_table("kp_map_ids", 3) {
            ui.table_next_row();
            for (i, map_id) in self.config.kp_map_ids.iter().enumerate() {
                ui.table_next_column();
                ui.text_colored(ERROR_COLOR, "[X]");
                ui.same_line_with_pos(-10f32);
                if ui.invisible_button(format!("-##km{}", map_id), [30f32, 30f32]) {
                    to_remove.push(i);
                }
                if ui.is_item_hovered() {
                    ui.tooltip_text(format!("Map id: {}", map_id));
                }
                ui.same_line_with_pos(24f32);
                let map_id_str = &map_id.to_string();
                let map_name = self
                    .context
                    .ui
                    .map_names
                    .get(map_id_str)
                    .unwrap_or(map_id_str);
                ui.text(map_name);
            }
        }
        for map_index in to_remove {
            self.config.kp_map_ids.remove(map_index);
        }

        ui.spacing();
        ui.input_text(
            "Search maps##km",
            &mut self.context.ui.new_kp_map_search_term,
        )
        .build();
        let search_term = &self.context.ui.new_kp_map_search_term.to_lowercase();

        self.search_maps(ui, search_term, SearchMapType::KpMap);
    }

    fn search_maps(&mut self, ui: &Ui, search_term: &String, map_type: SearchMapType) {
        if !search_term.is_empty() {
            let mut search_results: Vec<(&String, &String)> = self
                .context
                .ui
                .map_names
                .iter()
                .filter(|(map_id, map_name)| {
                    let map_id_u32 = &map_id.parse().unwrap();
                    format!("{} ({})", map_name.to_lowercase(), map_id).contains(search_term)
                        && !self.config.kp_map_ids.contains(map_id_u32)
                        && !self.config.retain_refresh_map_ids.contains(map_id_u32)
                })
                .take(6)
                .collect();

            let parsed_label;
            let parsed_map_id;
            if let Ok(map_id) = search_term.parse::<u32>() {
                parsed_map_id = map_id.to_string();
                parsed_label = "Add unknown map id".to_string();
                if !self.config.kp_map_ids.iter().any(|id| *id == map_id)
                    && !self
                        .config
                        .retain_refresh_map_ids
                        .iter()
                        .any(|id| *id == map_id)
                {
                    search_results.push((&parsed_map_id, &parsed_label));
                }
            }

            for chunk in search_results.chunks(2) {
                for (id, map_name) in chunk {
                    if ui.button(format!("{} ({})", map_name, id)) {
                        match map_type {
                            SearchMapType::KpMap => {
                                self.config.kp_map_ids.push(id.parse().unwrap())
                            }
                            SearchMapType::RetainMap => {
                                self.config.retain_refresh_map_ids.push(id.parse().unwrap())
                            }
                        }
                    }
                    ui.same_line();
                }
                ui.new_line();
            }
        }
    }

    fn render_retain_refresh_maps(&mut self, ui: &Ui) {
        let mut to_remove = Vec::new();
        if let Some(_t) = ui.begin_table("retain_refresh_map_ids", 3) {
            ui.table_next_row();
            for (i, map_id) in self.config.retain_refresh_map_ids.iter().enumerate() {
                ui.table_next_column();
                ui.text_colored(ERROR_COLOR, "[X]");
                ui.same_line_with_pos(-10f32);
                if ui.invisible_button(format!("##rrm{}", map_id), [30f32, 30f32]) {
                    to_remove.push(i);
                }
                if ui.is_item_hovered() {
                    ui.tooltip_text(format!("Map id: {}", map_id));
                }
                ui.same_line_with_pos(24f32);
                let map_id_str = &map_id.to_string();
                let map_name = self
                    .context
                    .ui
                    .map_names
                    .get(map_id_str)
                    .unwrap_or(map_id_str);
                ui.text(map_name);
            }
        }
        for map_index in to_remove {
            self.config.retain_refresh_map_ids.remove(map_index);
        }

        ui.spacing();
        ui.input_text(
            "Search maps##rrm",
            &mut self.context.ui.new_retain_map_search_term,
        )
        .build();
        let search_term = &self.context.ui.new_retain_map_search_term.to_lowercase();

        self.search_maps(ui, search_term, SearchMapType::RetainMap);
    }
}

enum SearchMapType {
    KpMap,
    RetainMap,
}
