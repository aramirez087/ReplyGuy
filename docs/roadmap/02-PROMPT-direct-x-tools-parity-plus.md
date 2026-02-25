# Task 02 Prompt: Direct X Tools Parity-Plus

## Objective

Deliver direct X action and retrieval tools that match or exceed x-v2-server utility while staying aligned with TuitBot architecture.

## Competitive Intent

Close the “tooling flexibility” gap immediately so TuitBot is not seen as less capable by agent developers.

## Dependencies

- [`00-PRD-tuitbot-vs-x-v2-mcp.md`](/Users/aramirez/Code/ReplyGuy/docs/roadmap/00-PRD-tuitbot-vs-x-v2-mcp.md)
- [`01-PROMPT-baseline-contracts-and-benchmarks.md`](/Users/aramirez/Code/ReplyGuy/docs/roadmap/01-PROMPT-baseline-contracts-and-benchmarks.md)

## Prompt To Run In Claude Code

```text
You are implementing Task 02 of the MCP superiority roadmap.

Goal:
Add direct X MCP tools for parity-plus against x-v2-server, reusing tuitbot-core adapters where possible.

Read first:
- crates/tuitbot-core/src/x_api/mod.rs
- crates/tuitbot-core/src/x_api/client.rs
- crates/tuitbot-mcp/src/server.rs
- crates/tuitbot-mcp/src/tools/

Implement these MCP tools (or closest practical equivalents with explicit docs):
Read tools:
- get_tweet_by_id
- get_user_by_username
- search_tweets
- get_user_mentions
- get_user_tweets

Mutation tools:
- post_tweet
- reply_to_tweet
- quote_tweet (if unavailable natively, provide fallback strategy and explicit limitation)
- like_tweet
- follow_user
- unfollow_user

Optional list/trending tools:
- get_trending_topics (only if API access is available and stable for current auth model)
- create_list / add_list_member / remove_list_member / get_owned_lists (only if feasible in current auth setup)

Implementation requirements:
1) Extend XApiClient trait and XApiHttpClient only where needed and test each new method.
2) Place MCP-facing wrappers in a dedicated module (e.g., tools/x_actions.rs).
3) Use Task 01 response envelope for all new tools.
4) Add clear error taxonomy for:
   - tier not permitted
   - auth expired
   - rate limited
   - endpoint unsupported
5) Update get_capabilities output to include a `direct_tools` capability map.
6) Add/update tests:
   - core client unit tests (wiremock where applicable)
   - MCP tool behavior tests for validation + error handling
7) Update docs/mcp-reference.md with all new tools and examples.

Constraints:
- Preserve existing behavior for automation loops.
- Do not add risky default behavior; these are explicit tool invocations.

Validation:
- cargo test -p tuitbot-core
- cargo test -p tuitbot-mcp

Deliverables:
- New direct tools live and documented
- Capability map extended
- Test evidence for successful and failure paths
```

## PM Acceptance Checklist

- High-value parity tools are present.
- Endpoint limitations are explicit and machine-detectable.
- Responses are schema-consistent.
- Existing automation flows are unaffected.
