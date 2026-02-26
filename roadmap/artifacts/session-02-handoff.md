# Session 02 — Read-Only Tool Curation and Enforcement

**Generated:** 2026-02-26

## Summary

Split the monolithic `ApiMcpServer` into two focused, auditable server structs — one per read-only profile — ensuring neither exposes mutation tools at registration time.

## Changes

| File | Action |
|------|--------|
| `crates/tuitbot-mcp/src/state.rs` | Renamed `ApiState` → `ReadonlyState`, removed `idempotency` field |
| `crates/tuitbot-mcp/src/server/readonly.rs` | **CREATED** — `ReadonlyMcpServer` (10 tools) |
| `crates/tuitbot-mcp/src/server/api_readonly.rs` | **CREATED** — `ApiReadonlyMcpServer` (20 tools) |
| `crates/tuitbot-mcp/src/server/api.rs` | **DELETED** — replaced by readonly + api_readonly |
| `crates/tuitbot-mcp/src/server/mod.rs` | Updated exports for new server structs |
| `crates/tuitbot-mcp/src/lib.rs` | Three-way dispatch, shared `init_readonly_state` helper |
| `crates/tuitbot-mcp/src/tools/manifest.rs` | 3 profile variants: `Workflow`, `Readonly`, `ApiReadonly` |
| `crates/tuitbot-mcp/src/tools/boundary_tests.rs` | Updated + added 7 safety tests |
| `crates/tuitbot-mcp/src/kernel/mod.rs` | `#[allow(dead_code)]` on engage/write/media modules |
| `roadmap/artifacts/session-05-tool-manifest.json` | Regenerated |

## Profile Tool Counts

| Profile | Tools | Mutations |
|---------|-------|-----------|
| `readonly` | 10 | 0 |
| `api-readonly` | 20 | 0 |
| `full` (workflow) | 64 | unchanged |

## CI Results

- `cargo fmt --all --check` — PASS
- `cargo clippy --workspace -- -D warnings` — PASS
- `RUSTFLAGS="-D warnings" cargo test --workspace` — 1,098 passed, 0 failed

## Future Cleanup

- **Dead kernel code**: `kernel::engage`, `kernel::write`, and `kernel::media` modules are
  currently `#[allow(dead_code)]` — they were only used by the deleted `api.rs` server.
  The conformance tests (`conformance_tests/engage.rs`, `conformance_tests/write.rs`)
  still exercise them. A future session should either:
  1. Remove these kernel modules and their conformance tests entirely (if the workflow
     profile's `x_actions` tests provide sufficient coverage), or
  2. Re-purpose them for a future `api-full` profile that exposes mutations without
     workflow gating.
