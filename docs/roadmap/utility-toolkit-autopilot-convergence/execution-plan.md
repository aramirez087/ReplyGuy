# Execution Plan: Sessions 02-08

**Status:** Locked (Session 01)
**Date:** 2026-02-26

---

## Overview

Seven sessions transform TuitBot from autopilot-first to utility-first. Each session is self-contained with explicit inputs, outputs, and exit criteria. Sessions within the same phase may share a branch but each session produces a passing CI state.

```
Phase 1: Extract Toolkit    [Sessions 02-03]  ██████░░░░░░░░░░░░░░
Phase 2: Extract Workflow    [Sessions 04-05]  ░░░░░░██████░░░░░░░░
Phase 3: Rewire Consumers    [Sessions 06-07]  ░░░░░░░░░░░░██████░░
Phase 4: Validate & Ship     [Session 08]      ░░░░░░░░░░░░░░░░░░██
```

---

## Session 02 — Toolkit Layer: Read + Scoring

**Mission:** Create `tuitbot-core::toolkit` module with all X API read operations and scoring as standalone functions.

**Inputs:**
- `crates/tuitbot-core/src/lib.rs` — add `pub mod toolkit;`
- `crates/tuitbot-core/src/x_api/mod.rs` — existing `XApiClient` trait (read methods)
- `crates/tuitbot-core/src/scoring/mod.rs` — existing scoring logic
- `crates/tuitbot-mcp/src/tools/workflow/x_actions/read.rs` — current MCP read implementations
- `crates/tuitbot-mcp/src/kernel/read.rs` — kernel read implementations
- `docs/roadmap/utility-toolkit-autopilot-convergence/charter.md` — layer 1 specification

**Tasks:**
1. Create `crates/tuitbot-core/src/toolkit/mod.rs` with public re-exports
2. Create `crates/tuitbot-core/src/toolkit/read.rs` — extract stateless read functions: `search_tweets`, `get_tweet`, `get_user_by_username`, `get_user_by_id`, `get_users_by_ids`, `get_user_mentions`, `get_user_tweets`, `get_home_timeline`, `get_followers`, `get_following`, `get_liked_tweets`, `get_bookmarks`, `get_tweet_liking_users`, `get_me`
3. Create `crates/tuitbot-core/src/toolkit/scoring.rs` — re-export or move scoring logic
4. Define `ToolkitError` enum in `crates/tuitbot-core/src/toolkit/mod.rs` that maps to existing `XApiError` variants and MCP error codes
5. Write unit tests for each toolkit read function using mock `XApiClient`
6. Verify: `cargo test --workspace`, `cargo clippy`, `cargo fmt`

**Outputs:**
- `crates/tuitbot-core/src/toolkit/mod.rs`
- `crates/tuitbot-core/src/toolkit/read.rs`
- `crates/tuitbot-core/src/toolkit/scoring.rs`
- `docs/roadmap/utility-toolkit-autopilot-convergence/session-02-handoff.md`

**Exit criteria:**
- All toolkit read functions are callable with just `&dyn XApiClient`
- Existing tests still pass (no regressions)
- CI checklist green
- Toolkit functions have module-level unit tests

**Risks:**
- `XApiClient` trait methods may return raw API types that need transformation. Mitigate: toolkit functions return the same types; transformation stays in MCP layer.
- Scoring depends on `Config` for weights. Mitigate: scoring functions take `&ScoringConfig` (a subset), not full `&Config`.

**Scope boundary:** Read operations and scoring only. No writes, no engages, no safety checks in this session.

---

## Session 03 — Toolkit Layer: Write + Engage + Safety

**Mission:** Complete the toolkit layer with write, engage, media, and safety check functions.

**Inputs:**
- `crates/tuitbot-core/src/toolkit/` — from Session 02
- `crates/tuitbot-mcp/src/tools/workflow/x_actions/{write,engage,media}.rs` — current MCP implementations
- `crates/tuitbot-mcp/src/kernel/{write,engage,media}.rs` — kernel implementations
- `crates/tuitbot-core/src/safety/` — existing safety module
- `docs/roadmap/utility-toolkit-autopilot-convergence/session-02-handoff.md`

