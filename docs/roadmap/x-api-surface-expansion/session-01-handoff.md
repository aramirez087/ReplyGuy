# Session 01 Handoff — Charter and Scope Lock

**Date:** 2026-02-26
**Session:** 01 of 11
**Branch:** `feat/init_simplification`
**Status:** Complete

---

## 1. What Was Accomplished

1. **Full audit of current MCP surface.** Catalogued all 64 tools across 3
   profiles, mapped every tool to its XApiClient trait method, and identified
   the manifest generation pipeline.

2. **Gap analysis against X API v2.** Identified 36 missing endpoints across 7
   categories (Lists, Mutes, Blocks, Spaces, batch lookups, tweet metadata,
   pin management).

3. **Technical charter created.** `charter.md` defines the two-layer
   architecture, safety constraints, KPIs, scope decisions (DMs and Ads
   excluded), and full endpoint inventory with profile assignments.

4. **Session sequencing locked.** 10 remaining sessions (02–11) each have
   concrete scope, deliverable count, and dependency chain.

---

## 2. Decision Log

### D-001: Two-Layer Architecture

**Decision:** Keep curated workflow tools (Layer 1) separate from universal X
API surface tools (Layer 2).

**Rationale:** Layer 1 tools have business logic (scoring, LLM generation,
approval routing) that does not apply to raw API passthrough. Mixing them
creates testing complexity and makes the manifest harder to reason about.

**Consequence:** New tools use a code-generation pattern from an endpoint
registry. Existing tools are untouched.

---

### D-002: DMs Excluded

**Decision:** All 6 DM endpoints are permanently out of scope.

**Rationale:** DM automation is the single highest-risk activity for account
suspension. The trust model (per-conversation consent, anti-spam) differs from
public tweets. Tuitbot's positioning is public engagement.

**Consequence:** Agents that need DMs must use a separate MCP server. This is
documented in the charter.

---

### D-003: Ads API Excluded

**Decision:** Ads API is permanently out of scope.

**Rationale:** Different auth model (OAuth 1.0a app-level), separate billing,
and financial risk (ad spend) that requires purpose-built approval workflows
beyond Tuitbot's model.

**Consequence:** No Ads API endpoints will appear in any profile.

---

### D-004: Compliance Endpoints Excluded

**Decision:** Compliance batch jobs (3 endpoints) are excluded.

**Rationale:** Enterprise-only, different use case (data governance), no
relevance to growth co-pilot workflow.

---

### D-005: Full-Archive Search Gated by Tier

**Decision:** `x_search_tweets_all` will be defined in the endpoint registry
but returns `x_forbidden` with an actionable message when the user's API tier
does not support full-archive search.

**Rationale:** The endpoint exists and some users have access. Tier detection
is already implemented in `core/x_api/tier.rs`. Gating is cheap.

**Consequence:** Session 04 implements this alongside other read tools.

---

### D-006: New Categories — List and Moderation

**Decision:** Add `List` and `Moderation` variants to `ToolCategory` enum.

**Rationale:** Lists (15 tools) and mutes/blocks/hide-replies (8 tools) are
large enough categories to justify first-class grouping. Putting them in
`Read`/`Engage` would dilute those categories.

**Consequence:** `manifest.rs` gains two new variants. Profile manifests
include the new categories. `category_counts` test threshold increases.

---

### D-007: Profile Assignment Strategy

**Decision:** New read tools go into `full` + `api-readonly`. Batch lookups
that are universally safe also go into `readonly`. New mutation tools go into
`full` only.

**Rationale:** Maintains the existing principle: readonly = minimal safe
surface, api-readonly = all reads, full = everything. New batch reads
(`x_get_tweets_by_ids`, `x_get_users_by_usernames`, `x_get_retweeting_users`,
`x_get_quote_tweets`) are safe enough for the readonly profile.

---

### D-008: Host Allowlist Over Per-Tool Guards

**Decision:** SSRF prevention is enforced at the `XApiHttpClient` level, not
per-tool.

**Rationale:** A single enforcement point is easier to audit and impossible to
bypass by adding a new tool that forgets the check. The host allowlist is
`["api.x.com", "api.twitter.com", "upload.twitter.com"]`.

