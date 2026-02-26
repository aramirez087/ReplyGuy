# Architecture Charter: Utility Toolkit + Autopilot Convergence

**Status:** Approved (Session 01)
**Date:** 2026-02-26
**Author:** Session 01 architect agent

---

## 1. Mission

Transform TuitBot from an autopilot-first system into a **utility-first toolkit** where every X API operation is a standalone, composable function — while preserving autopilot as an optional orchestration layer on top.

**Before:** MCP server exposes 109 tools, but toolkit operations (search tweets, post tweet, get user) are only accessible through MCP transport or buried inside automation loops.

**After:** A clean three-layer architecture where:
- Layer 1 (Toolkit) provides stateless X/API utilities usable by any consumer
- Layer 2 (Workflow) composes toolkit operations with DB/LLM state
- Layer 3 (Autopilot) schedules and orchestrates workflows on a timer

---

## 2. Gap Analysis: Current State vs Utility Toolkit

### 2.1 Current Architecture

```
┌─────────────────────────────────────────────────────────────┐
│  tuitbot-mcp (MCP server)                                   │
│  ┌──────────────────┐  ┌──────────────────────────────────┐ │
│  │ tools/ (shared)   │  │ tools/workflow/                  │ │
│  │ - config          │  │ - x_actions/ (read/write/engage) │ │
│  │ - scoring         │  │ - composite/ (find/draft/queue)  │ │
│  │ - response        │  │ - analytics, approval, content   │ │
│  │ - manifest        │  │ - policy_gate, discovery, ...    │ │
│  └──────────────────┘  └──────────────────────────────────┘ │
│          │                          │                        │
│          ▼                          ▼                        │
│  ┌──────────────────────────────────────────────────────────┐│
│  │  tuitbot-core                                            ││
│  │  ┌─────────┐ ┌─────────┐ ┌──────────┐ ┌──────────────┐ ││
│  │  │ x_api/  │ │ llm/    │ │ storage/ │ │ automation/  │ ││
│  │  │ Client  │ │ Provider│ │ SQLite   │ │ Runtime+Loops│ ││
│  │  └─────────┘ └─────────┘ └──────────┘ └──────────────┘ ││
│  └──────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────┘
```

### 2.2 Identified Gaps

| # | Gap | Impact | Severity |
|---|-----|--------|----------|
| G1 | **Toolkit ops trapped in MCP** — X API reads/writes/engages are implemented as MCP tool handlers in `tuitbot-mcp/src/tools/workflow/x_actions/`. They cannot be called programmatically from core or CLI without going through MCP transport. | Non-MCP consumers (CLI commands, HTTP server, tests) must reimplement X API call patterns or use raw `XApiClient` directly. | High |
| G2 | **Automation loops bypass any shared toolkit** — Loops in `core/automation/` call `XApiClient` methods directly (via trait objects injected through `loop_helpers` traits like `TweetSearcher`, `PostSender`). No intermediate toolkit layer provides scoring, safety checks, or rate-limit-aware wrappers. | Safety logic (dedup, banned phrases, per-author limits) is scattered across loop implementations rather than centralized in reusable toolkit functions. | High |
| G3 | **Composite workflows are MCP-only** — `find_reply_opportunities`, `draft_replies_for_candidates`, `propose_and_queue_replies`, `generate_thread_plan` live in `tuitbot-mcp/src/tools/workflow/composite/`. They cannot be reused from the HTTP server, CLI, or other consumers. | The HTTP server (`tuitbot-server`) must reimplement similar logic or expose different APIs for the same operations. | Medium |
| G4 | **No stateless execution path** — Even simple utility operations (search tweets, get user) require initializing `AppState` with DB pool, LLM provider, and config. The readonly MCP profiles work around this with `ReadonlyState`, but this is MCP-specific. | Cannot use toolkit operations in lightweight contexts (shell scripts, CI, simple API calls) without full initialization. | Medium |
| G5 | **Policy gate is MCP-specific** — The policy enforcement logic in `tools/workflow/policy_gate.rs` is only invoked from MCP tool handlers. The automation loops have their own separate safety checks in `core/safety/`. | Two parallel safety systems with different rules, making it hard to guarantee consistent mutation safety. | High |
| G6 | **Scoring lives in core but is wrapped in MCP** — `core/scoring/` is already stateless and clean, but the MCP `score_tweet` tool adds its own parameter parsing and response wrapping. This is correct but the pattern is not followed consistently. | Inconsistent: some operations are clean in core (scoring), others exist only in MCP (composite workflows). | Low |

