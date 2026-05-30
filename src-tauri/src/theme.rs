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

/// Optional per-theme styling for the modular Codex injection. Every field is
/// optional; missing fields fall back to neutral dark-glass defaults so legacy
/// themes (no `style` block) still render correctly. See `ResolvedStyle`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThemeStyle {
    pub ink: Option<String>,
    pub ink2: Option<String>,
    pub ink3: Option<String>,
    pub ink4: Option<String>,
    pub accent: Option<String>,
    pub accent_soft: Option<String>,
    pub focus: Option<String>,
    pub surface: Option<String>,
    pub glass: Option<String>,
    pub glass_strong: Option<String>,
    pub glass_soft: Option<String>,
    pub border: Option<String>,
    pub border_soft: Option<String>,
    pub border_strong: Option<String>,
    pub blur: Option<String>,
    pub hover: Option<String>,
    pub selection: Option<String>,
    pub scrim_top: Option<String>,
    pub scrim_mid: Option<String>,
    pub scrim_bot: Option<String>,
    pub base_color: Option<String>,
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
    pub preview_data_uri: String,
    #[serde(default)]
    pub background_fit: Option<String>,
    #[serde(default)]
    pub background_position: Option<String>,
    #[serde(default)]
    pub style: Option<ThemeStyle>,
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
                                    meta.preview_data_uri =
                                        encode_image_data_uri(&meta.dir.join(&meta.preview))
                                            .unwrap_or_default();
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
            preview_data_uri: encode_image_data_uri(&custom_dir.join("preview.jpg"))
                .unwrap_or_default(),
            background_fit: None,
            background_position: None,
            style: None,
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
    encode_image_data_uri(&bg_path)
}

