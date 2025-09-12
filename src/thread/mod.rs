use crate::addon::Addon;
use crate::api::kp::refresh::refresh_kp_thread;
use crate::context::scheduled_refresh::ScheduledRefresh;
use chrono::Local;
use function_name::named;
use log::{debug, info};
use std::thread;
use std::time::Duration;

pub fn background_thread() {
    Addon::threads().push(thread::spawn(|| loop {
        if !Addon::lock().context.run_background_thread {
            info!("Stopping background thread");
            break;
        }

        clean_finished_threads();

        let autodetect_account_name = Addon::lock().config.autodetect_account_name;
        let account_name = Addon::lock().context.detected_account_name.clone();
        if autodetect_account_name {
            Addon::lock().config.kp_identifiers.main_id = account_name.clone();
        }
        if !autodetect_account_name || !account_name.is_empty() {
            refresh_on_load();
            schedule_on_map_enter();
            refresh_on_schedule();
        }

        thread::sleep(Duration::from_millis(500));
    }));
}

#[named]
fn refresh_on_load() {
    let mut addon = Addon::lock();
    if let Some(m) = addon.context.mumble {
        let current_map_id = &m.read_map_id();
        if addon.config.refresh_on_next_load && current_map_id != &0 {
            info!("[{}] refreshing / scheduling refresh", function_name!());
            if (addon.config.kp_map_ids.contains(current_map_id)
                || addon.config.retain_refresh_map_ids.contains(current_map_id))
                && addon.config.scheduling_on_map_enter_enabled
            {
                addon.context.scheduled_refresh = Some(ScheduledRefresh::OnNormalMapEnter);
            } else {
                refresh_kp_thread();
            }
            addon.config.refresh_on_next_load = false;
        }
    }
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
    let mut retain_refresh = false;
    if let Some(m) = addon.context.mumble {
        retain_refresh = addon
            .config
            .retain_refresh_map_ids
            .contains(&m.read_map_id());
    }

    if !addon.context.on_kp_map
        && !retain_refresh
        && addon
            .context
            .scheduled_refresh
            .as_ref()
            .is_some_and(|sr| matches!(sr, ScheduledRefresh::OnNormalMapEnter))
    {
        info!("[{}] map enter refresh executed", function_name!());
        addon.context.scheduled_refresh = None;
        refresh_kp_thread();
    }
}

#[named]
fn schedule_on_map_enter() {
    let mut addon = Addon::lock();
    if !addon.config.scheduling_on_map_enter_enabled {
        return;
    }
    match addon.context.mumble {
        Some(m) => {
            let previous_map_on_kp = addon.context.on_kp_map;
            addon.context.on_kp_map = addon.config.kp_map_ids.contains(&m.read_map_id());
            if !previous_map_on_kp && addon.context.on_kp_map {
                info!("[{}] refresh on enter scheduled", function_name!());
                addon.context.scheduled_refresh = Some(ScheduledRefresh::OnNormalMapEnter);
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
