# Session 09 Handoff — Conformance Harness & Coverage Report

## What Changed

Before this session, the MCP tool suite had extensive mock-based conformance tests (kernel, contract, golden fixtures, eval scenarios) but lacked:
1. A way to run tests against real X API sandbox credentials
2. A machine-readable coverage report showing which endpoints are implemented vs untested
3. A harness wrapper script for CI/release gating
4. Clear separation between deterministic (offline) and networked (live) tests

This session adds all four.

### Deliverables

1. **Live conformance tests** (`conformance_tests/live.rs`) — 10 `#[ignore]` tests that exercise real X API when env vars are set. Covers app-only auth, user OAuth auth, safe write+delete cycle, like+unlike cycle, pagination, rate-limit detection, and aggregate reporting.

2. **Coverage report generator** (`conformance_tests/coverage.rs`) — Introspects the tool manifest to produce JSON + markdown reports. Categorizes every tool by layer (curated L1 vs generated L2), category, auth requirements, test coverage status, tier gating, and credential gating.

3. **Harness wrapper script** (`scripts/run-conformance.sh`) — Orchestrates all test phases: deterministic conformance, coverage report generation, and optionally live sandbox tests.

4. **Coverage report artifacts** — Published to `docs/generated/coverage-report.{json,md}` and `roadmap/artifacts/session-09-coverage-report.{json,md}`.

5. **Manifest snapshot regeneration** — Updated `session-06-tool-manifest.json` to reflect current tool set (109 tools).

## Coverage Report Summary

### Tool Inventory

| Metric | Count |
|--------|-------|
| Total tools | 109 |
| Curated (L1) | 73 |
| Generated (L2) | 36 |
| Mutation tools | 38 |
| Read-only tools | 71 |

### Test Coverage

| Test Type | Count | Description |
|-----------|-------|-------------|
| Kernel conformance | 27 | Mock-based envelope validation for all X API kernel tools |
| Contract envelope | 18 | Workflow tool envelope validation (analytics, approval, etc.) |
| Live (sandbox) | 9 | Real X API tests (gated on credentials) |
| **Unique tested** | **45/109 (41.3%)** | Tools with at least one test |
| Untested | 64 | Tools lacking any test coverage |

### By Profile

| Profile | Total | Mutations | Read-Only |
|---------|-------|-----------|-----------|
| readonly | 14 | 0 | 14 |
| api_readonly | 40 | 0 | 40 |
| write | 104 | 35 | 69 |
| admin | 108 | 38 | 70 |

### Tier-Gated Distribution

- **All tiers**: 14 tools (available to readonly profile)
- **api_readonly+**: 26 tools
- **write+**: 65 tools
- **admin only**: 4 tools (universal request tools)

## Largest Coverage Gaps

### By Category (completely untested)

| Category | Untested | Total | Gap |
|----------|----------|-------|-----|
| list | 15 | 15 | 100% — All generated L2 spec-pack tools |
| moderation | 8 | 8 | 100% — All generated L2 spec-pack tools |
| composite | 4 | 4 | 100% — Workflow orchestration tools |
| content | 4 | 4 | 100% — LLM content generation tools |
| media | 1 | 1 | 100% — Upload media tool |

### Key Untested Areas

1. **All 36 generated (L2) spec-pack tools** — Lists (15), Moderation/Mutes/Blocks (8), Spaces (6), Tweet metadata (4), Pin management (2), Batch lookups (3). These are auto-generated from endpoint definitions and route through the universal request handler. They need integration tests.

2. **Composite orchestration tools** — `draft_replies_for_candidates`, `find_reply_opportunities`, `propose_and_queue_replies`, `generate_thread_plan`. These are multi-step workflows that require full state (DB + LLM + X client).

3. **Content generation tools** — `generate_reply`, `generate_tweet`, `generate_thread`, `suggest_topics`. Require LLM provider.

4. **Dry-run tools** — `x_post_tweet_dry_run`, `x_post_thread_dry_run`. Validation-only paths.

## Design Decisions

### Live Test Architecture

- All live tests use `#[ignore]` — they never run in normal CI
- Credentials via env vars: `TUITBOT_TEST_BEARER_TOKEN`, `TUITBOT_TEST_USER_ID`, `TUITBOT_TEST_KNOWN_TWEET_ID`, `TUITBOT_TEST_KNOWN_USERNAME`
- Graceful skip: if an env var is missing, the test logs and returns rather than panicking
- The `build_app_only_client()` helper constructs `XApiHttpClient::new(token)` directly
- Live tests validate the same envelope structure as mock tests: `success`, `data`, `meta`, `tool_version`

### Coverage Report Design

