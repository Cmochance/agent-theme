use crate::config::AgentKind;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
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

fn is_agent_bundle_process_running(kind: &AgentKind) -> bool {
    let mut sys = System::new_all();
    sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);
    let bundle_path = kind.app_bundle_path();
    sys.processes().values().any(|process| {
        process
            .exe()
            .map(|e| e.to_string_lossy().contains(bundle_path))
            .unwrap_or(false)
    })
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

pub async fn is_debug_port_responsive(port: u16) -> bool {
    let Ok(client) = reqwest::Client::builder().no_proxy().build() else {
        return false;
    };

    let version_url = format!("http://127.0.0.1:{}/json/version", port);
    if client
        .get(&version_url)
        .send()
        .await
        .map(|r| r.status().is_success())
        .unwrap_or(false)
    {
        return true;
    }

    let list_url = format!("http://127.0.0.1:{}/json/list", port);
    client
        .get(&list_url)
        .send()
        .await
        .map(|r| r.status().is_success())
        .unwrap_or(false)
}

pub async fn discover_debug_port(kind: &AgentKind) -> Option<u16> {
    let port = read_port_from_file(kind)?;
    if is_debug_port_responsive(port).await {
        Some(port)
    } else {
        None
    }
}

async fn wait_until_bundle_processes_exit(kind: &AgentKind) -> bool {
    for _ in 0..20 {
        if !is_agent_bundle_process_running(kind) {
            return true;
        }
        sleep(Duration::from_millis(250)).await;
    }
    false
}

async fn stop_agent_bundle_processes(kind: &AgentKind) -> Result<(), String> {
    if !is_agent_bundle_process_running(kind) {
        return Ok(());
    }

    let pattern = kind.app_bundle_path();
    let status = Command::new("/usr/bin/pkill")
        .args(["-TERM", "-f", pattern])
        .status()
        .map_err(|e| format!("Failed to stop {}: {}", kind, e))?;

    if !status.success() {
        log::warn!("pkill -TERM returned {:?} for {}", status.code(), kind);
    }

    if wait_until_bundle_processes_exit(kind).await {
        return Ok(());
    }

    let status = Command::new("/usr/bin/pkill")
        .args(["-KILL", "-f", pattern])
        .status()
        .map_err(|e| format!("Failed to force stop {}: {}", kind, e))?;

    if !status.success() {
        return Err(format!("Failed to force stop {}", kind));
    }

    if wait_until_bundle_processes_exit(kind).await {
        Ok(())
    } else {
        Err(format!("{} did not exit after restart request", kind))
    }
}

pub async fn restart_agent_with_debug_port(kind: &AgentKind) -> Result<u16, String> {
    let app_path = PathBuf::from(kind.app_bundle_path());
    if !app_path.exists() {
        return Err(format!("{} app was not found at {:?}", kind, app_path));
    }
    let executable_path = PathBuf::from(kind.executable_path());
    if !executable_path.exists() {
        return Err(format!(
            "{} executable was not found at {:?}",
            kind, executable_path
        ));
    }

    stop_agent_bundle_processes(kind).await?;

    let child = Command::new(&executable_path)
        .args(["--remote-debugging-port=0", "--remote-allow-origins=*"])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("Failed to start {}: {}", kind, e))?;
    log::info!(
        "Started {} with local debug-port arguments, pid {}",
        kind,
        child.id()
    );

    for _ in 0..60 {
        if let Some(port) = discover_debug_port(kind).await {
            return Ok(port);
        }
        sleep(Duration::from_millis(500)).await;
    }

    Err(format!(
        "Timed out waiting for {} to expose a local debug port",
        kind
    ))
}