### 2.3 What Already Works Well

- **`XApiClient` trait** (`core/x_api/mod.rs`): Clean async trait with 25+ methods. Supports mocking via `async_trait`. This is the correct foundation for the toolkit layer.
- **`LlmProvider` trait** (`core/llm/mod.rs`): Clean provider abstraction with factory pattern.
- **MCP response envelope** (`contract/envelope.rs`): Consistent `{success, data, error, meta}` contract across all 109 tools.
- **MCP profile model** (`state.rs`): Four profiles with clean separation — readonly profiles are safe by construction.
- **Manifest system** (`tools/manifest.rs`): Declarative tool registry with categories, lanes, mutation flags, and profile assignments.
- **Spec pack** (`spec/`): Generated tools from `EndpointDef` specifications — clean code generation pipeline.
- **Error taxonomy**: 28 typed error codes with retryable/policy metadata.

---

## 3. Target Architecture: Three Layers

```
┌──────────────────────────────────────────────────────────────────┐
│  Consumers: MCP Server, HTTP Server, CLI, Tests                  │
│                                                                  │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │  Layer 3: AUTOPILOT  (core::automation/)                   │  │
│  │  Runtime, LoopScheduler, discovery_loop, mentions_loop,    │  │
│  │  content_loop, thread_loop, target_loop, analytics_loop,   │  │
│  │  posting_queue, approval_poster, circuit_breaker            │  │
│  │  ─────────────────────────────────────────────────────────  │  │
│  │  Calls: Workflow layer only. Never calls XApiClient.        │  │
│  └────────────────────────────────────────────────────────────┘  │
│                              │ uses                               │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │  Layer 2: WORKFLOW  (core::workflow/)                       │  │
│  │  content_gen, approval_queue, analytics, discovery_feed,    │  │
│  │  context_intel, composite (find→draft→queue), strategy,     │  │
│  │  policy_gate, mutation_audit                                │  │
│  │  ─────────────────────────────────────────────────────────  │  │
│  │  Requires: DB + optional LLM. Calls Toolkit layer only.     │  │
│  └────────────────────────────────────────────────────────────┘  │
│                              │ uses                               │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │  Layer 1: TOOLKIT  (core::toolkit/)                        │  │
│  │  x_read, x_write, x_engage, x_media, scoring,              │  │
│  │  safety_checks (dedup, banned phrases, per-author limits)   │  │
│  │  ─────────────────────────────────────────────────────────  │  │
│  │  Stateless over XApiClient trait. No DB, no LLM required.   │  │
│  └────────────────────────────────────────────────────────────┘  │
│                              │ uses                               │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │  Foundation: x_api::XApiClient trait, storage, llm, config  │  │
│  └────────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────────┘
```

### 3.1 Layer 1 — Toolkit (`tuitbot-core::toolkit`)

**Purpose:** Stateless X/API utility functions. Every operation takes an `&dyn XApiClient` (and optionally `&Config` for scoring weights) and returns typed results. No DB, no LLM.

**Module structure:**

```
core/src/toolkit/
├── mod.rs              # Public API re-exports
├── read.rs             # search_tweets, get_tweet, get_user, get_mentions, ...
├── write.rs            # post_tweet, reply_to_tweet, quote_tweet, delete_tweet, post_thread
├── engage.rs           # like, unlike, follow, unfollow, retweet, unretweet, bookmark, unbookmark
├── media.rs            # upload_media
├── scoring.rs          # score_tweet (moved from core/scoring/, or re-exported)
└── safety.rs           # dedup_check, banned_phrase_check, per_author_limit_check
```

**Key properties:**
- Every function is `pub async fn` taking `&dyn XApiClient` as first arg
- Returns `Result<T, ToolkitError>` where `ToolkitError` maps cleanly to MCP error codes
- Scoring functions take `&Config` for weight configuration
- Safety functions take `&Config` for rule configuration
- No `Arc`, no `Pool`, no `AppState` — pure functions over trait objects

