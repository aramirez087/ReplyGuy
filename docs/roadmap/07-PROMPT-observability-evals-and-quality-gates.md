# Task 07 Prompt: Observability, Evals, and Quality Gates

## Objective

Measure and prove that TuitBot MCP is better than thin wrappers on reliability, safety, and outcome-oriented workflows.

## Strategic Rationale

Without evidence, superiority claims are marketing-only.

## Dependencies

- [`01-PROMPT-baseline-contracts-and-benchmarks.md`](/Users/aramirez/Code/ReplyGuy/docs/roadmap/01-PROMPT-baseline-contracts-and-benchmarks.md)
- [`04-PROMPT-composite-goal-oriented-tools.md`](/Users/aramirez/Code/ReplyGuy/docs/roadmap/04-PROMPT-composite-goal-oriented-tools.md)

## Prompt To Run In Claude Code

```text
You are implementing Task 07 of the MCP superiority roadmap.

Goal:
Add observability and an eval harness to quantify improvements from new MCP capabilities.

Read first:
- crates/tuitbot-core/src/storage/action_log.rs
- crates/tuitbot-mcp/src/tools/actions.rs
- docs/operations.md

Implementation requirements:
1) Add MCP execution telemetry schema/table or extend existing action log with MCP dimensions:
   - tool_name
   - category
   - latency_ms
   - success
   - error_code
   - policy_decision
2) Add MCP tools:
   - get_mcp_tool_metrics (time-windowed aggregates)
   - get_mcp_error_breakdown
3) Build eval harness for defined scenarios (script or test command):
   - Scenario A: raw direct reply flow
   - Scenario B: composite find->draft->queue flow
   - Scenario C: blocked-by-policy mutation
4) Produce artifact files:
   - docs/roadmap/artifacts/task-07-eval-results.json
   - docs/roadmap/artifacts/task-07-eval-summary.md
5) Add quality gates in CI docs (or workflow if in scope):
   - fail if schema validation drops below threshold
   - fail if unknown errors exceed threshold in scenario tests

Constraints:
- Keep telemetry privacy-safe (no secret leakage).
- Keep storage overhead bounded.

Validation:
- cargo test -p tuitbot-core
- cargo test -p tuitbot-mcp
- run eval harness command and verify artifacts exist

Deliverables:
- telemetry capture
- metrics MCP tools
- eval artifacts and summary
```

## PM Acceptance Checklist

- Metrics are queryable via MCP.
- Eval results are reproducible and committed.
- Quality gates are explicit and enforceable.
