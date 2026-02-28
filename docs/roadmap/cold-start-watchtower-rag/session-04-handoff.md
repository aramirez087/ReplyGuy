# Session 04 Handoff → Session 05

## What was completed

Session 04 implemented the Winning DNA classification and analytics-weighted retrieval pipeline. This includes rule-based archetype classification for tweets and replies, engagement score normalization, recency-weighted ancestor retrieval, cold-start seed fallback, RAG context injection into the draft generation pipeline, and a background seed pre-compute worker.

### Files created

| File | Purpose |
|------|---------|
| `crates/tuitbot-core/src/context/winning_dna.rs` | Archetype classification, engagement scoring, retrieval weight computation, ancestor/seed retrieval, prompt formatting (~380 lines code + ~350 lines tests) |
| `crates/tuitbot-core/src/automation/seed_worker.rs` | Background worker: extracts draft seeds from content nodes via LLM, parses HOOK/FORMAT response blocks (~180 lines code + ~150 lines tests) |
| `docs/roadmap/cold-start-watchtower-rag/rag-ranking.md` | Scoring formulas, classification heuristics, retrieval algorithm, cold-start behavior, thresholds, prompt injection format |
| `docs/roadmap/cold-start-watchtower-rag/session-04-handoff.md` | This document |

### Files modified

| File | Change |
|------|--------|
| `crates/tuitbot-core/src/context/mod.rs` | Added `pub mod winning_dna;` |
| `crates/tuitbot-core/src/automation/mod.rs` | Added `pub mod seed_worker;` and `pub use seed_worker::SeedWorker;` |
| `crates/tuitbot-core/src/storage/analytics.rs` | Added `update_tweet_archetype`, `update_reply_archetype`, `update_tweet_engagement_score`, `update_reply_engagement_score`, `get_max_performance_score`, `AncestorRow`, `AncestorQueryRow`, `ancestor_row_from_tuple`, `get_scored_ancestors` + 8 tests |
| `crates/tuitbot-core/src/storage/watchtower/mod.rs` | Added `get_pending_content_nodes`, `mark_node_processed`, `insert_draft_seed_with_weight`, `SeedWithContext`, `get_seeds_for_context` |
| `crates/tuitbot-core/src/storage/watchtower/tests.rs` | Added 4 tests: `get_pending_content_nodes_returns_pending_only`, `mark_node_processed_changes_status`, `insert_seed_with_weight_persists`, `get_seeds_for_context_joins_with_nodes` |
| `crates/tuitbot-core/src/content/generator.rs` | Added `generate_reply_with_context` method, refactored to private `generate_reply_inner` with `rag_context: Option<&str>` parameter, 2 new tests |
| `crates/tuitbot-core/src/workflow/draft.rs` | Integrated RAG context: builds topic keywords, calls `build_draft_context`, passes RAG prompt to `generate_reply_with_context` |

## Decisions made in this session

| Decision | Detail |
|----------|--------|
| **Deterministic classification** | Reply archetypes and tweet formats classified via keyword/pattern matching (no LLM). Conservative heuristics with safe fallback categories (`agree_and_expand` / `storytelling`). |
| **Exponential recency decay** | `retrieval_weight = engagement_score * exp(-0.693 * days / half_life)`. 14-day half-life: recent successes weighted more, old content decays to ~6% at 8 weeks. |
| **Engagement score normalization** | `performance_score / max_performance_score` normalized to 0.0–1.0. Cold-start baseline: 0.5. |
| **Dynamic SQL for topic filtering** | Sequential `?` placeholders for SQLite with proper bind ordering. `IN` clause for tweet topics, `LIKE` for reply content matching. |
| **Backward-compatible generator** | `generate_reply_with_archetype` preserved unchanged. New `generate_reply_with_context` is additive. Both delegate to private `generate_reply_inner`. |
| **Graceful RAG degradation** | If `build_draft_context` fails or returns empty, the draft pipeline proceeds exactly as before (no RAG injection). |
| **Seed worker LLM prompt** | Extracts 1–3 hooks per content node. Response parsed as `HOOK: ... FORMAT: ...` blocks with `---` separators. |
| **Seed engagement_weight defaults** | New seeds start at `COLD_START_WEIGHT` (0.5). `insert_draft_seed_with_weight` allows explicit weight for LLM-scored seeds. |
| **Type alias for query rows** | `AncestorQueryRow` type alias avoids clippy `type_complexity` on 7-tuple. `ancestor_row_from_tuple` helper converts to `AncestorRow`. |

## Quality gate results

```
cargo fmt --all --check          ✅ clean
RUSTFLAGS="-D warnings" cargo test --workspace  ✅ 1645 tests pass, 0 failures
cargo clippy --workspace -- -D warnings         ✅ clean
```

## New tests added (~34 total)

### Winning DNA tests (~20)
- Classification: `classify_reply_ask_question`, `classify_reply_share_experience`, `classify_reply_add_data`, `classify_reply_respectful_disagree`, `classify_reply_agree_and_expand_default`, `classify_tweet_list`, `classify_tweet_most_people_think`, `classify_tweet_contrarian`, `classify_tweet_before_after`, `classify_tweet_question`, `classify_tweet_tip`, `classify_tweet_storytelling_default`
- Scoring: `engagement_score_normal`, `engagement_score_cold_start`, `retrieval_weight_no_decay`, `retrieval_weight_half_decay`, `retrieval_weight_full_decay`
- Formatting: `format_ancestors_prompt_truncates`, `format_seeds_prompt_with_context`, `format_empty_returns_empty`

