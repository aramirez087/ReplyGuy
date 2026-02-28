# Validation Report — Cold-Start Watchtower RAG

## Epic Summary

The Cold-Start Watchtower RAG epic (Sessions 01–07) adds a content ingestion pipeline to Tuitbot that lets new users leverage their existing writing (Obsidian notes, markdown files, Google Drive docs) to generate high-quality tweets from day one — before any engagement history exists.

The pipeline watches configured content sources, extracts tweetable hooks via LLM, and injects them as cold-start context into the draft generation pipeline via Winning DNA retrieval. This replaces the empty-context problem for new accounts with source-backed, personalized seed content.

Sessions 08–09 extended the pipeline with deployment-mode-aware source capabilities, ensuring the UI and backend correctly handle Desktop, SelfHost, and Cloud deployment scenarios. Session 10 validated the full deployment-aware pipeline end-to-end and fixed two remaining issues.

## Quality Gate Results

| Check | Result | Notes |
|-------|--------|-------|
| `cargo fmt --all --check` | PASS | |
| `RUSTFLAGS="-D warnings" cargo test --workspace` | PASS | All tests pass including 22 new deployment mode tests |
| `cargo clippy --workspace -- -D warnings` | PASS | 0 warnings |
| `cd dashboard && npm run check` | PASS | 0 errors, 5 pre-existing a11y/style warnings |

## Test Coverage Summary

### Content Pipeline (Sessions 01–07)

| Component | Tests | Status |
|-----------|-------|--------|
| LocalFsProvider (scan, read, filter, hidden, error) | 5 | PASS |
| GoogleDriveProvider (ID extraction) | 2 | PASS |
| Ingest pipeline (parity, dedup) | 2 | PASS |
| Storage helpers (ensure, find, coexist) | 3 | PASS |
| Config round-trip (Drive, mixed, JSON patch) | 3 | PASS |
| Front-matter parsing (YAML, tags, malformed) | 5 | PASS |
| Pattern matching (md, txt, nested) | 4 | PASS |
| Loopback metadata (write, idempotent, multiple, preserve) | 5 | PASS |
| Seed worker (parse, mock LLM, batch) | 6 | PASS |
| Winning DNA (classify, score, retrieve, format) | 15+ | PASS |
| SourceFile hash (equality, difference) | 2 | PASS |
| **E2E: Local folder → seed pipeline** | 1 | PASS |
| **E2E: Google Drive → seed pipeline** | 1 | PASS |
| **E2E: Mixed sources → draft context** | 1 | PASS |
| **E2E: Inline node → manual source** | 1 | PASS |
| **E2E: Loopback → re-ingest detects change** | 1 | PASS |

### Deployment Mode (Sessions 08–10)

| Component | Tests | Status |
|-----------|-------|--------|
| DeploymentMode allows_source_type (desktop, self_host, cloud, unknown) | 4 | PASS |
| DeploymentCapabilities per mode (desktop, self_host, cloud) | 3 | PASS |
| DeploymentMode serde roundtrip (TOML, JSON) | 2 | PASS |
| DeploymentMode default is Desktop | 1 | PASS |
| DeploymentMode missing from config defaults | 1 | PASS |
| DeploymentMode env var override | 1 | PASS |
| DeploymentMode env var self_host variants | 1 | PASS |
| DeploymentMode env var invalid | 1 | PASS |
| DeploymentCapabilities JSON roundtrip | 1 | PASS |
| DeploymentMode Display trait | 1 | PASS |
| Config validation rejects local_fs in cloud | part of existing validation tests | PASS |
| **API: config_status includes capabilities** | 1 | PASS |
| **API: config_status cloud mode capabilities** | 1 | PASS |

## Deployment Mode Validation — Three-Scenario Shakeout

### Scenario 1: Desktop with Native Picker

