mod killproof_identifiers;
mod notifications;

use crate::addon::{Addon, VERSION};
use crate::config::killproof_identifiers::KillproofIdentifiers;
use crate::config::notifications::Notifications;
use chrono::{DateTime, Local};
use log::info;
use nexus::paths::get_addon_dir;
use regex::Regex;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use std::sync::MutexGuard;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_version")]
    pub version: String,
    #[serde(default = "KillproofIdentifiers::default")]
    pub kp_identifiers: KillproofIdentifiers,
    pub last_refresh_date: Option<DateTime<Local>>,
    pub refresh_on_next_load: bool,
    #[serde(default = "default_kp_map_ids")]
    pub kp_map_ids: Vec<u32>,
    #[serde(default = "default_retain_refresh_map_ids")]
    pub retain_refresh_map_ids: Vec<u32>,
    #[serde(default = "Notifications::default")]
    pub notifications: Notifications,
    #[serde(default = "yes")]
    pub scheduling_on_map_enter_enabled: bool,
    pub use_arcdps: bool,
}

const REGEX_KP_ID: &str = r"^([a-zA-Z0-9]{3,17}|[a-zA-Z0-9]+\.[0-9]{4})$";

impl Default for Config {
    fn default() -> Self {
        Self {
            version: VERSION.to_string(),
            kp_identifiers: KillproofIdentifiers::default(),
            last_refresh_date: None,
            refresh_on_next_load: false,
            kp_map_ids: default_kp_map_ids(),
            retain_refresh_map_ids: default_retain_refresh_map_ids(),
            notifications: Notifications::default(),
            scheduling_on_map_enter_enabled: yes(),
            use_arcdps: false,
        }
    }
}

impl Config {
    pub fn try_load() -> Option<Self> {
        let path = Self::file();
        let file = File::open(&path)
            .inspect_err(|err| log::warn!("Failed to read config: {err}"))
            .ok()?;
        let reader = BufReader::new(file);
        let config = serde_json::from_reader(reader)
            .inspect_err(|err| log::warn!("Failed to parse config: {err}"))
            .ok()?;
        info!("Loaded config from \"{}\"", path.display());
        Some(config)
    }

    pub fn save(&self) {
        let path = Self::file();
        match File::create(&path) {
            Ok(file) => {
                let writer = BufWriter::new(file);
                serde_json::to_writer_pretty(writer, &self).expect("failed to serialize config");
                info!("Saved config to \"{}\"", path.display())
            }
            Err(err) => log::error!("Failed to save config: {err}"),
        }
    }

    pub fn file() -> PathBuf {
        config_dir().join("config.json")
    }

    pub fn valid(&self) -> bool {
        let re = Regex::new(REGEX_KP_ID).unwrap();
        re.is_match(self.kp_identifiers.main_id.as_str())
    }
}

pub fn config_dir() -> PathBuf {
    get_addon_dir("kp_sync").expect("invalid config directory")
}

fn default_version() -> String {
    VERSION.to_string()
}

fn default_kp_map_ids() -> Vec<u32> {
    vec![
        1339, 1351, 1432, 1450, 1451, 1437, 1485, 1515, 1520, // strikes
        1062, 1149, 1156, 1188, 1264, 1303, 1323, 1564, // raids
    ]
}

fn default_retain_refresh_map_ids() -> Vec<u32> {
    vec![
        1154, 1155, 1370, 1509, 1428, // hubs
    ]
}

pub fn migrate_configs(addon: &mut MutexGuard<Addon>) {
    if version_older_than(addon.config.version.as_str(), "0.9.6")
        && !addon.config.retain_refresh_map_ids.contains(&1154)
    {
        addon.config.retain_refresh_map_ids.push(1154);
    }
    if version_older_than(addon.config.version.as_str(), "1.0.0")
        && !addon.config.kp_map_ids.contains(&1564)
    {
        addon.config.kp_map_ids.push(1564);
    }
    addon.config.version = VERSION.to_string();
}

fn version_older_than(older: &str, than: &str) -> bool {
    Version::parse(older).unwrap() < Version::parse(than).unwrap()
}

fn yes() -> bool {
    true
}
