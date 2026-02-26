# Session 03: Runtime Profile Design

## Decision

Introduce two separate server structs (`TuitbotMcpServer` and `ApiMcpServer`) to provide compile-time profile separation:

- **`workflow`** (default): Full TuitBot stack — DB, LLM, policy gating, ~60+ tools.
- **`api`**: Lightweight X client — no DB, no LLM, no policy gating, ~24 tools.

## Why Two Structs (Not Runtime Filtering)

The `#[tool_router]` macro (rmcp) registers all `#[tool]` methods at compile time — there is no runtime `tools/list` filtering. Two server structs give each profile an accurate `tools/list`, and the compile-time separation prevents accidentally calling workflow tools from the API server.

## Profile Enum

```rust
pub enum Profile {
    Api,
    Workflow,
}
```

- `Display` / `FromStr` implemented for CLI parsing.
- Case-insensitive parsing: `"api"`, `"API"`, `"workflow"`, etc.

## State Structs

| Struct | DB | LLM | X Client | User ID |
|--------|----|-----|----------|---------|
| `AppState` (workflow) | Required | Optional | Optional | Optional |
| `ApiState` (api) | None | None | **Required** | **Required** |

`ApiState.x_client` is non-optional. `run_api_server` fails fast if tokens are missing/expired — an API profile with no X client has zero usable tools.

## API Profile Tool Inventory (~24 tools)

| Category | Count | Tools |
|----------|-------|-------|
| Read | 7 | get_tweet_by_id, x_get_user_by_username, x_search_tweets, x_get_user_mentions, x_get_user_tweets, x_get_home_timeline, x_get_me |
| Write | 5 | x_post_tweet, x_reply_to_tweet, x_quote_tweet, x_delete_tweet, x_post_thread |
| Engage | 5 | x_like_tweet, x_follow_user, x_unfollow_user, x_retweet, x_unretweet |
| Media | 1 | x_upload_media |
| Utils | 3 | get_config, validate_config, score_tweet |
| Meta | 2 | get_capabilities, health_check |
| Mode | 1 | get_mode |

## CLI Usage

```bash
tuitbot mcp serve                     # default: workflow
tuitbot mcp serve --profile api       # lightweight X client
tuitbot mcp serve --profile workflow   # explicit workflow
```
