# Session 01 Handoff → Session 02

## What was completed

Session 01 audited the existing codebase and produced the implementation charter for the Cold-Start Watchtower RAG epic. Four documents were created:

1. **`implementation-charter.md`** — Requirement-to-module mapping, complete file manifest, session sequence, non-goals, and success criteria.
2. **`architecture-decisions.md`** — Six architecture decisions (AD-1 through AD-6) covering source model, schema, layer placement, runtime pattern, Winning DNA retrieval, and HTTP contract.
3. **`test-strategy.md`** — Test plan organized by session with specific test names, assertions, mock patterns, and quality gates.
4. **`session-01-handoff.md`** — This document.

No source code was modified.

## Decisions already made (do not re-decide)

| Decision | Summary |
|----------|---------|
| AD-1 | `ContentSource` trait in `core/source/mod.rs` with `LocalFileSource` impl. Config via `[[content_sources.sources]]` TOML section. |
| AD-2 | Migration `20260228000019_watchtower_ingestion.sql` creates `source_contexts`, `content_nodes`, `draft_seeds`. Adds `archetype_vibe TEXT` and `engagement_score REAL` to `tweet_performance` and `reply_performance`. Adds `source_node_id` to `original_tweets`. |
| AD-3 | Source trait at Foundation layer. Storage CRUD at Foundation. Winning DNA at L2 Workflow (context). Watchtower watcher at L3 Autopilot. Ingest route at Server. |
| AD-4 | Watchtower uses existing `Runtime.spawn()` pattern with `CancellationToken`. `notify` crate for filesystem events. Shared ingest pipeline for auto + manual triggers. |
| AD-5 | Engagement-weighted retrieval with recency decay (half-life 14 days). Cold-start fallback to draft_seeds then to existing behavior. RAG context capped at 2000 tokens. |
| AD-6 | `POST /api/ingest` with `file_hints`, `force`, and `source_type` fields. Response includes `ingested`, `skipped`, `errors`, `duration_ms`. |

## Exact files Session 02 must create

| File | Purpose |
|------|---------|
| `migrations/20260228000019_watchtower_ingestion.sql` | Schema: 3 new tables + additive columns (see AD-2 for exact SQL) |
| `crates/tuitbot-core/src/source/mod.rs` | `ContentSource` trait, `SourceFile`, `LoopBackMetadata`, `SourceError` |
| `crates/tuitbot-core/src/source/local_fs.rs` | `LocalFileSource` implementation |
| `crates/tuitbot-core/src/storage/watchtower.rs` | CRUD: source_contexts, content_nodes, draft_seeds |
| `crates/tuitbot-server/src/routes/ingest.rs` | POST /api/ingest handler |
| `docs/roadmap/cold-start-watchtower-rag/session-02-handoff.md` | Handoff to Session 03 |

## Exact files Session 02 must modify

| File | Modification |
|------|-------------|
| `crates/tuitbot-core/src/lib.rs` | Add `pub mod source;` |
| `crates/tuitbot-core/src/storage/mod.rs` | Add `pub mod watchtower;` |
| `crates/tuitbot-core/src/config/types.rs` | Add `ContentSourcesConfig` and `ContentSourceEntry` structs |
| `crates/tuitbot-core/src/config/mod.rs` | Add `content_sources: ContentSourcesConfig` to `Config` struct |
| `crates/tuitbot-core/src/error.rs` | Add `SourceError` enum (or in `source/mod.rs`) |
| `crates/tuitbot-server/src/routes/mod.rs` | Add `pub mod ingest;` |
| `crates/tuitbot-server/src/lib.rs` | Register `.route("/ingest", post(routes::ingest::ingest))` in the API router |
| `crates/tuitbot-core/Cargo.toml` | Add `sha2` and `serde_yaml` dependencies |

## New crate dependencies for Session 02

| Crate | Version | Purpose |
|-------|---------|---------|
| `sha2` | `0.10` | Content hashing for change detection |
| `serde_yaml` | `0.9` | Parse/write YAML front-matter in markdown files |

(`notify` and `notify-debouncer-full` are deferred to Session 03.)

## Open questions resolved

| Question | Resolution |
|----------|-----------|
| Where does the source trait live? | `core/source/mod.rs` — Foundation layer, alongside `x_api`, `storage`, `llm` |
| How is the vault path configured? | `[[content_sources.sources]]` TOML section with `path`, `source_type`, `watch`, `file_patterns`, `loop_back_enabled` |
| Is front-matter parsing needed at ingest time? | Yes — `content_nodes.front_matter_json` stores parsed YAML, `title` extracted from front-matter or first heading, `tags` from front-matter tags field |
| How are duplicate ingestions prevented? | `UNIQUE(source_id, relative_path)` constraint + `content_hash` comparison on upsert |
| Does the new config break existing configs? | No — `content_sources` defaults to empty sources list. All new fields are `serde(default)`. |

## Context for Session 02 developer

Read these files before starting:
1. `docs/roadmap/cold-start-watchtower-rag/implementation-charter.md` — full file manifest
2. `docs/roadmap/cold-start-watchtower-rag/architecture-decisions.md` — AD-1 (source model), AD-2 (schema), AD-6 (ingest API)
3. `docs/roadmap/cold-start-watchtower-rag/test-strategy.md` — Session 02 test section
4. `crates/tuitbot-core/src/storage/analytics.rs` — pattern for storage CRUD (typed helpers, `init_test_db()` in tests)
5. `crates/tuitbot-core/src/config/types.rs` — pattern for config structs (`#[serde(default)]`, default value functions)
6. `crates/tuitbot-core/src/config/mod.rs` — `Config` struct with 14 fields, all `#[serde(default)]`. Add `pub content_sources: ContentSourcesConfig` and re-export from the `pub use types::` line
7. `crates/tuitbot-server/src/routes/settings.rs` — pattern for thin server routes (extract State, delegate to core, return Json)
8. `crates/tuitbot-server/src/lib.rs` — router registration: add `.route("/ingest", post(routes::ingest::ingest))` inside the `let api = Router::new()` chain
9. `crates/tuitbot-core/src/lib.rs` — top-level module declarations, add `pub mod source;` after `pub mod scoring;`

## Key codebase anchors (verified against current code)

| Pattern | Location | Detail |
|---------|----------|--------|
| Multi-account isolation | All tables | `account_id TEXT NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'` — use this exact default UUID |
| Storage test pattern | `storage/analytics.rs:653` | `init_test_db().await` → run operations → assert results |
| Config struct pattern | `config/mod.rs:67-135` | All fields are `#[serde(default)]` on the `Config` struct |
| Config re-export | `config/mod.rs:21-25` | Types are re-exported: `pub use types::{...}` — add new types here |
| Route registration | `server/lib.rs:26-230` | Routes chained in `Router::new()`, auth middleware applied at end |
| Error pattern | `error.rs` | Domain-specific `thiserror` enums; `SourceError` should live in `source/mod.rs` like `LlmError` lives in `llm/` |

## Migration naming

The latest existing migration is `20260228000018_sessions.sql`. The new migration is `20260228000019_watchtower_ingestion.sql` — this name is final and must not change (Session 03+ references depend on it).
