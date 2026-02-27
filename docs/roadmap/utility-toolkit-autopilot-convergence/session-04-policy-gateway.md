# Session 04: Unified Mutation Policy Gateway

**Date:** 2026-02-26
**Session:** 04 of 08
**Branch:** `feat/mcp_final`

---

## Summary

All mutation paths now route through a single `MutationGateway` in `tuitbot-core`. The gateway enforces a strict four-step sequence — policy evaluation, DB-backed idempotency, audit record creation, and post-execution recording — replacing scattered, duplicated logic across MCP tool handlers.

---

## Architecture

### Gateway Sequence

```
Caller (MCP / Autopilot / HTTP)
  │
  ▼
MutationGateway::evaluate(MutationRequest)
  │
  ├─ Step 1: Policy Evaluation
  │    └─ McpPolicyEvaluator::evaluate()
  │         ├─ Hard rules (priority 0-10)
  │         ├─ Template rules (100-199)
  │         ├─ User rules (200+)
  │         ├─ V1 compat rules (300+)
  │         └─ Rate limits (global + per-dimension)
  │
  │    Decision:
  │    ├─ Deny         → GatewayDecision::Denied(reason, rule_id)
  │    ├─ Approval     → GatewayDecision::RoutedToApproval(queue_id, ...)
  │    ├─ DryRun       → GatewayDecision::DryRun(rule_id)
  │    └─ Allow        → continue ↓
  │
  ├─ Step 2: DB-Backed Idempotency (5-min window)
  │    └─ mutation_audit::find_recent_duplicate()
  │         ├─ Hit (success)  → GatewayDecision::Duplicate(info)
  │         └─ Miss / Failed  → continue ↓
  │
  ├─ Step 3: Pending Audit Record
  │    └─ mutation_audit::insert_pending(correlation_id, tool, hash)
  │         → GatewayDecision::Proceed(MutationTicket)
  │
  ▼
Caller executes the mutation
  │
  ├─ Success → MutationGateway::complete_success()
  │              ├─ mutation_audit::complete_success()
  │              └─ McpPolicyEvaluator::record_mutation() (rate counters)
  │
  └─ Failure → MutationGateway::complete_failure()
                 └─ mutation_audit::complete_failure()
```

### Two-Layer Idempotency

| Layer | Location | Window | Purpose |
|-------|----------|--------|---------|
| In-memory | `IdempotencyStore` (MCP crate) | 30s | Fast-path agent retry storm protection |
| DB-backed | `MutationGateway` (core crate) | 5 min | Cross-process dedup with cached results |

The MCP adapter calls the in-memory store first, then delegates to the core gateway for the DB check.

---

## Types

### `MutationRequest<'a>`

All dependencies passed per-call — the gateway is stateless:

| Field | Type | Source |
|-------|------|--------|
| `pool` | `&DbPool` | SQLite connection pool |
| `policy_config` | `&McpPolicyConfig` | Policy rules, rate limits, blocked tools |
| `mode` | `&OperatingMode` | Autopilot / Composer |
| `tool_name` | `&str` | e.g. `"post_tweet"`, `"like_tweet"` |
| `params_json` | `&str` | Serialized tool parameters |

### `GatewayDecision`

| Variant | Meaning |
|---------|---------|
| `Proceed(MutationTicket)` | Execute the mutation; carry ticket through |
| `Denied(GatewayDenial)` | Policy blocked — contains reason + rule_id |
| `RoutedToApproval { queue_id, reason, rule_id }` | Enqueued for manual approval |
| `DryRun { rule_id }` | Would execute but intercepted by dry-run rule |
| `Duplicate(DuplicateInfo)` | Identical recent mutation already succeeded |

### `MutationTicket`

Carries the audit trail through execution:

| Field | Type | Purpose |
|-------|------|---------|
| `audit_id` | `i64` | DB row for this mutation attempt |
| `correlation_id` | `String` | UUID v4-like ID for tracing |
| `tool_name` | `String` | Tool name for rate-limit recording |

---

## Failure Semantics

| Scenario | Gateway returns | Side effects |
|----------|----------------|--------------|
| Policy deny (blocked/rate-limit/hard-rule) | `Denied` | Policy decision logged |
| Policy approval routing | `RoutedToApproval` | Item enqueued in approval_queue |
| Policy dry-run | `DryRun` | Decision logged, no audit |
| DB idempotency hit | `Duplicate` | Duplicate audit record created, linked to original |
| Policy evaluation DB error | `Err(StorageError)` | None |
| Mutation succeeds | N/A (caller calls `complete_success`) | Audit completed, rate counters incremented |
| Mutation fails | N/A (caller calls `complete_failure`) | Audit marked as failure |

---

## MCP Adapter Pattern

The MCP layer (`policy_gate.rs`) is now a thin adapter:

```rust
// Before (3 scattered steps):
let policy = check_policy(state, "post_tweet", &params, start).await;
let guard = begin_audited_mutation(state, "post_tweet", &params, start).await;
// ... execute ...
record_mutation(state, "post_tweet", &result, start).await;

// After (unified gateway):
let ticket = match run_gateway(state, "post_tweet", &params, start).await {
    GatewayResult::Proceed(t) => t,
    GatewayResult::EarlyReturn(r) => return r,
};
// ... execute ...
let meta = complete_gateway_success(state, &ticket, &result_data, start).await;
```

---

## Test Coverage (15 tests)

| Test | Scenario |
|------|----------|
| `gateway_allows_valid_mutation` | Normal allow path |
| `gateway_allows_when_enforcement_disabled` | Bypass when `enforce_for_mutations = false` |
| `gateway_denies_blocked_tool` | Tool in `blocked_tools` list |
| `gateway_denies_when_rate_limited` | Global rate limit exhausted |
| `gateway_denies_per_dimension_rate_limit` | Per-tool rate limit |
| `gateway_routes_to_approval` | User rule with `RequireApproval` action |
| `gateway_returns_dry_run` | User rule with `DryRun` action |
| `gateway_detects_duplicate` | Same params within 5-min window |
| `gateway_allows_retry_after_failure` | Retry after failed attempt (not deduped) |
| `gateway_records_success` | Audit trail + rollback action recorded |
| `gateway_records_failure` | Audit trail with error message |
| `gateway_denies_hard_rule` | `hard:` prefixed deny rule at priority 10 |
| `gateway_routes_delete_to_approval` | Built-in `hard:delete_approval` at priority 0 |
| `correlation_id_is_uuid_v4_format` | UUID v4 format validation |
| `correlation_ids_are_unique` | 100 IDs, all unique |
