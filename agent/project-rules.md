# Project Rules

## Agent process management scope

- The companion may restart the currently selected supported agent app, currently Codex or Antigravity, to launch it with `--remote-debugging-port=0`.
- Restart is intentionally scoped to those known app bundles under `/Applications/`; do not add arbitrary app path execution without an explicit product decision.
- Restart may terminate processes whose executable path is inside the selected app bundle, but it must not clean lock files in the agent app data directory or modify the agent app bundle contents.