**Tasks:**
1. Create `crates/tuitbot-core/src/toolkit/write.rs` — `post_tweet`, `reply_to_tweet`, `quote_tweet`, `delete_tweet`, `post_thread`
2. Create `crates/tuitbot-core/src/toolkit/engage.rs` — `like_tweet`, `unlike_tweet`, `follow_user`, `unfollow_user`, `retweet`, `unretweet`, `bookmark_tweet`, `unbookmark_tweet`
3. Create `crates/tuitbot-core/src/toolkit/media.rs` — `upload_media`
4. Create `crates/tuitbot-core/src/toolkit/safety.rs` — `check_dedup`, `check_banned_phrases`, `check_per_author_limit`, `check_self_reply_prevention`. These are pure functions taking `&Config` and relevant context.
5. Extend `ToolkitError` for write/engage/media error variants
6. Write unit tests for all new toolkit functions
7. Verify CI checklist

**Outputs:**
- `crates/tuitbot-core/src/toolkit/write.rs`
- `crates/tuitbot-core/src/toolkit/engage.rs`
- `crates/tuitbot-core/src/toolkit/media.rs`
- `crates/tuitbot-core/src/toolkit/safety.rs`
- `docs/roadmap/utility-toolkit-autopilot-convergence/session-03-handoff.md`

**Exit criteria:**
- Complete toolkit layer: 14 read + 6 write + 8 engage + 1 media + scoring + safety functions
- All functions callable with `&dyn XApiClient` (+ `&Config` for safety)
- CI checklist green

**Risks:**
- Write/engage functions in MCP are policy-gated. Mitigate: toolkit write/engage functions are raw (no policy). Policy gate lives in workflow layer. Document this clearly.
- Safety functions may need DB access for dedup history. Mitigate: `check_dedup` takes a `&[String]` of recent reply hashes, not a DB pool. The caller (workflow layer) fetches history.

**Scope boundary:** Toolkit layer complete after this session. No workflow extraction yet.

---

## Session 04 — Workflow Layer: Content Generation + Policy Gate

**Mission:** Create `tuitbot-core::workflow` module with content generation and the centralized policy gate.

**Inputs:**
- `crates/tuitbot-core/src/toolkit/` — complete from Session 03
- `crates/tuitbot-mcp/src/tools/workflow/content.rs` — current MCP content gen
- `crates/tuitbot-mcp/src/tools/workflow/policy_gate.rs` — current MCP policy gate
- `crates/tuitbot-core/src/content/` — existing content generation logic
- `crates/tuitbot-core/src/mcp_policy/` — existing policy framework
- `docs/roadmap/utility-toolkit-autopilot-convergence/session-03-handoff.md`

**Tasks:**
1. Create `crates/tuitbot-core/src/workflow/mod.rs` with `WorkflowCtx` struct and re-exports
2. Define `WorkflowCtx`: `{ db: &DbPool, config: &Config, x_client: &dyn XApiClient, llm: Option<&dyn LlmProvider> }`
3. Create `crates/tuitbot-core/src/workflow/content_gen.rs` — `generate_reply`, `generate_tweet`, `generate_thread`, `suggest_topics`
4. Create `crates/tuitbot-core/src/workflow/policy.rs` — centralized `PolicyGate` that evaluates mutation requests against config rules, rate limits, blocked tools, and hard/user safety rules. Single entry point: `evaluate_mutation(tool_name, params) -> PolicyDecision`
5. Define `WorkflowError` enum
6. Write unit tests
7. Verify CI checklist

**Outputs:**
- `crates/tuitbot-core/src/workflow/mod.rs`
- `crates/tuitbot-core/src/workflow/content_gen.rs`
- `crates/tuitbot-core/src/workflow/policy.rs`
- `docs/roadmap/utility-toolkit-autopilot-convergence/session-04-handoff.md`

**Exit criteria:**
- `WorkflowCtx` established as the standard context object for stateful operations
- Content gen functions callable without MCP transport
- Policy gate is a single, testable function in core
- CI checklist green

**Risks:**
- Content generation currently depends on MCP-specific request types. Mitigate: define workflow-native request types; MCP handlers convert.
- Policy gate depends on DB for rate limit tracking. Mitigate: `PolicyGate` takes `&DbPool` through `WorkflowCtx`.

**Scope boundary:** Content gen + policy only. No analytics, approval, or composite workflows yet.

