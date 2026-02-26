# Architecture Decisions: Utility Toolkit + Autopilot Convergence

**Status:** Locked (Session 01)
**Date:** 2026-02-26

All decisions below are non-negotiable for this initiative. Each was made with full context of the current codebase and explicit rationale.

---

## AD-01: Module-Level Layering, Not Crate-Level

**Decision:** The three layers (Toolkit, Workflow, Autopilot) are top-level modules within `tuitbot-core`, not separate crates.

**Rationale:**
- The existing 4-crate workspace (`core`, `cli`, `mcp`, `server`) is well-designed. Adding 2-3 crates would fragment the build graph and complicate dependency management.
- Module-level visibility (`pub(crate)`) provides sufficient encapsulation. Rust's module system enforces access boundaries at compile time.
- Intra-crate refactoring is vastly simpler than inter-crate refactoring (no Cargo.toml version coordination, no circular dependency risk).
- The layers share types from `core::error`, `core::config`, and `core::x_api` — same-crate access is cleaner.

**Alternatives rejected:**
- Separate crates (`tuitbot-toolkit`, `tuitbot-workflow`): Adds release coordination overhead for no meaningful encapsulation benefit. The `tuitbot-mcp` crate already depends on `tuitbot-core` — adding intermediate crates would create a deeper dependency tree.
- Feature flags per layer: Overly clever. Feature flags are for optional dependencies, not architectural boundaries.

---

## AD-02: Toolkit Layer Is Stateless Over `&dyn XApiClient`

**Decision:** Every toolkit function takes `&dyn XApiClient` as its first parameter. No `Arc`, no `AppState`, no DB pool.

**Rationale:**
- Statelessness is the core value proposition of the toolkit layer. It makes functions testable with mock clients, composable without initialization, and usable from any context (MCP, CLI, tests, WASM).
- `XApiClient` is already a well-defined async trait with 25+ methods. It's the correct abstraction boundary.
- Scoring functions additionally take `&ScoringConfig` (a subset of `Config`) — this is acceptable because scoring weights are pure configuration, not state.
- Safety check functions take `&Config` and context data (not DB) — the caller fetches dedup history and passes it in.

**Concrete signatures:**

```rust
// Toolkit read
pub async fn search_tweets(
    client: &dyn XApiClient,
    query: &str,
    max_results: Option<u32>,
    since_id: Option<&str>,
) -> Result<Vec<Tweet>, ToolkitError>;

// Toolkit write (raw, no policy)
pub async fn post_tweet(
    client: &dyn XApiClient,
    text: &str,
    media_ids: Option<&[String]>,
) -> Result<Tweet, ToolkitError>;

// Toolkit scoring
pub fn score_tweet(
    tweet: &ScoringInput,
    config: &ScoringConfig,
) -> ScoreResult;

// Toolkit safety (pure function, caller provides context)
pub fn check_dedup(
    text: &str,
    recent_hashes: &[String],
    threshold: f64,
) -> DedupResult;
```

**Alternatives rejected:**
- Passing `Arc<dyn XApiClient>`: Unnecessary ownership semantics for stateless functions. Borrows are sufficient.
- Passing `&Config` to all functions: Only scoring and safety need config. Read/write/engage functions don't.

---

## AD-03: Workflow Layer Uses `WorkflowCtx` Struct

**Decision:** All workflow functions take a `WorkflowCtx` reference that bundles DB pool, config, X client, and optional LLM provider.

**Rationale:**
- Workflow functions need 3-4 dependencies (DB, config, client, LLM). Passing them individually to every function creates unwieldy signatures.
- A context struct is explicit — it documents exactly what a workflow function needs. No hidden globals.
- The context is borrowed (`&WorkflowCtx`), not owned — no ownership contention.
- `llm` is `Option<&dyn LlmProvider>` because content generation is optional (analytics, approval, discovery work without LLM).

**Concrete definition:**

```rust
pub struct WorkflowCtx<'a> {
    pub db: &'a sqlx::SqlitePool,
    pub config: &'a Config,
    pub x_client: &'a dyn XApiClient,
    pub llm: Option<&'a dyn LlmProvider>,
}
```

**Alternatives rejected:**
- Individual parameters: Too many params per function (4+), violates readability.
- Trait object (`&dyn WorkflowServices`): Over-abstraction. The concrete struct is sufficient.
- `Arc<AppState>` reuse: `AppState` is MCP-specific (includes `IdempotencyStore`, `granted_scopes`). Workflow doesn't need those.

---

