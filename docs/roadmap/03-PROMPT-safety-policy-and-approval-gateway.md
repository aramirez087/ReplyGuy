# Task 03 Prompt: Safety Policy and Approval Gateway for MCP Mutations

## Objective

Guarantee that all mutation tools (post/reply/follow/unfollow/like/etc.) flow through a policy engine with optional approval routing.

## Strategic Rationale

This is TuitBot’s moat versus thin MCP wrappers: powerful actions with production safety and control.

## Dependencies

- [`00-PRD-tuitbot-vs-x-v2-mcp.md`](/Users/aramirez/Code/ReplyGuy/docs/roadmap/00-PRD-tuitbot-vs-x-v2-mcp.md)
- [`02-PROMPT-direct-x-tools-parity-plus.md`](/Users/aramirez/Code/ReplyGuy/docs/roadmap/02-PROMPT-direct-x-tools-parity-plus.md)

## Prompt To Run In Claude Code

```text
You are implementing Task 03 of the MCP superiority roadmap.

Goal:
Introduce a centralized MCP mutation policy gateway that can allow, block, or route actions to approval queue.

Read first:
- crates/tuitbot-core/src/safety/
- crates/tuitbot-core/src/storage/approval_queue.rs
- crates/tuitbot-mcp/src/server.rs
- crates/tuitbot-mcp/src/tools/
- config.example.toml

Implementation requirements:
1) Add policy config section (example):
   [mcp_policy]
   enforce_for_mutations = true
   require_approval_for = ["post_tweet", "reply_to_tweet", "follow_user", "like_tweet"]
   blocked_tools = []
   dry_run_mutations = false
   max_mutations_per_hour = N
2) Implement a policy evaluator module in tuitbot-core or tuitbot-mcp (choose best ownership, document reasoning).
3) Every mutation tool must call policy evaluator before side effects.
4) Policy decisions:
   - allow_execute
   - route_to_approval
   - deny
5) Add structured audit records for policy decisions to action log storage.
6) Add a new MCP tool:
   - get_policy_status (effective policy, recent blocks/routes)
7) Ensure Composer mode semantics remain conservative; if conflicting settings exist, safest option wins.
8) Add tests:
   - policy deny
   - approval route
   - allowed execution
   - dry-run mutation behavior

Constraints:
- Policy checks must be deterministic.
- Do not bypass existing safety/rate-limit checks.

Validation:
- cargo test -p tuitbot-core
- cargo test -p tuitbot-mcp

Deliverables:
- policy config in config.example.toml + docs/configuration.md
- mutation tools guarded
- policy status tool added
- tests proving no bypass path exists
```

## PM Acceptance Checklist

- No mutation tool bypasses policy.
- Approval routing works from MCP path.
- Policy status is introspectable by agents.
- Config docs updated.
