use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use lazy_static::lazy_static;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    pub enabled: bool,
    pub selected_theme_id: String,
    pub auto_launch_agent: bool,
    pub active_identifier: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            selected_theme_id: "carton".to_string(),
            auto_launch_agent: true,
            active_identifier: None,
        }
    }
}

pub fn get_config_dir() -> PathBuf {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("~"));
    home.join(".agent-theme")
}

pub fn get_config_file() -> PathBuf {
    get_config_dir().join("config.json")
}

pub fn ensure_config_dir() {
    let dir = get_config_dir();
    if !dir.exists() {
        let _ = fs::create_dir_all(&dir);
    }
    let custom_dir = dir.join("themes").join("custom");
    if !custom_dir.exists() {
        let _ = fs::create_dir_all(&custom_dir);
    }
}

pub fn load_config() -> AppConfig {
    ensure_config_dir();
    let file = get_config_file();
    if !file.exists() {
        return AppConfig::default();
    }

    match fs::read_to_string(&file) {
        Ok(raw) => match serde_json::from_str(&raw) {
            Ok(parsed) => parsed,
            Err(e) => {
                log::error!("Error parsing config: {}", e);
                AppConfig::default()
            }
        },
        Err(e) => {
            log::error!("Error reading config file: {}", e);
            AppConfig::default()
        }
    }
}

pub fn save_config(config: &AppConfig) {
    ensure_config_dir();
    let file = get_config_file();
    let temp = file.with_extension("tmp");

    if let Ok(data) = serde_json::to_string_pretty(config) {
        if fs::write(&temp, data).is_ok() {
            let _ = fs::rename(temp, file);
        }
    }
}

lazy_static! {
    static ref CONFIG_MUTEX: Mutex<()> = Mutex::new(());
}

pub fn update_config<F>(updater: F) -> AppConfig
where
    F: FnOnce(&mut AppConfig),
{
    let _lock = CONFIG_MUTEX.lock().unwrap();
    let mut config = load_config();
    updater(&mut config);
    save_config(&config);
    config
}