## AD-04: Toolkit Write/Engage Functions Are Raw (No Policy)

**Decision:** Toolkit layer write and engage functions execute operations directly against the X API. They do not enforce policy rules, rate limits, or approval routing.

**Rationale:**
- The toolkit layer is stateless by design (AD-02). Policy evaluation requires DB access (rate limit counters, approval queue), which violates statelessness.
- Consumers that need policy enforcement (MCP handlers, automation loops) call `workflow::policy::evaluate_mutation()` before calling toolkit write functions. This is explicit and auditable.
- Raw toolkit functions are useful for testing, debugging, and admin operations where policy bypass is intentional.
- The admin MCP profile's universal request tools (`x_get`, `x_post`, etc.) already bypass policy — this is consistent.

**Policy enforcement flow:**

```
MCP handler → workflow::policy::evaluate_mutation() → toolkit::write::post_tweet()
                      │                                        │
                      ▼                                        ▼
              (rate limit check,                        (raw X API call)
               approval routing,
               blocked tool check)
```

**Alternatives rejected:**
- Policy in toolkit layer: Requires DB, violates statelessness, creates circular deps.
- Policy as middleware/decorator: Clever but opaque. Explicit call chain is more debuggable.

---

## AD-05: Centralized Policy Gate in Workflow Layer

**Decision:** All mutation safety (MCP policy rules, rate limits, blocked tools, approval routing, safety rules) is evaluated by a single `workflow::policy::PolicyGate` struct.

**Rationale:**
- Currently, policy enforcement lives in two places: `tuitbot-mcp/tools/workflow/policy_gate.rs` (MCP-specific) and `tuitbot-core/safety/` (automation-specific). This dual-path creates risk of inconsistent enforcement.
- A single `PolicyGate` in the workflow layer serves all consumers: MCP handlers, automation loops, CLI commands, HTTP server endpoints.
- The existing `core::mcp_policy/` module (rules, evaluator, types) already contains the policy logic — `PolicyGate` wraps it with a clean interface and adds safety checks from `core::safety/`.

**Interface:**

```rust
pub struct PolicyGate<'a> {
    ctx: &'a WorkflowCtx<'a>,
}

impl PolicyGate<'_> {
    pub async fn evaluate_mutation(
        &self,
        tool_name: &str,
        params: &serde_json::Value,
    ) -> Result<PolicyDecision, WorkflowError>;
}

pub enum PolicyDecision {
    Allow,
    RouteToApproval { queue_id: i64, reason: String },
    DryRun { would_execute: String, params: String },
    Deny { code: ErrorCode, message: String },
}
```

**Alternatives rejected:**
- Keep separate policy paths for MCP and automation: Inconsistent enforcement is a safety risk.
- Policy as a trait: Only one implementation exists. A trait adds abstraction without benefit.

---

## AD-06: Autopilot Never Calls `XApiClient` Directly

**Decision:** All files under `core/automation/` (except `mod.rs` for token refresh and `adapters.rs` for trait impl wiring) are forbidden from importing `x_api::XApiClient` or calling its methods.

**Rationale:**
- This is the fundamental architectural invariant of the three-layer model. If autopilot loops bypass the toolkit layer, the refactoring provides no value.
- Token refresh in `mod.rs` is the only exception — it needs raw client access to update bearer tokens. This is infrastructure, not business logic.
- `adapters.rs` provides trait implementations that bridge the loop helper traits to toolkit/workflow functions. It may reference `XApiClient` to construct toolkit calls.

**Enforcement:**
- Session 07 adds a CI lint script that greps for `XApiClient` imports in automation modules.
- Code review gate: any PR touching `automation/` must verify this constraint.

**Alternatives rejected:**
- Allow toolkit bypass for "simple" operations: Creates precedent erosion. If one loop bypasses, others will follow.
- Remove the constraint for read operations: Reads should still go through toolkit for consistent error handling and future instrumentation (telemetry, caching).

---

## AD-07: MCP Response Envelope Contract Unchanged

**Decision:** The MCP response envelope `{ success, data, error, meta }` is preserved exactly as-is. No field additions, removals, or type changes.

**Rationale:**
- Existing MCP clients (Claude Code configurations, custom agents) depend on this contract. Breaking it would require coordinated client updates.
- The envelope is already well-designed with error codes, retryable flags, rate limit reset timestamps, and policy decision metadata.
- The internal refactoring should be invisible to MCP consumers.

**Concrete contract (preserved):**

