# Session 07 — Workflow Tool Boundary

## Module Structure

```
crates/tuitbot-mcp/src/tools/
├── mod.rs              ← Shared modules + workflow gateway
├── config.rs           ← Shared: get_config, validate_config
├── idempotency.rs      ← Shared: IdempotencyStore
├── manifest.rs         ← Shared: ToolManifest, Lane enum
├── response.rs         ← Shared: ToolResponse, ToolMeta, ErrorCode
├── scoring.rs          ← Shared: score_tweet (pure function)
├── benchmark.rs        ← Test: baseline performance
├── boundary_tests.rs   ← Test: structural isolation enforcement
├── contract_tests.rs   ← Test: envelope schema validation
├── eval_harness.rs     ← Test: observability quality gates
└── workflow/
    ├── mod.rs          ← Workflow gateway
    ├── actions.rs
    ├── analytics.rs
    ├── approval.rs
    ├── capabilities.rs
    ├── content.rs
    ├── context.rs
    ├── discovery.rs
    ├── health.rs
    ├── policy_gate.rs
    ├── rate_limits.rs
    ├── replies.rs
    ├── targets.rs
    ├── telemetry.rs
    ├── composite/
    │   ├── mod.rs
    │   ├── draft_replies.rs
    │   ├── find_opportunities.rs
    │   ├── propose_queue.rs
    │   ├── tests.rs
    │   └── thread_plan.rs
    └── x_actions/
        ├── mod.rs
        ├── engage.rs
        ├── media.rs
        ├── read.rs
        ├── validate.rs
        ├── write.rs
        └── tests/
```

## Import Contract

| Consumer | Allowed Imports |
|----------|----------------|
| `server/api.rs` | `crate::tools::response`, `crate::tools::scoring`, `crate::tools::config`, `crate::tools::idempotency` |
| `server/workflow.rs` | All of the above + `crate::tools::workflow::*` |
| Workflow submodules | `crate::tools::response::*`, sibling `super::` within `workflow/` |

**Enforced by:** `boundary_tests::api_server_does_not_import_workflow_modules` — scans `api.rs` source at compile time.

## Lane Enum

```rust
pub enum Lane {
    Shared,   // tools/ root — available to all profiles
    Workflow,  // tools/workflow/ — requires full stack
}
```

Every `ToolEntry` in the manifest carries a `lane` field. Boundary tests verify:
- WF-only tools → `Lane::Workflow`
- API-profile tools → `Lane::Shared`

## Metadata Fix

`score_tweet` corrected: `requires_db: false` (pure function on `&Config`), `DbError` removed from error codes.
