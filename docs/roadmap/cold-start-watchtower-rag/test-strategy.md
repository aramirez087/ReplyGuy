# Test Strategy — Cold-Start Watchtower RAG

## Testing Principles

1. **Deterministic by default.** Every test produces the same result on every run. No timing-dependent assertions without explicit tolerance windows.
2. **In-memory SQLite.** Use `init_test_db()` for all storage tests. No filesystem database files in tests.
3. **Trait-based mocking.** External services (`ContentSource`, `LlmProvider`) use mock implementations. No network calls in tests.
4. **Existing patterns.** Follow `storage/analytics.rs` test style for CRUD, `workflow/e2e_tests.rs` for pipeline tests.
5. **File size limit.** Test files > 100 lines go in a `tests.rs` submodule per the project convention.

## Quality Gates (every session)

```bash
cargo fmt --all && cargo fmt --all --check
RUSTFLAGS="-D warnings" cargo test --workspace
cargo clippy --workspace -- -D warnings
```

---

## Session 02: Schema and Ingest API

### Migration Tests

**Location:** `crates/tuitbot-core/src/storage/watchtower.rs` (inline `#[cfg(test)]` module)

| Test | Assertions |
|------|-----------|
| `migration_creates_new_tables` | After `init_test_db()`, query `sqlite_master` for `source_contexts`, `content_nodes`, `draft_seeds` — all must exist. |
| `migration_adds_columns_to_performance` | After `init_test_db()`, `PRAGMA table_info(tweet_performance)` includes `archetype_vibe` and `engagement_score` columns. Same for `reply_performance`. |
| `migration_adds_source_node_id_to_tweets` | `PRAGMA table_info(original_tweets)` includes `source_node_id`. |

### Storage CRUD Tests

**Location:** `crates/tuitbot-core/src/storage/watchtower.rs` (`#[cfg(test)]`)

| Test | Assertions |
|------|-----------|
| `insert_and_get_source_context` | Insert a source context → retrieve by ID → fields match. |
| `update_sync_cursor` | Insert source → update cursor → retrieve → cursor updated. |
| `insert_content_node` | Insert node with source_id FK → retrieve → fields match. |
| `content_node_upsert_by_hash` | Insert node → insert same path with different hash → row updated, not duplicated. |
| `content_node_dedup_same_hash` | Insert node → insert same path with same hash → no change, returns "skipped". |
| `insert_draft_seed` | Insert seed with node_id FK → retrieve by status → seed returned. |
| `get_pending_seeds_ordered_by_weight` | Insert 3 seeds with weights 0.3, 0.9, 0.5 → query pending ordered by weight DESC → [0.9, 0.5, 0.3]. |
| `mark_seed_used` | Insert seed → mark used → status = "used", used_at populated. |
| `get_nodes_for_source` | Insert 3 nodes for source A, 2 for source B → query by source A → returns 3. |

### Source Trait Tests

**Location:** `crates/tuitbot-core/src/source/local_fs.rs` (`#[cfg(test)]`)

| Test | Assertions |
|------|-----------|
| `scan_finds_md_files` | Create tempdir with 2 .md + 1 .jpg → scan → returns exactly 2 SourceFiles. |
| `scan_respects_patterns` | Configure patterns ["*.txt"] → create .md + .txt → scan → returns 1. |
| `scan_computes_content_hash` | Create file with known content → scan → hash matches expected SHA-256. |
| `read_content_returns_utf8` | Create file with UTF-8 content → read_content → matches. |
| `read_content_missing_file` | read_content for nonexistent path → returns SourceError. |
| `scan_since_filters_by_mtime` | Create 2 files, set mtime of one to past → scan_for_changes(Some(cutoff)) → returns only newer file. |

### Config Tests

**Location:** `crates/tuitbot-core/src/config/` (existing test module)

| Test | Assertions |
|------|-----------|
| `content_sources_config_serde_roundtrip` | Serialize `ContentSourceEntry` to TOML → deserialize → fields match. |
| `content_sources_defaults` | Default `ContentSourceEntry` has `source_type = "local_fs"`, `watch = true`, `file_patterns = ["*.md", "*.txt"]`. |
| `content_sources_optional_in_config` | Parse a config TOML without `[content_sources]` → succeeds with empty sources list. |

