# Session 02: Schema And Ingest API

Paste this into a new Claude Code session:

```md
Continue from Session 01 artifacts.
Continuity
- Read docs/roadmap/cold-start-watchtower-rag/implementation-charter.md, docs/roadmap/cold-start-watchtower-rag/architecture-decisions.md, docs/roadmap/cold-start-watchtower-rag/test-strategy.md, and docs/roadmap/cold-start-watchtower-rag/session-01-handoff.md.

Mission
Implement the schema, storage, configuration, and HTTP ingest contract that establish the Watchtower ingestion foundation.

Repository anchors
- migrations/20260228000018_sessions.sql
- crates/tuitbot-core/src/storage/mod.rs
- crates/tuitbot-core/src/storage/analytics.rs
- crates/tuitbot-core/src/config/types.rs
- crates/tuitbot-server/src/routes/mod.rs
- crates/tuitbot-server/src/lib.rs
- crates/tuitbot-server/src/routes/settings.rs
- crates/tuitbot-server/tests/api_tests.rs

Tasks
1. Add migrations/20260228000019_watchtower_ingestion.sql to create source_context, content_nodes, and draft_seeds, and to extend tweet analytics storage with engagement_score and archetype_vibe using additive SQL only.
2. Create crates/tuitbot-core/src/storage/watchtower.rs and wire it through crates/tuitbot-core/src/storage/mod.rs with typed CRUD helpers for source registration, ingest checkpoints, node linkage, and pending seed retrieval.
3. Extend configuration types so a content source can point at a local filesystem path for an Obsidian vault while remaining provider-agnostic in shape.
4. Add POST /api/ingest as a thin authenticated route, register it in the router, and define a request and response contract that supports manual file hints from Shortcuts or Telegram.
5. Add focused tests that cover the new storage lifecycle and the ingest endpoint contract.

Deliverables
- migrations/20260228000019_watchtower_ingestion.sql
- crates/tuitbot-core/src/storage/watchtower.rs
- crates/tuitbot-core/src/storage/mod.rs
- crates/tuitbot-core/src/config/types.rs
- crates/tuitbot-server/src/routes/ingest.rs
- crates/tuitbot-server/src/routes/mod.rs
- crates/tuitbot-server/src/lib.rs
- crates/tuitbot-server/tests/api_tests.rs
- docs/roadmap/cold-start-watchtower-rag/session-02-handoff.md

Quality gates
cargo fmt --all && cargo fmt --all --check
RUSTFLAGS="-D warnings" cargo test --workspace
cargo clippy --workspace -- -D warnings

Exit criteria
- The migration applies cleanly on a fresh test database.
- POST /api/ingest is registered in the router and has automated contract coverage.
- The new config shape can express a local Obsidian source without hard-coding Obsidian semantics into the storage layer.
```
