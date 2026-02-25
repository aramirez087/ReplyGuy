# Task 08 Prompt: Dashboard MCP Control Center and UX

## Objective

Give operators a first-class interface to govern MCP behavior, monitor policy decisions, and inspect tool execution quality.

## Strategic Rationale

Enterprise trust and day-to-day operability require UI controls and transparent traces, not only CLI/MCP primitives.

## Dependencies

- [`03-PROMPT-safety-policy-and-approval-gateway.md`](/Users/aramirez/Code/ReplyGuy/docs/roadmap/03-PROMPT-safety-policy-and-approval-gateway.md)
- [`07-PROMPT-observability-evals-and-quality-gates.md`](/Users/aramirez/Code/ReplyGuy/docs/roadmap/07-PROMPT-observability-evals-and-quality-gates.md)

## Prompt To Run In Claude Code

```text
You are implementing Task 08 of the MCP superiority roadmap.

Goal:
Add a dashboard section for MCP governance, policy settings, and execution insights.

Read first:
- dashboard/src/routes/(app)/settings/+page.svelte
- dashboard/src/lib/stores/settings.ts
- crates/tuitbot-server/src/routes/settings.rs
- crates/tuitbot-server/src/routes/activity.rs
- crates/tuitbot-server/src/routes/approval.rs

Implementation requirements:
1) Add MCP Governance page (new route suggested: /mcp):
   - policy toggle summary
   - mutation approval requirements
   - blocked tools list editor
   - dry-run mode toggle
2) Add MCP Activity panel:
   - recent tool executions
   - success/error counts
   - policy blocks and approval routes
3) Add API endpoints in tuitbot-server as needed:
   - GET/PUT policy config
   - GET metrics summary
   - GET recent MCP executions
4) Ensure state management via dashboard stores with optimistic UI where safe.
5) Add basic UX safeguards:
   - confirm on risky policy relaxations
   - inline hints for security impact
6) Add tests (frontend where available + server route tests).

Constraints:
- Preserve existing UI visual language.
- Keep mobile and desktop usability intact.

Validation:
- cargo test -p tuitbot-server
- npm --prefix dashboard run test (if configured)
- npm --prefix dashboard run build

Deliverables:
- new governance UI
- server routes backing the UI
- docs update with screenshots or flow description
```

## PM Acceptance Checklist

- Operators can inspect and modify MCP policy without editing files manually.
- MCP execution health is visible in dashboard.
- Risky config changes require explicit confirmation.