**What moves here:**
- The X API call logic currently in `tuitbot-mcp/src/tools/workflow/x_actions/{read,write,engage,media}.rs`
- The scoring logic currently in `tuitbot-core/src/scoring/`
- Safety check functions currently scattered in `core/safety/` and `core/automation/loop_helpers.rs`

### 3.2 Layer 2 — Workflow (`tuitbot-core::workflow`)

**Purpose:** Stateful composite operations that combine toolkit functions with DB and LLM. These are the reusable building blocks that both MCP and autopilot consume.

**Module structure:**

```
core/src/workflow/
├── mod.rs              # Public API re-exports
├── content_gen.rs      # generate_reply, generate_tweet, generate_thread, suggest_topics
├── approval.rs         # list_pending, approve, reject, approve_all
├── analytics.rs        # get_stats, follower_trend, action_log, action_counts, x_usage
├── discovery.rs        # discovery_feed, unreplied_tweets, target_accounts
├── context.rs          # author_context, engagement_recommendation, topic_performance
├── composite.rs        # find_reply_opportunities, draft_replies, propose_and_queue
├── strategy.rs         # weekly_report (re-export from core/strategy/)
├── policy.rs           # PolicyGate — centralized mutation policy evaluation
├── mutation_audit.rs   # Record and query mutation audit trail
└── drafts.rs           # Draft lifecycle: create, schedule, post
```

**Key properties:**
- Functions take a `WorkflowCtx` struct: `{ db: &DbPool, config: &Config, toolkit: &dyn XApiClient, llm: Option<&dyn LlmProvider> }`
- Returns `Result<T, WorkflowError>` where `WorkflowError` maps to MCP error codes
- Never calls `XApiClient` directly — always through `toolkit::*` functions
- Policy evaluation is centralized here — one path for all mutation safety

**What moves here:**
- Composite workflow logic from `tuitbot-mcp/src/tools/workflow/composite/`
- Content generation orchestration from `tuitbot-mcp/src/tools/workflow/content.rs`
- Approval queue logic from `tuitbot-mcp/src/tools/workflow/approval.rs`
- Analytics query logic from `tuitbot-mcp/src/tools/workflow/analytics.rs`
- Policy gate from `tuitbot-mcp/src/tools/workflow/policy_gate.rs`

### 3.3 Layer 3 — Autopilot (`tuitbot-core::automation` — refactored)

**Purpose:** Scheduled orchestration. Runs background loops that invoke workflow and toolkit functions on a timer with jitter, circuit breaking, and graceful shutdown.

**Module structure:** (unchanged file layout, refactored internals)

```
core/src/automation/
├── mod.rs              # Runtime, spawn, shutdown, token refresh
├── scheduler.rs        # LoopScheduler (unchanged)
├── schedule.rs         # schedule_gate, ActiveSchedule (unchanged)
├── circuit_breaker.rs  # CircuitBreaker (unchanged)
├── posting_queue.rs    # PostAction queue (unchanged API, uses workflow internally)
├── approval_poster.rs  # Approval poster (uses workflow::approval)
├── status_reporter.rs  # StatusQuerier (unchanged)
├── loop_helpers.rs     # Trait definitions — simplified, delegates to toolkit/workflow
├── adapters.rs         # Trait impls — thin wrappers calling toolkit/workflow functions
├── discovery_loop.rs   # Uses toolkit::read + toolkit::scoring + workflow::*
├── mentions_loop.rs    # Uses toolkit::read + workflow::content_gen
├── content_loop.rs     # Uses workflow::content_gen + workflow::policy
├── thread_loop.rs      # Uses workflow::content_gen + workflow::policy
├── target_loop.rs      # Uses toolkit::read + workflow::*
└── analytics_loop.rs   # Uses workflow::analytics
```

**Key constraint:** Autopilot modules never import `x_api::XApiClient` directly. All X API access goes through `toolkit::*` functions. All stateful operations go through `workflow::*` functions.

### 3.4 MCP Layer (unchanged crate boundary, simplified internals)

**Purpose:** MCP transport, parameter parsing, response envelope wrapping. Thin adapter over toolkit and workflow layers.

