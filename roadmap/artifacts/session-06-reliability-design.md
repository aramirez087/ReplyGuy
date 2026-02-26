# Session 06: Reliability Design — Architecture Decision Record

## Decision: Retry at the Provider Layer

**Context:** MCP tools need automatic retry for transient failures without polluting kernel logic.

**Decision:** `RetryingProvider<P>` wraps any `SocialReadProvider` with retry logic. The kernel stays pure.

**Consequences:**
- Retry is transparent to kernel and tool layers
- `RetryPolicy` is configurable (default: 2 retries, 500ms-5s exponential backoff)
- Only `Network` and `ServerError` auto-retry; `RateLimited` passes through with `retry_after_ms`

## Decision: Rate-Limited Errors Return Immediately

**Context:** 429 responses from X API include retry-after hints. Auto-retrying them wastes the caller's budget.

**Decision:** Rate-limited errors are `is_retryable()` but NOT `is_transient()`. The `retry_after_ms` field in `ToolError` lets agents decide when to retry.

**Consequences:**
- Agents get explicit timing guidance
- No wasted retries against rate limits
- `retry_after_ms = retry_after_seconds * 1000`

## Decision: ServerError Variant for 5xx

**Context:** `XApiError::ApiError { status >= 500 }` was falling into the generic `Other` catch-all, making it indistinguishable from 4xx client errors.

**Decision:** New `ProviderError::ServerError { status, message }` variant, mapped from 5xx status codes in `map_x_error()`.

**Consequences:**
- 5xx errors are retryable (transient)
- 4xx errors remain non-retryable (caught by `Other`)
- `RetryPolicy::should_retry()` matches on `ServerError`

## Decision: Pagination Normalization in Kernel

**Context:** `SearchMeta` and `UsersMeta` have different shapes but both carry `next_token` and `result_count`.

**Decision:** `PaginationInfo { next_token, result_count, has_more }` extracted in kernel read functions and attached to `ToolMeta.pagination`.

**Consequences:**
- Agents get a uniform pagination contract regardless of response type
- `has_more = next_token.is_some()` — simple boolean check

## Decision: In-Memory Idempotency Guard

**Context:** Agent retry storms can send the same mutation twice within seconds.

**Decision:** `IdempotencyStore` hashes `(tool_name, params_json)` via `DefaultHasher` and rejects duplicates within a 30-second window. Applied to mutation tools only (post, reply, quote, thread, like, follow, retweet, bookmark). Skip undo operations and delete (inherently idempotent).

**Consequences:**
- Zero external dependencies (in-memory `HashMap<u64, Instant>`)
- Expired entries evicted on each `check_and_record()` call
- Both workflow and API profiles share the same guard via `Arc<IdempotencyStore>` in state
