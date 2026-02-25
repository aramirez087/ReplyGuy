# Task 01 Prompt: Baseline Contracts and Benchmark Harness

## Objective

Establish a deterministic MCP contract foundation and benchmark baseline so all later tasks can be validated against measurable improvements.

## Why This Comes First

Without standardized response shapes and baseline measurements, future tool additions and quality claims are hard to verify.

## Dependencies

- Read PRD: [`00-PRD-tuitbot-vs-x-v2-mcp.md`](/Users/aramirez/Code/ReplyGuy/docs/roadmap/00-PRD-tuitbot-vs-x-v2-mcp.md)

## Prompt To Run In Claude Code

```text
You are implementing Task 01 of the MCP superiority roadmap.

Goal:
Create a unified MCP response envelope and a baseline benchmark harness for current tool quality/latency.

Read first:
- crates/tuitbot-mcp/src/server.rs
- crates/tuitbot-mcp/src/tools/*.rs
- docs/mcp-reference.md

Implementation requirements:
1) Add a shared tool response model in tuitbot-mcp (e.g., crates/tuitbot-mcp/src/tools/response.rs):
   - success: bool
   - data: object or array
   - error: nullable { code, message, retryable }
   - meta: optional { tool_version, elapsed_ms, mode, approval_mode }
2) Refactor at least 5 representative existing tools to use the shared response helper:
   - get_capabilities
   - health_check
   - get_stats
   - list_pending_approvals
   - get_discovery_feed
3) Add a migration-safe strategy: old text payload compatibility remains acceptable but must be documented.
4) Create a benchmark utility:
   - command or test helper that executes selected MCP tools and records response times and schema validation pass/fail.
   - output file: docs/roadmap/artifacts/task-01-baseline-benchmark.json
5) Add docs section in docs/mcp-reference.md describing the response envelope.
6) Add tests for serialization and error envelope behavior.

Constraints:
- Do not break existing tool names.
- Keep changes incremental and compile-safe.
- Avoid introducing new runtime dependencies unless justified.

Validation:
- cargo test -p tuitbot-mcp
- cargo test -p tuitbot-core

Deliverables:
- Code changes + tests
- docs update
- benchmark artifact JSON committed under docs/roadmap/artifacts/
- short markdown summary at docs/roadmap/artifacts/task-01-summary.md with:
  - tools measured
  - p50/p95 latency
  - schema pass rate
```

## PM Acceptance Checklist

- Shared response envelope exists and is used by representative tools.
- Benchmark artifact and summary are committed.
- No MCP tool name regressions.
- Tests pass.
