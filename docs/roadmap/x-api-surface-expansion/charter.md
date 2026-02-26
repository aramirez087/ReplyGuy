# X API Surface Expansion — Technical Charter

**Status:** Approved
**Owner:** Session 01
**Last updated:** 2026-02-26

---

## 1. Problem Statement

Tuitbot's MCP server exposes 64 curated tools covering the most common X API v2
operations plus workflow-specific intelligence (scoring, content generation,
analytics, approval queues). This surface is strong for Tuitbot's autonomous
growth co-pilot use case, but it only covers approximately **55% of X API v2
endpoints**.

Missing capabilities include:

| Gap Area | Impact |
|----------|--------|
| **Lists** — create, manage, read list tweets/members | Agents cannot organize monitoring targets into lists or consume list timelines |
| **Mutes / Blocks** — read and manage | Agents cannot enforce safety boundaries (mute spam, block abusive accounts) |
| **Batch tweet lookup** — `GET /2/tweets?ids=` | Forces N+1 fetches for multi-tweet analysis |
| **Batch username lookup** — `GET /2/users/by?usernames=` | Same N+1 problem for user resolution |
| **Quote tweets** — `GET /2/tweets/:id/quote_tweets` | Cannot discover quote-tweet engagement on own content |
| **Retweeting users** — `GET /2/tweets/:id/retweeted_by` | Cannot analyze retweet amplification |
| **Hide replies** — `PUT /2/tweets/:id/hidden` | Cannot moderate reply threads |
| **Tweet counts** — `GET /2/tweets/counts/recent` | Cannot assess topic volume without fetching full tweets |
| **Spaces** (read-only) | Cannot discover live audio events for engagement |
| **Pinned tweets** — manage tweet pinning | Cannot pin high-performing content |

Competing MCP servers for X offer thin wrappers with full endpoint parity.
Tuitbot's curated tools are a differentiator, but the *missing* endpoints are a
blocker for agents that need the full API surface.

**Goal:** Expand to ≥95% X API v2 endpoint coverage while preserving the curated
workflow layer and hardening safety for autonomous agents.

---

## 2. Goals and Non-Goals

### Goals

1. **Systematic endpoint coverage.** Expose every X API v2 endpoint that has a
   stable v2 path as an MCP tool, organized into the existing category/profile
   system.

2. **Two-layer architecture.** Maintain a clean separation between:
   - **Layer 1 — Curated workflow tools** (existing 35 workflow-only tools):
     hand-crafted, tested, business-logic-rich.
   - **Layer 2 — Universal X API surface tools** (new): one-to-one X API
     endpoint mapping, generated from an endpoint registry, minimal business
     logic.

3. **Agent safety by construction.** Enforce host allowlist, SSRF prevention,
   restricted header policy, and mutation policy gating at the infrastructure
   level — not per-tool.

4. **Idempotent mutations.** Every mutation tool accepts an optional
   `idempotency_key` parameter. The server deduplicates within a configurable
   TTL window.

5. **Profile-aware registration.** New tools slot into the existing profile
   matrix (full / readonly / api-readonly) based on whether they are reads or
   mutations. Tools not assigned to a profile are never registered on the
   server.

6. **Measurable quality.** Coverage ratio, idempotency coverage, and
   conformance test pass rate are tracked as KPIs with defined thresholds.

### Non-Goals

1. **Ads API support.** The Ads API uses a separate authentication model,
   billing system, and permission structure. It is a different product surface
   and is excluded from this initiative.

2. **Direct Messages (DMs).** DM automation carries elevated account-suspension
   risk and requires a fundamentally different trust model (per-conversation
   consent, anti-spam). DMs are excluded from scope.

3. **Compliance endpoints.** Enterprise-only batch compliance jobs serve a
   different use case (data governance) and are excluded.

4. **Full-archive search.** `GET /2/tweets/search/all` requires Academic or
   Enterprise access. The tool will be defined but gated behind tier detection.

