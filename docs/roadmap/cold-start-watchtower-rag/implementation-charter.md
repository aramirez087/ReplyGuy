# Implementation Charter — Cold-Start Watchtower RAG

## Epic Summary

Build a local-first Watchtower that ingests content from an Obsidian vault (or future remote sources), computes analytics-weighted draft seeds, and enriches the content generation pipeline with "Winning DNA" retrieval. This removes cold-start friction for new users and improves content quality for established accounts.

## Problem Statement

New Tuitbot users face a cold-start problem: the content generation pipeline has no historical performance data to guide archetype selection, topic weighting, or tone calibration. Users must manually configure persona fields (opinions, experiences, pillars) and wait for the analytics loop to accumulate enough data for meaningful recommendations.

The Watchtower solves this by:
1. Ingesting existing content (notes, drafts, ideas) as structured content nodes.
2. Pre-computing draft seeds (hooks, angles) from ingested content.
3. As tweets are published and measured, classifying successful patterns as "Winning DNA."
4. Using engagement-weighted retrieval to preference winning patterns in future drafts.

## Requirements → Module Mapping

| # | Requirement | Existing Module | New/Modified |
|---|------------|----------------|--------------|
| R1 | Configure an Obsidian vault path as content source | `config/types.rs` | Add `ContentSourcesConfig`, `ContentSourceEntry` |
| R2 | Scan local filesystem for .md/.txt files | — | New: `source/mod.rs`, `source/local_fs.rs` |
| R3 | Persist ingested content with deduplication | `storage/mod.rs` | New: `storage/watchtower.rs` |
| R4 | Watch filesystem for real-time changes | — | New: `automation/watchtower.rs` |
| R5 | Manual ingest trigger via HTTP API | `server/lib.rs`, `server/routes/` | New: `server/routes/ingest.rs` |
| R6 | Pre-compute draft seeds from content nodes | — | New: `storage/watchtower.rs` (seed CRUD) |
| R7 | Classify published tweets by archetype/vibe | `storage/analytics.rs` | Add `archetype_vibe`, `engagement_score` columns |
| R8 | Engagement-weighted ancestor retrieval | `context/` | New: `context/winning_dna.rs` |
| R9 | Enrich draft pipeline with RAG context | `workflow/draft.rs`, `content/generator.rs` | Extend with optional RAG context injection |
| R10 | Write-back metadata to source files | — | `source/local_fs.rs` write_metadata + `automation/watchtower.rs` loop-back |
| R11 | Schema for source tracking + content nodes | `migrations/` | New: `20260228000019_watchtower_ingestion.sql` |
| R12 | Adapter boundary for future Google Drive | — | `ContentSource` trait in `source/mod.rs` |

## New Files (complete manifest)

### Foundation Layer

| File | Purpose | Session |
|------|---------|---------|
| `crates/tuitbot-core/src/source/mod.rs` | `ContentSource` trait, `SourceFile`, `LoopBackMetadata`, `SourceError` | S02 |
| `crates/tuitbot-core/src/source/local_fs.rs` | `LocalFileSource` impl: scan, read, write_metadata | S02 |

### Storage Layer

| File | Purpose | Session |
|------|---------|---------|
| `migrations/20260228000019_watchtower_ingestion.sql` | Schema: `source_contexts`, `content_nodes`, `draft_seeds`, additive columns | S02 |
| `crates/tuitbot-core/src/storage/watchtower.rs` | CRUD: source registration, node upsert, seed lifecycle, checkpoint updates | S02 |

### Config

| File | Purpose | Session |
|------|---------|---------|
| `crates/tuitbot-core/src/config/types.rs` | Add `ContentSourcesConfig`, `ContentSourceEntry` structs | S02 |

### Server

| File | Purpose | Session |
|------|---------|---------|
| `crates/tuitbot-server/src/routes/ingest.rs` | POST /api/ingest handler (thin delegation) | S02 |

### Automation Layer

| File | Purpose | Session |
|------|---------|---------|
| `crates/tuitbot-core/src/automation/watchtower.rs` | Notify-driven file watcher + debounce + ingest | S03 |

### Context Layer

| File | Purpose | Session |
|------|---------|---------|
| `crates/tuitbot-core/src/context/winning_dna.rs` | Archetype classification, engagement scoring, ancestor retrieval | S04 |

### Documentation

| File | Purpose | Session |
|------|---------|---------|
| `docs/roadmap/cold-start-watchtower-rag/rag-ranking.md` | Scoring thresholds and retrieval algorithm | S04 |
| `docs/roadmap/cold-start-watchtower-rag/validation-report.md` | Go/no-go report | S05 |

## Modified Files (by session)

### Session 02

| File | Modification |
|------|-------------|
| `crates/tuitbot-core/src/storage/mod.rs` | Add `pub mod watchtower;` |
| `crates/tuitbot-core/src/lib.rs` | Add `pub mod source;` |
| `crates/tuitbot-core/src/config/types.rs` | Add `ContentSourcesConfig` fields |
| `crates/tuitbot-core/src/config/mod.rs` | Wire `content_sources` into `Config` struct |
| `crates/tuitbot-server/src/routes/mod.rs` | Add `pub mod ingest;` |
| `crates/tuitbot-server/src/lib.rs` | Register `/api/ingest` route |

### Session 03

