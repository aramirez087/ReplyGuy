# Task 05 Prompt: Context Memory and Recommendation Engine

## Objective

Create MCP tools that leverage TuitBot’s historical data to generate context-aware recommendations and improve action quality.

## Strategic Rationale

x-v2-style wrappers return raw API data; TuitBot should return informed strategy-grade context.

## Dependencies

- [`04-PROMPT-composite-goal-oriented-tools.md`](/Users/aramirez/Code/ReplyGuy/docs/roadmap/04-PROMPT-composite-goal-oriented-tools.md)

## Prompt To Run In Claude Code

```text
You are implementing Task 05 of the MCP superiority roadmap.

Goal:
Expose context-aware intelligence tools powered by existing storage and analytics.

Read first:
- crates/tuitbot-core/src/storage/author_interactions.rs
- crates/tuitbot-core/src/storage/replies.rs
- crates/tuitbot-core/src/storage/analytics.rs
- crates/tuitbot-core/src/strategy/
- crates/tuitbot-mcp/src/tools/analytics.rs

Implement MCP tools:
1) get_author_context
   Input: author username or ID
   Output: prior interactions, response rates, top topics, sentiment/risk signals

2) recommend_engagement_action
   Input: author + tweet text + campaign objective
   Output: recommended action (reply/skip/observe), confidence, policy considerations

3) topic_performance_snapshot
   Input: lookback window
   Output: topics ranked by outcomes + “double down / reduce” recommendations

Implementation requirements:
- Build a context aggregation service module (prefer tuitbot-core if reusable outside MCP).
- Reuse existing strategy report metrics where possible.
- Provide clear provenance in response fields (which signals informed recommendation).
- Integrate with policy metadata when recommendation includes mutation action.
- Add tests with seeded DB fixtures.

Constraints:
- Avoid opaque “black box” recommendations; include contributing factors.
- Keep latency practical for interactive agent use.

Validation:
- cargo test -p tuitbot-core
- cargo test -p tuitbot-mcp

Deliverables:
- context aggregation module
- three MCP tools with schema-compliant responses
- docs update with recommendation interpretation guidance
```

## PM Acceptance Checklist

- Recommendation outputs are explainable and grounded in stored data.
- Context tool gives immediate value beyond raw endpoint data.
- Tests cover empty history and rich history cases.
