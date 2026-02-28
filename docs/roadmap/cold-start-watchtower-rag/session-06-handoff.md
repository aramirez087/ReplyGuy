# Session 06 Handoff — Google Drive Source Adapter

## Summary

Added Google Drive as a provider-backed content source that feeds the same ingest pipeline and Winning DNA workflow as local folders. Introduced a `ContentSourceProvider` trait abstraction, refactored the Watchtower to dispatch between local filesystem watching and remote polling, and extended both the configuration layer and dashboard UI to support Google Drive sources.

## What Was Built

### Provider Abstraction (`crates/tuitbot-core/src/source/`)
- **`mod.rs`**: `ContentSourceProvider` async trait with `scan_for_changes()` and `read_content()` methods, `SourceFile` metadata struct, `SourceError` error type
- **`local_fs.rs`**: `LocalFsProvider` implementing the trait via directory walking + tokio file reading
- **`google_drive.rs`**: `GoogleDriveProvider` implementing the trait via Drive API v3 polling with service-account JWT auth (self-contained RSA signing using `sha2`/`base64`)
- **`tests.rs`**: 13 tests covering both providers, dedup, ingest parity, and storage helpers

### Config Extensions (`crates/tuitbot-core/src/config/types.rs`)
- Added `folder_id`, `service_account_key`, `poll_interval_seconds` optional fields to `ContentSourceEntry`
- All new fields use `#[serde(default)]` for backward compatibility

### Watchtower Refactor (`crates/tuitbot-core/src/automation/watchtower/mod.rs`)
- **`ingest_content()`**: New provider-agnostic ingest function accepting raw text content
- **`ingest_file()`**: Now delegates to `ingest_content()` after reading from disk
- **`WatchtowerLoop::run()`**: Splits sources into local (notify watcher) and remote (polling), runs both concurrently
- **`poll_remote_sources()`**: Polls all remote providers, ingests changed files, updates sync cursors
- **`remote_only_loop()`**: Dedicated loop for configurations with only remote sources
- **`RemoteSource` type alias**: Keeps complex tuple type clean

### Storage Helpers (`crates/tuitbot-core/src/storage/watchtower/mod.rs`)
- **`ensure_google_drive_source()`**: Idempotent registration for Drive sources
- **`find_source_by_folder_id()`**: Lookup by folder ID in config_json

### Dashboard
- **`api.ts`**: Extended content source type with `folder_id`, `service_account_key`, `poll_interval_seconds`
- **`ContentSourcesSection.svelte`**: Source-type selector (Local Folder / Google Drive), conditional fields, poll interval input, hidden loop-back for Drive
- **`SourcesStep.svelte`**: Same source-type selector for onboarding flow
- **`onboarding.ts`**: Added `source_type`, `folder_id`, `service_account_key`, `poll_interval_seconds` to store
- **`+page.svelte`**: Submit logic handles both source types

## Key Decisions

1. **Provider trait covers scan + read, not watch**: Local sources use `notify` watcher; remote sources poll. The `WatchtowerLoop` orchestrates both, but providers only implement stateless operations.
2. **Self-contained RSA signing**: No external JWT crate needed. Minimal DER parsing + big-integer arithmetic for PKCS#1 v1.5 signing, using existing `sha2` and `base64` dependencies.
3. **Flat config with optional fields**: Preserves backward compatibility and works with the generic JSON→TOML merge in the settings API.
4. **`ingest_content()` as the shared code path**: Both local file reads and remote content fetches funnel through the same function, ensuring identical front-matter parsing, hashing, and storage.
5. **Provider ID format `gdrive://<id>/<name>`**: Stable deduplication for remote files using Google Drive's immutable file IDs.

## Quality Gates

| Check | Result |
|-------|--------|
| `cargo fmt --all --check` | Pass |
| `RUSTFLAGS="-D warnings" cargo test --workspace` | 1664 pass (990 core, 495 cli, 36 server, 24 mcp, 118 proc-macro, 1 build) |
| `cargo clippy --workspace -- -D warnings` | Pass (0 warnings) |
| `cd dashboard && npm run check` | 0 errors, 5 warnings (all pre-existing) |