5. **Scraper backend parity.** New Layer 2 tools target the official X API only.
   Scraper backend support for new endpoints is not a goal.

6. **Breaking changes to existing tools.** All 64 current tools retain their
   names, schemas, and behavior. This is an additive expansion.

---

## 3. Architecture Overview

### 3.1 Two-Layer Model

```
┌─────────────────────────────────────────────────────────┐
│                   MCP Server (profile-gated)             │
│                                                         │
│  ┌─────────────────────────┐  ┌──────────────────────┐  │
│  │   Layer 1: Curated      │  │ Layer 2: Universal   │  │
│  │   Workflow Tools (35)   │  │ X API Surface (~40)  │  │
│  │                         │  │                      │  │
│  │  - Composite workflows  │  │  - Lists CRUD        │  │
│  │  - Content generation   │  │  - Mutes / Blocks    │  │
│  │  - Analytics            │  │  - Batch lookups     │  │
│  │  - Approval queue       │  │  - Quote tweets      │  │
│  │  - Context intelligence │  │  - Retweeting users  │  │
│  │  - Discovery            │  │  - Hide replies      │  │
│  │  - Scoring              │  │  - Tweet counts      │  │
│  │  - Telemetry            │  │  - Spaces (read)     │  │
│  │  - Policy               │  │  - Pin management    │  │
│  │                         │  │                      │  │
│  │  (hand-crafted, tested, │  │  (systematic, from   │  │
│  │   business-logic-rich)  │  │   endpoint registry) │  │
│  └────────────┬────────────┘  └──────────┬───────────┘  │
│               │                          │               │
│  ┌────────────┴──────────────────────────┴───────────┐  │
│  │              Shared Infrastructure                 │  │
│  │  - Response envelope (v1.0)                       │  │
│  │  - Error taxonomy (28+ codes)                     │  │
│  │  - Policy engine (mutations)                      │  │
│  │  - Idempotency layer                              │  │
│  │  - Host allowlist + SSRF guard                    │  │
│  │  - Telemetry recording                            │  │
│  │  - Profile-based tool registration                │  │
│  └───────────────────────────────────────────────────┘  │
│               │                          │               │
│  ┌────────────┴────────┐  ┌──────────────┴───────────┐  │
│  │  SocialReadProvider  │  │    XApiClient trait      │  │
│  │  (read abstraction)  │  │    (full abstraction)    │  │
│  └─────────────────────┘  └──────────────────────────┘  │
│                          │                               │
│                 ┌────────┴────────┐                      │
│                 │  XApiHttpClient  │                      │
│                 │  (reqwest-based) │                      │
│                 └─────────────────┘                      │
└─────────────────────────────────────────────────────────┘
```

### 3.2 Endpoint Registry

A new `endpoint_registry.rs` module defines every X API v2 endpoint as a
declarative struct:

```rust
struct EndpointDef {
    /// MCP tool name (e.g. "x_list_tweets")
    tool_name: &'static str,
    /// HTTP method
    method: HttpMethod,
    /// URL path template (e.g. "/2/lists/{id}/tweets")
    path: &'static str,
    /// Category for manifest grouping
    category: ToolCategory,
    /// Whether this is a mutation
    mutation: bool,
    /// Required OAuth scopes
    scopes: &'static [&'static str],
    /// Minimum API tier required
    min_tier: ApiTier,
    /// Parameters with types and validation rules
    params: &'static [ParamDef],
}
```

