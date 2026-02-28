# Session 05: Validation And Obsidian Shakeout

Paste this into a new Claude Code session:

```md
Continue from Session 04 artifacts.
Continuity
- Read docs/roadmap/cold-start-watchtower-rag/session-04-handoff.md, docs/roadmap/cold-start-watchtower-rag/rag-ranking.md, and the files modified in Sessions 02 through 04.

Mission
Validate the end-to-end Cold Start flow, close consistency gaps, and issue a documented go or no-go release recommendation.

Repository anchors
- docs/architecture.md
- docs/configuration.md
- crates/tuitbot-core/src/automation/watchtower.rs
- crates/tuitbot-core/src/context/winning_dna.rs
- crates/tuitbot-core/src/storage/watchtower.rs
- crates/tuitbot-server/src/routes/ingest.rs
- crates/tuitbot-server/tests/api_tests.rs

Tasks
1. Run the required Rust quality gates and fix any failures caused by the new feature work.
2. Perform an end-to-end manual shakeout against a temp or sample Obsidian-style vault path, including a filesystem-triggered ingest, a manual POST /api/ingest, seed creation, and loop-back metadata writing.
3. Update the architecture and configuration docs so the local-path setup, operational constraints, and future remote-source adapter boundary are explicit.
4. Produce a go or no-go validation report with unresolved risks, rollback notes, and concrete follow-up work if the epic is not production-ready.

Deliverables
- docs/architecture.md
- docs/configuration.md
- docs/roadmap/cold-start-watchtower-rag/validation-report.md
- docs/roadmap/cold-start-watchtower-rag/session-05-handoff.md

Quality gates
cargo fmt --all && cargo fmt --all --check
RUSTFLAGS="-D warnings" cargo test --workspace
cargo clippy --workspace -- -D warnings

Exit criteria
- The quality gates pass without suppressing warnings.
- The documented manual test covers both filesystem and API ingest paths.
- The validation report ends with a clear go or no-go recommendation.
```