| File | Modification |
|------|-------------|
| `crates/tuitbot-core/src/automation/mod.rs` | Add `pub mod watchtower;` + exports |
| `crates/tuitbot-server/src/state.rs` | Optional watchtower handle in `AppState` |
| `crates/tuitbot-server/src/routes/ingest.rs` | Reuse shared ingest pipeline |

### Session 04

| File | Modification |
|------|-------------|
| `crates/tuitbot-core/src/context/mod.rs` | Add `pub mod winning_dna;` |
| `crates/tuitbot-core/src/storage/analytics.rs` | Queries for `archetype_vibe` + `engagement_score` |
| `crates/tuitbot-core/src/workflow/draft.rs` | Inject RAG context before LLM call |
| `crates/tuitbot-core/src/content/generator.rs` | Accept optional `rag_context: &str` parameter |

### Session 05

| File | Modification |
|------|-------------|
| `docs/architecture.md` | Document Watchtower layer + source adapter |
| `docs/configuration.md` | Document `[content_sources]` config section |

## Dependencies (new crate dependencies)

| Crate | Version | Purpose | Session |
|-------|---------|---------|---------|
| `notify` | `7.x` | Cross-platform filesystem event watching | S03 |
| `notify-debouncer-full` | `0.4.x` | Debounced event delivery | S03 |
| `sha2` | `0.10.x` | Content hashing for change detection | S02 |
| `serde_yaml` | `0.9.x` | Parse/write YAML front-matter in notes | S02 |

All dependencies are well-maintained, pure-Rust, and add no external service requirements.

## Session Sequence

| Session | Title | Key Outputs | Depends On |
|---------|-------|-------------|------------|
| S01 | Charter and Scope | This document, architecture decisions, test strategy, handoff | — |
| S02 | Schema and Ingest API | Migration, storage CRUD, config, source trait, POST /api/ingest | S01 |
| S03 | Watchtower Runtime and Loop-Back | Notify watcher, shared ingest pipeline, loop-back | S02 |
| S04 | Winning DNA Retrieval | Archetype classification, engagement scoring, draft enrichment | S03 |
| S05 | Validation and Obsidian Shakeout | E2E testing, doc updates, go/no-go report | S04 |

## Risk Register

| ID | Risk | Impact | Likelihood | Mitigation |
|----|------|--------|-----------|------------|
| R1 | `notify` crate cross-platform behavior — inconsistent debounce across macOS FSEvents, Linux inotify, Windows ReadDirectoryChanges | Medium — duplicate or missed ingestion events | Medium | Use `notify-debouncer-full` for consistent cross-platform behavior. Add fallback interval scan (5 min) as safety net. Test with tempdir fixtures. |
| R2 | Large Obsidian vaults (10K+ notes) causing slow initial scan | Low — initial scan is one-time and background | Low | Content hash (SHA-256) skips unchanged files. Paginate initial scan. Store last-scan cursor in `source_contexts.sync_cursor`. |
| R3 | Loop-back metadata corrupting user notes | High — user data loss | Low | Use standard YAML front-matter format (Obsidian-compatible). Idempotent writes (check before append). Parse existing front-matter before modifying. Configurable `loop_back_enabled` flag (default: true). |
| R4 | RAG context bloating LLM token usage | Medium — cost and latency increase | Medium | Cap RAG context at 2000 tokens (configurable). Truncate ancestors to 120 chars each. Track RAG-specific token usage in `llm_usage`. |
| R5 | Schema migration breaking existing deployments | Low — additive changes only | Low | Use only `CREATE TABLE IF NOT EXISTS` and `ALTER TABLE ADD COLUMN` (supported since SQLite 3.2.0). Pre-migration backup already exists in `storage/mod.rs` (line 54-69). Test on fresh DB and DB with existing data. |
| R6 | Watchtower detects its own loop-back writes, creating infinite re-ingestion | Medium — runaway ingestion cycles | Medium | Cooldown set for recently-written paths (5-second expiry). Content hash check as second safety net — if hash matches, no re-ingestion. |
| R7 | New `content_sources` config section breaks existing config files | Low — users cannot load config | Very Low | All new fields use `#[serde(default)]`. Empty sources list is the default. Config without `[content_sources]` parses identically to today. Verified by `content_sources_optional_in_config` test. |

## Non-Goals (explicitly out of scope)

- Google Drive adapter implementation (future work — only the trait boundary ships)
- Dashboard UI for Watchtower management (future work — API-first)
- Full-text search / vector embedding (v1 uses keyword matching for retrieval)
- Multi-vault support (v1 supports one source per entry; multiple entries allowed)
- Obsidian plugin (Watchtower reads files directly — no Obsidian-specific API)

## Success Criteria

1. A user can configure `[[content_sources.sources]]` with a path to their Obsidian vault.
2. Tuitbot ingests .md/.txt files and creates content_nodes + draft_seeds.
3. The Watchtower detects file changes and re-ingests automatically.
4. Manual POST /api/ingest triggers the same pipeline.
5. Published tweets are classified by archetype and scored for engagement.
6. Draft generation uses winning ancestors as context when available.
7. Loop-back writes tweet metadata to source files without corruption.
8. All existing functionality (content, approval, analytics, runtime, settings) is unchanged.
9. All quality gates pass: `cargo fmt`, `cargo test`, `cargo clippy` with zero warnings.
