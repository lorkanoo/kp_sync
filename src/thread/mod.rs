use crate::addon::Addon;
use crate::api::kp::refresh::refresh_kp_thread;
use crate::context::scheduled_refresh::ScheduledRefresh;
use chrono::Local;
use function_name::named;
use log::{debug, error, info};
use std::thread;
use std::time::Duration;
use nexus::alert::send_alert;
use nexus::data_link::rtapi::read_rtapi;
use nexus::rtapi::PlayerData;

const BACKGROUND_THREAD_SLEEP_DURATION_MS: u64 = 50;
const CONFIG_SAVE_INTERVAL_SEC: u64 = 5;
const REFRESH_DAEMON_INTERVAL_SEC: u64 = 1;

pub fn background_thread() {
    Addon::threads().push(thread::spawn(|| loop {
        if !Addon::lock().context.run_background_thread {
            info!("Stopping background thread");
            break;
        }

        clean_finished_threads();

        let now = Local::now();
        if now
            > Addon::lock().context.last_config_save_date + Duration::from_secs(CONFIG_SAVE_INTERVAL_SEC)
        {
            let addon = &mut Addon::lock();
            if addon.context.scheduled_refresh.is_some() {
                addon.config.refresh_on_next_load = true;
            }
            addon.config.save();
            addon.context.last_config_save_date = now;
        }

        if now
            > Addon::lock().context.last_refresh_daemon_tick_date + Duration::from_secs(REFRESH_DAEMON_INTERVAL_SEC)
        {
            unsafe { Addon::lock().context.update_rtapi() };
            let rtapi = read_rtapi();
            if let Some(rtapi) = &rtapi {
                let player_data = unsafe { PlayerData::read(rtapi) };
                let mut addon = Addon::lock();
                addon.context.detected_account_name = player_data.account_name.clone();
            }

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

            Addon::lock().context.last_refresh_daemon_tick_date = now;
        }

        thread::sleep(Duration::from_millis(BACKGROUND_THREAD_SLEEP_DURATION_MS));
    }));
}

#[named]
fn refresh_on_load() {
    let mut addon = Addon::lock();
    if let Some(m) = addon.context.mumble {
        let current_map_id = &m.read_map_id();
        if current_map_id != &0 {
            if addon.context.first_map_tick && addon.config.refresh_on_next_load {
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
            addon.context.first_map_tick = false;
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

pub fn copy_kp_id_to_clipboard() {
    Addon::threads().push(thread::spawn(|| {
        let id = Addon::lock().config.kp_identifiers.main_id.clone();
        match Addon::lock().context.clipboard.set_text(id.as_str()) {
            Ok(_) => send_alert("KP ID copied to clipboard."),
            Err(_) => error!("Error copying KP ID")
        }
    }));
}