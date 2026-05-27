use crate::config::AgentKind;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;
use sysinfo::System;
use tokio::time::sleep;

pub fn get_agent_data_dir(kind: &AgentKind) -> PathBuf {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("~"));
    home.join("Library")
        .join("Application Support")
        .join(kind.data_dir_name())
}

pub fn get_devtools_port_file(kind: &AgentKind) -> PathBuf {
    get_agent_data_dir(kind).join("DevToolsActivePort")
}

pub fn clean_locks(kind: &AgentKind) {
    log::info!("Cleaning {:?} locks...", kind);
    let dir = get_agent_data_dir(kind);
    let files = [
        "SingletonLock",
        "SingletonCookie",
        "SingletonSocket",
        "DevToolsActivePort",
    ];
    for f in files {
        let _ = fs::remove_file(dir.join(f));
    }
}

pub fn is_agent_process_running(kind: &AgentKind) -> bool {
    let mut sys = System::new_all();
    sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);
    let name_patterns = kind.process_name_patterns();
    let bin_patterns = kind.binary_path_patterns();
    for process in sys.processes().values() {
        let pname = process.name().to_string_lossy();
        let name_match = name_patterns.iter().any(|p| pname.contains(p));
        let exe_match = process
            .exe()
            .map(|e| {
                let estr = e.to_string_lossy();
                bin_patterns.iter().any(|p| estr.contains(p))
            })
            .unwrap_or(false);
        if name_match || exe_match {
            return true;
        }
    }
    false
}

pub fn force_kill_agent(kind: &AgentKind) {
    log::info!("Force killing {:?} processes...", kind);
    for pattern in kind.pkill_patterns() {
        let _ = Command::new("pkill")
            .arg("-9")
            .arg("-f")
            .arg(pattern)
            .status();
    }

    // Wait brief moment
    for _ in 0..10 {
        if !is_agent_process_running(kind) {
            break;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}

pub fn read_port_from_file(kind: &AgentKind) -> Option<u16> {
    let file = get_devtools_port_file(kind);
    if !file.exists() {
        return None;
    }
    if let Ok(content) = fs::read_to_string(&file) {
        if let Some(first_line) = content.lines().next() {
            if let Ok(port) = first_line.parse::<u16>() {
                if port > 0 {
                    return Some(port);
                }
            }
        }
    }
    None
}

pub async fn launch_agent(kind: &AgentKind, force_clean: bool) -> Result<u16, String> {
    if force_clean {
        force_kill_agent(kind);
        clean_locks(kind);
    } else if is_agent_process_running(kind) {
        if let Some(active_port) = read_port_from_file(kind) {
            // Test if responsive via the browser WebSocket endpoint
            let version_url = format!("http://127.0.0.1:{}/json/version", active_port);
            if reqwest::get(&version_url).await.is_ok() {
                log::info!(
                    "{:?} is already running and listening on port {}",
                    kind,
                    active_port
                );
                return Ok(active_port);
            }
            // Also try /json/list (works for both Codex and Antigravity)
            let list_url = format!("http://127.0.0.1:{}/json/list", active_port);
            if reqwest::get(&list_url).await.is_ok() {
                log::info!(
                    "{:?} is already running and listening on port {}",
                    kind,
                    active_port
                );
                return Ok(active_port);
            }
        }
        log::info!(
            "{:?} process found but debug port is unresponsive. Restarting...",
            kind
        );
        force_kill_agent(kind);
        clean_locks(kind);
    } else {
        clean_locks(kind);
    }

    let binary = kind.binary_path();
    log::info!("Starting {:?} at {:?}...", kind, binary);
    let mut child = Command::new(&binary)
        .arg("--remote-debugging-port=0")
        .arg("--remote-allow-origins=*")
        .spawn()
        .map_err(|e| format!("Failed to start {:?}: {}", kind, e))?;

    // Polling for DevTools port
    for _ in 1..=30 {
        if let Some(port) = read_port_from_file(kind) {
            let url = format!("http://127.0.0.1:{}/json/list", port);
            if reqwest::get(&url).await.is_ok() {
                log::info!("{:?} bound to port {}", kind, port);
                return Ok(port);
            }
        }
        sleep(Duration::from_millis(500)).await;
    }

    let _ = child.kill();
    Err(format!(
        "Timeout waiting for {:?} DevToolsActivePort to become available",
        kind
    ))
}

/// Convenience: read the selected agent kind from the current config.
pub fn current_kind() -> AgentKind {
    crate::config::load_config().selected_agent
}

/// Convenience: detect if the selected agent's process is running.
pub fn is_current_running() -> bool {
    is_agent_process_running(&current_kind())
}

/// Convenience: get the DevTools port for the selected agent.
pub fn current_port() -> Option<u16> {
    read_port_from_file(&current_kind())
}
