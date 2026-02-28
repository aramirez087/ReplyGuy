# Session 01: Charter And Scope

Paste this into a new Claude Code session:

```md
Continuity
- Read docs/architecture.md, docs/configuration.md, crates/tuitbot-core/src/storage/mod.rs, crates/tuitbot-core/src/storage/analytics.rs, crates/tuitbot-core/src/context/mod.rs, crates/tuitbot-core/src/workflow/draft.rs, crates/tuitbot-server/src/lib.rs, and crates/tuitbot-server/src/routes/settings.rs.
- Treat docs/roadmap/cold-start-watchtower-rag/ as the source of truth for this epic.

Mission
Audit the existing codebase and produce an implementation charter for a local-first Watchtower plus analytics-weighted RAG that removes cold-start friction.

Repository anchors
- docs/architecture.md
- docs/configuration.md
- crates/tuitbot-core/src/storage/mod.rs
- crates/tuitbot-core/src/storage/analytics.rs
- crates/tuitbot-core/src/context/mod.rs
- crates/tuitbot-core/src/workflow/draft.rs
- crates/tuitbot-server/src/lib.rs
- crates/tuitbot-server/src/routes/settings.rs

Tasks
1. Map each new requirement to the existing storage, context, workflow, automation, and server layers.
2. Define a v1 source model that uses a server-local Obsidian vault path now and preserves an adapter boundary for future Google Drive-backed sources.
3. Specify the schema evolution for source tracking, content-node relationships, draft seeds, and enriched tweet performance without making broad code changes in this session.
4. Write a concrete implementation sequence, risk register, and test strategy that the next sessions can execute without re-planning.

Deliverables
- docs/roadmap/cold-start-watchtower-rag/implementation-charter.md
- docs/roadmap/cold-start-watchtower-rag/architecture-decisions.md
- docs/roadmap/cold-start-watchtower-rag/test-strategy.md
- docs/roadmap/cold-start-watchtower-rag/session-01-handoff.md

Exit criteria
- The charter assigns each requirement to explicit modules or new files.
- The v1 source-selection plan states how an Obsidian vault path is configured.
- The handoff names the exact files and decisions Session 02 must pick up.
```
