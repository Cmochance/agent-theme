# Agent Theme

> [!NOTE]
> 🎨 **Agent Theme** is a standalone theming companion for **Codex Desktop / Antigravity**.
> It injects CSS into the agent's WebView at runtime via the Chrome DevTools Protocol to render a "character wallpaper + frosted-glass panels" look — **without touching the agent's source, fully reversible**.
> Ships **11** built-in anime themes, each colour-matched to its own background image, plus custom upload & crop. One click to reskin.

<p align="center">
  <a href="README.md">简体中文</a> |
  <a href="README.en.md">English</a> |
  <a href="https://github.com/Cmochance/agent-theme/releases">Releases</a>
</p>

<p align="center">
  <a href="https://github.com/Cmochance/agent-theme/stargazers"><img alt="GitHub stars" src="https://img.shields.io/github/stars/Cmochance/agent-theme?style=social"></a>
  <a href="LICENSE"><img alt="License" src="https://img.shields.io/github/license/Cmochance/agent-theme"></a>
  <a href="https://www.rust-lang.org/"><img alt="Rust" src="https://img.shields.io/badge/Rust-1.77%2B-orange?logo=rust"></a>
  <a href="https://v2.tauri.app/"><img alt="Tauri" src="https://img.shields.io/badge/Tauri-2.x-24C8DB?logo=tauri"></a>
  <a href="#"><img alt="Platform" src="https://img.shields.io/badge/macOS-000000?logo=apple"></a>
</p>

Agent Theme is a standalone desktop app (Tauri v2). The agent (Codex Desktop / Antigravity) is launched with `--remote-debugging-port` to expose a CDP port; this app connects over WebSocket and uses `Page.addScriptToEvaluateOnNewDocument` to inject a script before the page loads — adding a background layer, overriding the agent UI's design tokens, and applying `backdrop-filter` frosted glass to each panel. The whole theme lives only at runtime: turn off the toggle or restart the agent and it disappears, **without modifying the agent's binary or any config file**.

## Theme Showcase

Every theme is **colour-matched individually** to its own background — glass tinted from the image's dark tones, accent taken from the character's signature colour, and the legibility scrim calibrated per-wallpaper by brightness — so chat text stays readable on any image instead of hiding behind a uniform dark overlay. Below is the actual look on Antigravity (sidebar & input text blurred for privacy):

| 长离 · Changli | 霜银 · Frost |
|---|---|
| ![Changli](docs/antigravity/changli.jpg) | ![Frost](docs/antigravity/frost.jpg) |