### Storage tests (12)
- Analytics: `update_and_get_tweet_archetype`, `update_and_get_reply_archetype`, `update_and_get_engagement_score`, `get_max_performance_score_empty`, `get_max_performance_score_with_data`, `get_scored_ancestors_empty`, `get_scored_ancestors_returns_scored_items`, `get_scored_ancestors_filters_low_engagement`
- Watchtower: `get_pending_content_nodes_returns_pending_only`, `mark_node_processed_changes_status`, `insert_seed_with_weight_persists`, `get_seeds_for_context_joins_with_nodes`

### Generator tests (2)
- `generate_reply_with_context_injects_rag`, `generate_reply_with_context_none_matches_archetype`

### Seed worker tests (6)
- `parse_seed_response_single`, `parse_seed_response_multiple`, `parse_seed_response_empty`, `parse_seed_response_no_format`, `parse_seed_response_trailing_separator`, `seed_worker_process_node_with_mock_llm`

## What Session 05 must do

### Primary deliverables

1. **Wire loop-back to publish flow** — When a tweet/thread is published from a draft seed, call `loopback::write_metadata_to_file()` and `cooldown.mark()` to write metadata back to the originating source file
2. **Spawn SeedWorker in server** — Start `SeedWorker` as a background task alongside `WatchtowerLoop` in server startup; cancel on shutdown
3. **Backfill classification on existing data** — Run `classify_reply_archetype` and `classify_tweet_format` over existing `tweet_performance` / `reply_performance` rows with NULL `archetype_vibe`
4. **Backfill engagement scores** — Compute and store `engagement_score` for existing performance rows using `get_max_performance_score` normalization
5. **`/api/watchtower/status` endpoint** — Expose watchtower state (watching sources, last scan time, node counts, seed counts)

### Key anchors from this session

| Resource | Location | Notes |
|----------|----------|-------|
| Archetype classification | `context/winning_dna.rs:classify_reply_archetype()`, `classify_tweet_format()` | Deterministic keyword matching. Ready for batch backfill. |
| Engagement scoring | `context/winning_dna.rs:compute_engagement_score()` | Normalized 0.0–1.0. Uses `get_max_performance_score()` from storage. |
| Retrieval weight | `context/winning_dna.rs:compute_retrieval_weight()` | Exponential recency decay. Used in `retrieve_ancestors()`. |
| Draft context builder | `context/winning_dna.rs:build_draft_context()` | Orchestrates ancestor retrieval → cold-start seed fallback → prompt formatting. |
| Ancestor query | `storage/analytics.rs:get_scored_ancestors()` | Dynamic SQL with topic filtering. Returns `AncestorRow` structs. |
| Content node helpers | `storage/watchtower/mod.rs:get_pending_content_nodes()`, `mark_node_processed()` | Batch retrieval of unprocessed nodes for SeedWorker. |
| Seed helpers | `storage/watchtower/mod.rs:insert_draft_seed_with_weight()`, `get_seeds_for_context()` | Weighted seed insertion and context-enriched retrieval (JOIN with content_nodes). |
| Generator with RAG | `content/generator.rs:generate_reply_with_context()` | Injects RAG context between persona and rules sections of system prompt. |
| Draft pipeline | `workflow/draft.rs` | Builds topic keywords from config, calls `build_draft_context`, passes to generator. |
| SeedWorker | `automation/seed_worker.rs:SeedWorker` | `run()` loop ready for server integration. Needs `DbPool` + `Arc<dyn LlmProvider>`. |
| Scoring spec | `docs/roadmap/cold-start-watchtower-rag/rag-ranking.md` | Full specification: formulas, heuristics, thresholds, prompt format. |

### Architecture notes for S05

- The `SeedWorker` follows the same pattern as other automation loops: `CancellationToken` + `LoopScheduler`. It should be spawned in `main.rs` alongside the `WatchtowerLoop`.
- Backfill classification can be a one-shot startup task: query rows with `archetype_vibe IS NULL`, classify, and update. No LLM needed — it's deterministic.
- Backfill engagement scores must run after classification and after computing `get_max_performance_score()`. Can be a startup task or a periodic refresh.
- The publish flow integration needs to:
  1. Look up the `source_node_id` on the original tweet (added in S02 migration)
  2. Find the content node's `relative_path` and source context's base path
  3. Call `loopback::write_metadata_to_file()` with the tweet metadata
  4. Mark the cooldown in the `CooldownSet` (or rely on content hash dedup per S03 decision)

### Open items

- **Seed quality scoring**: Seeds currently get `COLD_START_WEIGHT` (0.5). Future sessions could adjust weight based on LLM confidence or engagement feedback.
- **Archetype accuracy tracking**: No mechanism to validate classification accuracy. Could log misclassifications if users manually tag content.
- **RAG prompt size budget**: `RAG_MAX_CHARS` is 2000. If system prompts grow, this may need adjustment to stay under 4K tokens total.
- **Multi-topic queries**: `build_draft_context` currently extracts topic keywords from all configured content topics. May want per-draft topic filtering in the future.
