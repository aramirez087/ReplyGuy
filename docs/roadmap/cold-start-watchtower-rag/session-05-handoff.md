# Session 05 Handoff — Source Picker and Local Vault UX

## Summary

Added desktop and dashboard UX for selecting, saving, and inspecting a local content-source folder for the Watchtower. Desktop users get a native folder picker via Tauri's dialog plugin; browser/LAN users get a manual path text input fallback. The chosen path round-trips cleanly through the existing config API (`GET/PATCH /api/settings`).

## What Was Built

### Rust (Tauri Shell)
- **`tauri-plugin-dialog`** added to `dashboard/src-tauri/Cargo.toml` and registered in `lib.rs`
- **`dialog:allow-open`** permission added to `capabilities/default.json`
- **Compile blocker fixed**: `AppState` construction in `lib.rs` now includes `watchtower_cancel: None` and `content_sources: ContentSourcesConfig::default()` fields that were added in Session 03 but not wired into the Tauri binary

### Frontend (Dashboard)
- **`ContentSourcesSection.svelte`**: Full settings section with path input, Browse button (Tauri-only via dynamic import), watch/loop-back toggles, and read-only file patterns display
- **`SourcesStep.svelte`**: Onboarding step (index 5, optional) with path input, Browse button, and watch/loop-back toggles
- **`api.ts`**: `content_sources` field added to `TuitbotConfig` interface
- **`onboarding.ts`**: `vault_path`, `vault_watch`, `vault_loop_back` fields added to onboarding store
- **Onboarding page**: Sources step inserted at index 5; submit includes `content_sources` when vault_path is non-empty
- **Settings page**: Sources section added between Storage and LAN

### Tests
- **`content_sources_json_patch_roundtrip`**: Verifies the JSON shape the frontend sends via PATCH deserializes correctly and round-trips through TOML
- **`content_sources_empty_json_patch`**: Verifies empty sources array round-trips
- 3 pre-existing content_sources tests continue to pass (total: 5)

## Desktop vs Browser Behavior

| Feature | Desktop (Tauri) | Browser / LAN |
|---------|----------------|---------------|
| Folder picker | Native OS dialog via Browse button | Hidden — not available |
| Manual path input | Available | Available (primary UX) |
| Watch toggle | Available | Available |
| Loop-back toggle | Available | Available |
| File patterns | Read-only display | Read-only display |
| Path validation | Filesystem validation at ingest time | Same |

## Key Decisions

1. **No new API endpoints**: The existing `GET/PATCH /api/settings` already handles `content_sources` through generic serde — keeping the server layer thin per architecture rules.
2. **Sources step is optional in onboarding**: `canAdvance()` returns `true` unconditionally for the Sources step. Users without a vault can skip it.
3. **Dynamic import for Tauri dialog**: Uses `await import('@tauri-apps/plugin-dialog')` which gracefully fails in browser context, matching the existing `StorageSection.svelte` pattern for `@tauri-apps/plugin-autostart`.
4. **Single source in v1**: The UI manages the first element of `content_sources.sources[]`. The underlying schema supports multiple sources for future expansion.

## Known Limitations (v1)

- **Single source only**: UI manages `sources[0]`; multiple sources require config.toml editing
- **File patterns read-only**: Displayed but not editable in the UI; change via config.toml
- **No path existence validation in UI**: Path validity is checked at ingest/watch time, not at save time
- **No inline preview**: No file listing or vault health check in the settings UI

## Quality Gates

| Check | Result |
|-------|--------|
| `cargo fmt --all --check` | Pass |
| `RUSTFLAGS="-D warnings" cargo test --workspace` | 24/24 pass |
| `cargo clippy --workspace -- -D warnings` | Pass (0 warnings) |
| `cd dashboard && npm run check` | 0 errors, 5 warnings (all pre-existing) |

## Files Changed

### Created
- `dashboard/src/routes/(app)/settings/ContentSourcesSection.svelte`
- `dashboard/src/lib/components/onboarding/SourcesStep.svelte`
- `docs/roadmap/cold-start-watchtower-rag/session-05-handoff.md`

### Modified
- `dashboard/src-tauri/Cargo.toml` — added `tauri-plugin-dialog`
- `dashboard/src-tauri/src/lib.rs` — dialog plugin registration, AppState fix
- `dashboard/src-tauri/capabilities/default.json` — `dialog:allow-open`
- `dashboard/package.json` — `@tauri-apps/plugin-dialog`
- `dashboard/src/lib/api.ts` — `content_sources` in `TuitbotConfig`
- `dashboard/src/lib/stores/onboarding.ts` — vault fields
- `dashboard/src/routes/onboarding/+page.svelte` — Sources step + submit payload
- `dashboard/src/routes/(app)/settings/+page.svelte` — Sources nav + section
- `crates/tuitbot-core/src/config/tests.rs` — 2 new round-trip tests

## Next Session Inputs (Session 06 — Google Drive Source Adapter)

- `ContentSourcesConfig` and `ContentSourceEntry` in `crates/tuitbot-core/src/config/types.rs` already model `source_type` as a string; Session 06 should add `"google_drive"` as a new variant
- The `path` field on `ContentSourceEntry` is `Option<String>` — Google Drive sources would use a different identifier (folder ID or URL)
- The settings UI (`ContentSourcesSection.svelte`) currently only handles `sources[0]` of type `local_fs`; Session 06 would need a source-type selector and conditional fields
- The `@tauri-apps/plugin-dialog` is now available for any future file/folder picking needs
- The Watchtower ingest pipeline (`core/watchtower/`) already supports the `ContentSourcesConfig` — the Google Drive adapter needs to implement the same trait contract
