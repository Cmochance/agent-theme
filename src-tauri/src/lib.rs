pub mod config;
pub mod agent;
pub mod cdp;
pub mod theme;

use tauri::{AppHandle, State, Manager};
use std::sync::Mutex;
use crate::config::{AppConfig, update_config, load_config};
use crate::theme::{get_themes, get_theme, save_custom_theme, delete_custom_theme, generate_injection_script, Theme};

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
async fn get_agent_status(state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    let is_running = agent::is_agent_process_running();
    let port = *state.cdp_port.lock().unwrap();
    Ok(serde_json::json!({
        "running": is_running,
        "cdpPort": port
    }))
}

#[tauri::command]
async fn restart_agent(state: State<'_, AppState>) -> Result<(), String> {
    let port = agent::launch_agent(true).await?;
    *state.cdp_port.lock().unwrap() = Some(port);
    Ok(())
}

#[tauri::command]
async fn apply_theme(app: AppHandle, state: State<'_, AppState>, theme_id: String) -> Result<(), String> {
    let theme = get_theme(&app, &theme_id).ok_or("Theme not found")?;
    
    // Read port once, release lock before any async work
    let need_launch = {
        let p = state.cdp_port.lock().unwrap();
        p.is_none() || !agent::is_agent_process_running()
    };
    
    if need_launch {
        let config = load_config();
        if config.auto_launch_agent {
            let new_port = agent::launch_agent(false).await?;
            *state.cdp_port.lock().unwrap() = Some(new_port);
        }
    }
    
    let port = *state.cdp_port.lock().unwrap();

    if let Some(p) = port {
        let script = generate_injection_script(&theme)?;
        
        let identifier = cdp::inject_theme(p, &script).await?;
        
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
    let port = *state.cdp_port.lock().unwrap();
    
    if let Some(p) = port {
        if agent::is_agent_process_running() {
            let ident = state.active_identifier.lock().unwrap().clone().or(config.active_identifier);
            cdp::clear_theme(p, ident.as_deref()).await?;
            
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
async fn upload_custom_theme(_app: AppHandle, bg_base64: String, preview_base64: String) -> Result<(), String> {
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
            let _ = app.get_webview_window("main").expect("no main window").set_focus();
        }))
        .manage(AppState {
            cdp_port: Mutex::new(None),
            active_identifier: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            get_config,
            set_enabled,
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
            let app_handle = app.handle().clone();
            
            tauri::async_runtime::spawn(async move {
                if config.auto_launch_agent {
                    log::info!("Auto-launch enabled, checking Agent...");
                    if let Ok(port) = agent::launch_agent(false).await {
                        let state: State<'_, AppState> = app_handle.state();
                        *state.cdp_port.lock().unwrap() = Some(port);
                        
                        if config.enabled {
                            log::info!("Auto-applying theme {}", config.selected_theme_id);
                            if let Some(theme) = get_theme(&app_handle, &config.selected_theme_id) {
                                if let Ok(script) = generate_injection_script(&theme) {
                                    if let Ok(ident) = cdp::inject_theme(port, &script).await {
                                        update_config(|c| { c.active_identifier = Some(ident.clone()); });
                                        *state.active_identifier.lock().unwrap() = Some(ident);
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