```json
{
  "success": true|false,
  "data": <any>,
  "error": { "code": "<ErrorCode>", "message": "...", "retryable": true|false, ... },
  "meta": { "tool_version": "1.0", "elapsed_ms": <u64>, "mode": "...", "approval_mode": true|false }
}
```

**Alternatives rejected:**
- Add a `layer` field to meta: Leaks internal architecture to consumers. No benefit.
- Bump envelope version: No external-facing change justifies a version bump.

---

## AD-08: MCP Profile Model Unchanged

**Decision:** The four MCP profiles (readonly, api-readonly, write, admin) and their tool assignments are preserved exactly.

**Rationale:**
- The profiles already map cleanly to the three-layer model (see charter.md section 4).
- Profile documentation, CLI help text, and user configurations reference these profiles.
- Renaming or restructuring profiles would be a breaking change with no functional benefit.
- Tool counts per profile remain: readonly=14, api-readonly=40, write=104, admin=108.

**Alternatives rejected:**
- Add a "toolkit" profile: Overlaps with "api-readonly". Users don't think in terms of internal layers.
- Rename "write" to "full": Breaking change, no benefit.
- Merge api-readonly into readonly: They serve different use cases (minimal vs broad read access).

---

## AD-09: Existing `loop_helpers.rs` Traits Retained (Simplified)

**Decision:** The fine-grained traits in `loop_helpers.rs` (`TweetSearcher`, `PostSender`, `ReplyGenerator`, `SafetyChecker`, etc.) are retained but their implementations in `adapters.rs` are simplified to delegate to toolkit/workflow functions.

**Rationale:**
- Loops depend on these traits for dependency injection and testing. Removing them would require rewriting all loop files and their test mocks simultaneously — too much scope for a single session.
- The traits serve a valid purpose: they allow loop unit tests to inject mock behavior without full system initialization.
- Simplification happens in `adapters.rs`: instead of reimplementing X API calls, adapters call `toolkit::*` functions. This achieves the architectural goal (no direct XApiClient in loops) while preserving the testing interface.
- Full trait removal/consolidation can be a follow-up initiative once the three-layer model is stable.

**Alternatives rejected:**
- Remove all loop_helpers traits: Too much simultaneous churn. Risky.
- Replace traits with direct toolkit/workflow calls: Breaks loop unit testability.

---

## AD-10: `ToolkitError` and `WorkflowError` Map to Existing `ErrorCode`

**Decision:** New error types (`ToolkitError`, `WorkflowError`) defined in the toolkit and workflow layers map 1:1 to the existing 28 `ErrorCode` variants in the MCP contract.

**Rationale:**
- The MCP error taxonomy is comprehensive and already covers all error scenarios.
- MCP handlers need to convert layer errors to MCP error codes. A 1:1 mapping makes this trivial (`From` impl).
- Keeping error semantics aligned prevents lossy translation at layer boundaries.

**Concrete approach:**

```rust
// In core::toolkit
pub enum ToolkitError {
    XRateLimited { reset: Option<DateTime<Utc>> },
    XAuthExpired,
    XForbidden,
    XNetworkError(String),
    XNotConfigured,
    XApiError(String),
    TweetTooLong { len: usize },
    InvalidInput(String),
    UnsupportedMediaType(String),
    FileReadError(String),
    MediaUploadError(String),
    ScraperMutationBlocked,
}

// In tuitbot-mcp: trivial conversion
impl From<ToolkitError> for ErrorCode { ... }
```

**Alternatives rejected:**
- Reuse `XApiError` directly: `XApiError` is tightly coupled to the HTTP client. Toolkit errors include validation and safety errors that aren't X API errors.
- New error taxonomy: Would require updating MCP error code documentation. No benefit.

---

## AD-11: No Backward Compatibility Layers

**Decision:** Per operator rules, no aliases, migration shims, deprecated re-exports, or `// removed` comments are created during this refactoring.

**Rationale:**
- This is an internal refactoring. External interfaces (MCP tools, CLI commands, HTTP API) are unchanged.
- Internal consumers (automation loops, MCP handlers) are updated in the same sessions that create the new layers.
- Keeping old code paths alongside new ones creates confusion and maintenance burden.
- If something is moved, the old location is deleted in the same commit.

**Alternatives rejected:**
- Deprecation period: No external API changes, so no deprecation needed.
- Re-export from old locations: Creates phantom modules that confuse code navigation.

---

## AD-12: Safety Checks Split Between Toolkit and Workflow

**Decision:** Stateless safety checks (dedup similarity, banned phrase matching, tweet length validation) live in `toolkit::safety`. Stateful safety checks (per-author daily limits, rate limit counters, policy rule evaluation) live in `workflow::policy`.

