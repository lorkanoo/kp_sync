use crate::api::gw2::fetch_map_names_thread;
use crate::config::{config_dir, migrate_configs, Config};
use crate::context::{init_context, Context};
use crate::thread::background_thread;
use function_name::named;
use log::info;
use nexus::event::event_raise_notification;
use nexus::event_subscribe;
use nexus::gui::{register_render, RenderType};
use nexus::quick_access::add_quick_access_context_menu;
use std::ffi::CStr;
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

#[derive(Debug, Default)]
pub struct Addon {
    pub config: Config,
    pub context: Context,
}

impl Addon {
    pub fn lock() -> MutexGuard<'static, Addon> {
        MULTITHREADED_ADDON
            .addon
            .get_or_init(|| Mutex::new(Addon::default()))
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
        unsafe {
            event_subscribe!("EV_ACCOUNT_NAME" => std::ffi::c_char, |name| {
                if let Some(name) = name {
                    let ptr = std::ptr::addr_of!(*name).cast_mut();
                    let account_name_c = unsafe { CStr::from_ptr(ptr) };
                    let account_name = account_name_c.to_string_lossy().to_string().replace(":", "");
                    info!("[EV_ACCOUNT_NAME] Received account name: {:?}", account_name);
                    Addon::lock().context.arcdps_account_name = account_name;
                }
            })
        }.revert_on_unload();

        event_raise_notification("EV_REQUEST_ACCOUNT_NAME");
        fetch_map_names_thread();
        background_thread();

        register_render(
            RenderType::OptionsRender,
            nexus::gui::render!(|ui| Addon::lock().render_options(ui)),
        )
        .revert_on_unload();

        add_quick_access_context_menu(
            "kp_sync",
            None::<&str>,
            nexus::gui::render!(|ui| Addon::lock().render_quick_access(ui)),
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
