use nexus::imgui::Ui;

pub mod options;

fn separate_with_spacing(ui: &Ui) {
    ui.spacing();
    ui.separator();
    ui.spacing();
}

trait Renderable {
    fn render(&mut self, ui: &Ui);
}

impl Renderable for String {
    fn render(&mut self, ui: &Ui) {
        ui.table_next_column();
        ui.text(self);
    }
}

impl<T: Renderable, Rest: Renderable> Renderable for (T, Rest) {
    fn render(&mut self, ui: &Ui) {
        let (ref mut first, ref mut rest) = self;
        first.render(ui);
        rest.render(ui);
    }
}

fn table_rows<T: Renderable>(ui: &Ui, rows: Vec<T>) {
    for mut row in rows {
        row.render(ui);
    }
}
