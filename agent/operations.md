# Operations

## Local macOS test bundle

Verified command sequence:

```bash
npm run check
npm run build
cargo tauri build --bundles app
codesign --force --deep --sign - /Users/alysechen/alysechen/github/agent-theme/src-tauri/target/release/bundle/macos/agent_theme_companion.app
codesign --verify --deep --strict --verbose=4 /Users/alysechen/alysechen/github/agent-theme/src-tauri/target/release/bundle/macos/agent_theme_companion.app
```

The generated test app is:

```text
/Users/alysechen/alysechen/github/agent-theme/src-tauri/target/release/bundle/macos/agent_theme_companion.app
```

The extra `codesign --force --deep --sign -` step is needed for local testing because the Tauri-generated app bundle can otherwise have an incomplete ad-hoc bundle signature for resources.

On 2026-05-30, the generated ad-hoc signed app still made `spctl --assess --type execute` return `internal error in Code Signing subsystem`, while `codesign --verify --deep --strict` passed and `open -n` successfully launched the app. Treat `open -n` plus process verification as the practical local launch check for this unsigned test bundle.
