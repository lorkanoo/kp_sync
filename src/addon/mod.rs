use crate::api::gw2::fetch_map_names_thread;
use crate::config::{config_dir, Config};
use crate::context::Context;
use crate::thread::background_thread;
use function_name::named;
use log::info;
use nexus::gui::{register_render, RenderType};
use semver::Version;
use std::fs;
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::thread::JoinHandle;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
static MULTITHREADED_ADDON: MultithreadedAddon = MultithreadedAddon {
    addon: OnceLock::new(),
    threads: OnceLock::new(),
};

pub struct MultithreadedAddon {
    pub addon: OnceLock<Mutex<Addon>>,
    pub threads: OnceLock<Mutex<Vec<JoinHandle<()>>>>,
}

#[derive(Debug)]
pub struct Addon {
    pub config: Config,
    pub context: Context,
}

impl Addon {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
            context: Context::default(),
        }
    }
    pub fn lock() -> MutexGuard<'static, Addon> {
        MULTITHREADED_ADDON
            .addon
            .get_or_init(|| Mutex::new(Addon::new()))
            .lock()
            .unwrap()
    }

    pub fn threads() -> MutexGuard<'static, Vec<JoinHandle<()>>> {
        MULTITHREADED_ADDON
            .threads
            .get_or_init(|| Mutex::new(Vec::new()))
            .lock()
            .unwrap()
    }

    #[named]
    pub fn load() {
        info!("[{}] Loading kp_sync v{}", function_name!(), VERSION);
        let _ = fs::create_dir(config_dir());
        {
            if let Some(config) = Config::try_load() {
                Addon::lock().config = config;
            }
        }

        migrate_configs(&mut Addon::lock());
        init_context(&mut Addon::lock());
        fetch_map_names_thread();
        background_thread();

        register_render(
            RenderType::OptionsRender,
            nexus::gui::render!(|ui| Addon::lock().render_options(ui)),
        )
        .revert_on_unload();

        info!("[{}] kp_sync loaded", function_name!());
    }
    #[named]
    pub fn unload() {
        info!("[{}] Unloading kp_sync v{VERSION}", function_name!());
        Self::lock().context.run_background_thread = false;
        let mut threads = Self::threads();
        while let Some(thread) = threads.pop() {
            info!("[{}] Waiting for a thread to end..", function_name!());
            match thread.join() {
                Ok(_) => info!("[{}] Thread unloaded successfully", function_name!()),
                Err(_) => log::error!("[{}] Thread unloaded with error", function_name!()),
            }
        }
        let addon = &mut Self::lock();
        if addon.context.scheduled_refresh.is_some() {
            info!("[{}] refresh_on_next_load scheduled", function_name!());
            addon.config.refresh_on_next_load = true;
        }
        info!("[{}] Saving configuration..", function_name!());
        addon.config.save();
        info!("[{}] kp_sync unloaded", function_name!());
    }
}

fn migrate_configs(addon: &mut MutexGuard<Addon>) {
    if version_older_than(addon.config.version.as_str(), "0.9.6") {
        addon.config.retain_refresh_map_ids.push(1154);
    }
    addon.config.version = VERSION.to_string();
}

fn version_older_than(older: &str, than: &str) -> bool {
    Version::parse(older).unwrap() < Version::parse(than).unwrap()
}

fn init_context(addon: &mut MutexGuard<Addon>) {
    addon.context.ui.previous_main_id = addon.config.kp_identifiers.main_id.clone();
}
