# Task 04 Prompt: Composite Goal-Oriented MCP Tools

## Objective

Build high-level MCP tools that outperform primitive endpoint wrappers by combining discovery, scoring, context, and safe execution pathways.

## Strategic Rationale

Parity gets entry; composite workflows win outcomes.

## Dependencies

- [`03-PROMPT-safety-policy-and-approval-gateway.md`](/Users/aramirez/Code/ReplyGuy/docs/roadmap/03-PROMPT-safety-policy-and-approval-gateway.md)

## Prompt To Run In Claude Code

```text
You are implementing Task 04 of the MCP superiority roadmap.

Goal:
Add high-value composite MCP tools that encapsulate multi-step growth workflows.

Read first:
- crates/tuitbot-core/src/automation/discovery_loop.rs
- crates/tuitbot-core/src/scoring/
- crates/tuitbot-core/src/content/
- crates/tuitbot-mcp/src/tools/

Implement composite tools (names can vary, keep clear semantics):
1) find_reply_opportunities
   Input: query or keyword set, min_score, limit
   Output: scored candidates with rationale and recommended_action

2) draft_replies_for_candidates
   Input: candidate IDs, tone/style options, mention_policy
   Output: drafts + confidence + risks + char_count

3) propose_and_queue_replies
   Input: candidate IDs and draft strategy
   Behavior: applies policy and either queues approvals or executes based on policy result
   Output: per-item result matrix (queued/executed/blocked + reason)

4) generate_thread_plan
   Input: topic, objective, target audience
   Output: thread outline + suggested hooks + estimated performance reason

Implementation requirements:
- Reuse existing scoring and content generation engines, do not duplicate logic.
- Every action path must call policy gateway from Task 03.
- Return deterministic structured responses only.
- Include rationale fields so agents can explain their decisions.
- Add tests for happy path + partial failure path.

Constraints:
- No hidden side effects in “find” or “draft” tools.
- Execute/queue must be explicit.

Validation:
- cargo test -p tuitbot-mcp
- cargo test -p tuitbot-core

Deliverables:
- new composite tool module
- server tool registration updates
- docs/mcp-reference.md examples for each composite tool
```

## PM Acceptance Checklist

- Composite tools reduce required agent steps.
- Side effects are explicit and policy-governed.
- Output includes rationale and actionable next state.