---

## Session 05 — Workflow Layer: Analytics, Approval, Discovery, Composites

**Mission:** Complete the workflow layer with all remaining stateful operations.

**Inputs:**
- `crates/tuitbot-core/src/workflow/` — from Session 04
- `crates/tuitbot-mcp/src/tools/workflow/{analytics,approval,discovery,context,telemetry}.rs`
- `crates/tuitbot-mcp/src/tools/workflow/composite/`
- `crates/tuitbot-core/src/context/` — existing context module
- `crates/tuitbot-core/src/strategy/` — existing strategy module
- `docs/roadmap/utility-toolkit-autopilot-convergence/session-04-handoff.md`

**Tasks:**
1. Create `crates/tuitbot-core/src/workflow/analytics.rs` — `get_stats`, `follower_trend`, `action_log`, `action_counts`, `reply_count_today`, `x_usage`
2. Create `crates/tuitbot-core/src/workflow/approval.rs` — `list_pending`, `get_pending_count`, `approve_item`, `reject_item`, `approve_all`
3. Create `crates/tuitbot-core/src/workflow/discovery.rs` — `discovery_feed`, `unreplied_tweets`, `target_accounts`
4. Create `crates/tuitbot-core/src/workflow/context.rs` — `author_context`, `recommend_engagement`, `topic_performance`
5. Create `crates/tuitbot-core/src/workflow/composite.rs` — `find_reply_opportunities`, `draft_replies_for_candidates`, `propose_and_queue_replies`, `generate_thread_plan`
6. Create `crates/tuitbot-core/src/workflow/mutation_audit.rs` — `record_mutation`, `recent_mutations`, `mutation_detail`
7. Create `crates/tuitbot-core/src/workflow/drafts.rs` — draft lifecycle functions
8. Write unit tests for each module
9. Verify CI checklist

**Outputs:**
- Complete `crates/tuitbot-core/src/workflow/` module
- `docs/roadmap/utility-toolkit-autopilot-convergence/session-05-handoff.md`

**Exit criteria:**
- All workflow functions callable with `WorkflowCtx` (no MCP transport required)
- Composite workflows (find→draft→queue) testable in isolation
- CI checklist green

**Risks:**
- Approval queue `approve_item` currently executes the approved action (calls X API). Mitigate: `workflow::approval::approve_item` calls `toolkit::write::*` through the toolkit layer.
- Large number of functions to extract. Mitigate: focus on API signatures first, then fill implementations.

**Scope boundary:** Workflow layer complete after this session. No consumer rewiring yet.

---

## Session 06 — Rewire MCP Handlers

**Mission:** Point all MCP tool handlers at the toolkit and workflow layers. MCP handlers become thin adapters: parse params → call toolkit/workflow → wrap response.

**Inputs:**
- `crates/tuitbot-core/src/toolkit/` — complete from Session 03
- `crates/tuitbot-core/src/workflow/` — complete from Session 05
- All files under `crates/tuitbot-mcp/src/tools/`
- `crates/tuitbot-mcp/src/kernel/`
- `docs/roadmap/utility-toolkit-autopilot-convergence/session-05-handoff.md`

**Tasks:**
1. Rewire `tools/workflow/x_actions/read.rs` — each handler calls `core::toolkit::read::*`
2. Rewire `tools/workflow/x_actions/write.rs` — each handler calls `core::toolkit::write::*` (through `workflow::policy` for mutation gating)
3. Rewire `tools/workflow/x_actions/engage.rs` — same pattern as write
4. Rewire `tools/workflow/x_actions/media.rs` — calls `core::toolkit::media::*`
5. Rewire `tools/workflow/analytics.rs` — calls `core::workflow::analytics::*`
6. Rewire `tools/workflow/approval.rs` — calls `core::workflow::approval::*`
7. Rewire `tools/workflow/content.rs` — calls `core::workflow::content_gen::*`
8. Rewire `tools/workflow/discovery.rs` — calls `core::workflow::discovery::*`
9. Rewire `tools/workflow/composite/` — calls `core::workflow::composite::*`
10. Rewire `tools/workflow/policy_gate.rs` — calls `core::workflow::policy::*`
11. Remove duplicated business logic from MCP crate (handlers should be < 30 lines each)
12. Run full test suite including conformance tests, eval harness, boundary tests
13. Verify manifest tool counts are unchanged per profile
14. Verify CI checklist

