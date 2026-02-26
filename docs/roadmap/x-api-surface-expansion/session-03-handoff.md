# Session 03 Handoff — Universal X API Request Layer

**Date:** 2026-02-26
**Session:** 03
**Branch:** `feat/init_simplification`
**Status:** Complete

---

## 1. What Was Accomplished

Implemented a universal X API tool layer so the MCP server can call any
authorized X endpoint safely and predictably, without needing a dedicated tool
per endpoint. Four new tools: `x_get`, `x_post`, `x_put`, `x_delete`.

### Capabilities

| Capability | Status |
|------------|--------|
| Hard host allowlist (`api.x.com`, `upload.x.com`, `upload.twitter.com`) | Done |
| Path validation (leading `/`, no `..` traversal, no `?`/`#` in path) | Done |
| SSRF protection (reject IP-literal hosts, HTTPS-only) | Done |
| Header blocklist (`authorization`, `host`, `cookie`, etc.) | Done |
| Built-in retry/backoff for 429/5xx (via existing `RetryPolicy`) | Done |
| Auto-pagination with `next_token` cursor following (max 10 pages) | Done |
| Structured response: status, headers, parsed JSON, raw text fallback | Done |
| Rate-limit introspection (`remaining`, `reset_at`, `recommended_wait_ms`) | Done |
| Comprehensive test suite (25+ tests) | Done |
| Manifest + boundary test integration | Done |

---

## 2. Implementation Summary

### New Files

| File | Lines | Purpose |
|------|-------|---------|
| `crates/tuitbot-mcp/src/tools/workflow/x_actions/x_request/mod.rs` | ~340 | Core safety validation + execution logic |
| `crates/tuitbot-mcp/src/tools/workflow/x_actions/x_request/tests.rs` | ~300 | Unit tests for all guardrails |

### Modified Files

| File | Change |
|------|--------|
| `crates/tuitbot-core/src/x_api/types.rs` | Added `RawApiResponse` struct |
| `crates/tuitbot-core/src/x_api/mod.rs` | Added `raw_request` method to `XApiClient` trait (default impl) |
| `crates/tuitbot-core/src/x_api/client.rs` | Implemented `raw_request` for `XApiHttpClient` |
| `crates/tuitbot-mcp/src/contract/error_code.rs` | Added `XRequestBlocked` error code variant |
| `crates/tuitbot-mcp/src/tools/workflow/x_actions/mod.rs` | Added `pub mod x_request` |
| `crates/tuitbot-mcp/src/requests.rs` | Added `KeyValue`, `XGetRequest`, `XPostRequest`, `XPutRequest`, `XDeleteRequest` |
| `crates/tuitbot-mcp/src/server/workflow.rs` | Registered 4 new `#[tool]` handlers + `kv_to_tuples` helper |
| `crates/tuitbot-mcp/src/tools/manifest.rs` | Added 4 tool entries with `X_REQUEST_ERR` error codes |
| `crates/tuitbot-mcp/src/tools/boundary_tests.rs` | Added 3 mutation tools to denylist, updated count 64→68 |
| `roadmap/artifacts/session-05-tool-manifest.json` | Regenerated snapshot (68 tools) |

---

## 3. API Contract

### Tool: `x_get`

**Category:** Read | **Mutation:** No | **Profile:** Workflow only

Parameters:
- `path` (string, required): API path starting with `/`
- `host` (string, optional): Override host (default: `api.x.com`)
- `query` (array of `{key, value}`, optional): Query parameters
- `headers` (array of `{key, value}`, optional): Extra headers
- `auto_paginate` (bool, optional): Follow `next_token` cursors
- `max_pages` (u32, optional): Max pages to fetch (capped at 10)

### Tools: `x_post`, `x_put`

**Category:** Write | **Mutation:** Yes | **Profile:** Workflow only

Parameters:
- `path`, `host`, `query`, `headers`: Same as `x_get`
- `body` (string, optional): JSON request body

### Tool: `x_delete`

**Category:** Write | **Mutation:** Yes | **Profile:** Workflow only

Parameters:
- `path`, `host`, `query`, `headers`: Same as `x_get`

### Response Envelope

All four tools return:
```json
{
  "success": true,
  "data": {
    "status": 200,
    "headers": { "content-type": "application/json", ... },
    "json": { ... },
    "body_text": "...",
    "rate_limit": {
      "remaining": 14,
      "reset_at": 1740600000,
      "recommended_wait_ms": 45000
    }
  },
  "meta": { "elapsed_ms": 123, "retry_count": 1 }
}
```

HTTP error statuses (4xx/5xx) still return `success: true` — the tool succeeded
in making the request. The caller interprets the `status` field.

### Pagination Response (auto_paginate=true)

```json
{
  "success": true,
  "data": {
    "pages": [
      { "page": 1, "status": 200, "data": { ... } },
      { "page": 2, "status": 200, "data": { ... } }
    ],
    "total_pages": 2,
    "rate_limit": { ... }
  },
  "meta": {
    "elapsed_ms": 456,
    "pagination": {
      "next_token": null,
      "result_count": 2,
      "has_more": false
    }
  }
}
```

