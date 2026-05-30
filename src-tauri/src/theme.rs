use crate::config::{get_config_dir, AgentKind};
use base64::{engine::general_purpose::STANDARD as base64_engine, Engine as _};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::AppHandle;
use tauri::Manager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayName {
    pub en: String,
    pub zh: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Theme {
    pub id: String,
    pub display_name: DisplayName,
    #[serde(default)]
    pub is_custom: bool,
    pub background: String,
    pub preview: String,
    #[serde(default)]
    pub dir: PathBuf,
}

pub fn get_internal_themes_dir(app: &AppHandle) -> PathBuf {
    // 1. Tauri resource_dir (bundled apps + some dev setups)
    if let Ok(res_dir) = app.path().resource_dir() {
        let themes = res_dir.join("themes");
        if themes.exists() {
            return themes;
        }
    }
    // 2. macOS bundle: exe at Contents/MacOS/<bin>, themes at Contents/Resources/themes
    if let Ok(exe) = std::env::current_exe() {
        if let Some(contents) = exe.parent().and_then(|p| p.parent()) {
            let themes = contents.join("Resources").join("themes");
            if themes.exists() {
                return themes;
            }
        }
    }
    // 3. Dev fallback: CWD is src-tauri, themes is ../themes
    PathBuf::from("../themes")
}

pub fn get_custom_theme_dir() -> PathBuf {
    get_config_dir().join("themes").join("custom")
}

pub fn get_themes(app: &AppHandle) -> Vec<Theme> {
    let mut themes = vec![];
    let internal_dir = get_internal_themes_dir(app);

    if internal_dir.exists() {
        if let Ok(entries) = fs::read_dir(&internal_dir) {
            for entry in entries.flatten() {
                if let Ok(ft) = entry.file_type() {
                    if ft.is_dir() {
                        let theme_json_path = entry.path().join("theme.json");
                        if theme_json_path.exists() {
                            if let Ok(raw) = fs::read_to_string(&theme_json_path) {
                                if let Ok(mut meta) = serde_json::from_str::<Theme>(&raw) {
                                    meta.is_custom = false;
                                    meta.dir = entry.path();
                                    themes.push(meta);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let custom_dir = get_custom_theme_dir();
    let bg_path = custom_dir.join("bg.jpg");
    if bg_path.exists() {
        themes.push(Theme {
            id: "custom".to_string(),
            display_name: DisplayName {
                zh: "自定义背景 (Custom)".to_string(),
                en: "Custom Background".to_string(),
            },
            is_custom: true,
            background: "bg.jpg".to_string(),
            preview: "preview.jpg".to_string(),
            dir: custom_dir,
        });
    }

    themes
}

pub fn get_theme(app: &AppHandle, id: &str) -> Option<Theme> {
    get_themes(app).into_iter().find(|t| t.id == id)
}

const MAX_THEME_IMAGE_BYTES: usize = 5 * 1024 * 1024; // 5MB

pub fn save_custom_theme(bg_base64: &str, preview_base64: &str) -> Result<(), String> {
    let custom_dir = get_custom_theme_dir();
    let _ = fs::create_dir_all(&custom_dir);

    let bg_data = parse_base64_data_uri(bg_base64)?;
    let preview_data = parse_base64_data_uri(preview_base64)?;

    if bg_data.len() > MAX_THEME_IMAGE_BYTES {
        return Err(format!(
            "Background image too large ({}MB max)",
            MAX_THEME_IMAGE_BYTES / 1024 / 1024
        ));
    }
    if preview_data.len() > MAX_THEME_IMAGE_BYTES {
        return Err(format!(
            "Preview image too large ({}MB max)",
            MAX_THEME_IMAGE_BYTES / 1024 / 1024
        ));
    }

    fs::write(custom_dir.join("bg.jpg"), bg_data).map_err(|e| e.to_string())?;
    fs::write(custom_dir.join("preview.jpg"), preview_data).map_err(|e| e.to_string())?;

    Ok(())
}

pub fn delete_custom_theme() {
    let custom_dir = get_custom_theme_dir();
    let _ = fs::remove_file(custom_dir.join("bg.jpg"));
    let _ = fs::remove_file(custom_dir.join("preview.jpg"));
}

fn parse_base64_data_uri(uri: &str) -> Result<Vec<u8>, String> {
    let parts: Vec<&str> = uri.splitn(2, ',').collect();
    if parts.len() != 2 {
        return Err("Invalid base64 data URI".to_string());
    }
    base64_engine.decode(parts[1]).map_err(|e| e.to_string())
}

/// Dispatch to the correct injection script based on agent kind.
pub fn generate_injection_script(theme: &Theme, kind: &AgentKind) -> Result<String, String> {
    match kind {
        AgentKind::Codex => generate_codex_injection_script(theme),
        AgentKind::Antigravity => generate_antigravity_injection_script(theme),
    }
}

/// Read theme background image and encode as a base64 data URI.
fn encode_background(theme: &Theme) -> Result<String, String> {
    let bg_path = theme.dir.join(&theme.background);
    let bg_bytes = fs::read(&bg_path)
        .map_err(|e| format!("Failed to read background {:?}: {}", bg_path, e))?;
    let bg_ext = if theme.background.ends_with(".png") {
        "png"
    } else {
        "jpeg"
    };
    Ok(format!(
        "data:image/{};base64,{}",
        bg_ext,
        base64_engine.encode(&bg_bytes)
    ))
}

/// Wrap agent-specific CSS into the JS injection boilerplate.
fn wrap_injection_css(css: &str, agent_label: &str) -> String {
    format!(
        r#"
        (function() {{
            const existingStyle = document.getElementById('agent-theme-style');
            if (existingStyle) existingStyle.remove();

            const style = document.createElement('style');
            style.id = 'agent-theme-style';
            style.textContent = `{css}`;
            document.head.appendChild(style);

            console.log('{agent_label} theme applied successfully.');
        }})();
    "#,
        css = css,
        agent_label = agent_label,
    )
}

/// Common CSS snippet for body background image.
fn body_bg_css(bg_data_uri: &str) -> String {
    format!(
        r#"body {{
                    background-image: url('{bg}') !important;
                    background-size: cover !important;
                    background-position: center !important;
                    background-repeat: no-repeat !important;
                    background-attachment: fixed !important;
                }}"#,
        bg = bg_data_uri,
    )
}

fn generate_codex_injection_script(theme: &Theme) -> Result<String, String> {
    let bg = encode_background(theme)?;
    let css = format!(
        r#"{body_bg}
                
                #sky-root > div,
                #sky-root > div > div {{
                    background-color: transparent !important;
                }}
                
                #sky-root .ds-sidebar,
                #sky-root .ds-panel,
                #sky-root [role="dialog"],
                #sky-root .sky-surface {{
                    background-color: rgba(26, 26, 26, 0.75) !important;
                    backdrop-filter: blur(12px) !important;
                    -webkit-backdrop-filter: blur(12px) !important;
                    border: 1px solid rgba(255, 255, 255, 0.1) !important;
                }}

                .theme-dark {{
                    --sk-surface: rgba(26, 26, 26, 0.75) !important;
                    --sk-surface-elevated: rgba(36, 36, 36, 0.85) !important;
                    --sk-background: transparent !important;
                }}
                
                #sky-root::before {{
                    content: '';
                    position: fixed;
                    top: 0; left: 0;
                    width: 100vw; height: 100vh;
                    background-color: rgba(0, 0, 0, 0.4) !important;
                    pointer-events: none;
                    z-index: 1;
                }}
                
                #sky-root > * {{
                    position: relative;
                    z-index: 2;
                }}
                
                #sky-root .ds-sidebar::after,
                #sky-root .ds-sidebar + div::before,
                .ds-sidebar-layout-resizer,
                .ds-sidebar-layout-resizer + div {{
                    display: none !important;
                }}"#,
        body_bg = body_bg_css(&bg),
    );
    Ok(wrap_injection_css(&css, "Codex"))
}

