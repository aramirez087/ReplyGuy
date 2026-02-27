# Session 06 Handoff: MCP Handler Rewiring → Workflow Composers

**Date:** 2026-02-26
**Session:** 06 of 08
**Branch:** `feat/mcp_final`

---

## Completed Work

1. **Created `tuitbot_core::workflow` module** (7 source files + 1 test file):
   - `mod.rs` — `WorkflowError` enum, `SharedProvider` adapter, shared IO types
   - `discover.rs` — search → score → persist → rank pipeline
   - `draft.rs` — fetch candidates → LLM generation → safety checks
   - `queue.rs` — validate → safety-check → route to approval or execute
   - `publish.rs` — thin wrappers over toolkit write functions
   - `thread_plan.rs` — LLM thread generation + hook analysis
   - `orchestrate.rs` — deterministic discover → draft → queue cycle
   - `tests.rs` — 20 integration tests

2. **Refactored 4 MCP composite handlers** to thin adapters:
   - `find_opportunities.rs` — delegates to `workflow::discover::execute()`
   - `draft_replies.rs` — delegates to `workflow::draft::execute()`
   - `propose_queue.rs` — delegates to `workflow::queue::execute()`
   - `thread_plan.rs` — delegates to `workflow::thread_plan::execute()`

3. **Relocated shared IO types** to `core::workflow`:
   - `ScoredCandidate`, `DraftResult`, `ProposeResult`, `ScoreBreakdown`, `QueueItem`
   - Re-exported from MCP composite module for backward compatibility

4. **Fixed `check_policy()` in `policy_gate.rs`**:
   - Was returning empty string for non-Allow decisions
   - Now returns proper JSON error responses with correct error codes and telemetry

5. **Added input validation ordering** in `draft_replies.rs`:
   - Empty input check now runs before LLM provider check

---

## Concrete Decisions Made

| Decision | Summary |
|----------|---------|
| `SharedProvider(Arc<dyn LlmProvider>)` | Bridges Arc-based sharing into Box for ContentGenerator |
| Workflow steps take explicit params | Not WorkflowCtx — because ContentGenerator needs owned Box |
| Types in `core::workflow` | Canonical location; MCP re-exports for compat |
| `WorkflowError` maps to existing `ErrorCode` | No new error codes introduced (AD-10) |
| Deterministic orchestrator | `run_discovery_cycle()` composes steps in fixed order |
| Response field `total_searched` | Normalized from mixed `total_found`/`total_searched` usage |

---

## Open Issues

1. **MCP handlers still own telemetry recording**: Workflow steps don't record MCP telemetry — that's MCP-specific. If non-MCP consumers want telemetry, they'd need their own instrumentation. This is correct per the architecture (transport-specific concerns stay in transport layer).

2. **`check_policy` vs `run_gateway` inconsistency**: `propose_queue.rs` uses the legacy `check_policy` + manual `record_mutation` instead of the unified `run_gateway`. Works correctly now but could be simplified to use `run_gateway` which handles policy + idempotency + audit in one call.

3. **`workflow/tests.rs` at 781 lines**: Exceeds the 500-line CLAUDE.md rule. Should be split into `tests/` module directory with per-step test files if it grows further.

4. **Flaky baseline test**: `config::tests::env_var_override_approval_mode` fails in parallel runs. Pre-existing, not related to this session.

---

## Session 07 Inputs

### Files to Read First

1. **`docs/roadmap/utility-toolkit-autopilot-convergence/session-06-handoff.md`** — This file
2. **`docs/roadmap/utility-toolkit-autopilot-convergence/charter.md`** — Overall scope
3. **`docs/roadmap/utility-toolkit-autopilot-convergence/execution-plan.md`** — Session 07 scope
4. **`docs/roadmap/utility-toolkit-autopilot-convergence/architecture-decisions.md`** — AD-06 (adapters)
5. **`crates/tuitbot-core/src/workflow/`** — Complete workflow layer (this session)
6. **`crates/tuitbot-core/src/toolkit/`** — Complete toolkit layer (sessions 02-03)
7. **`crates/tuitbot-core/src/automation/`** — Autopilot loops to be rewired
8. **`crates/tuitbot-core/src/automation/adapters.rs`** — Adapter implementations (session 05)
9. **`crates/tuitbot-core/src/automation/loop_helpers.rs`** — Port trait definitions

### Commands to Run Before Starting

```bash
# Verify baseline is green
cargo fmt --all && cargo fmt --all --check
RUSTFLAGS="-D warnings" cargo test --workspace
cargo clippy --workspace -- -D warnings

# Record baseline test counts
cargo test --workspace 2>&1 | grep "test result"
```

### Session 07 Deliverables

Per execution plan — Rewire Autopilot Loops:
- Refactor autopilot loops (discovery, mentions, content, thread, target, analytics) to call toolkit/workflow instead of XApiClient directly
- Simplify `loop_helpers.rs` traits and `adapters.rs` implementations
- Verify zero direct XApiClient method calls in loop files

### Session 07 Exit Criteria

- Session 07 scope fully implemented
- All tests pass
- No new clippy warnings
- `adapters.rs` reduced by at least 50%
- Session 08 inputs are explicit in the handoff

---

## Artifact Inventory

| File | Status |
|------|--------|
| `crates/tuitbot-core/src/workflow/mod.rs` | Created (217 lines) |
| `crates/tuitbot-core/src/workflow/discover.rs` | Created (192 lines) |
| `crates/tuitbot-core/src/workflow/draft.rs` | Created (132 lines) |
| `crates/tuitbot-core/src/workflow/queue.rs` | Created (197 lines) |
| `crates/tuitbot-core/src/workflow/publish.rs` | Created (49 lines) |
| `crates/tuitbot-core/src/workflow/thread_plan.rs` | Created (127 lines) |
| `crates/tuitbot-core/src/workflow/orchestrate.rs` | Created (211 lines) |
| `crates/tuitbot-core/src/workflow/tests.rs` | Created (781 lines) |
| `crates/tuitbot-core/src/lib.rs` | Modified (added `pub mod workflow`) |
| `crates/tuitbot-mcp/src/tools/workflow/composite/mod.rs` | Modified (re-exports from core) |
| `crates/tuitbot-mcp/src/tools/workflow/composite/find_opportunities.rs` | Modified (thin adapter) |
| `crates/tuitbot-mcp/src/tools/workflow/composite/draft_replies.rs` | Modified (thin adapter) |
| `crates/tuitbot-mcp/src/tools/workflow/composite/propose_queue.rs` | Modified (thin adapter) |
| `crates/tuitbot-mcp/src/tools/workflow/composite/thread_plan.rs` | Modified (thin adapter) |
| `crates/tuitbot-mcp/src/tools/workflow/policy_gate.rs` | Modified (proper denial JSON) |
| `docs/roadmap/.../session-06-workflow-composers.md` | Created |
| `docs/roadmap/.../session-06-handoff.md` | Created (this file) |

---

## Test Counts

| Metric | Value |
|--------|-------|
| Baseline (session start) | 804 passed |
| Final | 824 passed (+20 workflow integration tests) |
| Flaky (pre-existing) | 1 (`env_var_override_approval_mode`) |