**Consequence:** Session 03 implements this in the HTTP client with integration
tests that verify rejection of off-allowlist hosts.

---

### D-009: Endpoint Registry Over Macro Generation

**Decision:** Use a declarative Rust struct registry (`Vec<EndpointDef>`) with
test-time validation rather than procedural macros.

**Rationale:** Proc macros add compile-time complexity and are hard to debug.
A plain `Vec<EndpointDef>` is inspectable, testable, and diffable. The build
cost of iterating over ~70 entries at test time is negligible.

**Consequence:** Session 02 builds the registry. No proc-macro crate needed.

---

### D-010: Idempotency via SQLite Table

**Decision:** Extend the existing `tools/idempotency.rs` pattern to all new
mutation tools. Idempotency keys stored in `mcp_idempotency` SQLite table.

**Rationale:** SQLite is already the data layer. Adding a table is cheaper
than introducing Redis or an in-memory cache with durability concerns.

**Consequence:** Session 10 hardens idempotency coverage across all mutation
tools and adds TTL-based expiration.

---

## 3. Risk Register

### R-001: Tool Count Approaches MCP Client Limits

**Risk:** After expansion, the full profile will have ~100 tools. Some MCP
clients may have performance issues with large tool lists.

**Likelihood:** Medium
**Impact:** Medium
**Mitigation:** The profile system already provides smaller surfaces (14 and
39 tools). Document recommended profiles for resource-constrained clients. If
needed, introduce a `full-lite` profile that includes Layer 1 + most-used
Layer 2 tools.

---

### R-002: X API v2 Endpoint Deprecation

**Risk:** X may deprecate or change endpoints during implementation.

**Likelihood:** Low (v2 has been stable)
**Impact:** Low (individual tool removal is straightforward)
**Mitigation:** Endpoint registry includes a `deprecated` flag. Deprecated
tools are logged but still functional until removal.

---

### R-003: Rate Limit Exhaustion from Broader Surface

**Risk:** Agents using more tools consume rate limits faster, leading to
degraded experience.

**Likelihood:** Medium
**Impact:** Medium
**Mitigation:** All tools already surface `rate_limit_reset` timestamps.
Session 03 adds a rate-limit budget advisor tool that reports remaining quota
across endpoint families. The policy engine's `max_mutations_per_hour` cap
provides a hard ceiling.

---

### R-004: Manifest Snapshot Drift

**Risk:** Adding 36 tools increases the chance of manifest snapshot drift
between code and committed JSON artifacts.

**Likelihood:** Medium
**Impact:** Low (CI catches it)
**Mitigation:** Existing `scripts/check-mcp-manifests.sh` CI guard prevents
merge if snapshots drift. Session 10 regenerates all manifests.

---

### R-005: Scope Creep from Spaces API

**Risk:** Spaces API has been partially deprecated by X. Some endpoints may
return empty results or 404.

**Likelihood:** Medium
**Impact:** Low
**Mitigation:** Space tools are read-only and isolated. If endpoints return
errors, tools will return `x_api_error` cleanly. Session 08 includes live
validation if possible, otherwise documents known limitations.

---

### R-006: Idempotency Key Collision

**Risk:** Two different mutation calls with the same idempotency key return
the wrong cached response.

**Likelihood:** Low (keys are scoped to `(tool_name, key)`)
**Impact:** Medium (wrong action taken)
**Mitigation:** The idempotency table uses a composite key of `(tool_name,
idempotency_key)`. Keys are recommended to be UUIDs. TTL expiration limits
the collision window.

---

## 4. Session-by-Session Implementation Backlog

### Session 02: Endpoint Registry and Code Generation Scaffold

**Inputs:** Charter (this document), current `manifest.rs` structure.

**Tasks:**
1. Create `crates/tuitbot-mcp/src/tools/endpoint_registry.rs` with the
   `EndpointDef` struct and a `fn all_endpoints() -> Vec<EndpointDef>` that
   declares all 36 new endpoints plus the 29 existing ones.
2. Add `List` and `Moderation` variants to `ToolCategory` enum in
   `manifest.rs`.
3. Write a test-time validator that ensures every `EndpointDef` has a
   corresponding entry in `all_tools()` and vice versa.