| Step | Code Path | Verified |
|------|-----------|----------|
| Server starts with `DeploymentMode::Desktop` (default) | `main.rs:152–155` | Yes |
| Tauri sidecar sets `DeploymentMode::Desktop` explicitly | `lib.rs:92` | Yes (fixed in Session 10) |
| `GET /api/runtime/status` returns all capabilities `true` | `runtime.rs:27–34` | Yes |
| `GET /api/settings/status` returns same capabilities (no auth) | `settings.rs:113–121` | Yes + test |
| `runtime.ts` store loads desktop capabilities | `runtime.ts:34–38` | Yes |
| Settings UI: `canLocalFs=true`, `canNativePicker=true` | `ContentSourcesSection.svelte:20–23` | Yes |
| Both "Local Folder" and "Google Drive" options shown | `ContentSourcesSection.svelte:141–146` | Yes |
| Browse button visible | `ContentSourcesSection.svelte:170–175` | Yes |
| Onboarding: same full experience | `SourcesStep.svelte:16–18, 92–97, 120–125` | Yes |
| Config validation allows `local_fs` | `validation.rs:250–260` | Yes |
| Watchtower starts normally for `local_fs` sources | `main.rs:158–177` | Yes |

**Result: PASS** — Desktop users get the full experience with native file picker, manual path entry, and all source types.

### Scenario 2: Self-Host Browser with Manual Local Path

| Step | Code Path | Verified |
|------|-----------|----------|
| Server starts with `TUITBOT_DEPLOYMENT_MODE=self_host` | `main.rs:152–155` + env override | Yes |
| Capabilities: `local_folder=true`, `file_picker_native=false` | `types.rs:573–579` | Yes + test |
| `runtime.ts` onboarding fallback to `configStatus()` | `runtime.ts:42–47` | Yes |
| Settings UI: `canLocalFs=true`, `canNativePicker=false`, `canManualPath=true` | `ContentSourcesSection.svelte:20–23` | Yes |
| "Local Folder" option shown, Browse button hidden | Lines 141–142 visible, 170–175 gated | Yes |
| Hint text: "Enter the full server-side path..." | `ContentSourcesSection.svelte:181` | Yes |
| Onboarding: same — no Browse, manual path entry | `SourcesStep.svelte:120–131` | Yes |
| Config validation allows `local_fs` | `validation.rs:250–260` | Yes |
| Watchtower starts normally | `main.rs:158–177` | Yes |

**Result: PASS** — Self-host users get local folder with manual path entry and no native picker.

### Scenario 3: Cloud with Connector-Only Sources

| Step | Code Path | Verified |
|------|-----------|----------|
| Server starts with `TUITBOT_DEPLOYMENT_MODE=cloud` | `main.rs:152–155` + env override | Yes |
| Capabilities: `local_folder=false`, `google_drive=true` | `types.rs:580–587` | Yes + test |
| Settings UI: `canLocalFs=false`, dropdown hides "Local Folder" | `ContentSourcesSection.svelte:141` | Yes |
| `.capability-notice` banner shown explaining cloud | `ContentSourcesSection.svelte:152–155` | Yes |
| `$effect` auto-switches from `local_fs` to `google_drive` | `ContentSourcesSection.svelte:32–41` | Yes |
| Onboarding: only Google Drive shown, capability hint displayed | `SourcesStep.svelte:92–103` | Yes |
| Config validation rejects `local_fs` on save | `validation.rs:250–260` | Yes |
| Watchtower skips `local_fs` sources with warning log | `main.rs:161–168` | Yes (fixed in Session 10) |

**Result: PASS** — Cloud users see only Google Drive and manual ingest. Local folder is completely hidden with explanatory messaging.

### Edge Case: Pre-existing local_fs Config in Cloud Mode

| Step | Expected Behavior | Verified |
|------|-------------------|----------|
| Server loads config with `local_fs` source | Startup succeeds (no crash) | Yes |
| Watchtower filters incompatible sources | `local_fs` skipped with `tracing::warn!` | Yes (Session 10 fix) |
| `GET /api/settings` returns full config including `local_fs` | Config preserved, UI auto-switches | Yes |
| `PATCH /api/settings` with `local_fs` | Validation rejects with clear field-level error | Yes |
| Config file preservation | `local_fs` entries NOT deleted from TOML | Yes (by design) |

**Result: PASS** — Graceful degradation with structured logging, user-visible messaging, and validation protection.

## Issues Found and Fixed (Session 10)

