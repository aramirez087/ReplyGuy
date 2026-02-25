# TuitBot MCP Superiority Roadmap (Execution Index)

This roadmap turns competitive analysis into implementation-ready prompts for Claude Code.

## Goal

Make TuitBot definitively stronger than `NexusX-MCP/x-v2-server` by combining:
- Direct X action flexibility (tooling parity and beyond)
- TuitBot-native safety, approval, scoring, and analytics loops
- Higher-level orchestration tools that produce growth outcomes, not just API calls

## Source Inputs Reviewed

- `https://github.com/NexusX-MCP/x-v2-server`
- `/Users/aramirez/Code/ReplyGuy/crates/tuitbot-mcp`
- `/Users/aramirez/Code/ReplyGuy/crates/tuitbot-core`
- `/Users/aramirez/Code/ReplyGuy/plugins/openclaw-tuitbot`
- `/Users/aramirez/Code/ReplyGuy/docs/architecture.md`
- `/Users/aramirez/Code/ReplyGuy/docs/mcp-reference.md`

## Execution Order

1. [`00-PRD-tuitbot-vs-x-v2-mcp.md`](/Users/aramirez/Code/ReplyGuy/docs/roadmap/00-PRD-tuitbot-vs-x-v2-mcp.md)
2. [`01-PROMPT-baseline-contracts-and-benchmarks.md`](/Users/aramirez/Code/ReplyGuy/docs/roadmap/01-PROMPT-baseline-contracts-and-benchmarks.md)
3. [`02-PROMPT-direct-x-tools-parity-plus.md`](/Users/aramirez/Code/ReplyGuy/docs/roadmap/02-PROMPT-direct-x-tools-parity-plus.md)
4. [`03-PROMPT-safety-policy-and-approval-gateway.md`](/Users/aramirez/Code/ReplyGuy/docs/roadmap/03-PROMPT-safety-policy-and-approval-gateway.md)
5. [`04-PROMPT-composite-goal-oriented-tools.md`](/Users/aramirez/Code/ReplyGuy/docs/roadmap/04-PROMPT-composite-goal-oriented-tools.md)
6. [`05-PROMPT-context-memory-and-recommendation-engine.md`](/Users/aramirez/Code/ReplyGuy/docs/roadmap/05-PROMPT-context-memory-and-recommendation-engine.md)
7. [`06-PROMPT-openclaw-integration-upgrade.md`](/Users/aramirez/Code/ReplyGuy/docs/roadmap/06-PROMPT-openclaw-integration-upgrade.md)
8. [`07-PROMPT-observability-evals-and-quality-gates.md`](/Users/aramirez/Code/ReplyGuy/docs/roadmap/07-PROMPT-observability-evals-and-quality-gates.md)
9. [`08-PROMPT-dashboard-control-center-and-ux.md`](/Users/aramirez/Code/ReplyGuy/docs/roadmap/08-PROMPT-dashboard-control-center-and-ux.md)
10. [`09-PROMPT-hardening-release-and-positioning.md`](/Users/aramirez/Code/ReplyGuy/docs/roadmap/09-PROMPT-hardening-release-and-positioning.md)

## How To Use

- Run one prompt file at a time in order.
- Do not skip acceptance criteria; each task defines objective completion.
- Keep commits small and atomic per task.
- If a task introduces schema changes, include migration and rollback notes.

## PM Guardrails

- Maintain X policy compliance posture (no engagement manipulation patterns).
- Preserve Composer mode safety defaults.
- Prefer explicit approvals for high-risk actions (follow/unfollow/like bursts).
- Keep deterministic JSON responses in MCP to maximize agent reliability.