4. Create a codegen helper that produces MCP tool input schemas (JSON Schema)
   from `EndpointDef.params`.
5. Update the `category_counts` test threshold.

**Exit criteria:**
- `cargo test -p tuitbot-mcp` passes.
- Registry declares all 65 endpoints (29 existing + 36 new).
- No new MCP tools registered yet — registry is data-only.

**Handoff:** `session-02-handoff.md` with registry design decisions.

---

### Session 03: Safety Infrastructure — Host Allowlist, SSRF Guard, Restricted Headers

**Inputs:** Session 02 registry, charter safety constraints.

**Tasks:**
1. Add host allowlist validation to `XApiHttpClient::new()` and all request
   methods.
2. Configure `reqwest::Client` with same-host-only redirect policy.
3. Add path-template interpolation with strict slug validation.
4. Write integration tests:
   - Request to `api.x.com` succeeds.
   - Request to `evil.com` is rejected before network I/O.
   - Path traversal attempts (`../`, `%2e%2e`) are rejected.
   - Cross-host redirects are rejected.
5. Document the security model in `docs/security.md`.

**Exit criteria:**
- All safety tests pass.
- `cargo clippy` clean.
- Security doc reviewed.

**Handoff:** `session-03-handoff.md` with security audit results.

---

### Session 04: Batch Lookups + Tweet Metadata Reads

**Inputs:** Session 02 registry, Session 03 safety infra.

**Tasks:**
1. Add 3 new methods to `XApiClient` trait: `get_tweets_by_ids`,
   `get_users_by_usernames`, `get_retweeting_users`.
2. Add 4 more: `get_quote_tweets`, `get_tweet_counts`, `hide_reply`,
   `unhide_reply`.
3. Implement all 7 methods in `XApiHttpClient`.
4. Add corresponding `SocialReadProvider` methods for the 5 read endpoints.
5. Register 7 new MCP tools in `all_tools()` and all 3 server profiles.
6. Write conformance tests with golden fixtures.
7. Implement tier gating for `x_get_tweet_counts` (requires Basic+ tier).

**Exit criteria:**
- 7 new tools appear in manifests.
- Conformance tests pass.
- `x_get_tweet_counts` returns `x_forbidden` on Free tier.

**Handoff:** `session-04-handoff.md` with coverage ratio update.

---

### Session 05: Lists — Read Endpoints

**Inputs:** Session 04 code, registry.

**Tasks:**
1. Add 8 new read methods to `XApiClient` trait for lists.
2. Implement in `XApiHttpClient`.
3. Add `SocialReadProvider` methods.
4. Register 8 new MCP tools in `all_tools()`.
5. Add tools to `full` and `api-readonly` profiles.
6. Write conformance tests.

**Exit criteria:**
- 8 list read tools functional.
- Profile manifests updated.
- All tests pass.

**Handoff:** `session-05-handoff.md`.

---

### Session 06: Lists — Mutation Endpoints

**Inputs:** Session 05 list reads.

**Tasks:**
1. Add 7 mutation methods to `XApiClient`: create/update/delete list,
   add/remove member, follow/unfollow list, pin list.
2. Implement in `XApiHttpClient`.
3. Register 7 new mutation tools (workflow profile only).
4. Integrate with policy engine (rate limits, approval routing, dry-run).
5. Add idempotency support to all 7 tools.
6. Write conformance tests.

**Exit criteria:**
- 7 list mutation tools functional and policy-gated.
- Idempotency works for all 7.
- All tests pass.

**Handoff:** `session-06-handoff.md`.

---

### Session 07: Mutes + Blocks

**Inputs:** Session 06 code.

**Tasks:**
1. Add 6 methods to `XApiClient`: get/mute/unmute, get/block/unblock.
2. Implement in `XApiHttpClient`.
3. Register 6 new MCP tools.
4. Read tools in `full` + `api-readonly`; mutation tools in `full` only.
5. Policy integration for mutation tools.
6. Conformance tests.

**Exit criteria:**
- 6 mute/block tools functional.
- Mutations are policy-gated.
- All tests pass.

**Handoff:** `session-07-handoff.md`.

---

### Session 08: Spaces (Read-Only)

**Inputs:** Session 07 code.