| Issue | Severity | Fix |
|-------|----------|-----|
| Tauri `lib.rs` missing `deployment_mode` field in `AppState` construction | High | Added explicit `deployment_mode: DeploymentMode::Desktop` (was a compile error in the excluded Tauri crate) |
| Watchtower startup did not filter sources by deployment mode | Medium | Added `allows_source_type()` guard with structured warning log in `main.rs` |
| Watchtower startup filter missed Google Drive sources | Low | Extended filter from `s.path.is_some()` to `s.path.is_some() \|\| s.folder_id.is_some()` |

## Unresolved Risks

| ID | Risk | Severity | Mitigation |
|----|------|----------|------------|
| 1 | Google Drive JWT auth untested with real service account | Medium | Self-contained RSA signing works in unit tests; real key testing deferred to first user deployment. Rollback: disable Drive sources in config. |
| 2 | Dashboard manages only `sources[0]` | Low | Multiple sources work via `config.toml` editing. Dashboard multi-source UX is post-v1. |
| 3 | No retry/backoff for remote source errors | Low | Errors logged and source status set to "error". Next poll retries automatically. Exponential backoff is a follow-up. |
| 4 | Seed worker requires LLM availability | Low | If LLM is down, nodes stay 'pending' and retry next tick (5 min). No data loss. |
| 5 | BigUint RSA implementation in `google_drive.rs` | Medium | Minimal, correct for PKCS#1 v1.5 signing. Tested with known vectors. Consider replacing with `rsa` crate if issues arise in production. |
| 6 | Tauri folder-pick not tested in headless CI | Low | Component code verified via svelte-check; Tauri sidecar pattern is well-established. Manual testing on developer machines recommended before release. |
| 7 | Tauri crate excluded from workspace — not covered by `cargo test --workspace` | Low | The Tauri crate is a thin shell (no business logic). Session 10 fixed the missing `deployment_mode` field. Future structural changes to `AppState` should be manually verified against `lib.rs`. |

## Rollback Plan

If issues are discovered post-release:

1. **Disable Watchtower:** Remove `[[content_sources.sources]]` from `config.toml`. The watcher exits immediately with no sources configured.
2. **Remove Drive sources:** Change `source_type` back to `"local_fs"` or remove the source entry entirely.
3. **Data cleanup:** `DELETE FROM draft_seeds; DELETE FROM content_nodes; DELETE FROM source_contexts;` — these tables are additive and have no FK constraints from existing core tables (only `original_tweets.source_node_id` is a nullable FK).
4. **Migration is additive:** The watchtower migration uses `CREATE TABLE IF NOT EXISTS` and `ALTER TABLE ADD COLUMN`. No destructive schema changes. Safe to leave in place.

## Follow-Up Work (post-release)

| Priority | Item | Effort |
|----------|------|--------|
| P1 | Test Google Drive auth with real service account key | 1 session |
| P2 | Dashboard multi-source management UI | 1-2 sessions |
| P2 | Exponential backoff for remote source errors | 0.5 sessions |
| P2 | `TUITBOT_DEPLOYMENT_MODE` in Docker compose templates | 0.5 sessions |
| P3 | Google Docs → Markdown export (binary format support) | 1 session |
| P3 | Interactive OAuth flow for personal Drive accounts | 1-2 sessions |
| P3 | Additional providers (Notion, Dropbox) | 1 session each |
| P3 | Source migration assistant (cloud: convert local_fs → google_drive) | 0.5 sessions |

## Recommendation

**GO**: The Cold-Start Watchtower RAG pipeline with deployment-mode-aware source selection is release-ready for v1. All four quality gates pass. The deployment mode system (22 new tests across core and server) correctly gates source type availability across Desktop, SelfHost, and Cloud scenarios. Three issues were found and fixed during Session 10 validation: a missing `deployment_mode` field in the Tauri sidecar, a missing deployment-mode guard in the Watchtower startup, and a filter that would have excluded Google Drive sources from Watchtower startup. The frontend capability-gated UI correctly shows/hides source options, the Browse button, and explanatory messaging based on the server's declared deployment mode. Pre-existing `local_fs` configs in cloud mode are gracefully handled: skipped at runtime, rejected on save, preserved in the config file. The remaining medium-severity risks (Drive JWT auth, RSA implementation) are isolated to an optional provider and do not block the core value proposition.
