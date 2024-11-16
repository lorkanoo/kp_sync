use crate::addon::Addon;
use crate::render::separate_with_spacing;
use nexus::imgui::Ui;

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
        if let Some(_t) = ui.begin_table("kp_map_ids", 3) {
            ui.table_next_row();
            for (i, map_id) in self.config.kp_map_ids.iter().enumerate() {
                ui.table_next_column();
                if ui.small_button(format!("-##km{}", map_id)) {
                    to_remove.push(i);
                }
                ui.same_line();
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
            let search_results: Vec<(&String, &String)> = self
                .context
                .ui
                .map_names
                .iter()
                .filter(|(map_id, map_name)| {
                    let map_id_u32 = &map_id.parse().unwrap();
                    map_name.to_lowercase().contains(search_term)
                        && !self.config.kp_map_ids.contains(map_id_u32)
                        && !self.config.retain_refresh_map_ids.contains(map_id_u32)
                })
                .take(3)
                .collect();

            for (id, map_name) in search_results {
                if ui.button(format!("{} ({})", map_name, id)) {
                    match map_type {
                        SearchMapType::KpMap => self.config.kp_map_ids.push(id.parse().unwrap()),
                        SearchMapType::RetainMap => {
                            self.config.retain_refresh_map_ids.push(id.parse().unwrap())
                        }
                    }
                }
                ui.same_line();
            }
        }
    }

    fn render_retain_refresh_maps(&mut self, ui: &Ui) {
        let mut to_remove = Vec::new();
        if let Some(_t) = ui.begin_table("retain_refresh_map_ids", 3) {
            ui.table_next_row();
            for (i, map_id) in self.config.retain_refresh_map_ids.iter().enumerate() {
                ui.table_next_column();
                if ui.small_button(format!("-##rrm{}", map_id)) {
                    to_remove.push(i);
                }
                ui.same_line();
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