fn encode_image_data_uri(path: &PathBuf) -> Result<String, String> {
    let image_bytes =
        fs::read(path).map_err(|e| format!("Failed to read image {:?}: {}", path, e))?;
    let image_ext = if path
        .extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| ext.eq_ignore_ascii_case("png"))
    {
        "png"
    } else {
        "jpeg"
    };
    Ok(format!(
        "data:image/{};base64,{}",
        image_ext,
        base64_engine.encode(&image_bytes)
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

/// Concrete style values after applying defaults. Missing knobs fall back to a
/// neutral dark-glass look so themes without a `style` block still render well.
struct ResolvedStyle {
    ink: String,
    ink2: String,
    ink3: String,
    ink4: String,
    surface: String,
    glass: String,
    glass_strong: String,
    glass_soft: String,
    border: String,
    border_soft: String,
    border_strong: String,
    blur: String,
    hover: String,
    selection: String,
    scrim_top: String,
    scrim_mid: String,
    scrim_bot: String,
    base_color: String,
    accent: Option<String>,
    accent_soft: String,
    focus: String,
}

fn resolve_style(style: &Option<ThemeStyle>) -> ResolvedStyle {
    let s = style.clone().unwrap_or_default();
    let or = |opt: Option<String>, def: &str| opt.unwrap_or_else(|| def.to_string());
    // accent-derived fallbacks (only used when `accent` is set)
    let accent_soft = s
        .accent_soft
        .clone()
        .or_else(|| s.accent.clone())
        .unwrap_or_else(|| "#7aa2ff".to_string());
    let focus = s
        .focus
        .clone()
        .or_else(|| s.accent.clone())
        .unwrap_or_else(|| "#7aa2ff".to_string());
    ResolvedStyle {
        ink: or(s.ink, "#f1ece4"),
        ink2: or(s.ink2, "rgba(241,236,228,.74)"),
        ink3: or(s.ink3, "rgba(241,236,228,.56)"),
        ink4: or(s.ink4, "rgba(241,236,228,.40)"),
        surface: or(s.surface, "rgba(20,20,24,.50)"),
        glass: or(s.glass, "rgba(24,24,29,.60)"),
        glass_strong: or(s.glass_strong, "rgba(16,16,20,.78)"),
        glass_soft: or(s.glass_soft, "rgba(28,28,34,.52)"),
        border: or(s.border, "rgba(255,255,255,.12)"),
        border_soft: or(s.border_soft, "rgba(255,255,255,.07)"),
        border_strong: or(s.border_strong, "rgba(255,255,255,.22)"),
        blur: or(s.blur, "6px"),
        hover: or(s.hover, "rgba(255,255,255,.08)"),
        selection: or(s.selection, "rgba(255,255,255,.14)"),
        scrim_top: or(s.scrim_top, "rgba(8,8,10,.26)"),
        scrim_mid: or(s.scrim_mid, "rgba(8,8,10,.34)"),
        scrim_bot: or(s.scrim_bot, "rgba(5,5,7,.60)"),
        base_color: or(s.base_color, "#0e0e10"),
        accent: s.accent,
        accent_soft,
        focus,
    }
}

/// Modular, design-token-driven theme for current Codex (Tailwind v4 +
/// `--color-token-*` + runtime `--color-*` design system, electron window).
/// Surface tokens become translucent glass (so token consumers like the
/// collapsed-sidebar fly-out stay opaque) while the big surface ELEMENTS are
/// forced transparent so the background image shows; overlay modules are
/// per-module frosted glass; text/icon tokens are tuned per level; the resize
/// handle + panel pseudo-elements are neutralised; the top-of-content fade is
/// forced always-on. Placeholders are filled from `ResolvedStyle`. The
/// structure is adapted from the proven codex-app-transfer injector.
const CODEX_CSS_TEMPLATE: &str = r#":root{
--cl-ink:__INK__;--cl-ink-2:__INK2__;--cl-ink-3:__INK3__;--cl-ink-4:__INK4__;
--cl-surface:__SURFACE__;--cl-glass:__GLASS__;--cl-glass-soft:__GLASS_SOFT__;--cl-glass-strong:__GLASS_STRONG__;
--cl-border:__BORDER__;--cl-border-soft:__BORDER_SOFT__;--cl-border-strong:__BORDER_STRONG__;
--cl-blur:__BLUR__;--cl-hover:__HOVER__;--cl-selection:__SELECTION__;
--cl-scrim-top:__SCRIM_TOP__;--cl-scrim-mid:__SCRIM_MID__;--cl-scrim-bot:__SCRIM_BOT__;
}
html{
color-scheme:dark !important;
--color-token-main-surface-primary:var(--cl-surface) !important;
--color-token-side-bar-background:var(--cl-glass) !important;
--vscode-sideBar-background:var(--cl-glass) !important;
--color-token-editor-background:transparent !important;
--vscode-editor-background:transparent !important;
--color-token-terminal-background:rgba(0,0,0,.5) !important;
--color-token-bg-primary:var(--cl-glass) !important;
--color-token-bg-secondary:var(--cl-glass-soft) !important;
--color-token-bg-tertiary:var(--cl-glass-soft) !important;
--color-token-bg-fog:var(--cl-surface) !important;
--color-token-dropdown-background:var(--cl-glass-strong) !important;
--vscode-dropdown-background:var(--cl-glass-strong) !important;
--color-token-menu-background:var(--cl-glass-strong) !important;
--vscode-menu-background:var(--cl-glass-strong) !important;
--color-token-input-background:var(--cl-glass-soft) !important;
--vscode-input-background:var(--cl-glass-soft) !important;
--color-token-text-code-block-background:rgba(0,0,0,.40) !important;
--vscode-textCodeBlock-background:rgba(0,0,0,.40) !important;
--color-token-text-preformat-background:rgba(0,0,0,.40) !important;
--color-token-diff-surface:rgba(255,255,255,.04) !important;
--color-background-surface:var(--cl-surface) !important;
--color-background-surface-under:transparent !important;
--codex-base-surface:var(--cl-surface) !important;
--color-background-panel:var(--cl-glass-strong) !important;
--color-background-elevated-primary:var(--cl-glass-strong) !important;
--color-background-elevated-primary-opaque:var(--cl-glass-strong) !important;
--color-background-elevated-secondary:var(--cl-glass-soft) !important;
--color-background-elevated-secondary-opaque:var(--cl-glass-soft) !important;
--color-background-editor-opaque:var(--cl-glass-soft) !important;
--color-background-control:var(--cl-glass-soft) !important;
--color-background-control-opaque:var(--cl-glass-strong) !important;
--color-token-foreground:var(--cl-ink) !important;
--vscode-foreground:var(--cl-ink) !important;
--color-token-text-primary:var(--cl-ink) !important;
--color-token-text-secondary:var(--cl-ink-2) !important;
--color-token-text-tertiary:var(--cl-ink-3) !important;
--color-token-description-foreground:var(--cl-ink-3) !important;
--vscode-descriptionForeground:var(--cl-ink-3) !important;
--color-token-disabled-foreground:var(--cl-ink-4) !important;
--color-token-icon-foreground:var(--cl-ink-2) !important;
--vscode-icon-foreground:var(--cl-ink-2) !important;
--color-text-foreground:var(--cl-ink) !important;
--color-text-foreground-secondary:var(--cl-ink-2) !important;
--color-text-foreground-tertiary:var(--cl-ink-3) !important;
--color-text-button-secondary:var(--cl-ink) !important;
--color-text-button-tertiary:var(--cl-ink-3) !important;
--codex-base-ink:var(--cl-ink) !important;
--color-icon-primary:var(--cl-ink) !important;
--color-icon-secondary:var(--cl-ink-2) !important;
--color-icon-tertiary:var(--cl-ink-3) !important;
--color-token-border:var(--cl-border) !important;
--color-token-border-default:var(--cl-border) !important;
--color-token-border-light:var(--cl-border-soft) !important;
--color-token-border-heavy:var(--cl-border) !important;
--color-border:var(--cl-border) !important;
--color-border-light:var(--cl-border-soft) !important;
--color-border-heavy:var(--cl-border) !important;
--color-token-list-hover-background:var(--cl-hover) !important;
--color-token-list-active-selection-background:var(--cl-selection) !important;
--color-token-list-active-selection-foreground:var(--cl-ink) !important;
--color-token-toolbar-hover-background:var(--cl-hover) !important;
--vscode-list-hoverBackground:var(--cl-hover) !important;
--vscode-list-activeSelectionBackground:var(--cl-selection) !important;
--vscode-toolbar-hoverBackground:var(--cl-hover) !important;
--color-background-button-secondary-hover:var(--cl-hover) !important;
--color-background-button-tertiary-hover:var(--cl-hover) !important;
--color-token-scrollbar-slider-background:var(--cl-border) !important;
--color-token-scrollbar-slider-hover-background:var(--cl-border-strong) !important;
}
html.electron-light,html.electron-dark,html{
background:__BASECOLOR__ url('__HERO__') __POS__ / __FIT__ no-repeat fixed !important;
}
body{background:transparent !important;}
#root > *,.app-shell,.app-shell-main,main.main-surface,.app-shell-main-content-viewport,.app-shell-main-content-frame,[class~="electron:bg-token-main-surface-primary"]{background-color:transparent !important;}
html main.main-surface{border-radius:0 !important;}
#root{
background:linear-gradient(180deg,var(--cl-scrim-top) 0%,var(--cl-scrim-mid) 55%,var(--cl-scrim-bot) 100%) !important;
}
html .app-shell-left-panel{
background:var(--cl-glass) !important;border-right:none !important;
-webkit-backdrop-filter:blur(var(--cl-blur)) saturate(118%);backdrop-filter:blur(var(--cl-blur)) saturate(118%);
}
html aside.fixed.bottom-0.left-0{
background:var(--cl-glass-strong) !important;
-webkit-backdrop-filter:blur(calc(var(--cl-blur) + 4px)) saturate(120%);backdrop-filter:blur(calc(var(--cl-blur) + 4px)) saturate(120%);
border:1px solid var(--cl-border-soft);box-shadow:0 14px 44px rgba(0,0,0,.55);
}
html .relative.flex.flex-col[class*="input-background"]{
background:var(--cl-glass-soft) !important;border:1px solid var(--cl-border-strong) !important;
box-shadow:0 10px 28px rgba(0,0,0,.5),inset 0 1px 0 rgba(255,255,255,.08) !important;
-webkit-backdrop-filter:blur(calc(var(--cl-blur) + 4px)) saturate(120%);backdrop-filter:blur(calc(var(--cl-blur) + 4px)) saturate(120%);
}
html [role="dialog"],html [role="menu"],html [role="listbox"],html [data-radix-menu-content]{
background-color:var(--cl-glass-strong) !important;
-webkit-backdrop-filter:blur(calc(var(--cl-blur) + 6px)) saturate(120%);backdrop-filter:blur(calc(var(--cl-blur) + 6px)) saturate(120%);
box-shadow:0 8px 24px rgba(0,0,0,.40) !important;
}
html .app-shell-left-panel::before,html .app-shell-left-panel::after,html .app-shell-main::before,html .app-shell-main::after,html .main-surface::before,html .main-surface::after{
background:transparent !important;background-image:none !important;box-shadow:none !important;mask:none !important;-webkit-mask:none !important;-webkit-mask-image:none !important;mask-image:none !important;filter:none !important;
}
html [role="separator"][aria-orientation="vertical"],html .sidebar-resize-handle-line,html [data-panel-resize-handle],html [data-resize-handle],html .resize-handle{
background:transparent !important;background-image:none !important;box-shadow:none !important;border:none !important;
}
html .app-shell-main-content-top-fade{opacity:1 !important;}
html [container-name="home-main-content"]{text-shadow:0 1px 16px rgba(0,0,0,.6),0 0 2px rgba(0,0,0,.45);}
html .text-token-text-tertiary,html [class*="placeholder"]{text-shadow:0 1px 8px rgba(0,0,0,.6),0 0 2px rgba(0,0,0,.5);}
html [container-name="home-main-content"] [role="list"] [role="listitem"]{
background-color:rgba(0,0,0,.22);
-webkit-backdrop-filter:blur(var(--cl-blur)) saturate(115%);backdrop-filter:blur(var(--cl-blur)) saturate(115%);
text-shadow:0 1px 6px rgba(0,0,0,.55),0 0 1px rgba(0,0,0,.45);
}
html .vscode-markdown code,html .vscode-markdown pre,html .monaco-editor{text-shadow:none !important;}
__ACCENT_BLOCK__"#;

