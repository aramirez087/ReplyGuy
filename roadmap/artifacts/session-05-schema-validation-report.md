# Session 05 Schema Validation Report

## Test Summary

All tests pass under `RUSTFLAGS="-D warnings"` with zero warnings.

| Suite | Tests | Status |
|-------|-------|--------|
| `contract::error_code::tests` | 5 | Pass |
| `contract::envelope::tests` | 19 | Pass |
| `contract::error::tests` | 7 | Pass |
| `tools::contract_tests` | ~20 | Pass |
| `tools::manifest::tests` | 7 | Pass |
| `kernel::tests` | ~30 | Pass |
| Other MCP tests | ~100+ | Pass |
| **Total** | **198** | **Pass** |

## ErrorCode Exhaustiveness

- **27 variants** in `ErrorCode::ALL` constant
- Round-trip serialization verified for all 27 variants
- `Display` output matches serde JSON output for every variant
- `is_retryable()` tested against explicit expected set
- `as_str()` matches `Display` for every variant

## Error Path Validation

For each of the 27 `ErrorCode` variants:
- `ToolResponse::error(code, message)` produces valid JSON
- Envelope contains `success: false`, `data: null`, correct `code` string
- `retryable` flag matches `code.is_retryable()`

## API Profile Field Isolation

Kernel functions (used by the API profile) verified to produce JSON **without** `mode` or `approval_mode` fields in metadata:
- `kernel::read::get_tweet` — no workflow fields
- `kernel::read::get_user_by_username` — no workflow fields
- `kernel::read::search_tweets` — no workflow fields
- `kernel::write::post_tweet` — no workflow fields
- `kernel::engage::like_tweet` — no workflow fields
- `kernel::media::upload_media` — no workflow fields

## Envelope Consistency

- Success envelope: `success: true`, `data` present, `error` absent, `meta` optional
- Error envelope: `success: false`, `data: null`, `error` present with typed `code`
- `rate_limit_reset` and `policy_decision` are skip-serialized when `None`
- Workflow context flattens correctly: `meta.mode` and `meta.approval_mode` at top level

## Tool Manifest Validation

- 65 unique tools registered (no duplicates)
- Every tool has at least one profile
- Every mutation tool requires either X client or database
- All referenced error codes are valid `ErrorCode` variants
- Manifest JSON snapshot matches generated output (CI-gated)

## Provider Error Mapping

- `ProviderError::error_code()` returns correct `ErrorCode` for all variants:
  - `RateLimited` → `XRateLimited`
  - `AuthExpired` → `XAuthExpired`
  - `Forbidden` → `XForbidden`
  - `AccountRestricted` → `XAccountRestricted`
  - `NetworkError` → `XNetworkError`
  - `ApiError` → `XApiError`
  - `NotFound` → `NotFound`

## Breaking Changes (Rust API Only)

Wire format unchanged. These are compile-time changes:
1. `ToolError.code`: `String` → `ErrorCode`
2. `ToolResponse::error()`: 3 params → 2 params (retryable derived)
3. `ToolResponse::not_configured(what)` → `llm_not_configured()`, `x_not_configured()`
4. `ToolMeta::with_mode()` → `with_workflow()`
5. `ProviderError::to_triple()` → `error_code()` + `error_message()`
