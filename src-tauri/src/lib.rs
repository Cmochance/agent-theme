pub mod agent;
pub mod cdp;
pub mod config;
pub mod theme;

use crate::config::{load_config, update_config, AgentKind, AppConfig};
use crate::theme::{
    delete_custom_theme, generate_injection_script, get_theme, get_themes, save_custom_theme, Theme,
};
use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};

struct AppState {
    pub cdp_port: Mutex<Option<u16>>,
    pub active_identifier: Mutex<Option<String>>,
}

#[tauri::command]
async fn get_config() -> Result<AppConfig, String> {
    Ok(load_config())
}

#[tauri::command]
async fn set_enabled(enabled: bool) -> Result<AppConfig, String> {
    Ok(update_config(|c| c.enabled = enabled))
}

#[tauri::command]
async fn set_selected_agent(
    state: State<'_, AppState>,
    agent: AgentKind,
) -> Result<AppConfig, String> {
    // Clear stale port/identifier from previous agent
    *state.cdp_port.lock().unwrap() = None;
    *state.active_identifier.lock().unwrap() = None;
    Ok(update_config(|c| {
        c.selected_agent = agent;
        c.active_identifier = None;
    }))
}

#[tauri::command]
async fn set_auto_launch(enabled: bool) -> Result<AppConfig, String> {
    Ok(update_config(|c| c.auto_launch_agent = enabled))
}

#[tauri::command]
async fn get_agent_status(state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    let config = load_config();
    let kind = &config.selected_agent;
    let is_running = agent::is_agent_process_running(kind);
    let port = *state.cdp_port.lock().unwrap();
    Ok(serde_json::json!({
        "running": is_running,
        "cdpPort": port,
        "agent": kind.to_string().to_lowercase()
    }))
}

#[tauri::command]
async fn restart_agent(state: State<'_, AppState>) -> Result<(), String> {
    let config = load_config();
    let kind = config.selected_agent;
    let port = agent::launch_agent(&kind, true).await?;
    *state.cdp_port.lock().unwrap() = Some(port);
    Ok(())
}

#[tauri::command]
async fn apply_theme(
    app: AppHandle,
    state: State<'_, AppState>,
    theme_id: String,
) -> Result<(), String> {
    let config = load_config();
    let kind = config.selected_agent;
    let theme = get_theme(&app, &theme_id).ok_or("Theme not found")?;

    // Read port once, release lock before any async work
    let need_launch = {
        let p = state.cdp_port.lock().unwrap();
        p.is_none() || !agent::is_agent_process_running(&kind)
    };

    if need_launch {
        let cfg = load_config();
        if cfg.auto_launch_agent {
            let new_port = agent::launch_agent(&kind, false).await?;
            *state.cdp_port.lock().unwrap() = Some(new_port);
        }
    }

    let port = *state.cdp_port.lock().unwrap();

    if let Some(p) = port {
        let script = generate_injection_script(&theme, &kind)?;

        let identifier = cdp::inject_theme(p, &kind, &script).await?;

        update_config(|c| {
            c.enabled = true;
            c.selected_theme_id = theme_id.clone();
            c.active_identifier = Some(identifier.clone());
        });

        *state.active_identifier.lock().unwrap() = Some(identifier);
    }

    Ok(())
}

#[tauri::command]
async fn clear_theme(state: State<'_, AppState>) -> Result<(), String> {
    let config = load_config();
    let kind = config.selected_agent;
    let port = *state.cdp_port.lock().unwrap();

    if let Some(p) = port {
        if agent::is_agent_process_running(&kind) {
            let ident = state
                .active_identifier
                .lock()
                .unwrap()
                .clone()
                .or(config.active_identifier);
            cdp::clear_theme(p, &kind, ident.as_deref()).await?;

            update_config(|c| {
                c.enabled = false;
                c.active_identifier = None;
            });
            *state.active_identifier.lock().unwrap() = None;
        }
    }

    Ok(())
}

#[tauri::command]
async fn get_all_themes(app: AppHandle) -> Result<Vec<Theme>, String> {
    Ok(get_themes(&app))
}

#[tauri::command]
async fn upload_custom_theme(
    _app: AppHandle,
    bg_base64: String,
    preview_base64: String,
) -> Result<(), String> {
    save_custom_theme(&bg_base64, &preview_base64)?;
    Ok(())
}

#[tauri::command]
async fn delete_custom_theme_cmd(_app: AppHandle) -> Result<(), String> {
    delete_custom_theme();
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::default().build())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }))
        .manage(AppState {
            cdp_port: Mutex::new(None),
            active_identifier: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            get_config,
            set_enabled,
            set_selected_agent,
            set_auto_launch,
            get_agent_status,
            restart_agent,
            apply_theme,
            clear_theme,
            get_all_themes,
            upload_custom_theme,
            delete_custom_theme_cmd,
        ])
        .setup(|app| {
            // Check auto launch
            let config = load_config();
            let kind = config.selected_agent;
            let app_handle = app.handle().clone();

            tauri::async_runtime::spawn(async move {
                if config.auto_launch_agent {
                    log::info!("Auto-launch enabled, checking {:?}...", kind);
                    if let Ok(port) = agent::launch_agent(&kind, false).await {
                        let state: State<'_, AppState> = app_handle.state();
                        *state.cdp_port.lock().unwrap() = Some(port);

                        if config.enabled {
                            log::info!("Auto-applying theme {}", config.selected_theme_id);
                            if let Some(theme) = get_theme(&app_handle, &config.selected_theme_id) {
                                if let Ok(script) = generate_injection_script(&theme, &kind) {
                                    if let Ok(ident) = cdp::inject_theme(port, &kind, &script).await
                                    {
                                        update_config(|c| {
                                            c.active_identifier = Some(ident.clone());
                                        });
                                        *state.active_identifier.lock().unwrap() = Some(ident);
                                    }
                                }
                            }
                        }
                    }
                }
            });

            // Background monitor: detect agent restart and re-inject theme
            let monitor_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                loop {
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

                    let config = load_config();
                    let kind = config.selected_agent;
                    let state: State<'_, AppState> = monitor_handle.state();

                    if !agent::is_agent_process_running(&kind) {
                        let old_port = *state.cdp_port.lock().unwrap();
                        if old_port.is_some() {
                            log::info!("{:?} process stopped, clearing CDP state", kind);
                            *state.cdp_port.lock().unwrap() = None;
                            *state.active_identifier.lock().unwrap() = None;
                        }
                        continue;
                    }

                    let current_port = agent::read_port_from_file(&kind);
                    let stored_port = *state.cdp_port.lock().unwrap();

                    if let Some(new_port) = current_port {
                        if Some(new_port) != stored_port {
                            log::info!(
                                "{:?} port changed from {:?} to {}, re-injecting theme",
                                kind,
                                stored_port,
                                new_port
                            );
                            *state.cdp_port.lock().unwrap() = Some(new_port);

                            if config.enabled {
                                if let Some(theme) =
                                    get_theme(&monitor_handle, &config.selected_theme_id)
                                {
                                    if let Ok(script) = generate_injection_script(&theme, &kind) {
                                        match cdp::inject_theme(new_port, &kind, &script).await {
                                            Ok(ident) => {
                                                log::info!("Re-injected theme after agent restart");
                                                update_config(|c| {
                                                    c.active_identifier = Some(ident.clone());
                                                });
                                                *state.active_identifier.lock().unwrap() =
                                                    Some(ident);
                                            }
                                            Err(e) => {
                                                log::error!(
                                                    "Failed to re-inject theme on port {}: {}",
                                                    new_port,
                                                    e
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
