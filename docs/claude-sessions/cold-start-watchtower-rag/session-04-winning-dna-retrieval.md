# Session 04: Winning DNA Retrieval

Paste this into a new Claude Code session:

```md
Continue from Session 03 artifacts.
Continuity
- Read docs/roadmap/cold-start-watchtower-rag/session-03-handoff.md, crates/tuitbot-core/src/storage/watchtower.rs, crates/tuitbot-core/src/storage/analytics.rs, crates/tuitbot-core/src/context/engagement.rs, and crates/tuitbot-core/src/workflow/draft.rs.

Mission
Implement Winning DNA classification and analytics-weighted retrieval so new drafts prefer successful ancestors from prior output and recent notes.

Repository anchors
- crates/tuitbot-core/src/context/mod.rs
- crates/tuitbot-core/src/context/engagement.rs
- crates/tuitbot-core/src/storage/analytics.rs
- crates/tuitbot-core/src/storage/watchtower.rs
- crates/tuitbot-core/src/workflow/draft.rs
- crates/tuitbot-core/src/content/generator.rs

Tasks
1. Create crates/tuitbot-core/src/context/winning_dna.rs to classify historical tweet output into archetypes, compute an engagement-weighted success score, and expose ranked ancestor retrieval APIs.
2. Persist archetype_vibe, engagement_score, and seed-generation state through additive storage helpers so the scoring model is inspectable and testable.
3. Extend the draft-generation path to blend recent ingested note context with winning ancestors, with engagement score acting as a retrieval weight instead of a post-filter.
4. Add the low-priority seed pre-compute worker that produces candidate hooks for newly ingested notes and stores them in draft_seeds.
5. Cover classification, ranking, and draft-context assembly with deterministic tests and document any thresholds that materially affect retrieval.

Deliverables
- crates/tuitbot-core/src/context/winning_dna.rs
- crates/tuitbot-core/src/context/mod.rs
- crates/tuitbot-core/src/storage/analytics.rs
- crates/tuitbot-core/src/storage/watchtower.rs
- crates/tuitbot-core/src/workflow/draft.rs
- crates/tuitbot-core/src/content/generator.rs
- docs/roadmap/cold-start-watchtower-rag/rag-ranking.md
- docs/roadmap/cold-start-watchtower-rag/session-04-handoff.md

Quality gates
cargo fmt --all && cargo fmt --all --check
RUSTFLAGS="-D warnings" cargo test --workspace
cargo clippy --workspace -- -D warnings

Exit criteria
- Retrieval favors higher-performing ancestors when semantic relevance is comparable.
- Newly ingested notes can produce persisted seed rows without blocking request handling.
- The scoring rules and thresholds are written down in the roadmap docs.
```
