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

/// Common logic: inject theme via CDP and persist the identifier.
async fn inject_and_save(
    app: &AppHandle,
    state: &AppState,
    port: u16,
    kind: &AgentKind,
    theme_id: &str,
) -> Result<String, String> {
    let theme = get_theme(app, theme_id).ok_or("Theme not found")?;
    let script = generate_injection_script(&theme, kind)?;
    let identifier = cdp::inject_theme(port, kind, &script).await?;

    update_config(|c| {
        c.enabled = true;
        c.selected_theme_id = theme_id.to_string();
        c.active_identifier = Some(identifier.clone());
    });
    *state.active_identifier.lock().unwrap() = Some(identifier.clone());

    Ok(identifier)
}

async fn resolve_debug_port(state: &AppState, kind: &AgentKind) -> Option<u16> {
    let stored_port = *state.cdp_port.lock().unwrap();
    if let Some(port) = stored_port {
        if agent::is_debug_port_responsive(port).await {
            return Some(port);
        }
    }

    let discovered_port = agent::discover_debug_port(kind).await;
    *state.cdp_port.lock().unwrap() = discovered_port;
    discovered_port
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
async fn get_agent_status(state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    let config = load_config();
    let kind = &config.selected_agent;
    let is_running = agent::is_agent_process_running(kind);
    let port = if is_running {
        resolve_debug_port(&state, kind).await
    } else {
        *state.cdp_port.lock().unwrap() = None;
        None
    };

    if port.is_none() {
        let had_state_identifier = state.active_identifier.lock().unwrap().take().is_some();
        if had_state_identifier || config.active_identifier.is_some() {
            update_config(|c| c.active_identifier = None);
        }
    }

    Ok(serde_json::json!({
        "running": is_running,
        "cdpPort": port,
        "agent": kind.to_string().to_lowercase()
    }))
}

#[tauri::command]
async fn apply_theme(
    app: AppHandle,
    state: State<'_, AppState>,
    theme_id: String,
) -> Result<(), String> {
    let config = load_config();
    let kind = config.selected_agent;
    let port = resolve_debug_port(&state, &kind).await.ok_or_else(|| {
        format!(
            "{} is not exposing a local debug port. Start it with remote debugging enabled first.",
            kind
        )
    })?;

    inject_and_save(&app, &state, port, &kind, &theme_id).await?;

    Ok(())
}

#[tauri::command]
async fn restart_agent(app: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    let config = load_config();
    let kind = config.selected_agent;
    let theme_id = config.selected_theme_id.clone();
    let should_inject = config.enabled;

    *state.cdp_port.lock().unwrap() = None;
    *state.active_identifier.lock().unwrap() = None;
    update_config(|c| c.active_identifier = None);

    let port = agent::restart_agent_with_debug_port(&kind).await?;
    *state.cdp_port.lock().unwrap() = Some(port);

    if should_inject {
        inject_and_save(&app, &state, port, &kind, &theme_id).await?;
    }

    Ok(())
}

#[tauri::command]
async fn clear_theme(state: State<'_, AppState>) -> Result<(), String> {
    let config = load_config();
    let kind = config.selected_agent;

    if let Some(port) = resolve_debug_port(&state, &kind).await {
        let ident = state
            .active_identifier
            .lock()
            .unwrap()
            .clone()
            .or(config.active_identifier);
        cdp::clear_theme(port, &kind, ident.as_deref()).await?;
    }

    update_config(|c| {
        c.enabled = false;
        c.active_identifier = None;
    });
    *state.active_identifier.lock().unwrap() = None;

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
            get_agent_status,
            apply_theme,
            restart_agent,
            clear_theme,
            get_all_themes,
            upload_custom_theme,
            delete_custom_theme_cmd,
        ])
        .setup(|app| {
            // Background monitor: discover existing debug ports and re-inject theme.
            let monitor_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                loop {
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

                    let config = load_config();
                    let kind = config.selected_agent;
                    let state: State<'_, AppState> = monitor_handle.state();

                    let current_port = agent::discover_debug_port(&kind).await;
                    let stored_port = *state.cdp_port.lock().unwrap();

                    if let Some(new_port) = current_port {
                        let port_changed = Some(new_port) != stored_port;
                        let missing_state_identifier =
                            state.active_identifier.lock().unwrap().is_none();
                        let missing_config_identifier = config.active_identifier.is_none();
                        let should_inject = config.enabled
                            && (port_changed
                                || missing_state_identifier
                                || missing_config_identifier);

                        if port_changed {
                            log::info!(
                                "{:?} port changed from {:?} to {}",
                                kind,
                                stored_port,
                                new_port
                            );
                            *state.cdp_port.lock().unwrap() = Some(new_port);
                        }

                        if should_inject {
                            let theme_id = &config.selected_theme_id;
                            match inject_and_save(
                                &monitor_handle,
                                &state,
                                new_port,
                                &kind,
                                theme_id,
                            )
                            .await
                            {
                                Ok(_) => log::info!("Injected theme on debug port {}", new_port),
                                Err(e) => {
                                    log::error!("Failed to re-inject on port {}: {}", new_port, e)
                                }
                            }
                        }
                    } else if stored_port.is_some() || config.active_identifier.is_some() {
                        if stored_port.is_some() {
                            log::info!("{:?} debug port is no longer available", kind);
                        }
                        *state.cdp_port.lock().unwrap() = None;
                        *state.active_identifier.lock().unwrap() = None;
                        if config.active_identifier.is_some() {
                            update_config(|c| c.active_identifier = None);
                        }
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
