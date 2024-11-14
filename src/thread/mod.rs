use crate::addon::Addon;
use crate::context::scheduled_refresh::ScheduledRefresh;
use crate::kp::refresh_kp_thread;
use chrono::Local;
use function_name::named;
use log::{debug, info};
use std::thread;
use std::time::Duration;

pub fn background_thread() {
    Addon::threads().push(thread::spawn(|| loop {
        if !Addon::lock().context.run_background_thread {
            break;
        }

        clean_finished_threads();
        schedule_on_map_exit();
        refresh_on_schedule();

        thread::sleep(Duration::from_millis(500));
    }));
}

#[named]
fn refresh_on_schedule() {
    let mut addon = Addon::lock();
    if let Some(ScheduledRefresh::OnTime(time)) = addon.context.scheduled_refresh {
        if time < Local::now() {
            info!("[{}] scheduled refresh executed", function_name!());
            addon.context.scheduled_refresh = None;
            refresh_kp_thread();
        }
    }

    if !addon.context.on_kp_map
        && addon
            .context
            .scheduled_refresh
            .as_ref()
            .is_some_and(|sr| matches!(sr, ScheduledRefresh::OnKPMapExit))
    {
        info!("[{}] map exit refresh executed", function_name!());
        addon.context.scheduled_refresh = None;
        refresh_kp_thread();
    }
}

#[named]
fn schedule_on_map_exit() {
    let mut addon = Addon::lock();
    match addon.context.mumble {
        Some(m) => {
            let previous_map_on_kp = addon.context.on_kp_map;
            addon.context.on_kp_map = addon.config.kp_map_ids.contains(&m.read_map_id());
            if !previous_map_on_kp && addon.context.on_kp_map {
                info!("[{}] refresh on kp map exit scheduled", function_name!());
                addon.context.scheduled_refresh = Some(ScheduledRefresh::OnKPMapExit);
            }
        }
        None => addon.context.on_kp_map = false,
    }
}

#[named]
fn clean_finished_threads() {
    Addon::threads().retain(|handle| {
        if handle.is_finished() {
            debug!("[{}] removed finished thread", function_name!());
            false
        } else {
            debug!("[{}] thread in progress..", function_name!());
            true
        }
    });
}
