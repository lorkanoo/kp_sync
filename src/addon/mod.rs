use crate::config::{config_dir, Config};
use crate::context::Context;
use crate::kp::refresh_kp_thread;
use crate::thread::background_thread;
use function_name::named;
use log::info;
use nexus::gui::{register_render, RenderType};
use std::fs;
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::thread::JoinHandle;

const VERSION: &str = env!("CARGO_PKG_VERSION");
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

        init_context(&mut Addon::lock());
        refresh_on_load(&mut Addon::lock());
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

fn refresh_on_load(addon: &mut MutexGuard<Addon>) {
    if addon.config.refresh_on_next_load {
        addon.config.refresh_on_next_load = false;
        refresh_kp_thread();
    }
}

fn init_context(addon: &mut MutexGuard<Addon>) {
    addon.context.previous_main_id = addon.config.kp_identifiers.main_id.clone();
}
