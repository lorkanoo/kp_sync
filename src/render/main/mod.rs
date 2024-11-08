use crate::addon::Addon;
use crate::context::ScheduledRefresh;
use crate::kp::refresh;
use chrono::Local;
use function_name::named;
use nexus::imgui::Ui;

impl Addon {
    pub fn render_main(&mut self, _ui: &Ui) {
        Self::schedule_on_map_exit(self);
        Self::refresh_on_schedule(self);
    }

    #[named]
    fn refresh_on_schedule(&mut self) {
        if let Some(ScheduledRefresh::OnTime(time)) = self.context.scheduled_refresh {
            if time < Local::now() {
                log::info!("[{}] scheduled refresh executed", function_name!());
                self.context.scheduled_refresh = None;
                refresh(self);
            }
        }

        if !self.context.on_kp_map
            && self
                .context
                .scheduled_refresh
                .as_ref()
                .is_some_and(|sr| matches!(sr, ScheduledRefresh::OnKPMapExit))
        {
            log::info!("[{}] map exit refresh executed", function_name!());
            self.context.scheduled_refresh = None;
            refresh(self);
        }
    }

    #[named]
    fn schedule_on_map_exit(&mut self) {
        match self.context.mumble {
            Some(m) => {
                let previous_map_on_kp = self.context.on_kp_map;
                self.context.on_kp_map = self.config.kp_map_ids.contains(&m.read_map_id());
                if !previous_map_on_kp && self.context.on_kp_map {
                    log::info!("[{}] refresh on kp map exit scheduled", function_name!());
                    self.context.scheduled_refresh = Some(ScheduledRefresh::OnKPMapExit);
                }
            }
            None => self.context.on_kp_map = false,
        }
    }
}