**Tasks:**
1. Add 6 read methods to `XApiClient` for Spaces.
2. Implement in `XApiHttpClient`.
3. Register 6 new read tools in `full` + `api-readonly`.
4. Handle potential deprecation gracefully (return `x_api_error` with
   descriptive message if endpoints 404).
5. Conformance tests (may use mock responses if live Spaces API is unstable).

**Exit criteria:**
- 6 Space read tools functional.
- Graceful handling of 404/deprecated endpoints.
- All tests pass.

**Handoff:** `session-08-handoff.md` with Spaces API stability assessment.

---

### Session 09: Pin Management + Hide Replies

**Inputs:** Session 08 code. Note: `x_hide_reply` and `x_unhide_reply` trait
methods were added in Session 04 but only as MCP tools. Session 09 adds pin
tweet/unpin tweet.

**Tasks:**
1. Add 2 methods to `XApiClient`: `pin_tweet`, `unpin_tweet`.
2. Implement in `XApiHttpClient`.
3. Register 2 new mutation tools (workflow only).
4. Policy integration and idempotency.
5. Conformance tests for all 4 tools (2 hide + 2 pin).

**Exit criteria:**
- 4 mutation tools functional and policy-gated.
- All tests pass.

**Handoff:** `session-09-handoff.md`.

---

### Session 10: Idempotency Hardening + Manifest Regeneration

**Inputs:** All tools from Sessions 04–09.

**Tasks:**
1. Audit every mutation tool for idempotency support. Fill any gaps.
2. Add TTL-based expiration to the `mcp_idempotency` table.
3. Add a background cleanup task for expired entries.
4. Regenerate all 3 profile manifests (`scripts/generate-mcp-manifests.sh`).
5. Update `all_tools()` in `manifest.rs` with final tool count.
6. Run `scripts/check-mcp-manifests.sh` to verify no drift.
7. Update snapshot test baselines.

**Exit criteria:**
- 100% mutation idempotency coverage.
- All 3 manifests regenerated and committed.
- Snapshot tests pass.
- `cargo test --workspace` passes.

**Handoff:** `session-10-handoff.md` with idempotency coverage report.

---

### Session 11: Coverage Validation, KPI Dashboard, Final Docs

**Inputs:** All sessions complete.

**Tasks:**
1. Compute final KPI values (coverage ratio, idempotency %, conformance pass
   rate).
2. Update `docs/mcp-reference.md` with all new tools, categories, and
   examples.
3. Add a "Coverage Report" section to `charter.md` with final numbers.
4. Run full CI suite: `cargo fmt`, `cargo clippy`, `cargo test --workspace`.
5. Create release notes summarizing the expansion.
6. Final handoff document with retrospective.

**Exit criteria:**
- Coverage ratio ≥95%.
- Idempotency coverage = 100%.
- Conformance test pass rate = 100%.
- All docs updated.
- CI green.

**Handoff:** `session-11-handoff.md` (final).

---

## 5. Dependencies Between Sessions

```
Session 02 (Registry)
    │
    ├──→ Session 03 (Safety)
    │        │
    │        └──→ Session 04 (Batch + Metadata)
    │                 │
    │                 ├──→ Session 05 (List Reads)
    │                 │        │
    │                 │        └──→ Session 06 (List Mutations)
    │                 │
    │                 ├──→ Session 07 (Mutes + Blocks)
    │                 │
    │                 ├──→ Session 08 (Spaces)
    │                 │
    │                 └──→ Session 09 (Pins + Hide)
    │
    └──→ Session 10 (Idempotency + Manifests)
              │
              └──→ Session 11 (Validation + Docs)
```

Sessions 05–09 can be parallelized after Session 04 completes. Session 10
requires all tool-adding sessions to be done. Session 11 is the final gate.

---

## 6. Open Questions (Resolved)

All questions were resolved during charter creation. No unresolved items
remain. See Decision Log above for the reasoning behind each resolution.

---

## 7. Files Modified This Session

| File | Action |
|------|--------|
| `docs/roadmap/x-api-surface-expansion/charter.md` | Created |
| `docs/roadmap/x-api-surface-expansion/session-01-handoff.md` | Created |

No code changes were made. This session was documentation and decision-only.
