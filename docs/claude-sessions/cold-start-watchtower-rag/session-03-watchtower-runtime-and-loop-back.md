# Session 03: Watchtower Runtime And Loop-Back

Paste this into a new Claude Code session:

```md
Continue from Session 02 artifacts.
Continuity
- Read docs/roadmap/cold-start-watchtower-rag/session-02-handoff.md, migrations/20260228000019_watchtower_ingestion.sql, crates/tuitbot-core/src/storage/watchtower.rs, and crates/tuitbot-server/src/routes/ingest.rs.

Mission
Implement the notify-based Watchtower runtime and the filesystem loop-back that keeps source notes in sync with published output.

Repository anchors
- crates/tuitbot-core/src/automation/mod.rs
- crates/tuitbot-core/src/automation/content_loop.rs
- crates/tuitbot-core/src/storage/watchtower.rs
- crates/tuitbot-core/src/config/types.rs
- crates/tuitbot-server/src/main.rs
- crates/tuitbot-server/src/state.rs
- crates/tuitbot-server/src/routes/ingest.rs

Tasks
1. Create crates/tuitbot-core/src/automation/watchtower.rs with a notify-driven service that watches configured local source paths for .md and .txt changes and records debounced ingest work in SQLite.
2. Start and stop the Watchtower from the server runtime without blocking existing HTTP handling, following the current Runtime and shared AppState patterns.
3. Reuse the same ingest pipeline for both filesystem events and POST /api/ingest so manual triggers and background watching stay behaviorally identical.
4. Implement source-file loop-back that writes posted metadata such as URL and published date back into the originating note in an idempotent, parseable format.
5. Add tempdir-based tests for watcher filtering, manual ingest replay, and loop-back idempotency.

Deliverables
- crates/tuitbot-core/src/automation/watchtower.rs
- crates/tuitbot-core/src/automation/mod.rs
- crates/tuitbot-server/src/main.rs
- crates/tuitbot-server/src/state.rs
- crates/tuitbot-core/src/storage/watchtower.rs
- crates/tuitbot-server/src/routes/ingest.rs
- docs/roadmap/cold-start-watchtower-rag/session-03-handoff.md

Quality gates
cargo fmt --all && cargo fmt --all --check
RUSTFLAGS="-D warnings" cargo test --workspace
cargo clippy --workspace -- -D warnings

Exit criteria
- Changing a watched .md or .txt file enqueues exactly one ingest record after debounce.
- Manual and automatic ingest paths share the same persisted state transitions.
- Re-running loop-back on an already-annotated note does not duplicate metadata.
```