### Ingest Route Tests

**Location:** `crates/tuitbot-server/tests/api_tests.rs` or `crates/tuitbot-server/src/routes/ingest.rs` (`#[cfg(test)]`)

| Test | Assertions |
|------|-----------|
| `post_ingest_returns_200` | POST /api/ingest with valid auth → 200 with `ingested` count. |
| `post_ingest_requires_auth` | POST /api/ingest without auth → 401. |
| `post_ingest_with_file_hints` | POST with `file_hints: ["a.md"]` → only specified files processed. |
| `post_ingest_idempotent` | POST twice with same content → second returns `skipped: N, ingested: 0`. |

---

## Session 03: Watchtower Runtime and Loop-Back

### Watcher Tests

**Location:** `crates/tuitbot-core/src/automation/watchtower.rs` (`#[cfg(test)]`)

| Test | Assertions |
|------|-----------|
| `watcher_detects_new_file` | Start watcher on tempdir → write .md file → within 5s, content_node row exists in DB. |
| `watcher_ignores_non_matching_files` | Start watcher → write .jpg → after debounce window, no content_node created. |
| `watcher_debounces_rapid_writes` | Write to same file 5 times in 500ms → exactly 1 content_node (or 1 update). |
| `watcher_respects_cancellation` | Start watcher → cancel token → watcher exits within 1s. |
| `manual_and_auto_ingest_identical` | Write file → watcher ingests → delete node → POST /api/ingest same file → same node shape. |

### Loop-Back Tests

**Location:** `crates/tuitbot-core/src/source/local_fs.rs` (`#[cfg(test)]`)

| Test | Assertions |
|------|-----------|
| `write_metadata_to_new_file` | File with no front-matter → write_metadata → file has YAML front-matter with tuitbot key. |
| `write_metadata_to_existing_frontmatter` | File with existing YAML front-matter → write_metadata → tuitbot key added, existing keys preserved. |
| `write_metadata_idempotent` | write_metadata twice with same tweet_id → only one entry in tuitbot array. |
| `write_metadata_multiple_tweets` | write_metadata with tweet A → write_metadata with tweet B → tuitbot array has 2 entries. |

### Self-Event Prevention Tests

| Test | Assertions |
|------|-----------|
| `cooldown_prevents_self_reingest` | Write file → ingest → loop-back writes metadata → no re-ingestion event fired. |
| `cooldown_expires_after_timeout` | Add path to cooldown → wait 6s → path no longer in cooldown set. |

---

## Session 04: Winning DNA Retrieval

### Classification Tests

**Location:** `crates/tuitbot-core/src/context/winning_dna.rs` (`#[cfg(test)]`)

| Test | Assertions |
|------|-----------|
| `classify_agree_and_expand` | Text starting with "Great point" or "Exactly" → classified as AgreeAndExpand. |
| `classify_ask_question` | Text ending with "?" → classified as AskQuestion. |
| `classify_add_data` | Text containing numbers/percentages → classified as AddData. |
| `classify_share_experience` | Text containing "In my experience" or "I've found" → classified as ShareExperience. |
| `classify_fallback` | Ambiguous text → classified as most likely archetype or "unknown". |

### Retrieval Tests

| Test | Assertions |
|------|-----------|
| `retrieve_ancestors_empty_db` | No performance data → returns empty Vec. |
| `retrieve_ancestors_ordered_by_weight` | Insert 3 tweets with scores 0.9, 0.5, 0.7 → retrieve → order is [0.9, 0.7, 0.5]. |
| `retrieve_ancestors_recency_decay` | Insert tweet from 30 days ago (score 0.9) and tweet from today (score 0.6) → today's tweet ranks higher due to recency. |
| `retrieve_ancestors_respects_max` | Insert 10 ancestors → retrieve with max=3 → returns 3. |
| `retrieve_ancestors_min_score_filter` | Insert tweet with score 0.05 → retrieve with min_score=0.1 → excluded. |

