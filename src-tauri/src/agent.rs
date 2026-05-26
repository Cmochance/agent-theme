use std::fs;
use std::path::PathBuf;
use std::process::Command;
use sysinfo::System;
use std::time::Duration;
use tokio::time::sleep;

pub fn get_agent_data_dir() -> PathBuf {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("~"));
    home.join("Library")
        .join("Application Support")
        .join("Codex")
}

pub fn get_devtools_port_file() -> PathBuf {
    get_agent_data_dir().join("DevToolsActivePort")
}

pub fn clean_locks() {
    log::info!("Cleaning Agent locks...");
    let dir = get_agent_data_dir();
    let files = ["SingletonLock", "SingletonCookie", "SingletonSocket", "DevToolsActivePort"];
    for f in files {
        let _ = fs::remove_file(dir.join(f));
    }
}

pub fn is_agent_process_running() -> bool {
    let mut sys = System::new_all();
    sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);
    for (_pid, process) in sys.processes() {
        let name_match = process.name().to_string_lossy().contains("Codex");
        let exe_match = process.exe()
            .map(|e| e.to_string_lossy().contains("/Applications/Codex.app/"))
            .unwrap_or(false);
        if name_match || exe_match {
            return true;
        }
    }
    false
}

pub fn force_kill_agent() {
    log::info!("Force killing existing Agent processes...");
    let _ = Command::new("pkill").arg("-9").arg("-f").arg("/Applications/Codex\\.app/").status();
    let _ = Command::new("pkill").arg("-9").arg("-f").arg("SkyComputerUseClient").status();
    let _ = Command::new("pkill").arg("-9").arg("-f").arg("Codex Computer Use\\.app").status();
    
    // Wait brief moment
    for _ in 0..10 {
        if !is_agent_process_running() {
            break;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}

pub fn read_port_from_file() -> Option<u16> {
    let file = get_devtools_port_file();
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

pub async fn launch_agent(force_clean: bool) -> Result<u16, String> {
    if force_clean {
        force_kill_agent();
        clean_locks();
    } else if is_agent_process_running() {
        if let Some(active_port) = read_port_from_file() {
            // Test if responsive
            let url = format!("http://127.0.0.1:{}/json/list", active_port);
            if reqwest::get(&url).await.is_ok() {
                log::info!("Agent is already running and listening on port {}", active_port);
                return Ok(active_port);
            }
        }
        log::info!("Agent process found but debug port is unresponsive. Restarting...");
        force_kill_agent();
        clean_locks();
    } else {
        clean_locks();
    }
    
    log::info!("Starting Agent App...");
    let mut child = Command::new("/Applications/Codex.app/Contents/MacOS/Codex")
        .arg("--remote-debugging-port=0")
        .arg("--remote-allow-origins=*")
        .spawn()
        .map_err(|e| format!("Failed to start Agent: {}", e))?;
    
    // Polling for DevTools port
    for _ in 1..=30 {
        if let Some(port) = read_port_from_file() {
            let url = format!("http://127.0.0.1:{}/json/list", port);
            if reqwest::get(&url).await.is_ok() {
                log::info!("Bound to port {}", port);
                return Ok(port);
            }
        }
        sleep(Duration::from_millis(500)).await;
    }
    
    let _ = child.kill();
    Err("Timeout waiting for Agent DevToolsActivePort to become available".to_string())
}
