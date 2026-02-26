# Session 02 Handoff: Toolkit Core Layer

**Date:** 2026-02-26
**Session:** 02 of 08
**Branch:** `feat/mcp_final`

---

## Completed Work

1. **Created `crates/tuitbot-core/src/toolkit/` module** with 4 submodules:
   - `mod.rs` — `ToolkitError` enum (6 variants), validation helpers, `MAX_TWEET_LENGTH`
   - `read.rs` — 14 stateless read functions + `get_me`, all taking `&dyn XApiClient`
   - `write.rs` — 5 write functions (post, reply, quote, delete, thread) with media support
   - `engage.rs` — 8 engagement functions (like, unlike, follow, unfollow, retweet, unretweet, bookmark, unbookmark)
   - `media.rs` — Media type inference, size validation, upload strategy, raw upload

2. **Refactored all MCP `x_actions` modules** to call toolkit:
   - `read.rs` — Direct toolkit::read calls (removed kernel/provider dependency)
   - `write.rs` — Toolkit::write calls with audited error handling
   - `engage.rs` — Toolkit::engage calls with audited error handling
   - `media.rs` — Toolkit::media for inference, validation, strategy, and upload

3. **Error mapping infrastructure**:
   - `toolkit_error_response` for non-audited reads
   - `audited_toolkit_error_response` for audited mutations

4. **CI checklist green**: 1,336 tests pass, clippy clean, fmt clean

---

## Concrete Decisions Made

| Decision | Summary |
|----------|---------|
| Toolkit scope | Read, write, engage, media — all stateless over `&dyn XApiClient` |
| Error strategy | `ToolkitError` wraps `XApiError` via `#[from]`, helpers map to `ErrorCode` variants |
| MCP read path | Toolkit called directly (kernel/provider bypassed); retry to be re-added in S04-05 |
| MCP mutation path | Policy gate + audit remain in workflow; toolkit is raw call only |
| Media boundary | File I/O, hashing, DB tracking, idempotency stay in MCP workflow; type inference, validation, upload move to toolkit |
| Double validation | MCP fast-fail checks retained as optimization; toolkit validation is authoritative |

---

## Open Issues

1. **Retry behavior removed from reads**: MCP reads previously went through `RetryingProvider`. Now calls `XApiClient` directly. Retry is a workflow concern — re-add in Sessions 04-05 if needed.

2. **Kernel/provider layer partially orphaned**: `kernel/read.rs` and the `SocialReadProvider` chain are no longer called by `x_actions/read.rs`. These should be cleaned up or repurposed in a later session.

---

## Session 03 Inputs

### Files to Read First

1. **`docs/roadmap/utility-toolkit-autopilot-convergence/session-02-handoff.md`** — This file
2. **`docs/roadmap/utility-toolkit-autopilot-convergence/charter.md`** — Section 3.2 (MCP profile model)
3. **`docs/roadmap/utility-toolkit-autopilot-convergence/architecture-decisions.md`** — AD-08 (profile model unchanged)
4. **`crates/tuitbot-mcp/src/state.rs`** — SharedState definition
5. **`crates/tuitbot-mcp/src/lib.rs`** — MCP crate root and wiring
6. **`crates/tuitbot-mcp/src/server/mod.rs`** — Server setup and profile routing
7. **`crates/tuitbot-mcp/src/server/write.rs`** — Write profile tool registration
8. **`crates/tuitbot-mcp/src/server/readonly.rs`** — Readonly profile tool registration
9. **`crates/tuitbot-mcp/src/spec/endpoints.rs`** — Endpoint specifications
10. **`crates/tuitbot-mcp/src/spec/generator.rs`** — Manifest generation
11. **`crates/tuitbot-mcp/src/tools/manifest.rs`** — Tool manifest types
12. **`crates/tuitbot-cli/src/commands/mcp.rs`** — CLI profile selection

### Commands to Run Before Starting

```bash
# Verify baseline is green
cargo fmt --all && cargo fmt --all --check
RUSTFLAGS="-D warnings" cargo test --workspace
cargo clippy --workspace -- -D warnings

# Record baseline test counts
cargo test --workspace 2>&1 | grep "test result"
```

### Session 03 Deliverables

1. New profile routing for utility-readonly and utility-write profiles backed by toolkit tools
2. Spec endpoint → utility tool registration with deterministic naming
3. Structural enforcement that workflow/autopilot tools are excluded from utility profiles
4. Profile-level manifest summaries with count verification in tests
5. `docs/generated/mcp-manifest-utility-readonly.json`
6. `docs/generated/mcp-manifest-utility-write.json`
7. `docs/roadmap/utility-toolkit-autopilot-convergence/session-03-profiles.md`
8. `docs/roadmap/utility-toolkit-autopilot-convergence/session-03-handoff.md`

### Session 03 Exit Criteria

- Utility profiles expose only toolkit-oriented tools
- Boundary tests enforce profile isolation and zero forbidden tool leakage
- Session 04 inputs are explicit in the handoff

---

## Artifact Inventory

| File | Status |
|------|--------|
| `crates/tuitbot-core/src/toolkit/mod.rs` | Created |
| `crates/tuitbot-core/src/toolkit/read.rs` | Created |
| `crates/tuitbot-core/src/toolkit/write.rs` | Created |
| `crates/tuitbot-core/src/toolkit/engage.rs` | Created |
| `crates/tuitbot-core/src/toolkit/media.rs` | Created |
| `crates/tuitbot-core/src/lib.rs` | Modified (added `pub mod toolkit`) |
| `crates/tuitbot-mcp/src/tools/workflow/x_actions/mod.rs` | Modified (added error helpers) |
| `crates/tuitbot-mcp/src/tools/workflow/x_actions/read.rs` | Rewritten (toolkit calls) |
| `crates/tuitbot-mcp/src/tools/workflow/x_actions/write.rs` | Rewritten (toolkit calls) |
| `crates/tuitbot-mcp/src/tools/workflow/x_actions/engage.rs` | Rewritten (toolkit calls) |
| `crates/tuitbot-mcp/src/tools/workflow/x_actions/media.rs` | Rewritten (toolkit calls) |
| `docs/roadmap/.../session-02-toolkit-core.md` | Created |
| `docs/roadmap/.../session-02-handoff.md` | Created (this file) |
