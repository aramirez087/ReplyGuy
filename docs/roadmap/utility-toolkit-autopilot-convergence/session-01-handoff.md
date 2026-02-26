# Session 01 Handoff: Charter and Architecture

**Date:** 2026-02-26
**Session:** 01 of 08
**Branch:** `feat/mcp_final`

---

## Completed Work

1. **Full architecture audit** of the current codebase:
   - Read and analyzed all 8 repository anchor files
   - Explored complete directory trees of `tuitbot-core/src/`, `tuitbot-mcp/src/`, and `tuitbot-mcp/src/tools/`
   - Mapped all 109 MCP tools to their implementation files
   - Identified the `XApiClient` trait (25+ methods), `LlmProvider` trait, and all automation loop traits

2. **Gap analysis** identifying 6 concrete gaps between current architecture and utility-toolkit behavior (documented in `charter.md` section 2.2)

3. **Target architecture** defined with three layers:
   - **Toolkit** (`core::toolkit/`): Stateless X/API utilities over `&dyn XApiClient`
   - **Workflow** (`core::workflow/`): Stateful composites with `WorkflowCtx` (DB + LLM + config)
   - **Autopilot** (`core::automation/`): Scheduled orchestration, refactored to use toolkit/workflow

4. **MCP profile model** confirmed unchanged: readonly/14, api-readonly/40, write/104, admin/108

5. **Execution plan** locked for Sessions 02-08 with scope boundaries, dependencies, risks, and exit criteria

6. **14 architecture decisions** recorded with full rationale and no unresolved items

---

## Concrete Decisions Made

| Decision | Summary |
|----------|---------|
| AD-01 | Module-level layering within `tuitbot-core`, not new crates |
| AD-02 | Toolkit functions take `&dyn XApiClient`, no state |
| AD-03 | Workflow functions take `&WorkflowCtx` (borrowed references) |
| AD-04 | Toolkit writes are raw (no policy enforcement) |
| AD-05 | Single `PolicyGate` in workflow layer for all mutation safety |
| AD-06 | Autopilot never calls `XApiClient` directly |
| AD-07 | MCP response envelope `{success, data, error, meta}` unchanged |
| AD-08 | MCP profile model (4 profiles) unchanged |
| AD-09 | Loop helper traits retained, implementations simplified |
| AD-10 | New error types (`ToolkitError`, `WorkflowError`) map to existing 28 `ErrorCode` variants |
| AD-11 | No backward compatibility layers, aliases, or migration shims |
| AD-12 | Safety split: stateless checks in toolkit, stateful checks in workflow |
| AD-13 | Spec pack and generated tools stay in MCP crate |
| AD-14 | `WorkflowCtx` uses borrowed references, not `Arc` |

---

## Open Issues

None. All decisions are resolved and documented.

---

## Session 02 Inputs

### Files to Read First

1. **`docs/roadmap/utility-toolkit-autopilot-convergence/charter.md`** — Section 3.1 (Toolkit layer specification)
2. **`docs/roadmap/utility-toolkit-autopilot-convergence/architecture-decisions.md`** — AD-02, AD-10, AD-12
3. **`docs/roadmap/utility-toolkit-autopilot-convergence/execution-plan.md`** — Session 02 section
4. **`crates/tuitbot-core/src/lib.rs`** — Add `pub mod toolkit;`
5. **`crates/tuitbot-core/src/x_api/mod.rs`** — `XApiClient` trait definition (read methods)
6. **`crates/tuitbot-core/src/x_api/types.rs`** — Return types for X API operations
7. **`crates/tuitbot-core/src/scoring/mod.rs`** — Existing scoring logic to re-export
8. **`crates/tuitbot-core/src/scoring/signals.rs`** — 6-signal heuristic implementation
9. **`crates/tuitbot-core/src/error.rs`** — Existing error types (reference for `ToolkitError`)
10. **`crates/tuitbot-mcp/src/tools/workflow/x_actions/read.rs`** — Current MCP read tool implementations
11. **`crates/tuitbot-mcp/src/kernel/read.rs`** — Kernel read implementations

### Commands to Run Before Starting

```bash
# Verify baseline is green
cargo fmt --all && cargo fmt --all --check
RUSTFLAGS="-D warnings" cargo test --workspace
cargo clippy --workspace -- -D warnings

# Record baseline test counts
cargo test --workspace 2>&1 | grep "test result"
```

### Session 02 Deliverables

1. `crates/tuitbot-core/src/toolkit/mod.rs` — Module root with `ToolkitError` enum and re-exports
2. `crates/tuitbot-core/src/toolkit/read.rs` — 14 stateless read functions
3. `crates/tuitbot-core/src/toolkit/scoring.rs` — Scoring functions (re-export or move)
4. Unit tests for all toolkit read + scoring functions
5. `docs/roadmap/utility-toolkit-autopilot-convergence/session-02-handoff.md`

### Session 02 Exit Criteria

- All toolkit read functions callable with just `&dyn XApiClient`
- Scoring callable with `&ScoringConfig` (no full `Config`)
- Existing tests pass (no regressions)
- CI checklist green: `cargo fmt`, `cargo clippy`, `cargo test`
- New toolkit functions have module-level unit tests

---

## Artifact Inventory

| File | Status |
|------|--------|
| `docs/roadmap/utility-toolkit-autopilot-convergence/charter.md` | Created |
| `docs/roadmap/utility-toolkit-autopilot-convergence/architecture-decisions.md` | Created |
| `docs/roadmap/utility-toolkit-autopilot-convergence/execution-plan.md` | Created |
| `docs/roadmap/utility-toolkit-autopilot-convergence/session-01-handoff.md` | Created (this file) |