**11** built-in themes (backgrounds are the respective character artworks — see [Disclaimer](#disclaimer)):

| ID | English | ID | English |
|---|---|---|---|
| `changli` | Changli | `nocturne` | Nocturne |
| `azurlane` | Azur Lane | `duet` | Duet |
| `sonata` | Sonata | `rose` | Rose |
| `zani` | Zani | `studio` | Studio |
| `nailin` | Nailin | `carton` | Carton |
| `frost` | Frost | | |

> The same theme applies to both **Codex Desktop** and **Antigravity** — the two agents share theme.json's colour knobs, each injecting platform-appropriate token overrides.

## Features

- 🎨 **11 built-in themes** — each derives dark glass + accent + tiered text colours from its own background; one-click switch
- 🖼️ **Custom themes** — drag-drop an image → 1:1 crop → save as a local theme
- 🔄 **Dual-agent support** — works with both **Codex Desktop** and **Antigravity**, switchable from the top bar
- 🚀 **Scoped restart** — restart the current agent with the debug-port flag attached; only the two supported agents are touched, nothing else
- 🔌 **CDP runtime injection** — injected via Chrome DevTools Protocol, no source changes, clears back to the original look
- 📊 **Live status** — shows agent run state and CDP port binding in real time
- 💾 **Persistent config** — settings saved to `~/.codex/agent-theme/config.json`
- 🔒 **Single-instance guard** — prevents conflicting companion windows

## Download & Install

Grab the `.dmg` for your chip from [Releases](https://github.com/Cmochance/agent-theme/releases) (each ships a `.sha256` checksum):

| Platform | Asset | Notes |
|---|---|---|
| macOS · Apple Silicon | `Agent-Theme-v<ver>-macOS-arm64.dmg` | M-series |
| macOS · Intel | `Agent-Theme-v<ver>-macOS-x64.dmg` | Intel x64 |

Open the `.dmg` and drag the app into Applications.

### macOS first launch (important)

Not Apple-notarized yet, so the app uses **ad-hoc signing** (which avoids the "is damaged / can't be opened" error), but Gatekeeper will warn "unidentified developer" on first launch — **right-click the app → Open** once to allow it, or go to `System Settings → Privacy & Security` and click "Open Anyway". Verify the download with `shasum -a 256 -c <file>.sha256`.

> **Runtime platform**: **macOS only** for now (`agent.rs` process detection & paths depend on `~/Library/Application Support/`). Windows / Linux support is planned.

## Quick Start

1. **Pick an agent** — Codex or Antigravity from the top switcher
2. **Get a debug port** — if the UI shows `No debug port`, click `Restart App`; the companion stops the current agent and relaunches it with `--remote-debugging-port=0`
3. **Pick a theme** — click any card in the theme grid to preview & apply instantly
4. **Toggle** — the "Theme" switch controls injection; turning it off restores the agent's original look

> Themes are injected via `Page.addScriptToEvaluateOnNewDocument` and take effect **only on page navigation / refresh**. They are lost when the agent restarts; the app re-injects automatically once a port is available — just keep the toggle on.

## Theme Management

### Create a custom theme

1. Click the "+" card in the theme grid
2. Drop an image (JPG/PNG, ≤ 20MB)
3. Adjust the background region with the crop tool
4. "Save & Apply" — the theme is written to `~/.codex/agent-theme/themes/`

### theme.json structure

```jsonc
{
  "id": "changli",
  "displayName": { "zh": "长离 (Changli)", "en": "Changli" },
  "background": "bg.jpg",
  "preview": "preview.jpg",
  "backgroundFit": "cover",
  "backgroundPosition": "50% 4%",
  "style": {
    "ink": "#f4ebdf", "ink2": "rgba(244,235,223,.74)",
    "ink3": "rgba(244,235,223,.56)", "ink4": "rgba(244,235,223,.40)",
    "accent": "#e08a55", "accentSoft": "#e6b48a", "focus": "#ffce86",
    "surface": "rgba(26,18,12,.50)",
    "glass": "rgba(30,21,14,.60)", "glassSoft": "rgba(34,24,16,.52)", "glassStrong": "rgba(22,15,10,.78)",
    "border": "rgba(255,228,201,.14)", "borderSoft": "rgba(255,228,201,.07)", "borderStrong": "rgba(255,228,201,.26)",
    "blur": "6px", "hover": "rgba(255,236,210,.10)", "selection": "rgba(255,236,210,.16)",
    "scrimTop": "rgba(18,12,8,.26)", "scrimMid": "rgba(17,11,7,.34)", "scrimBot": "rgba(11,7,5,.60)",
    "baseColor": "#160f0a"
  }
}
```

`background` / `backgroundFit` / `backgroundPosition` control the wallpaper itself; the optional `style` block is a set of agent-agnostic **colour knobs** (`--cl-*`) that the Codex and Antigravity injectors each translate into platform-specific token overrides:

- **Token overrides** — override the agent UI's semantic design tokens (not hard-coded container selectors), so they survive agent updates; the main surface is made transparent to reveal the wallpaper.
- **Per-panel frosted glass** — sidebar / input / dialogs each use a different `glass*` opacity + `blur` as a `backdrop-filter` that **samples the same wallpaper behind the panel** in real time, so each panel matches the background perfectly.
- **Tiered text colours** — `ink` / `ink2` / `ink3` / `ink4` map to body / secondary / tertiary / disabled; `accent` / `focus` unify links, focus, and the send button around the character's primary colour.
- **Legibility scrim** — `scrimTop` / `scrimMid` / `scrimBot` define a top-to-bottom darkening gradient. **Bright wallpapers need a stronger scrim** (or the chat text washes out), so each theme is calibrated per-image by brightness.

Omit `style` to fall back to neutral dark-glass defaults (works on any background).

## Architecture

```
agent-theme/
├── src-tauri/              # Rust backend (Tauri v2)
│   └── src/
│       ├── lib.rs          # Tauri commands, lifecycle, state
│       ├── agent.rs        # agent detect / launch / restart-with-debug-port
│       ├── cdp.rs          # CDP WebSocket, theme inject / clear
│       ├── config.rs       # config read/write (AppConfig)
│       └── theme.rs        # theme discovery, CSS generation, custom-theme CRUD
├── src/                    # frontend source (TypeScript + Svelte)
│   ├── App.svelte          # root component
│   └── lib/                # types / tauri-commands / stores / actions / polling / components
├── index.html              # Vite entry
├── web/                    # Vite build output (Tauri frontendDist)
└── themes/<id>/            # built-in theme assets (bg.jpg + preview.jpg + theme.json)
```

**Core tech:**

- **Backend** — Rust 1.77+ · Tauri 2.11 · `tokio-tungstenite` (CDP WebSocket) · `sysinfo` (process detection) · `reqwest` (port probing)
- **Frontend** — TypeScript + Svelte + Tailwind CSS, built with Vite, calling Tauri commands via `@tauri-apps/api`
- **Injection** — CDP `Page.addScriptToEvaluateOnNewDocument` injects a script before page load, creating a `<style>` + background layer; the `identifier` is saved so it can be cleared via `Page.removeScriptToEvaluateOnNewDocument`

## Development

```bash
git clone https://github.com/Cmochance/agent-theme.git
cd agent-theme

npm install
npm run build          # Vite build the frontend into web/

cargo install tauri-cli --version "^2"
cargo tauri dev        # desktop window, hot-rebuild on frontend changes
```

Checks:

```bash
npm run check                       # svelte-check
cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test --lib
```

CI (`.github/workflows/ci.yml`) runs Rust (fmt / clippy / check) + the Vite frontend build on PRs and pushes to main.

Debugging tips:

- The agent's CDP port is written to `~/Library/Application Support/<Agent>/DevToolsActivePort` (not a fixed port)
- `http://127.0.0.1:<port>/json/list` lists debuggable WebView targets
- Rust logs go through `tauri-plugin-log`; run `cargo tauri dev` in a terminal to see them

## FAQ

### Theme doesn't take effect after injection?

Check the CDP port status in the UI. If it shows `No debug port`, click `Restart App` to relaunch the agent in remote-debug mode; if the port is up but it still fails, the UI shows the backend error.

### Agent UI didn't update after switching theme?

Themes take effect on page **navigation / refresh**. After switching, switch the agent's tab or refresh the page.

### Theme lost after the agent restarts?

CDP-injected themes are lost when the agent restarts. The app re-injects automatically once a debug port is available — just keep the "Theme" toggle on.

### Does it modify the agent's source or config?

No. Injection is purely at the CDP runtime; no agent files are written. Clearing the injection restores the original look. Process management is scoped to the two supported agents only — it never touches their lock files or package contents.

### Antigravity's code syntax highlighting is still light?

The **rendered markdown code blocks** in chat are remapped to a dark syntax palette by the theme; but if you open Antigravity's embedded **Monaco code editor**, its syntax colours come from Antigravity's own colour theme — set Antigravity to a dark theme to match.

## Disclaimer

- This is an independent third-party theming tool, **not** an official OpenAI / Google project; it does not reuse their trademarks / logos / release identity. Codex / Antigravity are products of their respective owners.
- Injection happens entirely at the CDP runtime, connecting only to the local `127.0.0.1` debug port — it does not take over the system proxy, upload anything, or modify agent files.
- Built-in background images are artwork of the respective works / characters (e.g. *Wuthering Waves*, *Azur Lane*, etc.); copyright belongs to the original authors / publishers, and they are provided **for personal study and local appearance customisation only** — not for commercial use or redistribution. Open an issue if anything infringes and it will be removed promptly.

## License

MIT License — see [LICENSE](LICENSE).

## Activity

<p align="center">
  <a href="https://star-history.com/#Cmochance/agent-theme&Date"><img src="https://api.star-history.com/svg?repos=Cmochance/agent-theme&type=Date" alt="Star History" width="70%"></a>
</p>
