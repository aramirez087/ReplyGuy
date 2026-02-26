# Session 03: Runtime Decoupling Report

## Objective

Make the MCP server launchable without a database or LLM provider, enabling a lightweight "API profile" for generic X API client use cases.

## What Changed

### New Modules

| File | Lines | Purpose |
|------|-------|---------|
| `kernel/write.rs` | ~130 | DB-free write operations (post, reply, quote, delete, thread) |
| `kernel/engage.rs` | ~90 | DB-free engagement operations (like, follow, unfollow, retweet, unretweet) |
| `kernel/media.rs` | ~65 | DB-free media upload |
| `kernel/utils.rs` | ~70 | Tweet length validation + get_me |
| `server/api.rs` | ~250 | ApiMcpServer with ~24 tools |
| `server/mod.rs` | ~8 | Re-exports both servers |

### Modified Modules

| File | Change |
|------|--------|
| `state.rs` | Added `Profile` enum, `ApiState`, `SharedApiState` |
| `provider/mod.rs` | Added 4 trait methods with defaults |
| `provider/x_api.rs` | Implemented 4 new methods, `map_x_error` → `pub(crate)` |
| `kernel/mod.rs` | Registered 4 new submodules |
| `kernel/read.rs` | Added 3 functions (mentions, user_tweets, timeline) |
| `kernel/tests.rs` | +~200 lines of new tests |
| `lib.rs` | Added `run_api_server`, `run_server`, `Profile` export |
| `tools/x_actions/read.rs` | Delegated 3 reads to kernel |
| `tools/x_actions/validate.rs` | Re-exports from `kernel::utils` |
| `server.rs` → `server/workflow.rs` | Moved (no code changes) |
| `cli/commands/mod.rs` | Added `--profile` arg to `McpSubcommand::Serve` |
| `cli/commands/mcp.rs` | Parse profile, dispatch via `run_server` |
| `cli/main.rs` | Pass profile through |

### Provider Trait Extension

`SocialReadProvider` gained 4 new methods with default error implementations:
- `get_user_mentions` / `get_user_tweets` / `get_home_timeline` / `get_me`

Existing mock providers in tests don't break because defaults return `ProviderError::Other`.

### Kernel Layer Expansion

The kernel layer grew from 1 module (read) to 5 (read, write, engage, media, utils):
- **Read**: 3 → 6 functions
- **Write**: 5 DB-free functions taking `&dyn XApiClient`
- **Engage**: 5 DB-free functions taking `&dyn XApiClient`
- **Media**: 1 function extracted from `tools/x_actions/media.rs`
- **Utils**: `check_tweet_length` + `compute_weighted_length` + `get_me`

Write/engage take `&dyn XApiClient` directly (not a provider trait) — pragmatic for this session. A `SocialWriteProvider` trait can be introduced when a scraper backend needs write support.

## Test Coverage

153 tests pass for tuitbot-mcp:
- Kernel read tests: 10 (original) + 6 (new reads + get_me)
- Kernel write tests: 8 (post, reply, quote, delete, thread)
- Kernel engage tests: 7 (like, follow, unfollow, retweet, unretweet)
- Kernel utils tests: 2 (check_tweet_length)
- Profile tests: 3 (display, from_str valid, from_str invalid)
- All existing tool tests continue to pass

## What Did NOT Change

- `server/workflow.rs` (868 lines, over 500-line limit) — out of scope
- `requests.rs` — shared by both servers as-is
- No existing workflow behavior changed
- No DB schema changes
