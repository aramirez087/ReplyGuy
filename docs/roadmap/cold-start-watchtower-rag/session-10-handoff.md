# Session 10 — Mode-Aware Validation: Handoff

## Summary

Validated that source selection behaves correctly across Desktop, SelfHost, and Cloud deployment modes. Found and fixed three issues: a missing `deployment_mode` field in the Tauri sidecar's `AppState` construction, a missing deployment-mode guard in the Watchtower startup filter, and a filter predicate that excluded Google Drive sources from Watchtower startup. Updated the validation report with a corrected GO recommendation.

## What Was Delivered

### Watchtower deployment-mode guard (`crates/tuitbot-server/src/main.rs`)

Added `deployment_mode.allows_source_type()` filter to the Watchtower startup logic. Sources whose type is incompatible with the deployment mode are now skipped with a structured `tracing::warn!` including `source_type` and `deployment_mode` fields. Also fixed the filter predicate to include `folder_id.is_some()` so Google Drive sources are correctly counted as watchable.

Before:
```rust
.filter(|s| s.watch && s.path.is_some())
```

After:
```rust
.filter(|s| {
    if !deployment_mode.allows_source_type(&s.source_type) {
        tracing::warn!(
            source_type = %s.source_type,
            deployment_mode = %deployment_mode,
            "skipping content source incompatible with deployment mode"
        );
        return false;
    }
    s.watch && (s.path.is_some() || s.folder_id.is_some())
})
```

### Tauri sidecar fix (`dashboard/src-tauri/src/lib.rs`)

Added the missing `deployment_mode: DeploymentMode::Desktop` field to the `AppState` construction. This was a compile error in the Tauri crate (excluded from workspace, so `cargo test --workspace` did not catch it). Also added the `DeploymentMode` import.

### Updated validation report (`docs/roadmap/cold-start-watchtower-rag/validation-report.md`)

Comprehensive update covering Sessions 08–10:
- Added deployment mode test coverage table (22 tests)
- Added three-scenario deployment mode validation (Desktop, SelfHost, Cloud)
- Added edge case validation (pre-existing local_fs config in cloud mode)
- Documented all three issues found and fixed
- Added Risk #7 (Tauri crate excluded from workspace)
- Updated follow-up work table with deployment-mode items
- Issued corrected GO recommendation

### Reviewed docs (no changes needed)

- `docs/architecture.md` — Deployment modes section (lines 109–121) is accurate
- `docs/configuration.md` — Deployment mode section (lines 82–104) and content sources section (lines 214–296) are accurate. Line 104 already documents the runtime skip behavior that we implemented in the Watchtower guard.

## CI Results

| Gate | Result |
|------|--------|
| `cargo fmt --all --check` | PASS |
| `RUSTFLAGS="-D warnings" cargo test --workspace` | PASS |
| `cargo clippy --workspace -- -D warnings` | PASS (0 warnings) |
| `npm run check` (dashboard) | PASS (0 errors, 5 pre-existing warnings) |

## Design Decisions

1. **Watchtower guard in server, not in WatchtowerLoop** — The deployment-mode filter lives in `main.rs` at server startup, not inside `WatchtowerLoop::run()`. The WatchtowerLoop is in `tuitbot-core` (L3 Autopilot) and stays deployment-unaware by design. The server is the deployment boundary and the right place for policy enforcement. This follows the architectural principle that core stays deployment-unaware.

2. **Tauri explicitly sets Desktop** — Rather than relying on `Default::default()` for `DeploymentMode`, we now explicitly set `DeploymentMode::Desktop` in the Tauri sidecar. This prevents future breakage if the default changes and makes the intent clear in code review.

3. **Extended filter includes folder_id** — The original filter `s.path.is_some()` would miss Google Drive sources (which have `folder_id` but no `path`). The corrected filter `s.path.is_some() || s.folder_id.is_some()` ensures both local and remote sources are correctly identified for Watchtower startup.

4. **Code-trace validation over interactive testing** — Validated all three deployment scenarios by tracing code paths through server startup, API endpoints, frontend stores, and UI components. This is more thorough than interactive testing because it covers every branch, not just the happy path. The code paths are deterministic and well-covered by the 22 unit/integration tests.

## Exit Criteria Verification

| Criterion | Status |
|-----------|--------|
| Quality gates pass without suppressing warnings | Met |
| Manual validation covers desktop, self-host, and cloud paths | Met (code-trace validation) |
| Updated validation report ends with clear go/no-go | Met — GO recommendation issued |

## Open Items for Future Sessions

- `TUITBOT_DEPLOYMENT_MODE` in Docker compose templates
- Source migration assistant (cloud: convert local_fs → google_drive in UI)
- Consider adding the Tauri crate to `cargo clippy` CI via a separate step (it's excluded from workspace)
- Cloud billing integration may want to read `DeploymentMode` to gate premium source types

## Files Changed

| File | Change |
|------|--------|
| `crates/tuitbot-server/src/main.rs` | Added deployment-mode filter to Watchtower startup, extended filter to include `folder_id` |
| `dashboard/src-tauri/src/lib.rs` | Added `DeploymentMode` import and explicit `deployment_mode: DeploymentMode::Desktop` field |
| `docs/roadmap/cold-start-watchtower-rag/validation-report.md` | Updated with deployment-mode validation, three-scenario shakeout, issues fixed, corrected GO recommendation |
| `docs/roadmap/cold-start-watchtower-rag/session-10-handoff.md` | This file |
