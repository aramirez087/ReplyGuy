# Session 03 Handoff

## Status: Complete

All implementation steps from the Session 03 plan are done. CI passes (fmt, clippy, tests).

## What Was Delivered

1. **Profile enum** (`Api` / `Workflow`) with `Display`, `FromStr`, CLI `--profile` flag
2. **ApiState** — lightweight state struct (config + X client + user ID, no DB/LLM)
3. **SocialReadProvider** expanded with 4 new methods (mentions, user_tweets, timeline, get_me)
4. **Kernel layer** expanded from 1 to 5 modules (read, write, engage, media, utils)
5. **ApiMcpServer** — 24-tool server struct with compile-time tool registration
6. **Server directory** — `server.rs` → `server/workflow.rs` + `server/mod.rs` + `server/api.rs`
7. **Workflow reads delegated** — mentions, user_tweets, home_timeline now go through kernel
8. **validate.rs** — re-exports from `kernel::utils::check_tweet_length`
9. **153 tests pass** — including 36+ new kernel tests

## CLI Usage

```bash
tuitbot mcp serve                     # workflow (default, current behavior)
tuitbot mcp serve --profile api       # api (X client only, no DB)
tuitbot mcp serve --profile workflow   # explicit workflow
```

## Known Gaps / Future Work

- **`server/workflow.rs`** is 868 lines (over 500-line limit). Should be split in a future session.
- **Kernel write/engage take `&dyn XApiClient`** not a provider trait. A `SocialWriteProvider` trait can be added when a scraper backend needs write support.
- **API profile has no policy gating** — intentional for generic X client use. If rate limiting is needed, it can be added at the kernel level later.
- **No media test with real file I/O** — `kernel::media::upload_media` is tested via the existing `tools/x_actions/media.rs` tests for type inference. Integration tests would need temp files.

## Files Changed (Summary)

**New files (6):**
- `src/kernel/write.rs`, `src/kernel/engage.rs`, `src/kernel/media.rs`, `src/kernel/utils.rs`
- `src/server/mod.rs`, `src/server/api.rs`

**Modified files (11):**
- `src/state.rs`, `src/lib.rs`, `src/kernel/mod.rs`, `src/kernel/read.rs`, `src/kernel/tests.rs`
- `src/provider/mod.rs`, `src/provider/x_api.rs`
- `src/tools/x_actions/read.rs`, `src/tools/x_actions/validate.rs`
- `cli/commands/mod.rs`, `cli/commands/mcp.rs`, `cli/main.rs`

**Moved files (1):**
- `src/server.rs` → `src/server/workflow.rs`

**Artifacts (3):**
- `roadmap/artifacts/session-03-profile-design.md`
- `roadmap/artifacts/session-03-runtime-decoupling-report.md`
- `roadmap/artifacts/session-03-handoff.md`
