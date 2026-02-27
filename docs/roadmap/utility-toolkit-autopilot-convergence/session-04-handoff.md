# Session 04 Handoff: Unified Mutation Policy Gateway

**Date:** 2026-02-26
**Session:** 04 of 08
**Branch:** `feat/mcp_final`

---

## Completed Work

1. **Created `tuitbot-core::mutation_gateway` module** (`mod.rs` + `tests.rs`):
   - `MutationGateway` struct with `evaluate()`, `complete_success()`, `complete_failure()` static methods
   - `MutationRequest<'a>` — stateless input with pool, policy config, mode, tool name, params
   - `GatewayDecision` enum — Proceed, Denied, RoutedToApproval, DryRun, Duplicate
   - `MutationTicket` — audit_id + correlation_id + tool_name for post-execution recording
   - UUID v4-like `generate_correlation_id()` function
   - 15 tests covering all decision paths

2. **Rewrote MCP `policy_gate.rs`** as thin adapter over core gateway:
   - `run_gateway()` — single entry point replacing 3-step check/begin/record sequence
   - `complete_gateway_success()` — records success via core gateway, builds ToolMeta with rollback
   - `complete_gateway_failure()` — records failure via core gateway, builds ToolMeta
   - Kept `check_policy()` (legacy, used by dry-run tools) and `get_policy_status()` (read-only)
   - Added `format_denial()` and `format_duplicate()` helpers for JSON response formatting

3. **Refactored all 13 mutation tool handlers** to use unified gateway:
   - `write.rs` — post_tweet, reply_to_tweet, quote_tweet, delete_tweet, post_thread (5 tools)
   - `engage.rs` — like_tweet, unlike_tweet, follow_user, unfollow_user, retweet, unretweet, bookmark_tweet, unbookmark_tweet (8 tools)
   - All follow identical pattern: `run_gateway → execute → complete_gateway_success/failure`

4. **Removed dead code**:
   - Deleted `x_actions/audit.rs` entirely (functions replaced by gateway)
   - Removed `begin_mutation`, `MutationGuard`, `uuid_v4`, `DB_IDEMPOTENCY_WINDOW_SECS` from `idempotency.rs`
   - Kept only `IdempotencyStore` (in-memory 30s dedup) in `idempotency.rs`

5. **Added `format_toolkit_error_with_meta()`** to `x_actions/mod.rs`:
   - Replaces old `audited_toolkit_error_response()`
   - Maps `ToolkitError` variants to error codes with pre-built `ToolMeta` from gateway

---

## Concrete Decisions Made

| Decision | Summary |
|----------|---------|
| Gateway location | `tuitbot-core::mutation_gateway` — usable by all consumers (MCP, autopilot, HTTP) |
| Stateless design | All dependencies passed per-call via `MutationRequest`; no held state |
| Two-layer idempotency | In-memory 30s (MCP-specific) + DB-backed 5min (core gateway) |
| Audit module removal | `x_actions/audit.rs` deleted — all audit logic now in core gateway |
| Correlation ID | UUID v4-like format generated in core, carried via `MutationTicket` |
| Dry-run tools | Still use legacy `check_policy()` — no audit/idempotency needed |
| Error mapping | New `format_toolkit_error_with_meta()` takes pre-built meta from gateway |

---

## Open Issues

1. **Utility-write mutations skip the gateway**: By design per AD-04 — utility profiles are raw toolkit calls. The gateway is a workflow-layer concern. If a future session wants optional policy for utility mutations, it can wrap calls through `MutationGateway::evaluate()`.

2. **Dry-run policy check is legacy**: `check_policy()` in `policy_gate.rs` only checks policy (no idempotency/audit). This is correct for dry-run tools but uses a separate code path. Could be unified if dry-run tools gain idempotency needs.

3. **Flaky baseline test**: `config::tests::env_var_override_approval_mode` fails in parallel runs due to env-var race. Passes with `--test-threads=1`. Pre-existing, not related to this session.

---

## Session 05 Inputs

### Files to Read First

1. **`docs/roadmap/utility-toolkit-autopilot-convergence/session-04-handoff.md`** — This file
2. **`docs/roadmap/utility-toolkit-autopilot-convergence/charter.md`** — Section 5 (autopilot convergence)
3. **`docs/roadmap/utility-toolkit-autopilot-convergence/architecture-decisions.md`** — AD-06 through AD-08
4. **`crates/tuitbot-core/src/mutation_gateway/mod.rs`** — The gateway (session 04 deliverable)
5. **`crates/tuitbot-core/src/automation/mod.rs`** — Autopilot runtime
6. **`crates/tuitbot-core/src/automation/loops/`** — Engagement, posting, discovery loops
7. **`crates/tuitbot-core/src/automation/circuit_breaker.rs`** — Circuit breaker for X API failures
8. **`crates/tuitbot-mcp/src/tools/workflow/policy_gate.rs`** — MCP adapter over gateway
9. **`crates/tuitbot-mcp/src/server/utility_write.rs`** — Utility-write server (no gateway)
10. **`crates/tuitbot-mcp/src/server/utility_readonly.rs`** — Utility-readonly server

### Commands to Run Before Starting

```bash
# Verify baseline is green
cargo fmt --all && cargo fmt --all --check
RUSTFLAGS="-D warnings" cargo test --workspace
cargo clippy --workspace -- -D warnings

# Record baseline test counts
cargo test --workspace 2>&1 | grep "test result"
```

### Session 05 Deliverables

Per charter and execution plan — verify against those documents for exact scope.

### Session 05 Exit Criteria

- Session 05 scope fully implemented
- All tests pass
- No new clippy warnings
- Session 06 inputs are explicit in the handoff

---

## Artifact Inventory

| File | Status |
|------|--------|
| `crates/tuitbot-core/src/lib.rs` | Modified (added `pub mod mutation_gateway`) |
| `crates/tuitbot-core/src/mutation_gateway/mod.rs` | Created |
| `crates/tuitbot-core/src/mutation_gateway/tests.rs` | Created |
| `crates/tuitbot-mcp/src/tools/workflow/policy_gate.rs` | Rewritten (thin adapter over core gateway) |
| `crates/tuitbot-mcp/src/tools/workflow/x_actions/mod.rs` | Modified (removed audit module, added `format_toolkit_error_with_meta`) |
| `crates/tuitbot-mcp/src/tools/workflow/x_actions/audit.rs` | Deleted |
| `crates/tuitbot-mcp/src/tools/workflow/x_actions/write.rs` | Rewritten (unified gateway pattern) |
| `crates/tuitbot-mcp/src/tools/workflow/x_actions/engage.rs` | Rewritten (unified gateway pattern) |
| `crates/tuitbot-mcp/src/tools/idempotency.rs` | Modified (removed dead code, kept IdempotencyStore) |
| `docs/roadmap/.../session-04-policy-gateway.md` | Created |
| `docs/roadmap/.../session-04-handoff.md` | Created (this file) |

---

## Test Counts

| Metric | Value |
|--------|-------|
| Baseline (session start) | 777 passed |
| Final | 792 passed (+15 gateway tests) |
| Flaky (pre-existing) | 1 (`env_var_override_approval_mode`) |