fn generate_antigravity_injection_script(theme: &Theme) -> Result<String, String> {
    let bg = encode_background(theme)?;
    let css = format!(
        r#"{body_bg}
                
                #root,
                #root > div,
                #root > div > div,
                #root > div > div > div {{
                    background-color: transparent !important;
                }}

                [role="navigation"][aria-label="Sidebar"],
                .bg-sidebar {{
                    background-color: rgba(26, 26, 26, 0.75) !important;
                    backdrop-filter: blur(12px) !important;
                    -webkit-backdrop-filter: blur(12px) !important;
                    border-right: 1px solid rgba(255, 255, 255, 0.1) !important;
                }}

                .bg-background {{
                    background-color: transparent !important;
                }}

                :root {{
                    --background: rgba(0, 0, 0, 0) !important;
                    --foreground: rgba(255, 255, 255, 0.95) !important;
                    --sidebar: rgba(26, 26, 26, 0.75) !important;
                }}

                [role="dialog"],
                .bg-card {{
                    background-color: rgba(26, 26, 26, 0.75) !important;
                    backdrop-filter: blur(12px) !important;
                    -webkit-backdrop-filter: blur(12px) !important;
                    border: 1px solid rgba(255, 255, 255, 0.1) !important;
                }}

                #root::before {{
                    content: '';
                    position: fixed;
                    top: 0; left: 0;
                    width: 100vw; height: 100vh;
                    background-color: rgba(0, 0, 0, 0.4) !important;
                    pointer-events: none;
                    z-index: 1;
                }}

                #root > * {{
                    position: relative;
                    z-index: 2;
                }}"#,
        body_bg = body_bg_css(&bg),
    );
    Ok(wrap_injection_css(&css, "Antigravity"))
}