- Generated deterministically from manifest metadata — no network calls needed
- Dual format: JSON (machine-readable, 73KB) + markdown (human-readable, 4.6KB)
- Published to both `docs/generated/` (for repo consumers) and `roadmap/artifacts/` (for session continuity)
- Includes both positive (what's tested) and negative (what's not) coverage

### Harness Script

Three modes:
- `scripts/run-conformance.sh` — deterministic tests + coverage report
- `scripts/run-conformance.sh --live` — adds live sandbox tests
- `scripts/run-conformance.sh --report-only` — just regenerates coverage report

## Files Changed

| File | Change |
|------|--------|
| `crates/tuitbot-mcp/src/tools/conformance_tests/mod.rs` | +`coverage` and `live` modules, expanded docs |
| `crates/tuitbot-mcp/src/tools/conformance_tests/live.rs` | New: 10 live conformance tests |
| `crates/tuitbot-mcp/src/tools/conformance_tests/coverage.rs` | New: coverage report generator + 1 test |
| `scripts/run-conformance.sh` | New: harness wrapper script |
| `docs/generated/coverage-report.json` | Generated: machine-readable coverage |
| `docs/generated/coverage-report.md` | Generated: human-readable coverage |
| `roadmap/artifacts/session-09-coverage-report.json` | Generated: coverage artifact |
| `roadmap/artifacts/session-09-coverage-report.md` | Generated: coverage artifact |
| `roadmap/artifacts/session-06-tool-manifest.json` | Regenerated: manifest snapshot (109 tools) |
| `docs/roadmap/x-api-surface-expansion/session-09-handoff.md` | This document |

## Test Results

```
cargo fmt --all && cargo fmt --all --check          # clean
cargo clippy --workspace -- -D warnings             # clean
RUSTFLAGS="-D warnings" cargo test --workspace -- --test-threads=1
  tuitbot-cli:  118 passed, 0 failed
  tuitbot-core: 730 passed, 0 failed
  tuitbot-mcp:  408 passed, 0 failed, 10 ignored (live tests)
  tuitbot-server: 31 passed, 0 failed
  Total: 1288 passed, 0 failed, 10 ignored
```

## How to Run the Conformance Suite

### Deterministic (no credentials needed)

```bash
# Full suite
bash scripts/run-conformance.sh

# Individual phases
cargo test -p tuitbot-mcp conformance_    # kernel conformance
cargo test -p tuitbot-mcp contract_test   # contract envelope
cargo test -p tuitbot-mcp golden_fixtures # golden fixture drift
cargo test -p tuitbot-mcp boundary_       # profile isolation
cargo test -p tuitbot-mcp eval_session09  # eval scenarios D-G
cargo test -p tuitbot-mcp coverage        # generate coverage report
```

### Live (sandbox credentials required)

```bash
export TUITBOT_TEST_BEARER_TOKEN="your_bearer_token"
export TUITBOT_TEST_USER_ID="your_user_id"
export TUITBOT_TEST_KNOWN_TWEET_ID="a_known_tweet_id"
export TUITBOT_TEST_KNOWN_USERNAME="a_known_username"

bash scripts/run-conformance.sh --live

# Or run live tests directly
cargo test -p tuitbot-mcp live -- --ignored
```

## Recommended Follow-Up Backlog

### Priority 1 (High)
1. **Generated spec-pack tool tests** — Add integration tests for all 36 L2 tools. They share the universal request handler, so a single parameterized test per group (lists, mutes, blocks, spaces) would cover most paths.
2. **Composite workflow tests** — Add end-to-end tests for `draft_replies_for_candidates`, `find_reply_opportunities`, `propose_and_queue_replies` with full state setup (DB + LLM + X client mocks).
3. **Content generation tests** — Test `generate_reply`, `generate_tweet`, `generate_thread` with MockLlmProvider.

### Priority 2 (Medium)
4. **Dry-run tool tests** — Validate `x_post_tweet_dry_run` and `x_post_thread_dry_run`.
5. **Media upload test** — Test `x_upload_media` with mock file data.
6. **CI integration** — Wire `scripts/run-conformance.sh` into GitHub Actions as a quality gate.
7. **Coverage threshold enforcement** — Fail CI if tested percentage drops below a threshold (e.g., 40%).

### Priority 3 (Low)
8. **Sandbox CI credentials** — Set up GitHub Actions secrets for live conformance tests.
9. **Historical coverage tracking** — Store coverage JSON per commit for trend analysis.
10. **Caller-provided idempotency keys** — Expose the `idempotency_key` column from Session 08.

## What's NOT Changed

- Existing mock-based conformance tests — untouched
- Contract tests — untouched
- Golden fixture tests — untouched
- Eval harness (scenarios A-G) — untouched
- Boundary tests — untouched
- Idempotency/audit system from Session 08 — untouched
- All 109 tools in the manifest — no tools added or removed
