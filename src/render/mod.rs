use crate::context::scheduled_refresh::ScheduledRefresh;
use chrono::TimeDelta;
use nexus::imgui::Ui;

pub mod options;
mod quick_access;

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

pub fn scheduled_refresh_text(scheduled_refresh: &Option<ScheduledRefresh>) -> String {
    scheduled_refresh
        .as_ref()
        .map_or_else(|| "not planned".to_string(), |refresh| refresh.to_string())
}

pub fn countdown_str(delta: TimeDelta) -> String {
    if delta.num_minutes() > 0 {
        format!("{} minutes", delta.num_minutes() + 1)
    } else {
        let seconds = delta.num_seconds();
        if seconds > 0 {
            format!("{} second{}", seconds, if seconds > 1 { "s" } else { "" })
        } else {
            "starts soon..".to_string()
        }
    }
}
