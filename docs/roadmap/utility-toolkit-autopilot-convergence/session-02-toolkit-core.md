# Session 02: Toolkit Core Layer — Implementation Log

**Date:** 2026-02-26
**Session:** 02 of 08
**Branch:** `feat/mcp_final`

---

## Summary

Implemented the stateless Toolkit layer in `tuitbot-core::toolkit` and refactored all MCP `x_actions` modules to route through it. Every low-level X API operation now flows through toolkit functions that take `&dyn XApiClient` with no state dependencies.

---

## What Was Built

### 1. `crates/tuitbot-core/src/toolkit/mod.rs`

Module root with:
- `ToolkitError` enum (6 variants): `XApi`, `InvalidInput`, `TweetTooLong`, `UnsupportedMediaType`, `MediaTooLarge`, `ThreadPartialFailure`
- `validate_tweet_length(text)` — stateless 280-char check
- `validate_id(id, field_name)` — stateless empty-ID guard
- `MAX_TWEET_LENGTH` constant
- Unit tests for validation and error conversion

### 2. `crates/tuitbot-core/src/toolkit/read.rs`

14 stateless read functions + `get_me`:
- `get_tweet`, `get_user_by_username`, `get_user_by_id`, `get_me`
- `search_tweets`, `get_mentions`, `get_user_tweets`, `get_home_timeline`
- `get_followers`, `get_following`, `get_liked_tweets`, `get_bookmarks`
- `get_users_by_ids` (validates 1-100 count), `get_tweet_liking_users`

All validate inputs (empty IDs, empty queries, bounds) and delegate to `&dyn XApiClient`.

### 3. `crates/tuitbot-core/src/toolkit/write.rs`

5 write functions:
- `post_tweet` — with optional media IDs branching
- `reply_to_tweet` — validates reply-to ID
- `quote_tweet` — validates quoted tweet ID
- `delete_tweet` — validates tweet ID
- `post_thread` — validates all lengths upfront, chains sequential posts, returns `ThreadPartialFailure` on partial failure

### 4. `crates/tuitbot-core/src/toolkit/engage.rs`

8 engagement functions:
- `like_tweet`, `unlike_tweet`
- `follow_user`, `unfollow_user`
- `retweet`, `unretweet`
- `bookmark_tweet`, `unbookmark_tweet`

All take `(client, user_id, target_id)` pattern with input validation.

### 5. `crates/tuitbot-core/src/toolkit/media.rs`

Media utility functions:
- `upload_media` — raw upload with size validation via `&dyn XApiClient`
- `infer_media_type` — extension-based type inference (jpg, jpeg, png, webp, gif, mp4)
- `validate_media_size` — checks against X API limits per media type
- `requires_processing` — GIF/video detection
- `requires_chunked` — delegated to `MediaType::requires_chunked`
- `upload_strategy` — returns "simple" or "chunked" string

### 6. MCP `x_actions` Refactoring

All four MCP x_actions modules refactored:
- **read.rs** — Calls `tuitbot_core::toolkit::read::*` directly (removed kernel/provider dependency)
- **write.rs** — Calls `tuitbot_core::toolkit::write::*` with audited error handling
- **engage.rs** — Calls `tuitbot_core::toolkit::engage::*` with audited error handling
- **media.rs** — Calls `tuitbot_core::toolkit::media::*` for type inference, validation, strategy, and upload

Two new helper functions in `x_actions/mod.rs`:
- `toolkit_error_response` — maps `ToolkitError` to JSON for non-audited reads
- `audited_toolkit_error_response` — maps `ToolkitError` in audited mutation context

---

## Architecture Decisions Applied

| Decision | How Applied |
|----------|-------------|
| AD-02 | All toolkit functions take `&dyn XApiClient` only, no state/DB |
| AD-04 | Toolkit writes are raw — no policy enforcement, audit, or mutation recording |
| AD-10 | `ToolkitError` variants map to existing `ErrorCode` variants via helper functions |
| AD-11 | No backward compatibility layers — MCP x_actions call toolkit directly |
| AD-12 | Stateless validation (tweet length, ID format, media size) in toolkit; stateful checks (policy, audit, dedup) remain in MCP workflow layer |

---

## Test Results

| Crate | Tests | Status |
|-------|-------|--------|
| tuitbot-cli | 118 | Pass |
| tuitbot-core | 778 | Pass |
| tuitbot-mcp | 408 | Pass |
| tuitbot-server | 31 | Pass |
| **Total** | **1,336** | **All pass** |

CI checklist: `cargo fmt` clean, `cargo clippy` clean, `cargo test` all pass.

---

## Design Notes

1. **Double validation**: MCP write.rs keeps `check_tweet_length` as a fast-fail before the policy gate. Toolkit also validates. This is intentional belt-and-suspenders — MCP check is an optimization to avoid the policy gate for obviously invalid input, toolkit check is authoritative.

2. **Retry behavior**: MCP reads previously went through `RetryingProvider` in the kernel layer. The toolkit refactoring bypasses this, calling `XApiClient` directly. Retry is a workflow concern and will be re-added in Sessions 04-05.

3. **Thread partial failure**: `ToolkitError::ThreadPartialFailure` captures `posted_ids`, `failed_index`, `posted` count, `total` count, and a boxed source error. MCP `post_thread` has special pattern matching for this variant to provide rich error information.

4. **Media workflow concerns**: File I/O, SHA-256 hashing, DB tracking, idempotency checks, and dry-run support remain in the MCP `media.rs` workflow layer. Only type inference, size validation, and the raw upload call moved to toolkit.