## New Tests (13 added)

### Source Provider Tests (`source/tests.rs`)
- `local_fs_provider_scan_returns_files` — scan a tempdir with mixed file types
- `local_fs_provider_read_content` — read markdown file content
- `local_fs_provider_read_nonexistent_returns_error` — missing file error
- `local_fs_provider_filters_patterns` — only matching patterns
- `local_fs_provider_skips_hidden_dirs` — hidden directory exclusion
- `extract_drive_id_from_provider_format` — parse `gdrive://` format
- `extract_drive_id_from_raw_id` — pass-through raw ID
- `source_file_hash_equality` / `source_file_hash_difference` — hash comparison
- `ingest_parity_local_vs_direct_content` — same content produces identical nodes
- `ingest_content_dedup_by_hash` — skip unchanged, update changed
- `ensure_google_drive_source_creates_once` — idempotent DB registration
- `find_source_by_folder_id_returns_match` — config_json search
- `different_source_types_coexist` — local + Drive in same DB

### Config Tests (3 added in `config/tests.rs`)
- `content_sources_google_drive_roundtrip` — TOML with Drive fields
- `content_sources_mixed_sources_roundtrip` — local + Drive together
- `content_sources_google_drive_json_patch` — frontend JSON shape

## Files Created
- `crates/tuitbot-core/src/source/mod.rs`
- `crates/tuitbot-core/src/source/local_fs.rs`
- `crates/tuitbot-core/src/source/google_drive.rs`
- `crates/tuitbot-core/src/source/tests.rs`
- `docs/roadmap/cold-start-watchtower-rag/google-drive-integration.md`
- `docs/roadmap/cold-start-watchtower-rag/session-06-handoff.md`

## Files Modified
- `crates/tuitbot-core/src/lib.rs` — added `pub mod source;`
- `crates/tuitbot-core/src/config/types.rs` — new fields on `ContentSourceEntry`
- `crates/tuitbot-core/src/config/tests.rs` — 3 new round-trip tests
- `crates/tuitbot-core/src/automation/watchtower/mod.rs` — `ingest_content()`, remote polling, `RemoteSource` type alias
- `crates/tuitbot-core/src/automation/watchtower/tests.rs` — updated struct construction for new fields
- `crates/tuitbot-core/src/storage/watchtower/mod.rs` — `ensure_google_drive_source()`, `find_source_by_folder_id()`
- `dashboard/src/lib/api.ts` — extended content source type
- `dashboard/src/lib/stores/onboarding.ts` — new fields
- `dashboard/src/lib/components/onboarding/SourcesStep.svelte` — source-type selector, conditional fields
- `dashboard/src/routes/(app)/settings/ContentSourcesSection.svelte` — source-type selector, conditional fields
- `dashboard/src/routes/onboarding/+page.svelte` — submit handles both types

## Next Session Inputs (Session 07)

- The `ContentSourceProvider` trait is in place and both `LocalFsProvider` and `GoogleDriveProvider` implement it. However, the WatchtowerLoop directly instantiates `GoogleDriveProvider` — future sessions could add a factory/registry pattern for dynamic provider loading.
- The Google Drive JWT auth uses self-contained RSA signing. For production use, this should be tested with a real service account key to verify PKCS#8 parsing against Google's key format.
- Loop-back is disabled for Google Drive (read-only). Session 07 could add write support via Drive API if needed.
- The dashboard still manages `sources[0]` only. Multiple sources require config.toml editing.
- Error handling for remote sources is basic (log + set status to "error"). Retry with exponential backoff would improve reliability.
- The `source/` module exports the trait but is not yet used by the server's ingest route for file_hints — the inline_nodes path is already provider-agnostic, and file_hints continue to use `ingest_file()` for local sources only.
