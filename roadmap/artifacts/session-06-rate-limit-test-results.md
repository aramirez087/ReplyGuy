# Session 06: Rate Limit & Retry Test Results

## Test Suite: `cargo test -p tuitbot-mcp retry`

| Test | Status | Description |
|------|--------|-------------|
| `delay_computation_exponential` | PASS | Verifies 100ms, 200ms, 400ms base delays with <25% jitter |
| `delay_capped_at_max` | PASS | 2s * 2^3 = 16s, capped to 5s max |
| `should_retry_network_and_server` | PASS | Network + ServerError are retryable |
| `should_not_retry_rate_limited` | PASS | RateLimited is NOT retried |
| `should_not_retry_auth_expired` | PASS | AuthExpired + Forbidden skip retry |
| `retry_succeeds_after_transient` | PASS | 1 fail + 1 success = 2 calls total |
| `retry_exhausted_returns_error` | PASS | 1 initial + 2 retries = 3 calls, returns last error |
| `rate_limited_passes_through` | PASS | No retries, immediate RateLimited return |
| `server_error_retried` | PASS | ServerError(502) retried, succeeds on 2nd try |

## Test Suite: `cargo test -p tuitbot-mcp idempotency`

| Test | Status | Description |
|------|--------|-------------|
| `blocks_duplicate_within_window` | PASS | Same (tool, params) rejected within 30s |
| `allows_different_params` | PASS | Different params = different fingerprint |
| `allows_after_eviction` | PASS | Expired entries evicted, call proceeds |
| `different_tools_same_params_allowed` | PASS | Different tool names = independent |

## Test Suite: `cargo test -p tuitbot-mcp error_code`

| Test | Status | Description |
|------|--------|-------------|
| `is_transient_subset_of_retryable` | PASS | All transient codes are also retryable |
| `rate_limited_retryable_not_transient` | PASS | XRateLimited: retryable=true, transient=false |

## Test Suite: `cargo test -p tuitbot-mcp server_error`

| Test | Status | Description |
|------|--------|-------------|
| `server_error_maps_correctly` | PASS | 502 -> XApiError, retryable=true |
| `rate_limited_response_has_retry_after_ms` | PASS | 30s retry_after -> 30000ms in JSON |

## CI Checklist

```
cargo fmt --all --check       ✓ (0 errors)
RUSTFLAGS="-D warnings" cargo test --workspace  ✓ (220 MCP + 31 server + all others pass)
cargo clippy --workspace -- -D warnings          ✓ (0 warnings)
```
