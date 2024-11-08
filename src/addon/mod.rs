use crate::config::{config_dir, Config};
use crate::context::Context;
use crate::kp::refresh;
use function_name::named;
use nexus::gui::{register_render, RenderType};
use std::fs;
use std::sync::{Mutex, MutexGuard, OnceLock};

const VERSION: &str = env!("CARGO_PKG_VERSION");
static ADDON: OnceLock<Mutex<Addon>> = OnceLock::new();

//todo: redirect issue on invalid kp id
//todo: add alter text for inputs

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
        ADDON
            .get_or_init(|| Mutex::new(Addon::new()))
            .lock()
            .unwrap()
    }

    #[named]
    pub fn load() {
        log::info!("[{}] Loading kp_sync v{}", function_name!(), VERSION);
        let _ = fs::create_dir(config_dir());
        {
            if let Some(config) = Config::try_load() {
                Addon::lock().config = config;
            }
        }

        refresh_on_load(&mut Addon::lock());

        register_render(
            RenderType::Render,
            nexus::gui::render!(|ui| Addon::lock().render_main(ui)),
        )
        .revert_on_unload();

        register_render(
            RenderType::OptionsRender,
            nexus::gui::render!(|ui| Addon::lock().render_options(ui)),
        )
        .revert_on_unload();

        log::info!("[{}] kp_sync loaded", function_name!());
    }

    #[named]
    pub fn unload() {
        log::info!("[{}] Unloading kp_sync v{VERSION}", function_name!());
        let addon = &mut Self::lock();
        if addon.context.scheduled_refresh.is_some() {
            log::info!("[{}] refresh_on_next_load scheduled", function_name!());
            addon.config.refresh_on_next_load = true;
        }
        addon.config.save();
        log::info!("[{}] kp_sync unloaded", function_name!());
    }
}

fn refresh_on_load(addon: &mut MutexGuard<Addon>) {
    if addon.config.refresh_on_next_load {
        addon.config.refresh_on_next_load = false;
        refresh(addon);
    }
}