### Cold-Start Fallback Tests

| Test | Assertions |
|------|-----------|
| `cold_start_uses_draft_seeds` | No performance data, but content_nodes + seeds exist → retrieval returns seeds as context. |
| `cold_start_no_data_returns_empty` | No performance data, no content_nodes → returns empty → draft pipeline uses existing behavior unchanged. |

### Draft Enrichment Tests

| Test | Assertions |
|------|-----------|
| `draft_with_ancestors_includes_context` | Mock LLM captures system prompt → ancestors present in prompt text. |
| `draft_without_ancestors_unchanged` | No ancestors → system prompt matches existing format exactly. |
| `draft_rag_context_capped_at_max_tokens` | Insert many ancestors → RAG context truncated at ~2000 tokens. |

### Engagement Score Computation Tests

| Test | Assertions |
|------|-----------|
| `compute_engagement_score_basic` | performance_score=67, max_score=100 → engagement_score=0.67. |
| `compute_engagement_score_zero_max` | max_score=0 → engagement_score=0.0 (no division by zero). |
| `upsert_with_engagement_score` | upsert_tweet_performance with engagement_score → column populated. |

---

## Session 05: Validation and Obsidian Shakeout

### End-to-End Integration Test

**Location:** `crates/tuitbot-core/src/` (integration test or test module)

| Test | Assertions |
|------|-----------|
| `e2e_ingest_to_draft` | Create tempdir vault → configure source → ingest → generate seeds → draft with RAG context → verify draft contains note content influence. |
| `e2e_loop_back_round_trip` | Ingest note → mock publish → loop-back writes metadata → verify file has tuitbot front-matter → re-ingest → node not duplicated. |

### Manual Shakeout Checklist (documented in validation-report.md)

1. Create a temp Obsidian-style vault with 5 .md files and 2 .txt files.
2. Configure `[[content_sources.sources]]` in config.toml.
3. Start tuitbot-server → verify watchtower log shows initial scan.
4. Add a new .md file → verify content_node created within 5s.
5. POST /api/ingest → verify response with ingested/skipped counts.
6. Verify draft_seeds created for ingested nodes.
7. If performance data exists: verify Winning DNA retrieval returns ranked ancestors.
8. Verify loop-back writes metadata without corrupting the source file.
9. Re-run quality gates — all pass.

---

## Test Infrastructure Patterns

### Mock ContentSource

```rust
#[cfg(test)]
pub struct MockContentSource {
    pub files: Vec<SourceFile>,
    pub contents: HashMap<String, String>,
}

#[async_trait]
impl ContentSource for MockContentSource {
    fn source_type(&self) -> &str { "mock" }
    async fn scan_for_changes(&self, _since: Option<DateTime<Utc>>) -> Result<Vec<SourceFile>, SourceError> {
        Ok(self.files.clone())
    }
    async fn read_content(&self, path: &str) -> Result<String, SourceError> {
        self.contents.get(path).cloned().ok_or(SourceError::NotFound(path.to_string()))
    }
    async fn write_metadata(&self, _path: &str, _metadata: &LoopBackMetadata) -> Result<(), SourceError> {
        Ok(())
    }
}
```

### Tempdir Fixtures

For filesystem tests, use `tempfile::tempdir()` (already in dev-dependencies). Create fixture files programmatically:

```rust
fn create_test_vault(dir: &Path) -> Vec<PathBuf> {
    let files = vec![
        ("note1.md", "# Rust Tips\nSome content about Rust."),
        ("note2.md", "# AI Thoughts\nContent about AI."),
        ("image.jpg", "not a text file"),
    ];
    // ... write files and return paths
}
```

### Test Naming Convention

Follow existing pattern: `module_name::tests::descriptive_test_name`

Examples:
- `watchtower::tests::insert_and_get_source_context`
- `winning_dna::tests::retrieve_ancestors_ordered_by_weight`
- `local_fs::tests::scan_finds_md_files`