**Rationale:**
- Dedup similarity is a pure function: given text and a list of recent hashes, return pass/fail. No DB needed.
- Banned phrase matching is a pure function: given text and a config blocklist, return pass/fail.
- Per-author daily limits require DB queries (how many replies to this author today?). This is stateful.
- Rate limit tracking requires DB writes. This is stateful.
- The split follows the stateless/stateful boundary that defines the toolkit/workflow separation.

**Concrete allocation:**

| Safety Check | Layer | Reason |
|---|---|---|
| Dedup (Jaccard similarity) | Toolkit | Pure function; caller provides recent hashes |
| Banned phrase filter | Toolkit | Pure function; config provides blocklist |
| Tweet length validation | Toolkit | Pure function; 280 char limit |
| Self-reply prevention | Toolkit | Pure function; compare author IDs |
| Per-author daily limit | Workflow | Requires DB: count today's replies to author |
| Hourly mutation rate limit | Workflow | Requires DB: count hour's mutations |
| Blocked tools list | Workflow | Config-based but evaluated in policy context |
| Hard safety rules | Workflow | Evaluated in policy context with DB |
| User-configured rules | Workflow | Evaluated in policy context with DB |

---

## AD-13: Spec Pack and Generated Tools Stay in MCP Crate

**Decision:** The spec pack (`tuitbot-mcp/src/spec/`) and its generated tools (36 endpoint tools from `EndpointDef`) remain in the MCP crate. They are not moved to core.

**Rationale:**
- Generated tools are MCP-specific: they produce MCP tool entries with JSON schemas, profile assignments, and error code metadata.
- The `EndpointDef → ToolEntry` pipeline is tightly coupled to MCP manifest types.
- Generated tools call through to the same `XApiClient` methods that toolkit functions wrap. Moving the generation pipeline to core would create a dependency on MCP manifest types in core.
- The generated tools' runtime handlers will call through toolkit functions (same as hand-crafted MCP handlers).

**Alternatives rejected:**
- Move spec pack to core: Creates dependency on MCP manifest types in core. Wrong direction.
- Generate toolkit functions from spec: Over-engineering. Hand-crafted toolkit functions with clear signatures are more maintainable than generated ones.

---

## AD-14: `WorkflowCtx` Uses Borrowed References, Not `Arc`

**Decision:** `WorkflowCtx` holds borrowed references (`&DbPool`, `&Config`, etc.), not `Arc<DbPool>`, `Arc<Config>`, etc.

**Rationale:**
- Workflow functions are called from contexts that already own the dependencies (MCP `AppState`, automation `Runtime` closure captures, HTTP handler state).
- Borrowing avoids unnecessary reference counting overhead.
- Lifetime annotations make dependency requirements explicit at compile time.
- If a workflow function needs to spawn a task that outlives the call, it can clone the `Arc` from the caller's state — but this is the exception, not the rule.

**Concrete lifetime:**

```rust
pub struct WorkflowCtx<'a> {
    pub db: &'a sqlx::SqlitePool,
    pub config: &'a Config,
    pub x_client: &'a dyn XApiClient,
    pub llm: Option<&'a dyn LlmProvider>,
}
```

**Alternatives rejected:**
- `Arc`-based context: Unnecessary overhead; borrowed refs are sufficient for synchronous call chains.
- Static lifetime: Would require all deps to be `'static`, overly restrictive.

---

## Decision Index

| ID | Summary | Layer |
|----|---------|-------|
| AD-01 | Module-level layering, not crate-level | All |
| AD-02 | Toolkit is stateless over `&dyn XApiClient` | Toolkit |
| AD-03 | Workflow uses `WorkflowCtx` struct | Workflow |
| AD-04 | Toolkit writes are raw (no policy) | Toolkit |
| AD-05 | Centralized policy gate in workflow | Workflow |
| AD-06 | Autopilot never calls XApiClient directly | Autopilot |
| AD-07 | MCP response envelope unchanged | MCP |
| AD-08 | MCP profile model unchanged | MCP |
| AD-09 | Loop helper traits retained, impls simplified | Autopilot |
| AD-10 | Error types map to existing ErrorCode | All |
| AD-11 | No backward compatibility layers | All |
| AD-12 | Safety split: stateless in toolkit, stateful in workflow | Toolkit + Workflow |
| AD-13 | Spec pack stays in MCP crate | MCP |
| AD-14 | WorkflowCtx uses borrowed references | Workflow |
