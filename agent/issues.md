# Issues

## 2026-05-30: Svelte migration build diagnostics

- Symptom: `npm run check` failed after the Svelte migration because `svelte-check` was declared in `package.json` but missing from `package-lock.json` and `node_modules`.
- Confirmed cause: the local dependency install state was out of sync with `package.json`.
- Follow-up symptom after installing dependencies: TypeScript could not resolve `./App.svelte` from `src/main.ts`.
- Confirmed cause: the project lacked an explicit `*.svelte` module declaration for the current `svelte-check` setup.
- Resolution: run `npm install`, add the `*.svelte` declaration in `src/app.d.ts`, and use Svelte 5 `mount` in `src/main.ts`.

## 2026-05-30: Theme thumbnails not rendering

- Symptom: theme card thumbnails did not render after the Svelte migration.
- Confirmed cause: the frontend used `convertFileSrc(theme.dir + '/' + theme.preview)`, but Tauri's asset protocol was not enabled or scoped in `tauri.conf.json`.
- Resolution: avoid opening broader WebView file access for thumbnails. The Rust theme list now returns `previewDataUri`, and the Svelte card uses that directly as the `<img>` source.

## 2026-05-30: Theme application fails after switching to local debug-port-only mode

- Symptom: selecting a theme appeared to do nothing. The UI could show `Applied` even when `activeIdentifier` was `null`, and clicking the already selected theme did not call the backend again.
- Confirmed causes:
  - The local config can have `enabled: true` and `selectedThemeId` set while no CDP script is currently injected.
  - `reqwest::get` inherited the user's HTTP proxy and attempted to send `127.0.0.1` CDP HTTP probes through `http://127.0.0.1:7897/`, so local DevTools requests could parse proxy responses instead of CDP JSON.
  - Antigravity's current CDP target can expose a localhost URL with a port title such as `127.0.0.1:64653`; matching only a title containing `Antigravity` misses that valid page.
- Resolution: local CDP HTTP requests now use a `reqwest` client with `no_proxy()`, Antigravity target matching accepts localhost page URLs, the monitor re-injects when the port exists but no active identifier is present, and the frontend only shows `Applied` when an active identifier exists.

## 2026-05-30: Codex restart times out waiting for debug port

- Symptom: clicking `Restart App` for Codex made the Dock icon bounce for a long time, then the companion showed `Timed out waiting for Codex to expose a local debug port`. After force-quitting and reopening Codex manually, Codex launched without a debug port.
- Confirmed cause: Codex was running after the restart attempt, but its process arguments did not include `--remote-debugging-port=0`, and `~/Library/Application Support/Codex/DevToolsActivePort` remained an old stale file. The `open ... --args` launch path can start the app without delivering the remote-debugging arguments in this scenario.
- Resolution: launch the selected supported agent by executing its app bundle binary directly and passing the debug-port arguments to that executable, instead of relying on LaunchServices argument forwarding.