**Outputs:**
- Simplified `crates/tuitbot-mcp/src/tools/` (thin handlers only)
- `docs/roadmap/utility-toolkit-autopilot-convergence/session-06-handoff.md`

**Exit criteria:**
- Every MCP handler is a thin adapter (param parse → delegate → envelope)
- No business logic in `tuitbot-mcp` crate (only transport/serialization)
- Manifest reports identical tool counts: readonly/14, api-readonly/40, write/104, admin/108
- All conformance tests, eval harness, golden fixture tests pass
- CI checklist green

**Risks:**
- Behavioral differences from refactoring. Mitigate: run eval harness scenarios before and after; diff outputs.
- MCP-specific types (request structs) vs workflow types. Mitigate: convert at the boundary in each handler.

**Scope boundary:** MCP crate only. Autopilot loops untouched.

---

## Session 07 — Rewire Autopilot Loops

**Mission:** Refactor automation loops to call toolkit and workflow functions instead of `XApiClient` directly. Simplify `loop_helpers.rs` traits and `adapters.rs`.

**Inputs:**
- `crates/tuitbot-core/src/toolkit/` — complete
- `crates/tuitbot-core/src/workflow/` — complete
- `crates/tuitbot-core/src/automation/` — all loop files
- `crates/tuitbot-core/src/automation/loop_helpers.rs` — trait definitions
- `crates/tuitbot-core/src/automation/adapters.rs` — trait implementations
- `docs/roadmap/utility-toolkit-autopilot-convergence/session-06-handoff.md`

**Tasks:**
1. Refactor `discovery_loop.rs` — use `toolkit::read::search_tweets` + `toolkit::scoring::score_tweet` + `workflow::content_gen::generate_reply` + `workflow::policy::evaluate_mutation`
2. Refactor `mentions_loop.rs` — use `toolkit::read::get_user_mentions` + `workflow::content_gen::generate_reply`
3. Refactor `content_loop.rs` — use `workflow::content_gen::generate_tweet` + `workflow::policy::evaluate_mutation`
4. Refactor `thread_loop.rs` — use `workflow::content_gen::generate_thread` + `workflow::policy::evaluate_mutation`
5. Refactor `target_loop.rs` — use `toolkit::read::get_user_tweets` + `workflow::*`
6. Refactor `analytics_loop.rs` — use `workflow::analytics::*`
7. Refactor `approval_poster.rs` — use `workflow::approval::*`
8. Simplify `loop_helpers.rs` — remove traits that are now covered by toolkit/workflow functions. Keep only loop-specific traits (e.g., `LoopStorage` for cursor management).
9. Simplify `adapters.rs` — remove implementations that are replaced by toolkit/workflow. This file should shrink significantly.
10. Add CI lint: grep for `x_api::XApiClient` imports in `automation/` modules. Only `adapters.rs` and `mod.rs` (token refresh) may import it.
11. Verify CI checklist

**Outputs:**
- Simplified `crates/tuitbot-core/src/automation/` modules
- Significantly smaller `adapters.rs`
- `docs/roadmap/utility-toolkit-autopilot-convergence/session-07-handoff.md`

**Exit criteria:**
- Zero direct `XApiClient` method calls in loop files (discovery, mentions, content, thread, target, analytics)
- `adapters.rs` reduced by at least 50%
- All automation tests pass
- CI checklist green

**Risks:**
- `loop_helpers.rs` defines many fine-grained traits (`TweetSearcher`, `PostSender`, `ReplyGenerator`, etc.) that loops depend on. Mitigate: keep trait definitions for now but make their implementations delegate to toolkit/workflow. Full trait cleanup is optional in this session.
- `adapters.rs` is 39KB. Mitigate: systematic replacement, one trait at a time.

**Scope boundary:** Autopilot rewiring only. No new features.

---

## Session 08 — Validation, Documentation, Ship

**Mission:** Final validation pass. Sync all documentation. Run full test matrix. Produce final handoff.