/// Accent-cohesion rules, emitted only when the theme declares `style.accent`.
/// Defines the accent knobs in :root then threads them through links / focus /
/// the send button / selection. Omitted entirely for accent-less themes (which
/// then keep Codex's native blue link colour).
const CODEX_ACCENT_BLOCK: &str = r#":root{--cl-accent:__ACCENT__;--cl-accent-soft:__ACCENT_SOFT__;--cl-focus:__FOCUS__;}
html{
--codex-base-accent:var(--cl-accent) !important;
--color-accent-blue:var(--cl-accent) !important;
--color-text-accent:var(--cl-accent) !important;
--color-icon-accent:var(--cl-accent) !important;
--color-token-primary:var(--cl-accent) !important;
--color-token-link:var(--cl-accent) !important;
--color-token-text-link-foreground:var(--cl-accent) !important;
--vscode-textLink-foreground:var(--cl-accent) !important;
--color-token-focus-border:var(--cl-focus) !important;
--color-border-focus:var(--cl-focus) !important;
--color-background-accent:color-mix(in srgb,var(--cl-accent) 18%,transparent) !important;
--color-background-accent-hover:color-mix(in srgb,var(--cl-accent) 24%,transparent) !important;
--color-background-accent-active:color-mix(in srgb,var(--cl-accent) 28%,transparent) !important;
}
html button[data-testid="composer-send-button"],html .composer-send-button{color:var(--cl-accent) !important;border-color:color-mix(in srgb,var(--cl-accent) 30%,transparent) !important;}
html button[data-testid="composer-send-button"]:not(:disabled),html .composer-send-button:not(:disabled){background:color-mix(in srgb,var(--cl-accent) 30%,transparent) !important;}
html button[data-testid="composer-send-button"]:not(:disabled) svg,html .composer-send-button:not(:disabled) svg{color:var(--cl-accent) !important;opacity:1 !important;}
::selection{background:color-mix(in srgb,var(--cl-accent) 30%,transparent);}"#;