---

## 4. Security Controls

### Host Allowlist

Only three hosts accepted: `api.x.com`, `upload.x.com`, `upload.twitter.com`.
Case-insensitive comparison. Any other host → `XRequestBlocked` error before
any network I/O.

### SSRF Protection

IPv4 literals, IPv6 literals, and bracket-notation addresses are rejected
before the allowlist check. Combined with HTTPS-only URL construction, this
prevents SSRF against internal services.

### Path Validation

- Must start with `/`
- Must not contain `..` (path traversal)
- Must not contain `?` or `#` (query/fragment — use `query` parameter instead)
- Must not contain control characters

### Header Blocklist

Callers cannot set: `authorization`, `host`, `cookie`, `set-cookie`,
`transfer-encoding`, `proxy-authorization`, `proxy-connection`.
Case-insensitive. Multiple blocked headers listed in the error message.

### Authentication

The `authorization` header is set automatically by `XApiHttpClient` using the
configured OAuth bearer token. Callers cannot override or inspect it.

---

## 5. Design Decisions

### D-014: Four Separate Tools Instead of One

**Decision:** Expose `x_get`, `x_post`, `x_put`, `x_delete` as four separate
MCP tools rather than a single `x_request` with a `method` parameter.

**Rationale:** MCP schema-level separation lets the agent (and the manifest)
distinguish reads from mutations. The `mutation: true` flag on x_post/x_put/x_delete
triggers policy engine checks (approval routing, rate limits) that don't apply
to x_get. A single tool would require runtime method-based branching in policy code.

### D-015: Workflow Profile Only

**Decision:** All four universal tools are restricted to the Workflow (full)
profile. They do not appear in Readonly or ApiReadonly.

**Rationale:** These tools can reach any X API endpoint, including ones not yet
covered by curated tools. Exposing them in read-only profiles would bypass the
profile's purpose of limiting surface area. The curated read tools in
api-readonly are sufficient for safe read operations.

### D-016: HTTP Errors Are Tool Successes

**Decision:** When the X API returns a 4xx or 5xx status, the tool still returns
`success: true` with the status code in the response body.

**Rationale:** The tool's job is to make the HTTP request and return the result.
A 404 from the X API is a valid response, not a tool failure. This matches how
`curl` returns exit code 0 regardless of HTTP status. The agent interprets the
status field and decides what to do.

### D-017: RequestParams Struct

**Decision:** Bundle `execute_request` parameters into a `RequestParams` struct
instead of passing 9 individual arguments.

**Rationale:** Clippy enforces `too_many_arguments` at 7. Rather than suppressing
the lint, the struct makes the internal API self-documenting and extensible.

---

## 6. Test Evidence

### Unit tests (25+ in x_request/tests.rs)

- **Host allowlist**: allowed hosts, blocked hosts, case-insensitive matching
- **SSRF guards**: IPv4 literals, IPv6 literals, bracket notation, private ranges
- **Path validation**: valid paths, empty path, missing leading slash, traversal,
  query in path, fragment in path, control characters
- **Header validation**: allowed headers, blocked headers, case-insensitive,
  multiple blocked headers
- **Response building**: JSON content-type, non-JSON fallback, retry count
  injection, rate limit metadata extraction
- **Blocked response**: correct error code and shape

### CI validation

```
cargo fmt --all && cargo fmt --all --check     # clean
cargo clippy --workspace -- -D warnings         # clean
RUSTFLAGS="-D warnings" cargo test -p tuitbot-mcp  # 346 tests pass
```

One pre-existing flaky test (`config::tests::env_var_override_approval_mode`)
fails intermittently under parallel execution due to env var races — passes
with `--test-threads=1`. Not related to this session's changes.

---

## 7. Risks

### R-007: Universal Tool Bypasses Curated Safety

**Risk:** An agent could use `x_post` to perform actions that curated tools
would block (e.g., posting without policy checks).

**Likelihood:** Medium
**Impact:** Medium
**Mitigation:** x_post/x_put/x_delete are marked `mutation: true`, so the
policy engine's mutation rate limits and approval routing still apply at the
MCP server level. Future sessions should ensure the policy gate fires before
any `#[tool]` handler with `mutation: true`.

### R-008: Rate Limit Budget Consumption

**Risk:** Universal GET requests may consume rate limits that curated tools
also need, leading to unexpected 429s.

**Likelihood:** Low
**Impact:** Low
**Mitigation:** Rate limit metadata is returned in every response. The
`recommended_wait_ms` field tells the agent how long to wait. Future rate-limit
budget advisor (charter R-003) will provide cross-tool visibility.

---

## 8. Next Session Inputs

Session 03 delivered the safety infrastructure for universal requests. The
original charter planned a per-endpoint registry approach (Sessions 04–09),
but the universal tool layer now provides equivalent API coverage with four
tools instead of ~36 individual endpoints.

The next session should evaluate whether the per-endpoint approach is still
needed given that `x_get`/`x_post`/`x_put`/`x_delete` can reach any authorized
endpoint, or whether to skip directly to idempotency hardening and coverage
validation (Sessions 10–11).