**Inputs:**
- Complete refactored codebase from Sessions 02-07
- `docs/architecture.md` — needs updating
- `docs/mcp-reference.md` — verify still accurate
- `CLAUDE.md` — may need architecture section update
- `README.md` — verify still accurate
- `docs/roadmap/utility-toolkit-autopilot-convergence/session-07-handoff.md`

**Tasks:**
1. Run full CI checklist: `cargo fmt --all`, `cargo clippy --workspace -- -D warnings`, `RUSTFLAGS="-D warnings" cargo test --workspace`
2. Run MCP conformance tests: `cargo test -p tuitbot-mcp conformance`
3. Run eval harness: `cargo test -p tuitbot-mcp eval`
4. Run boundary tests: `cargo test -p tuitbot-mcp boundary`
5. Regenerate MCP manifests: `bash scripts/generate-mcp-manifests.sh`
6. Verify manifest tool counts match pre-refactor counts
7. Update `docs/architecture.md` — add three-layer description, update module ownership table
8. Update `CLAUDE.md` — update Architecture section with toolkit/workflow/autopilot layers and module paths
9. Verify `docs/mcp-reference.md` still accurate (should be unchanged)
10. Run `release-plz update --config release-plz.toml --allow-dirty` to verify release readiness
11. Run `cargo package --workspace --allow-dirty` to verify publishability
12. Write final session handoff summarizing the complete initiative

**Outputs:**
- Updated `docs/architecture.md`
- Updated `CLAUDE.md` (if needed)
- Regenerated `docs/generated/mcp-manifest-*.json`
- `docs/roadmap/utility-toolkit-autopilot-convergence/session-08-handoff.md` (final)

**Exit criteria:**
- Full CI green with zero warnings
- All MCP manifests regenerated and committed
- Tool counts per profile unchanged from pre-refactor baseline
- Architecture documentation reflects three-layer model
- `release-plz update` and `cargo package` succeed
- No TBD/TODO items remaining in any roadmap artifact

**Risks:**
- Manifest drift from tool count changes. Mitigate: explicit count verification (readonly=14, api-readonly=40, write=104, admin=108).
- Documentation inconsistencies. Mitigate: read each doc file before updating; diff changes.

**Scope boundary:** Validation and docs only. No new code changes unless fixing test failures.

---

## Dependency Graph

```
Session 02 ──▶ Session 03 ──▶ Session 04 ──▶ Session 05 ──┬──▶ Session 06 ──┐
  (Toolkit       (Toolkit       (Workflow       (Workflow   │    (MCP          │
   reads)         writes)        content+        complete)  │     rewire)      │
                                 policy)                    │                  │
                                                            ├──▶ Session 07 ──┤
                                                            │    (Autopilot   │
                                                            │     rewire)     │
                                                            │                  │
                                                            └────────────────▶ Session 08
                                                                               (Validate)
```

Sessions 06 and 07 are independent of each other (both depend on Session 05). They can run in parallel on separate branches if needed, merged before Session 08.

---

## Risk Controls

| Risk | Probability | Impact | Mitigation |
|------|:-----------:|:------:|------------|
| Behavioral regression in MCP tools | Medium | High | Eval harness + conformance tests before/after each session |
| Automation loop breakage | Medium | High | Existing automation tests + new integration tests in Session 07 |
| `adapters.rs` complexity (39KB) | High | Medium | Systematic trait-by-trait replacement; keep old impls until verified |
| Circular dependencies between layers | Low | High | Module visibility + CI lint for forbidden imports |
| Manifest tool count drift | Low | Medium | Explicit count assertion in Session 08 |
| Session scope creep | Medium | Medium | Hard scope boundaries defined per session; defer discoveries to later sessions |

---

## Baseline Metrics (Pre-Refactor)

Record these before Session 02 begins:

```bash
# Tool counts per profile
tuitbot mcp manifest --format json --profile readonly  | jq '.tool_count'   # Expected: 14
tuitbot mcp manifest --format json --profile api-readonly | jq '.tool_count' # Expected: 40
tuitbot mcp manifest --format json --profile write | jq '.tool_count'       # Expected: 104
tuitbot mcp manifest --format json --profile admin | jq '.tool_count'       # Expected: 108

# Test counts
cargo test --workspace 2>&1 | tail -1    # Record total pass/fail/ignore

# Clippy status
cargo clippy --workspace -- -D warnings  # Must be clean
```