fn generate_codex_injection_script(theme: &Theme) -> Result<String, String> {
    let bg = encode_background(theme)?;
    let st = resolve_style(&theme.style);
    let pos = theme
        .background_position
        .clone()
        .unwrap_or_else(|| "center top".to_string());
    let fit = theme
        .background_fit
        .clone()
        .unwrap_or_else(|| "cover".to_string());

    let accent_block = match &st.accent {
        Some(a) => CODEX_ACCENT_BLOCK
            .replace("__ACCENT_SOFT__", &st.accent_soft)
            .replace("__FOCUS__", &st.focus)
            .replace("__ACCENT__", a),
        None => String::new(),
    };

    let css = CODEX_CSS_TEMPLATE
        .replace("__HERO__", &bg)
        .replace("__BASECOLOR__", &st.base_color)
        .replace("__POS__", &pos)
        .replace("__FIT__", &fit)
        .replace("__INK2__", &st.ink2)
        .replace("__INK3__", &st.ink3)
        .replace("__INK4__", &st.ink4)
        .replace("__INK__", &st.ink)
        .replace("__SURFACE__", &st.surface)
        .replace("__GLASS_STRONG__", &st.glass_strong)
        .replace("__GLASS_SOFT__", &st.glass_soft)
        .replace("__GLASS__", &st.glass)
        .replace("__BORDER_STRONG__", &st.border_strong)
        .replace("__BORDER_SOFT__", &st.border_soft)
        .replace("__BORDER__", &st.border)
        .replace("__BLUR__", &st.blur)
        .replace("__HOVER__", &st.hover)
        .replace("__SELECTION__", &st.selection)
        .replace("__SCRIM_TOP__", &st.scrim_top)
        .replace("__SCRIM_MID__", &st.scrim_mid)
        .replace("__SCRIM_BOT__", &st.scrim_bot)
        .replace("__ACCENT_BLOCK__", &accent_block);

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AgentKind;

    fn load_changli() -> Theme {
        let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../themes/changli");
        let raw = std::fs::read_to_string(dir.join("theme.json")).expect("read changli theme.json");
        let mut t: Theme = serde_json::from_str(&raw).expect("parse changli theme.json");
        t.dir = dir;
        t
    }

    #[test]
    fn every_bundled_theme_parses_and_generates() {
        // all themes/<id>/theme.json must deserialize into Theme and produce a
        // non-empty Codex injection script (catches agent-written JSON typos /
        // field-type mismatches that get_themes() would otherwise silently skip).
        let themes_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../themes");
        let mut count = 0;
        for entry in std::fs::read_dir(&themes_dir)
            .expect("read themes dir")
            .flatten()
        {
            let dir = entry.path();
            let json = dir.join("theme.json");
            if !json.exists() {
                continue;
            }
            let raw = std::fs::read_to_string(&json).unwrap();
            let mut t: Theme = serde_json::from_str(&raw)
                .unwrap_or_else(|e| panic!("theme.json failed to parse at {:?}: {}", json, e));
            t.dir = dir.clone();
            let script = generate_injection_script(&t, &AgentKind::Codex)
                .unwrap_or_else(|e| panic!("inject script failed for {:?}: {}", dir, e));
            assert!(
                script.contains("data:image/") && script.contains(".app-shell-left-panel"),
                "theme {:?} produced an incomplete script",
                t.id
            );
            count += 1;
        }
        assert!(count >= 5, "expected several bundled themes, found {count}");
    }

    #[test]
    fn codex_script_applies_modular_style() {
        let theme = load_changli();
        let script = generate_injection_script(&theme, &AgentKind::Codex).unwrap();
        // background image inlined as a data URI
        assert!(script.contains("data:image/jpeg;base64,"));
        // main-surface token is translucent glass; the main ELEMENT is forced transparent
        assert!(script.contains("--color-token-main-surface-primary:var(--cl-surface)"));
        assert!(script.contains("main.main-surface"));
        // the runtime --color-* layer is overridden (settings cards / text)
        assert!(script.contains("--color-background-panel:var(--cl-glass-strong)"));
        assert!(script.contains("--color-text-foreground:var(--cl-ink)"));
        // warm accent from style.accent threaded through links + send button
        assert!(script.contains("#e08a55"));
        assert!(script.contains("composer-send-button"));
        // real current-Codex module selector (not the dead #sky-root ones)
        assert!(script.contains(".app-shell-left-panel"));
        assert!(!script.contains("#sky-root"));
        // background position from theme.json
        assert!(script.contains("50% 4%"));
        // optional dump for manual visual parity check against the lab CSS
        if let Ok(path) = std::env::var("DUMP_CODEX_SCRIPT") {
            std::fs::write(path, &script).unwrap();
        }
    }

    #[test]
    fn codex_script_falls_back_to_neutral_defaults_without_style() {
        let mut theme = load_changli();
        theme.style = None;
        theme.background_position = None;
        let script = generate_injection_script(&theme, &AgentKind::Codex).unwrap();
        // neutral default ink colour is used
        assert!(script.contains("#f1ece4"));
        // no accent declared -> accent-cohesion block is omitted entirely
        assert!(!script.contains("composer-send-button"));
        // default background position
        assert!(script.contains("center top"));
    }
}
