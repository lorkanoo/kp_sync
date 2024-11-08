use chrono::{DateTime, Local};
use nexus::paths::get_addon_dir;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub kp_id: String,
    pub last_refresh_date: Option<DateTime<Local>>,
    pub refresh_on_next_load: bool,
    #[serde(default = "default_retry_frequency")]
    pub retry_frequency: Duration,
    #[serde(default = "default_kp_map_ids")]
    pub kp_map_ids: Vec<u32>,
}

const REGEX_KP_ID: &str = r"^([a-zA-Z0-9]{4}|[a-zA-Z0-9]+\.[0-9]{4})$";

impl Config {
    pub fn default() -> Self {
        Self {
            kp_id: "".to_string(),
            last_refresh_date: None,
            refresh_on_next_load: false,
            retry_frequency: default_retry_frequency(),
            kp_map_ids: default_kp_map_ids(),
        }
    }

    pub fn try_load() -> Option<Self> {
        let path = Self::file();
        let file = File::open(&path)
            .inspect_err(|err| log::warn!("Failed to read config: {err}"))
            .ok()?;
        let reader = BufReader::new(file);
        let config = serde_json::from_reader(reader)
            .inspect_err(|err| log::warn!("Failed to parse config: {err}"))
            .ok()?;
        log::info!("Loaded config from \"{}\"", path.display());
        Some(config)
    }

    pub fn save(&self) {
        let path = Self::file();
        match File::create(&path) {
            Ok(file) => {
                let writer = BufWriter::new(file);
                serde_json::to_writer_pretty(writer, &self).expect("failed to serialize config");
                log::info!("Saved config to \"{}\"", path.display())
            }
            Err(err) => log::error!("Failed to save config: {err}"),
        }
    }

    pub fn file() -> PathBuf {
        config_dir().join("config.json")
    }

    pub fn valid(&self) -> bool {
        let re = Regex::new(REGEX_KP_ID).unwrap();
        re.is_match(self.kp_id.as_str())
    }
}

pub fn config_dir() -> PathBuf {
    get_addon_dir("kp_sync").expect("invalid config directory")
}

fn default_retry_frequency() -> Duration {
    Duration::new(5 * 60, 0)
}

fn default_kp_map_ids() -> Vec<u32> {
    vec![
        1339, 1351, 1432, 1450, 1451, 1437, 1485, 1515, 1520, // strikes
        1062, 1149, 1156, 1188, 1264, 1303, 1323, // raids
    ]
}
