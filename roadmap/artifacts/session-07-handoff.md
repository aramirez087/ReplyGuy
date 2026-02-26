# Session 07 — Handoff

## Summary

Physically isolated workflow-only MCP tools behind `tools/workflow/` so the generic API profile server (`server/api.rs`) can never depend on workflow internals. Added a `Lane` enum to the manifest for compile-time structural verification.

## What Changed

### File Moves
- 15 files/directories moved from `tools/` to `tools/workflow/`
- `tools/workflow/mod.rs` created as the workflow gateway

### Import Updates
- All moved files: `super::response` → `crate::tools::response`
- Composite files: `crate::tools::{content,policy_gate,telemetry}` → `crate::tools::workflow::*`
- `server/workflow.rs`: `tools::<module>` → `workflow::<module>` for all workflow calls
- `contract_tests.rs`, `eval_harness.rs`, `composite/tests.rs`, `benchmark.rs`: prefixed workflow paths

### Manifest Enhancements
- Added `Lane` enum (`Shared` / `Workflow`) to `ToolEntry`
- Fixed `score_tweet`: `requires_db: false`, removed `DbError` from error codes
- Added doc comments to `X_WRITE_ERR` / `X_ENGAGE_ERR` noting policy codes are workflow-only
- Regenerated `session-05-tool-manifest.json` snapshot

### New Tests (`boundary_tests.rs`)
1. `api_server_does_not_import_workflow_modules` — source scan guard
2. `workflow_only_tools_have_workflow_lane` — lane correctness
3. `shared_tools_have_shared_lane` — lane correctness
4. `score_tweet_does_not_require_db` — metadata regression
5. `api_profile_tool_count` / `workflow_profile_tool_count` — drift guards

## Files Modified

| File | Change |
|------|--------|
| `tools/mod.rs` | Reduced to shared modules + `pub mod workflow` |
| `tools/workflow/mod.rs` | **New** — 15 pub mod declarations |
| `tools/workflow/*.rs` | **Moved** + import fixes |
| `tools/workflow/composite/*` | **Moved** + cross-ref fixes |
| `tools/workflow/x_actions/*` | **Moved** + import fixes |
| `tools/manifest.rs` | `Lane` enum, `lane` field, `score_tweet` fix |
| `tools/boundary_tests.rs` | **New** — 6 structural tests |
| `tools/benchmark.rs` | Fixed `super::` → `super::workflow::` |
| `tools/contract_tests.rs` | Prefixed workflow paths |
| `tools/eval_harness.rs` | Prefixed workflow paths |
| `server/workflow.rs` | `use crate::tools::workflow;` + ~55 call site renames |
| `roadmap/artifacts/session-05-tool-manifest.json` | Regenerated |
| `roadmap/artifacts/session-07-*.md` | **New** — 3 artifact docs |

## CI Status

```
cargo fmt --all --check     ✓
cargo clippy -- -D warnings ✓
cargo test --workspace      ✓ (all 226 tests pass)
```

## Session 08 Preview

Session 08 adds a **scraper lane** — a third profile for read-only scraping without authentication. The workflow boundary established here makes this clean: scraper tools go into `tools/scraper/` alongside `tools/workflow/`, with their own provider implementation and `Lane::Scraper` variant.
