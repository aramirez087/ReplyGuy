# Session 07: Media Upload and Thread Determinism

Paste this into a new Claude Code session:

```md
Continue from Session 06 artifacts.

Mission:
Deliver first-class media and thread workflows that are deterministic, idempotent-aware, and reliable for agent execution.

Primary files:
- `crates/tuitbot-core/src/x_api/media.rs`
- `crates/tuitbot-core/src/storage/media.rs`
- `crates/tuitbot-mcp/src/tools/workflow/x_actions/media.rs`
- `crates/tuitbot-mcp/src/tools/workflow/x_actions/write.rs`
- `crates/tuitbot-mcp/src/tools/workflow/composite/thread_plan.rs`
- related tests under `crates/tuitbot-mcp/src/tools/workflow/x_actions/tests/`

Required capability:
1. `media_upload`:
   - chunked upload support
   - finalize
   - status polling
2. `tweet_create_with_media`
3. `thread_create` with automatic reply chaining
4. dry-run validation mode for all new mutation workflows

Determinism requirements:
- Return stable payloads including:
  - tweet IDs
  - media IDs
  - exact posted text
- Surface partial-failure states clearly and safely.

Deliverables:
1. Implemented tools and integration with provider/storage layers.
2. Tests for chunked upload, finalize/polling, thread chaining, and dry-run behavior.
3. `docs/roadmap/x-api-surface-expansion/session-07-handoff.md` with:
   - API contracts
   - failure modes and retry semantics
   - operational guidance

Quality gates:
- `cargo fmt --all && cargo fmt --all --check`
- `RUSTFLAGS="-D warnings" cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`

Exit criteria:
- Media and thread flows are production-grade and predictable.
- Dry-run mode prevents blind mutation attempts.
- Tests prove deterministic outputs and error handling.
```

