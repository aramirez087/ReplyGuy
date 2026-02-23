# 10 — Packaging, Distribution & Polish

> **Goal:** Package the Tauri app for distribution (DMG, MSI, AppImage), add system
> tray support, auto-update, onboarding flow, and final polish. After this task,
> tuitbot is a shippable desktop product.

## Prerequisites

- Tasks 01-09 completed: full dashboard with all pages functional.

## Context

The app works in development. Now we need to make it installable, auto-updating,
and polished enough for real users. This task covers the "last mile" of shipping.

## What to build

### 1. Tauri build configuration

#### `dashboard/src-tauri/tauri.conf.json` updates:

```json
{
  "bundle": {
    "active": true,
    "targets": ["dmg", "nsis", "appimage", "deb"],
    "identifier": "com.tuitbot.app",
    "icon": ["icons/32x32.png", "icons/128x128.png", "icons/icon.icns", "icons/icon.ico"],
    "macOS": {
      "minimumSystemVersion": "10.15",
      "signingIdentity": null
    },
    "windows": {
      "certificateThumbprint": null
    }
  }
}
```

#### App icons
- Create icon set: 32x32, 128x128, 256x256, 512x512, icon.icns (macOS), icon.ico (Windows)
- Simple, recognizable icon (suggest: speech bubble with a growth arrow, or a bird
  silhouette with a chart line)

### 2. System tray

Add system tray support in `dashboard/src-tauri/src/main.rs`:

- Tray icon showing running status (active = colored, stopped = grayed)
- Right-click menu:
  - "Open Dashboard" — bring window to front
  - "Start/Stop Automation" — toggle runtime
  - Separator
  - "Approval Queue (3)" — open to approval page, shows pending count
  - Separator
  - "Quit Tuitbot"
- Left-click: toggle window visibility
- Show notification count badge (pending approvals)

### 3. Auto-start on login

- macOS: Add a toggle in Settings to register a login item via `SMAppService` / LaunchAgent
- Windows: Registry `HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Run`
- Linux: XDG autostart `.desktop` file
- Tauri plugin: `tauri-plugin-autostart`

### 4. Auto-update

Configure Tauri's built-in updater:

- Set up an update endpoint (can be GitHub Releases or a custom URL)
- `dashboard/src-tauri/tauri.conf.json`:
  ```json
  {
    "plugins": {
      "updater": {
        "active": true,
        "dialog": true,
        "endpoints": ["https://github.com/aramirez087/TuitBot/releases/latest/download/latest.json"]
      }
    }
  }
  ```
- On app start: check for updates silently
- If update available: show a non-intrusive notification in the sidebar footer
- User clicks to install (downloads in background, applies on next restart)

### 5. Onboarding flow (first-run experience)

When the app detects no config file (`~/.tuitbot/config.toml` doesn't exist), show
an onboarding wizard instead of the dashboard:

#### Step 1: Welcome
- "Welcome to Tuitbot — your autonomous X growth assistant"
- Brief explanation of what it does
- "Get started" button

#### Step 2: X API setup
- Link to https://developer.x.com/en/portal/dashboard
- Input fields for client_id (required) and client_secret (optional)
- Trigger OAuth flow (open browser for auth)
- Show success when authenticated

#### Step 3: Business profile
- Product name, description, URL
- Target audience
- Product keywords (at least 1)
- Industry topics (at least 1)

#### Step 4: LLM provider
- Select provider (OpenAI / Anthropic / Ollama)
- API key input
- Model selection (with defaults)
- Test connection button

#### Step 5: Review & launch
- Summary of all configured settings
- "Start tuitbot" button
- Option to enable approval mode (recommended for new users)
- Creates `config.toml` and starts the automation runtime

Route: `src/routes/onboarding/+page.svelte` with step components.

### 6. Server embedding

Instead of spawning `tuitbot-server` as a separate process, embed it directly in
the Tauri Rust code:

```rust
// dashboard/src-tauri/src/main.rs
#[tauri::command]
async fn start_server(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let router = tuitbot_server::build_router(state.inner().clone());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001").await?;
    tokio::spawn(axum::serve(listener, router));
    Ok(())
}
```

This avoids sidecar complexity. The Tauri Cargo.toml depends on `tuitbot-server`
as a library. The server starts on a background tokio task when the app launches.

### 7. Native notifications

Use Tauri's notification API for:
- New approval items when window is minimized
- Follower milestone alerts (every 100 followers)
- Error alerts (consecutive errors from automation loops)
- Daily summary notification (optional, configurable)

### 8. CI/CD for builds

#### GitHub Actions workflow (`.github/workflows/build-desktop.yml`):

```yaml
name: Build Desktop App
on:
  push:
    tags: ['app-v*']

jobs:
  build:
    strategy:
      matrix:
        include:
          - os: macos-latest
            target: aarch64-apple-darwin
          - os: macos-13
            target: x86_64-apple-darwin
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
          - os: windows-latest
            target: x86_64-pc-windows-msvc
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with: { node-version: '20' }
      - uses: dtolnay/rust-toolchain@stable
      - run: cd dashboard && npm ci && npm run tauri build
      - uses: actions/upload-artifact@v4
        with:
          name: tuitbot-${{ matrix.target }}
          path: dashboard/src-tauri/target/release/bundle/
```

Upload artifacts to GitHub Releases for auto-updater consumption.

### 9. Polish pass

#### Loading states
- Every page should have a skeleton loader (not spinner) on first load
- Subsequent navigations use cached data with background refresh

#### Error states
- API unreachable: show reconnection banner at top
- Individual request failures: inline error with retry button
- Server not running: show "Connecting..." state with auto-retry

#### Empty states
- Each page has a helpful empty state (not just blank)
- Suggest next action (e.g., "Add your first target account to start building relationships")

#### Transitions
- Page transitions: subtle fade (not slide)
- List item actions (approve/reject): slide-out animation
- New items: slide-in from top

#### Keyboard shortcuts (global)
- `Cmd+1` through `Cmd+6` — navigate to dashboard sections
- `Cmd+N` — open compose modal (from any page)
- `Cmd+,` — open settings

### 10. Window state persistence

- Remember window size and position across restarts
- Remember sidebar collapsed state
- Remember last active page (reopen to same page)
- Use Tauri's `tauri-plugin-store` for persistent frontend state

## What NOT to build yet

- Code signing for macOS/Windows (requires developer certificates — set up separately)
- Crash reporting / telemetry
- Multi-account support
- Cloud sync
- Mobile companion app

## Acceptance criteria

- [ ] `npm run tauri build` produces a working DMG (macOS)
- [ ] App installs and launches from DMG without needing cargo/rust/node
- [ ] System tray icon shows with context menu
- [ ] Auto-start on login toggle works
- [ ] Onboarding wizard completes and creates a working config
- [ ] Server starts embedded (no separate process to manage)
- [ ] Native notifications work for approval items
- [ ] Auto-updater checks for updates on launch
- [ ] Window state persists across restarts
- [ ] Global keyboard shortcuts work
- [ ] All pages have proper loading, error, and empty states
- [ ] CI workflow builds for macOS (ARM + Intel), Linux, and Windows

## Reference files

- `dashboard/src-tauri/tauri.conf.json` — Tauri configuration
- `dashboard/src-tauri/src/main.rs` — Tauri entry point
- `crates/tuitbot-server/src/lib.rs` — `build_router()` for embedding
- `crates/tuitbot-cli/src/` — init command for reference on onboarding flow
- `crates/tuitbot-core/src/config/mod.rs` — config creation logic
- `.github/workflows/` — existing CI workflows for reference