A build-time or test-time code generator produces:
- MCP tool schemas (JSON Schema for each tool's input)
- Manifest entries (for `all_tools()`)
- Conformance test stubs

This avoids hand-writing boilerplate for each endpoint while keeping the
generated code auditable and version-controlled.

### 3.3 New Tool Categories

The existing `ToolCategory` enum gains two new variants:

| New Category | Purpose |
|--------------|---------|
| `List` | List CRUD, membership, following, tweets |
| `Moderation` | Mutes, blocks, hide replies |

Existing categories absorb the remaining new tools:

| Category | New Tools |
|----------|-----------|
| `Read` | `x_get_tweets_by_ids`, `x_get_users_by_usernames`, `x_get_retweeting_users`, `x_get_quote_tweets`, `x_get_tweet_counts` |
| `Engage` | `x_pin_tweet`, `x_unpin_tweet` |
| `Media` | (no new tools) |

### 3.4 Profile Assignment for New Tools

| Tool Type | `full` | `api-readonly` | `readonly` |
|-----------|--------|----------------|------------|
| List reads | yes | yes | no |
| List mutations (create/update/delete, manage members) | yes | no | no |
| Mute/Block reads | yes | yes | no |
| Mute/Block mutations | yes | no | no |
| Hide replies | yes | no | no |
| Batch tweet/user lookups | yes | yes | yes |
| Quote tweets / retweeting users | yes | yes | yes |
| Tweet counts | yes | yes | no |
| Space reads | yes | yes | no |
| Pin management | yes | no | no |

---

## 4. Safety Constraints

### 4.1 Host Allowlist

All outbound HTTP from the X API client is restricted to an explicit allowlist.
Requests to any other host are rejected before leaving the process.

```
ALLOWED_HOSTS = [
    "api.x.com",
    "api.twitter.com",        // legacy, some endpoints still resolve here
    "upload.twitter.com",     // v1.1 media upload
]
```

**Enforcement point:** `XApiHttpClient` constructor validates the base URL. The
`reqwest::Client` uses a custom redirect policy that rejects cross-host
redirects. No per-tool enforcement needed — it is structural.

### 4.2 SSRF Prevention

- Tool inputs that accept IDs (tweet IDs, user IDs, list IDs) are validated as
  numeric strings or alphanumeric tokens. No URL-type parameters are accepted.
- Path template interpolation uses strict slug validation (`[a-zA-Z0-9_-]+`)
  that rejects any attempt to inject path traversal or alternative hosts.
- The `reqwest::Client` is configured with `redirect(Policy::none())` or a
  same-host-only policy. No open redirects.

### 4.3 Restricted Header Policy

The MCP tool layer never exposes the following to callers:

| Header | Policy |
|--------|--------|
| `Authorization` | Injected by `XApiHttpClient` from token store. Never in tool input/output. |
| `Cookie` | Blocked entirely. Not sent, not accepted. |
| `X-Client-Transaction-Id` | Generated internally for debugging. Not exposed. |
| Custom headers | Only `User-Agent` and `Content-Type` are set. No caller-supplied headers. |

**Enforcement:** The `XApiHttpClient` constructs requests internally. Tool
handlers receive typed Rust structs, not raw HTTP. There is no code path from
tool input to arbitrary HTTP headers.

### 4.4 Mutation Safety

All mutation tools (existing and new) pass through the policy engine:

1. **Tool block check** — `blocked_tools` list in config.
2. **Rate limit check** — `max_mutations_per_hour` counter.
3. **Approval routing** — `require_approval_for` list routes to approval queue.
4. **Dry-run mode** — `dry_run_mutations = true` returns simulated response.
5. **Hard rules** — Banned phrases, self-reply prevention, per-author limits.
6. **User rules** — Custom regex/keyword filters.

New mutation tools (list management, mute, block, hide replies, pin) will be
registered in the policy engine with the same gating. No mutation bypasses the
policy layer.

### 4.5 Idempotency

Existing idempotency support (in `tools/idempotency.rs`) extends to all new
mutation tools. The contract:

- Every mutation tool accepts an optional `idempotency_key: String` parameter.
- If the key has been seen within the TTL window (default: 1 hour), the server
  returns the cached response without re-executing.
- Keys are stored in SQLite (`mcp_idempotency` table) with `(tool_name, key,
  response_json, expires_at)`.
- Expired entries are purged on a background schedule.

---

## 5. Coverage and Reliability KPIs

### 5.1 Endpoint Coverage Ratio

**Definition:** `(MCP tools mapping to X API v2 endpoints) / (total stable X API v2 endpoints)`

| Metric | Current | Target |
|--------|---------|--------|
| Core endpoints (tweets, users, timelines, likes, follows, retweets, bookmarks, search, media) | 29 / 29 (100%) | 29 / 29 (100%) |
| Lists endpoints | 0 / 15 | 15 / 15 (100%) |
| Mutes endpoints | 0 / 3 | 3 / 3 (100%) |
| Blocks endpoints | 0 / 3 | 3 / 3 (100%) |
| Batch lookups | 1 / 4 | 4 / 4 (100%) |
| Tweet metadata (quote tweets, retweeting users, counts, hide) | 0 / 5 | 5 / 5 (100%) |
| Spaces (read) | 0 / 6 | 6 / 6 (100%) |
| Pin management | 0 / 2 | 2 / 2 (100%) |
| **Total** | **30 / 67 (~45%)** | **67 / 67 (100%)** |

*Excludes DMs (6 endpoints), Compliance (3 endpoints), and Ads API (separate
API). These are documented non-goals.*

### 5.2 Mutation Idempotency Coverage

**Definition:** `(mutation tools with idempotency support) / (total mutation tools)`

| Metric | Current | Target |
|--------|---------|--------|
| Idempotency coverage | Existing tools have infrastructure | 100% of all mutation tools |

### 5.3 Conformance Test Pass Rate

**Definition:** Every MCP tool must pass:
1. **Schema validation** — Input/output matches declared JSON Schema.
2. **Golden fixture** — Response matches snapshot for deterministic inputs.
3. **Error taxonomy** — All declared error codes are exercised.
4. **Profile isolation** — Tool is only callable in its declared profiles.

| Metric | Current | Target |
|--------|---------|--------|
| Conformance pass rate | 27 / 64 tools have golden fixtures | 100% of all tools |
| Boundary test coverage | 32 tests | Proportional growth with new tools |

### 5.4 Reliability

| Metric | Target |
|--------|--------|
| Retry coverage | 100% of read tools use `RetryProvider` |
| Rate-limit propagation | 100% of tools surface `rate_limit_reset` on 429 |
| Graceful degradation | Tier-gated tools return `x_forbidden` with actionable message, not panic |

---

## 6. Explicit Scope Decisions: Ads and DMs

### 6.1 Ads API — Excluded

**Rationale:**
- The Ads API uses a completely separate authentication model (OAuth 1.0a with
  app-level tokens, not user-level PKCE).
- It has its own rate-limit regime, billing, and permission structure.
- Ad creation/management is a fundamentally different domain from organic growth.
- Exposing ad spend to an autonomous agent creates financial risk that requires
  a purpose-built approval workflow beyond Tuitbot's current model.

**Decision:** Ads API is permanently out of scope for the MCP server. If demand
emerges, it would be a separate MCP server with dedicated safety controls.

### 6.2 Direct Messages — Excluded

**Rationale:**
- DM automation is the single highest-risk activity for X account suspension.
  X's anti-spam systems heavily monitor automated DMs.
- DMs involve per-conversation consent semantics. An autonomous agent sending
  unsolicited DMs violates platform norms and potentially regulations.
- The trust model for DMs (two-party consent, message history, attachment
  handling) differs fundamentally from public tweet operations.
- Tuitbot's positioning is public engagement — not private outreach.

**Decision:** DM endpoints (6 total) are excluded from this initiative. If
future demand arises, DMs would require a separate consent-aware module with
its own approval flow and a dedicated safety review.

### 6.3 Spaces — Included (Read-Only)

**Rationale:**
- Spaces metadata is public and read-only.
- Discovering live Spaces relevant to the user's niche is valuable for
  engagement timing.
- No mutation risk — only `GET` endpoints.

**Decision:** All 6 Space read endpoints are in scope.

### 6.4 Lists — Included (Full CRUD)

**Rationale:**
- Lists are a core organizational primitive for monitoring targets.
- List management mutations (create, update, delete, add/remove members) are
  low-risk and idempotent by nature.
- Agent workflows benefit from programmatic list management (e.g., "add all
  accounts in my niche to a monitoring list").

**Decision:** All 15 List endpoints are in scope. Mutations route through the
policy engine.

### 6.5 Mutes and Blocks — Included

**Rationale:**
- Mute/block are safety-critical for autonomous agents. An agent that cannot
  mute spam or block abusive accounts lacks a basic safety mechanism.
- These are low-risk mutations with clear undo semantics.

**Decision:** All 6 mute/block endpoints are in scope. Mutations route through
the policy engine.

---

## 7. New Endpoint Inventory

The following X API v2 endpoints will be added as Layer 2 tools. Each row
specifies the MCP tool name, X API path, HTTP method, mutation status, and
target profile.

### 7.1 Batch Lookups (3 new tools)

| MCP Tool | X API Path | Method | Mutation | Profiles |
|----------|-----------|--------|----------|----------|
| `x_get_tweets_by_ids` | `GET /2/tweets` | GET | No | full, api-readonly, readonly |
| `x_get_users_by_usernames` | `GET /2/users/by` | GET | No | full, api-readonly, readonly |
| `x_get_retweeting_users` | `GET /2/tweets/:id/retweeted_by` | GET | No | full, api-readonly, readonly |

### 7.2 Tweet Metadata (4 new tools)

| MCP Tool | X API Path | Method | Mutation | Profiles |
|----------|-----------|--------|----------|----------|
| `x_get_quote_tweets` | `GET /2/tweets/:id/quote_tweets` | GET | No | full, api-readonly, readonly |
| `x_get_tweet_counts` | `GET /2/tweets/counts/recent` | GET | No | full, api-readonly |
| `x_hide_reply` | `PUT /2/tweets/:id/hidden` | PUT | Yes | full |
| `x_unhide_reply` | `PUT /2/tweets/:id/hidden` | PUT | Yes | full |

### 7.3 Pin Management (2 new tools)

| MCP Tool | X API Path | Method | Mutation | Profiles |
|----------|-----------|--------|----------|----------|
| `x_pin_tweet` | `PUT /2/users/:id/pinned_tweet` | PUT | Yes | full |
| `x_unpin_tweet` | `DELETE /2/users/:id/pinned_tweet` | DELETE | Yes | full |

### 7.4 Lists (15 new tools)

| MCP Tool | X API Path | Method | Mutation | Profiles |
|----------|-----------|--------|----------|----------|
| `x_get_list` | `GET /2/lists/:id` | GET | No | full, api-readonly |
| `x_get_owned_lists` | `GET /2/users/:id/owned_lists` | GET | No | full, api-readonly |
| `x_create_list` | `POST /2/lists` | POST | Yes | full |
| `x_update_list` | `PUT /2/lists/:id` | PUT | Yes | full |
| `x_delete_list` | `DELETE /2/lists/:id` | DELETE | Yes | full |
| `x_get_list_tweets` | `GET /2/lists/:id/tweets` | GET | No | full, api-readonly |
| `x_get_list_members` | `GET /2/lists/:id/members` | GET | No | full, api-readonly |
| `x_add_list_member` | `POST /2/lists/:id/members` | POST | Yes | full |
| `x_remove_list_member` | `DELETE /2/lists/:id/members/:user_id` | DELETE | Yes | full |
| `x_get_list_memberships` | `GET /2/users/:id/list_memberships` | GET | No | full, api-readonly |
| `x_get_list_followers` | `GET /2/lists/:id/followers` | GET | No | full, api-readonly |
| `x_follow_list` | `POST /2/users/:id/followed_lists` | POST | Yes | full |
| `x_unfollow_list` | `DELETE /2/users/:id/followed_lists/:list_id` | DELETE | Yes | full |
| `x_get_pinned_lists` | `GET /2/users/:id/pinned_lists` | GET | No | full, api-readonly |
| `x_pin_list` | `POST /2/users/:id/pinned_lists` | POST | Yes | full |

### 7.5 Mutes (3 new tools)

| MCP Tool | X API Path | Method | Mutation | Profiles |
|----------|-----------|--------|----------|----------|
| `x_get_muted_users` | `GET /2/users/:id/muting` | GET | No | full, api-readonly |
| `x_mute_user` | `POST /2/users/:id/muting` | POST | Yes | full |
| `x_unmute_user` | `DELETE /2/users/:id/muting/:target_user_id` | DELETE | Yes | full |

### 7.6 Blocks (3 new tools)

| MCP Tool | X API Path | Method | Mutation | Profiles |
|----------|-----------|--------|----------|----------|
| `x_get_blocked_users` | `GET /2/users/:id/blocking` | GET | No | full, api-readonly |
| `x_block_user` | `POST /2/users/:id/blocking` | POST | Yes | full |
| `x_unblock_user` | `DELETE /2/users/:id/blocking/:target_user_id` | DELETE | Yes | full |

### 7.7 Spaces (6 new tools)

| MCP Tool | X API Path | Method | Mutation | Profiles |
|----------|-----------|--------|----------|----------|
| `x_get_space` | `GET /2/spaces/:id` | GET | No | full, api-readonly |
| `x_get_spaces_by_ids` | `GET /2/spaces` | GET | No | full, api-readonly |
| `x_get_spaces_by_creator` | `GET /2/spaces/by/creator_ids` | GET | No | full, api-readonly |
| `x_search_spaces` | `GET /2/spaces/search` | GET | No | full, api-readonly |
| `x_get_space_buyers` | `GET /2/spaces/:id/buyers` | GET | No | full, api-readonly |
| `x_get_space_tweets` | `GET /2/spaces/:id/tweets` | GET | No | full, api-readonly |

### 7.8 Summary

| Category | New Read Tools | New Mutation Tools | Total New |
|----------|---------------|--------------------|-----------|
| Batch Lookups | 3 | 0 | 3 |
| Tweet Metadata | 2 | 2 | 4 |
| Pin Management | 0 | 2 | 2 |
| Lists | 8 | 7 | 15 |
| Mutes | 1 | 2 | 3 |
| Blocks | 1 | 2 | 3 |
| Spaces | 6 | 0 | 6 |
| **Total** | **21** | **15** | **36** |

**Post-expansion tool counts:**

| Profile | Current | Added | Total |
|---------|---------|-------|-------|
| `full` | 64 | 36 | 100 |
| `api-readonly` | 20 | 19 | 39 |
| `readonly` | 10 | 4 | 14 |

---

## 8. Implementation Sequencing

Implementation spans Sessions 02–11. Each session produces working, tested
code and a handoff document.

| Session | Focus | New Tools | Key Deliverable |
|---------|-------|-----------|-----------------|
| 02 | Endpoint registry + code generation scaffold | 0 | `endpoint_registry.rs`, codegen harness, `ToolCategory` enum updates |
| 03 | Host allowlist, SSRF guard, restricted headers | 0 | Safety infrastructure in `XApiHttpClient`, integration tests |
| 04 | Batch lookups + tweet metadata reads | 7 | 7 new read tools with conformance tests |
| 05 | Lists — read endpoints | 8 | 8 list read tools, `List` category |
| 06 | Lists — mutation endpoints | 7 | 7 list mutation tools, policy integration |
| 07 | Mutes + Blocks | 6 | 6 tools, `Moderation` category |
| 08 | Spaces (read-only) | 6 | 6 space tools |
| 09 | Pin management + hide replies | 4 | 4 mutation tools |
| 10 | Idempotency hardening + manifest regeneration | 0 | Full idempotency coverage, updated manifests |
| 11 | Coverage validation, KPI dashboard, final docs | 0 | KPI report, updated `mcp-reference.md` |

---

## 9. Revision History

| Date | Change |
|------|--------|
| 2026-02-26 | Initial charter created (Session 01) |
