# Task 06 Prompt: OpenClaw Integration Upgrade

## Objective

Upgrade the OpenClaw plugin bridge so agents can reliably discover tool capabilities, policy constraints, and error semantics.

## Strategic Rationale

Tool quality is not only server-side; integration UX determines adoption.

## Dependencies

- [`02-PROMPT-direct-x-tools-parity-plus.md`](/Users/aramirez/Code/ReplyGuy/docs/roadmap/02-PROMPT-direct-x-tools-parity-plus.md)
- [`03-PROMPT-safety-policy-and-approval-gateway.md`](/Users/aramirez/Code/ReplyGuy/docs/roadmap/03-PROMPT-safety-policy-and-approval-gateway.md)

## Prompt To Run In Claude Code

```text
You are implementing Task 06 of the MCP superiority roadmap.

Goal:
Improve OpenClaw plugin behavior and ergonomics for the expanded toolset.

Read first:
- plugins/openclaw-tuitbot/src/index.ts
- plugins/openclaw-tuitbot/src/mcp-client.ts
- plugins/openclaw-tuitbot/src/tool-bridge.ts

Implementation requirements:
1) Add tool metadata enrichment in bridge layer:
   - category (read/mutation/composite/ops)
   - risk_level (low/medium/high)
   - requires_policy_check (bool)
2) Improve error mapping:
   - parse structured MCP error envelope and surface useful actionable messages.
3) Add optional client-side allowlist/denylist by category and risk.
4) Add plugin config defaults for safe startup:
   - register read tools + composite non-mutating tools by default
   - mutation tools opt-in unless explicitly enabled
5) Add integration docs with example OpenClaw configuration snippets.
6) Add unit tests for bridge filtering and error parsing.

Constraints:
- Keep backward compatibility for existing plugin users.
- Do not silently suppress server-side policy; client filtering is additive.

Validation:
- npm --prefix plugins/openclaw-tuitbot test (or add test script first)
- npm --prefix plugins/openclaw-tuitbot run build

Deliverables:
- upgraded plugin bridge
- documented safe defaults
- tests for filtering + error handling
```

## PM Acceptance Checklist

- Plugin exposes clearer, safer tool registration behavior.
- Error handling reduces agent retry confusion.
- Build/test passes for plugin package.