**What changes:**
- `tools/workflow/x_actions/{read,write,engage,media}.rs` become thin wrappers calling `core::toolkit::*`
- `tools/workflow/composite/` becomes thin wrappers calling `core::workflow::composite::*`
- `tools/workflow/{analytics,approval,content,discovery}.rs` become thin wrappers calling `core::workflow::*`
- `tools/workflow/policy_gate.rs` becomes a thin wrapper calling `core::workflow::policy::*`

**What stays:**
- Server structs (`WriteMcpServer`, `AdminMcpServer`, etc.)
- Profile model (4 profiles)
- Response envelope (`contract/`)
- Spec pack (`spec/`)
- Manifest system (`tools/manifest.rs`)
- Idempotency store (MCP-transport-specific)
- Parameter parsing and JSON schema generation
- Test infrastructure (conformance, eval harness, golden fixtures)

---

## 4. MCP Profile Model (Target)

The existing four-profile model maps cleanly to the three-layer architecture:

| Profile | Toolkit (L1) | Workflow (L2) | Autopilot (L3) | Universal Request |
|---------|:------------:|:-------------:|:--------------:|:-----------------:|
| **readonly** | Reads + Scoring | - | - | - |
| **api-readonly** | All Reads + Scoring | - | - | - |
| **write** | All | All | - | - |
| **admin** | All | All | - | x_get/x_post/x_put/x_delete |

**No profile changes required.** The profile model is already correct. The architectural change is that profiles now map to clean layer boundaries rather than ad-hoc tool groupings.

**Tool family → Layer mapping:**

| MCP Tool Family | Layer | Profile(s) |
|-----------------|-------|-----------|
| X Read tools (14) | Toolkit | all four |
| X Write tools (6) | Toolkit | write, admin |
| X Engage tools (8) | Toolkit | write, admin |
| X Media tools (1) | Toolkit | write, admin |
| Scoring (1) | Toolkit | all four |
| Config/Health/Meta (5) | Toolkit | varies |
| Analytics (7) | Workflow | write, admin |
| Approval (5) | Workflow | write, admin |
| Content Gen (4) | Workflow | write, admin |
| Discovery (3) | Workflow | write, admin |
| Context Intelligence (3) | Workflow | write, admin |
| Composite (4) | Workflow | write, admin |
| Policy/Telemetry (4) | Workflow | write, admin |
| Generated spec-pack (36) | Toolkit | varies |
| Universal request (4) | Toolkit (escape hatch) | admin only |

---

## 5. Dependency Rules (Enforced)

```
Autopilot ──uses──▶ Workflow ──uses──▶ Toolkit ──uses──▶ XApiClient trait
     │                   │                                  ▲
     │                   │                                  │
     ▼                   ▼                                  │
  (scheduler,         (DB, LLM,                      (HTTP client,
   circuit breaker,    config)                         OAuth tokens)
   cancellation)
```

**Hard rules:**
1. Toolkit MUST NOT import from `workflow::` or `automation::`
2. Workflow MUST NOT import from `automation::`
3. Workflow MUST NOT use `XApiClient` directly — only through `toolkit::*`
4. Autopilot MUST NOT use `XApiClient` directly — only through `toolkit::*` or `workflow::*`
5. MCP tool handlers MUST NOT contain business logic — only parameter parsing + envelope wrapping + delegation to toolkit/workflow

**Enforcement:** Module visibility (`pub(crate)` on internal items) + CI lint that greps for forbidden imports.

---

## 6. Migration Strategy

This is a **refactor, not a rewrite**. Existing functionality is preserved. The change is where code lives, not what it does.

**Phase 1 (Sessions 02-03):** Extract Toolkit layer from existing code.
**Phase 2 (Sessions 04-05):** Extract Workflow layer, centralize policy gate.
**Phase 3 (Sessions 06-07):** Rewire Autopilot loops and MCP handlers.
**Phase 4 (Session 08):** Validation, docs sync, final CI pass.

See `execution-plan.md` for the detailed session breakdown.

---

## 7. Success Criteria

1. All 109 MCP tools continue to work with identical behavior (no regressions)
2. Every toolkit function is callable without DB or LLM initialization
3. Every workflow function is callable without MCP transport
4. Autopilot loops contain zero direct `XApiClient` method calls
5. Policy evaluation has a single code path for all mutation sources (MCP, automation, CLI)
6. `cargo test --workspace` passes with no new warnings
7. The manifest system reports identical tool counts per profile
8. Response envelope contract is unchanged
