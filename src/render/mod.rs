use nexus::imgui::Ui;

pub mod main;
pub mod options;

fn separate_with_spacing(ui: &Ui) {
    ui.spacing();
    ui.separator();
    ui.spacing();
}

fn table_rows(ui: &Ui, rows: Vec<(String, String)>) {
    for (c1, c2) in rows {
        ui.table_next_column();
        ui.text(c1);
        ui.table_next_column();
        ui.text(c2);
    }
}
