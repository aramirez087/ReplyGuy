# Session 06: Workflow Composers — MCP Handler Rewiring

**Date:** 2026-02-26
**Session:** 06 of 08
**Branch:** `feat/mcp_final`

---

## Objective

Rebuild MCP composite tools as explicit compositions of toolkit primitives via a
new `tuitbot_core::workflow` module. Each workflow step has typed IO contracts,
and MCP handlers become thin adapters: parse params → delegate to workflow → wrap
response envelope + telemetry.

## Architecture Before → After

### Before (inline logic in MCP handlers)

```
MCP Handler (find_opportunities.rs — 211 lines)
  ├─ search_tweets via state.x_client
  ├─ ScoringEngine::score_and_persist (inline)
  ├─ sort + rank (inline)
  └─ ToolResponse::success(json!({...}))
```

### After (thin adapter → core workflow step)

```
MCP Handler (find_opportunities.rs — 112 lines)
  ├─ Validate prerequisites (X client configured?)
  ├─ core::workflow::discover::execute(db, x_client, config, DiscoverInput)
  └─ ToolResponse::success(output) + telemetry
```

## What Changed

### 1. New `tuitbot_core::workflow` Module (7 files, 1906 lines)

All business logic extracted from MCP composite handlers into core workflow steps
with explicit typed IO. Each step is a standalone async function.

| File | Lines | Purpose |
|------|-------|---------|
| `workflow/mod.rs` | 217 | `WorkflowError`, `SharedProvider`, IO types (`ScoredCandidate`, `DraftResult`, `ProposeResult`, `QueueItem`, `ScoreBreakdown`), helpers |
| `workflow/discover.rs` | 192 | `discover::execute()` — search, score, persist, rank |
| `workflow/draft.rs` | 132 | `draft::execute()` — fetch candidates, generate LLM drafts, safety checks |
| `workflow/queue.rs` | 197 | `queue::execute()` — validate, safety-check, route to approval or execute |
| `workflow/publish.rs` | 49 | `reply()`, `tweet()`, `thread()` — thin wrappers over toolkit writes |
| `workflow/thread_plan.rs` | 127 | `thread_plan::execute()` — LLM thread generation + hook analysis |
| `workflow/orchestrate.rs` | 211 | `run_discovery_cycle()` — deterministic discover → draft → queue |
| `workflow/tests.rs` | 781 | 20 integration tests covering all steps |

### 2. SharedProvider Adapter

`ContentGenerator::new()` requires `Box<dyn LlmProvider>` (owned, `'static`).
Workflow steps receive `&Arc<dyn LlmProvider>` (shared reference). The
`SharedProvider(Arc<dyn LlmProvider>)` adapter bridges the gap:

```rust
pub(crate) struct SharedProvider(pub Arc<dyn LlmProvider>);

impl LlmProvider for SharedProvider {
    // Delegates all calls to inner Arc
}

pub(crate) fn make_content_gen(
    llm: &Arc<dyn LlmProvider>,
    business: &BusinessProfile,
) -> ContentGenerator {
    let provider = Box::new(SharedProvider(Arc::clone(llm)));
    ContentGenerator::new(provider, business.clone())
}
```

### 3. MCP Composite Handlers — Refactored to Thin Adapters

| Handler | Before | After | Reduction |
|---------|--------|-------|-----------|
| `find_opportunities.rs` | 211 lines | 112 lines | -47% |
| `draft_replies.rs` | 155 lines | 97 lines | -37% |
| `propose_queue.rs` | 212 lines | 128 lines | -40% |
| `thread_plan.rs` | 137 lines | 104 lines | -24% |

Each handler now follows the same pattern:
1. Validate prerequisites (X client, LLM provider, non-empty input)
2. Delegate to `core::workflow::<step>::execute(...)`
3. Record telemetry
4. Wrap result in `ToolResponse` envelope with `ToolMeta`

### 4. Shared IO Types Relocated

`ScoredCandidate`, `DraftResult`, `ProposeResult`, `ScoreBreakdown` — canonical
definitions moved to `core::workflow::mod`. Re-exported from
`tuitbot_mcp::tools::workflow::composite` for backward compatibility.

### 5. Deterministic Orchestrator

`core::workflow::orchestrate::run_discovery_cycle()` composes discover → draft →
queue into a single deterministic function. Returns a `CycleReport` with stats:

```rust
pub struct CycleReport {
    pub candidates_found: usize,
    pub candidates_above_threshold: usize,
    pub drafts_generated: usize,
    pub items_queued: usize,
    pub items_posted: usize,
    pub items_blocked: usize,
}
```

### 6. Policy Gate Fix

Fixed `check_policy()` in `policy_gate.rs` to return proper JSON error responses
for all `PolicyDecision` variants (`Deny`, `RouteToApproval`, `DryRun`) instead
of returning empty strings. This restores correct behavior for policy-blocked
mutation scenarios.

## Typed IO Contracts

### DiscoverInput / DiscoverOutput
```rust
pub struct DiscoverInput {
    pub query: Option<String>,
    pub min_score: Option<f64>,
    pub limit: Option<u32>,
    pub since_id: Option<String>,
}

pub struct DiscoverOutput {
    pub candidates: Vec<ScoredCandidate>,
    pub query_used: String,
    pub threshold: f64,
}
```

### DraftInput → Vec<DraftResult>
```rust
pub struct DraftInput {
    pub candidate_ids: Vec<String>,
    pub archetype: Option<String>,
    pub mention_product: bool,
}
```

### QueueInput → Vec<ProposeResult>
```rust
pub struct QueueInput {
    pub items: Vec<QueueItem>,
    pub mention_product: bool,
}
```

### ThreadPlanInput / ThreadPlanOutput
```rust
pub struct ThreadPlanInput {
    pub topic: String,
    pub objective: Option<String>,
    pub target_audience: Option<String>,
    pub structure: Option<String>,
}

pub struct ThreadPlanOutput {
    pub thread_tweets: Vec<String>,
    pub tweet_count: usize,
    pub structure_used: String,
    pub hook_type: String,
    pub first_tweet_preview: String,
    pub estimated_performance: String,
    pub objective_alignment: String,
    pub target_audience: String,
    pub topic_relevance: String,
}
```

## WorkflowError → ErrorCode Mapping

| WorkflowError | ErrorCode |
|---------------|-----------|
| `InvalidInput(_)` | `InvalidInput` |
| `XNotConfigured` | `XNotConfigured` |
| `LlmNotConfigured` | `LlmNotConfigured` |
| `Llm(_)` | `LlmError` |
| `Database(_)` / `Storage(_)` | `DbError` |
| `Toolkit(XApi(_))` | `XApiError` |
| `Toolkit(InvalidInput)` | `InvalidInput` |
| `Toolkit(TweetTooLong)` | `InvalidInput` |

## Test Results

| Metric | Value |
|--------|-------|
| Baseline (session start) | 804 passed |
| Final | 824 passed (+20 workflow integration tests) |
| Flaky (pre-existing) | 1 (`env_var_override_approval_mode`) |

### New Tests (20)

- **discover** (4): happy path, empty results, threshold filtering, default query
- **draft** (3): happy path, candidate not found, empty input validation
- **queue** (4): approval routing, auto-post, empty items, safety block
- **thread_plan** (2): happy path, empty topic validation
- **orchestrate** (2): full cycle, empty discover
- **publish** (2): reply routing, tweet routing
- **error propagation** (3): X not configured, LLM not configured, toolkit error mapping
