# Session 06 Handoff: Rate Limits, Retries, and Pagination

## What Was Done

### Contract Layer
- Added `ErrorCode::is_transient()` — subset of `is_retryable()` excluding `XRateLimited`
- Added `PaginationInfo` struct (next_token, result_count, has_more)
- Added `retry_after_ms: Option<u64>` to `ToolError`
- Added `pagination: Option<PaginationInfo>` and `retry_count: Option<u32>` to `ToolMeta`
- Builder methods: `with_pagination()`, `with_retry_count()`, `with_retry_after_ms()`
- Added `ProviderError::ServerError { status, message }` for 5xx errors
- `provider_error_to_response()` populates `retry_after_ms` for rate-limited errors

### Provider Layer
- `map_x_error()` now maps `XApiError::ApiError { status >= 500 }` to `ProviderError::ServerError`
- New `provider/retry.rs`: `RetryPolicy`, `with_retry()`, `RetryingProvider<P>`
- Default policy: 2 retries, 500ms-5s exponential backoff with jitter
- Only `Network` and `ServerError` auto-retry

### Kernel Layer
- All 10 paginated read functions extract `PaginationInfo` from response meta
- Both `SearchMeta` and `UsersMeta` sources handled

### Tools Layer
- All workflow read tools wrap `XApiProvider` with `RetryingProvider`
- Idempotency checks in `post_tweet`, `reply_to_tweet`, `quote_tweet`, `post_thread`
- Idempotency checks in `like_tweet`, `follow_user`, `retweet`, `bookmark_tweet`
- Undo operations and delete_tweet skip idempotency (inherently safe)

### Server Layer (API Profile)
- All 16 read/health tools wrapped with `RetryingProvider`
- Idempotency checks in 4 write tools + 4 engage tools

### State
- `IdempotencyStore` added to both `AppState` and `ApiState` (via `Arc`)
- Initialized in both `run_stdio_server()` and `run_api_server()`

## Files Changed

| File | Lines | Change |
|------|-------|--------|
| `contract/error_code.rs` | +25 | `is_transient()`, 2 tests |
| `contract/envelope.rs` | +70 | PaginationInfo, retry_after_ms, retry_count, builders, 4 tests |
| `contract/error.rs` | +35 | ServerError variant, retry_after_ms in responses, 3 tests |
| `contract/mod.rs` | +1 | Re-export PaginationInfo |
| `provider/x_api.rs` | +4 | 5xx detection in map_x_error |
| `provider/retry.rs` | +380 | **NEW** — RetryPolicy, RetryingProvider, 9 tests |
| `provider/mod.rs` | +1 | Register retry module |
| `kernel/read.rs` | +25 | Pagination extraction in 10 functions |
| `tools/idempotency.rs` | +110 | **NEW** — IdempotencyStore, 4 tests |
| `tools/mod.rs` | +1 | Register idempotency module |
| `tools/x_actions/read.rs` | +2 | RetryingProvider wrapping |
| `tools/x_actions/write.rs` | +16 | Idempotency checks (4 tools) |
| `tools/x_actions/engage.rs` | +16 | Idempotency checks (4 tools) |
| `tools/x_actions/tests/mod.rs` | +4 | IdempotencyStore in make_state |
| `tools/eval_harness.rs` | +1 | IdempotencyStore in state |
| `tools/composite/tests.rs` | +1 | IdempotencyStore in state |
| `server/api.rs` | +50 | RetryingProvider + idempotency |
| `state.rs` | +5 | IdempotencyStore fields |
| `lib.rs` | +3 | IdempotencyStore initialization |

## Test Results

- 220 MCP tests pass (18 new for this session)
- 31 server integration tests pass
- All `cargo fmt`, `cargo clippy`, `cargo test` green

## What's Next (Session 07 Guidance)

Possible focus areas:
1. **Observability** — Structured logging for retry attempts, idempotency hits, pagination traversals
2. **Retry count propagation** — `retry_count` is in `ToolMeta` but not yet populated from `RetryingProvider` (the `with_retry` return value is currently discarded). Wiring this through would require kernel changes.
3. **Write tool retry** — Currently only read tools have `RetryingProvider`. Write/engage tools could benefit from retry on transient errors (but need careful idempotency consideration).
4. **Rate limit budget tracking** — Track remaining API quota across tools to proactively throttle before hitting 429s.
5. **WebSocket events** — Emit real-time events for retry/rate-limit/idempotency occurrences.
