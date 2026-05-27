use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AgentKind {
    #[default]
    Codex,
    Antigravity,
}

impl fmt::Display for AgentKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AgentKind::Codex => write!(f, "Codex"),
            AgentKind::Antigravity => write!(f, "Antigravity"),
        }
    }
}

impl AgentKind {
    pub fn display_name_zh(&self) -> &str {
        match self {
            AgentKind::Codex => "Codex",
            AgentKind::Antigravity => "Antigravity",
        }
    }

    pub fn display_name_en(&self) -> &str {
        match self {
            AgentKind::Codex => "Codex",
            AgentKind::Antigravity => "Antigravity",
        }
    }

    /// Data directory name under ~/Library/Application Support/
    pub fn data_dir_name(&self) -> &str {
        match self {
            AgentKind::Codex => "Codex",
            AgentKind::Antigravity => "Antigravity",
        }
    }

    /// Binary path inside the .app bundle
    pub fn binary_path(&self) -> PathBuf {
        let app_name = match self {
            AgentKind::Codex => "Codex.app",
            AgentKind::Antigravity => "Antigravity.app",
        };
        PathBuf::from("/Applications")
            .join(app_name)
            .join("Contents")
            .join("MacOS")
            .join(self.data_dir_name())
    }

    /// Process name patterns for detection
    pub fn process_name_patterns(&self) -> Vec<&'static str> {
        match self {
            AgentKind::Codex => vec!["Codex"],
            AgentKind::Antigravity => vec!["Antigravity"],
        }
    }

    /// Binary path patterns for detection
    pub fn binary_path_patterns(&self) -> Vec<&'static str> {
        match self {
            AgentKind::Codex => vec!["/Applications/Codex.app/"],
            AgentKind::Antigravity => vec!["/Applications/Antigravity.app/"],
        }
    }

    /// pkill patterns for force kill
    pub fn pkill_patterns(&self) -> Vec<&'static str> {
        match self {
            AgentKind::Codex => vec![
                "/Applications/Codex\\.app/",
                "SkyComputerUseClient",
                "Codex Computer Use\\.app",
            ],
            AgentKind::Antigravity => vec!["/Applications/Antigravity\\.app/"],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    pub enabled: bool,
    pub selected_theme_id: String,
    pub auto_launch_agent: bool,
    pub active_identifier: Option<String>,
    #[serde(default)]
    pub selected_agent: AgentKind,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            selected_theme_id: "carton".to_string(),
            auto_launch_agent: true,
            active_identifier: None,
            selected_agent: AgentKind::default(),
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
