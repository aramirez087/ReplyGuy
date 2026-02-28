# Architecture Decisions — Cold-Start Watchtower RAG

> Decisions made in Session 01. Each decision is numbered for cross-referencing.

---

## AD-1: Source Model — Adapter-based content source abstraction

### Context

The Watchtower must ingest content from a local Obsidian vault today, but the operator rules require "model content sources so future Google Drive adapters can plug in without changing retrieval contracts." The codebase already uses trait-based abstractions for external services (`XApiClient`, `LlmProvider`).

### Decision

Define a `ContentSource` async trait in `crates/tuitbot-core/src/source/mod.rs` with a v1 `LocalFileSource` implementation.

```rust
/// A content source that the Watchtower can scan and read.
#[async_trait]
pub trait ContentSource: Send + Sync {
    /// Identifier for this source type (e.g., "local_fs", "google_drive").
    fn source_type(&self) -> &str;

    /// Scan for files changed since the given checkpoint.
    /// Returns all files if `since` is None (initial scan).
    async fn scan_for_changes(
        &self,
        since: Option<DateTime<Utc>>,
    ) -> Result<Vec<SourceFile>, SourceError>;

    /// Read the full text content of a file at the given path/URI.
    async fn read_content(&self, path: &str) -> Result<String, SourceError>;

    /// Write loop-back metadata (tweet URL, publish date) to the source file.
    /// Must be idempotent — calling twice with the same metadata is a no-op.
    async fn write_metadata(
        &self,
        path: &str,
        metadata: &LoopBackMetadata,
    ) -> Result<(), SourceError>;
}

/// A file discovered during a scan.
pub struct SourceFile {
    pub relative_path: String,
    pub content_hash: String,      // SHA-256 of file content
    pub modified_at: DateTime<Utc>,
    pub size_bytes: u64,
}

/// Metadata written back to source files after publication.
pub struct LoopBackMetadata {
    pub tweet_url: String,
    pub published_at: DateTime<Utc>,
    pub tweet_id: String,
    pub content_type: String,      // "tweet", "reply", "thread"
}
```

### Layer placement

`source/` sits alongside `x_api/`, `storage/`, `llm/` in the Foundation layer. It is a stateless I/O adapter — no DB or LLM dependency.

### v1 implementation: `LocalFileSource`

```rust
pub struct LocalFileSource {
    root_path: PathBuf,
    file_patterns: Vec<String>,  // ["*.md", "*.txt"]
}
```

- `scan_for_changes` walks the directory tree, filters by pattern, returns `SourceFile` with SHA-256 hash.
- `read_content` reads UTF-8 file contents.
- `write_metadata` parses existing YAML front-matter (if any) and appends/updates a `tuitbot` key:
  ```yaml
  ---
  tuitbot:
    - tweet_id: "1234567890"
      url: "https://x.com/user/status/1234567890"
      published_at: "2026-02-28T14:30:00Z"
      type: "tweet"
  ---
  ```

### Configuration

Add to `config/types.rs`:

```rust
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContentSourcesConfig {
    #[serde(default)]
    pub sources: Vec<ContentSourceEntry>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContentSourceEntry {
    /// Source type: "local_fs" (v1). Future: "google_drive".
    #[serde(default = "default_source_type")]
    pub source_type: String,

    /// Filesystem path (for local_fs). Supports ~ expansion.
    #[serde(default)]
    pub path: Option<String>,

    /// Whether to watch for changes in real-time.
    #[serde(default = "default_watch")]
    pub watch: bool,

    /// File patterns to include.
    #[serde(default = "default_file_patterns")]
    pub file_patterns: Vec<String>,

    /// Whether to write metadata back to source files.
    #[serde(default = "default_loop_back")]
    pub loop_back_enabled: bool,
}
```

TOML example:

```toml
[[content_sources.sources]]
source_type = "local_fs"
path = "~/notes/obsidian-vault"
watch = true
file_patterns = ["*.md", "*.txt"]
loop_back_enabled = true
```

### Rationale

- Matches established trait pattern (`XApiClient`, `LlmProvider`).
- `LocalFileSource` has zero external dependencies — local-first.
- Future `GoogleDriveSource` implements the same trait, selected at config time.
- The `write_metadata` method on the trait ensures loop-back is adapter-aware (Google Drive uses API, local uses file I/O).

### Alternatives considered

1. **Direct filesystem calls in Watchtower loop:** Rejected — couples the watcher to local filesystem, makes future adapters require rewriting the loop.
2. **Plugin/dynamic loading:** Rejected — over-engineered for 2 adapters. Static dispatch via trait object is sufficient.

---

## AD-2: Schema Evolution — Three new tables, additive columns

### Context

The Watchtower needs to track content sources, ingested content nodes, and pre-computed draft seeds. Performance analytics need archetype/vibe classification for Winning DNA retrieval.

### Decision

Create `migrations/20260228000019_watchtower_ingestion.sql` with:

**New tables:**

```sql
-- Registered content sources
CREATE TABLE IF NOT EXISTS source_contexts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    account_id TEXT NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000',
    source_type TEXT NOT NULL,             -- 'local_fs', 'google_drive'
    config_json TEXT NOT NULL DEFAULT '{}', -- serialized ContentSourceEntry
    sync_cursor TEXT,                       -- ISO-8601 of last successful scan
    status TEXT NOT NULL DEFAULT 'active',  -- 'active', 'paused', 'error'
    error_message TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Ingested content chunks from sources
CREATE TABLE IF NOT EXISTS content_nodes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    account_id TEXT NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000',
    source_id INTEGER NOT NULL REFERENCES source_contexts(id),
    relative_path TEXT NOT NULL,
    content_hash TEXT NOT NULL,            -- SHA-256, for change detection
    title TEXT,                             -- extracted from front-matter or first heading
    body_text TEXT NOT NULL,                -- full extracted text
    front_matter_json TEXT,                 -- parsed YAML/JSON front-matter
    tags TEXT,                              -- comma-separated tags from front-matter
    status TEXT NOT NULL DEFAULT 'pending', -- 'pending', 'processed', 'archived'
    ingested_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(source_id, relative_path)
);

-- Pre-computed draft seeds (hooks/angles from content nodes)
CREATE TABLE IF NOT EXISTS draft_seeds (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    account_id TEXT NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000',
    node_id INTEGER NOT NULL REFERENCES content_nodes(id),
    seed_text TEXT NOT NULL,               -- hook or angle text
    archetype_suggestion TEXT,             -- suggested ReplyArchetype or TweetFormat
    engagement_weight REAL NOT NULL DEFAULT 0.5, -- 0.0-1.0, updated by Winning DNA
    status TEXT NOT NULL DEFAULT 'pending', -- 'pending', 'used', 'expired'
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    used_at TEXT
);

CREATE INDEX IF NOT EXISTS idx_content_nodes_source ON content_nodes(source_id, status);
CREATE INDEX IF NOT EXISTS idx_content_nodes_hash ON content_nodes(content_hash);
CREATE INDEX IF NOT EXISTS idx_draft_seeds_status ON draft_seeds(status, engagement_weight DESC);
CREATE INDEX IF NOT EXISTS idx_draft_seeds_node ON draft_seeds(node_id);
```

**Additive columns on existing tables:**

```sql
-- Archetype/vibe classification for Winning DNA
ALTER TABLE tweet_performance ADD COLUMN archetype_vibe TEXT;
ALTER TABLE reply_performance ADD COLUMN archetype_vibe TEXT;

-- Normalized engagement score (0.0-1.0) for retrieval weighting
ALTER TABLE tweet_performance ADD COLUMN engagement_score REAL;
ALTER TABLE reply_performance ADD COLUMN engagement_score REAL;

-- Provenance: link published tweets back to their source content node
ALTER TABLE original_tweets ADD COLUMN source_node_id INTEGER REFERENCES content_nodes(id);
```

### Rationale

- All changes are additive — no existing column modifications or removals.
- `CREATE TABLE IF NOT EXISTS` and `ALTER TABLE ADD COLUMN` are safe for WAL-mode SQLite.
- Multi-account isolation maintained via `account_id` default.
- `content_hash` enables efficient change detection without re-reading file content.
- `UNIQUE(source_id, relative_path)` prevents duplicate ingestion of the same file.
- Indexes target the primary query patterns: scan by source/status, lookup by hash, retrieve seeds by weight.

---

## AD-3: Layer Placement

### Decision

| New Component | Layer | Module Path | Dependencies |
|--------------|-------|-------------|--------------|
| `ContentSource` trait | Foundation | `core/source/mod.rs` | None (stateless I/O) |
| `LocalFileSource` | Foundation | `core/source/local_fs.rs` | `std::fs`, `sha2` |
| `SourceError` | Foundation | `core/source/mod.rs` | `thiserror` |
| Storage CRUD | Foundation | `core/storage/watchtower.rs` | `sqlx`, `DbPool` |
| Winning DNA classifier | L2 Workflow | `core/context/winning_dna.rs` | DB + analytics queries |
| Draft RAG enrichment | L2 Workflow | `core/workflow/draft.rs` (extension) | DB + LLM + ContentSource |
| Watchtower watcher | L3 Autopilot | `core/automation/watchtower.rs` | ContentSource + storage + CancellationToken |
| Ingest route | Server | `server/routes/ingest.rs` | AppState (thin delegation) |

### Rationale

This placement follows the strict dependency rules in `docs/architecture.md`:
- Foundation modules have no upward dependencies
- Workflow calls Foundation (storage, source) but never Automation
- Automation calls Workflow + Foundation but never Server
- Server routes are thin adapters over Workflow/Foundation

---

## AD-4: Watchtower Runtime Pattern

### Context

The existing runtime spawns loops via `runtime.spawn(name, future)` with a shared `CancellationToken`. The Watchtower needs to run as a background service that watches filesystem changes.

### Decision

Create `automation/watchtower.rs` following the existing loop patterns:

```rust
pub struct WatchtowerLoop {
    source: Arc<dyn ContentSource>,
    db: DbPool,
    debounce_ms: u64,                                      // default 2000
    cooldown_paths: Arc<Mutex<HashMap<PathBuf, Instant>>>,  // path → expiry time
}

impl WatchtowerLoop {
    pub async fn run(
        &self,
        cancel: CancellationToken,
        scheduler: LoopScheduler,    // fallback interval scan
    ) { /* ... */ }
}
```

**Lifecycle:**
1. On startup, perform initial scan via `source.scan_for_changes(None)`.
2. Start `notify` watcher on configured paths (debounced, 2s window).
3. On each event: check cooldown → compute content_hash → skip if unchanged → ingest.
4. Fallback: periodic scan every `scheduler.interval()` (default 5 minutes).
5. On cancellation: flush pending ingests, close watcher.

**Integration with AppState:**
- Store optional `CancellationToken` for watchtower in AppState.
- Start when runtime starts (if `content_sources` configured).
- Stop when runtime stops.

**Shared ingest pipeline:**
- Both filesystem events and POST /api/ingest call the same `storage::watchtower::ingest_file()` function.
- This ensures behavioral identity between manual and automatic triggers.

### Self-event prevention

When loop-back writes metadata to a source file, that file's path is added to a cooldown set with a 5-second expiry. Filesystem events for paths in cooldown are silently ignored. The content_hash check provides a second safety net — if the hash matches, no re-ingestion occurs.

---

## AD-5: Analytics-Weighted RAG — Winning DNA

### Context

The cold-start problem: new Tuitbot users have no performance history to guide content generation. The Watchtower solves cold-start by ingesting the user's existing notes as content seeds. Over time, as tweets are published and measured, the Winning DNA system classifies successful patterns and weights retrieval toward them.

### Decision

Create `context/winning_dna.rs` in the existing context module:

```rust
/// Classified tweet with engagement data for retrieval.
pub struct WinningAncestor {
    pub tweet_id: String,
    pub content_preview: String,    // truncated to 120 chars
    pub archetype_vibe: String,     // classified archetype
    pub engagement_score: f64,      // 0.0-1.0 normalized
    pub published_at: DateTime<Utc>,
    pub retrieval_weight: f64,      // engagement * recency_decay
}

/// Retrieve top-K winning ancestors for a given topic.
pub async fn retrieve_ancestors(
    pool: &DbPool,
    topic_keywords: &[String],
    max_results: usize,           // default 5
    recency_half_life_days: f64,  // default 14.0
) -> Result<Vec<WinningAncestor>, StorageError>
```

**Retrieval algorithm:**

1. Query `tweet_performance` + `original_tweets` WHERE topic matches keywords AND `engagement_score IS NOT NULL`.
2. Compute `retrieval_weight = engagement_score * exp(-0.693 * days_since / half_life)`.
3. Order by `retrieval_weight DESC`, take top K.
4. For cold-start (no performance data): fall back to `content_nodes` + `draft_seeds` ordered by `engagement_weight DESC`.

**Classification pipeline (background worker):**

1. When `analytics_loop` measures tweet performance, also classify the archetype:
   - Match reply content against `ReplyArchetype` patterns (agree, question, experience, data, disagree).
   - Match tweet content against `TweetFormat` patterns (list, contrarian, storytelling, etc.).
2. Store classification in `archetype_vibe` column.
3. Compute normalized `engagement_score` = `performance_score / max(all_performance_scores)`.

**Integration with draft pipeline:**

In `workflow/draft.rs`, after fetching the candidate tweet and before calling the LLM:

1. Call `retrieve_ancestors(pool, matched_keywords, 5, 14.0)`.
2. Format ancestors as context block:
   ```
   ## Winning patterns from your best-performing content:
   - [AgreeAndExpand, score 0.92]: "Great observation about..."
   - [AddData, score 0.78]: "The data shows that..."
   ```
3. Inject into system prompt between persona context and format constraints.
4. Cap total RAG context at 2000 tokens.

**Cold-start behavior:**

When no performance data exists (new user):
1. If content_nodes exist (user configured Obsidian vault): use draft_seeds as context.
2. If no content_nodes exist: fall back to existing behavior (persona + business profile only).
3. This means the existing generation path is the zero-config default — RAG is purely additive.

### Thresholds (documented, configurable in future)

| Parameter | Default | Rationale |
|-----------|---------|-----------|
| `recency_half_life_days` | 14 | Balances recent success vs. historical data |
| `max_ancestors` | 5 | Keeps prompt token count reasonable |
| `cold_start_weight` | 0.5 | Baseline weight for unscored nodes |
| `min_engagement_score` | 0.1 | Filter out lowest-performing ancestors |
| `rag_max_tokens` | 2000 | Prevents context window bloat |

---

## AD-6: HTTP Contract — POST /api/ingest

### Decision

Add `POST /api/ingest` as a thin authenticated route:

```
POST /api/ingest
Content-Type: application/json
Authorization: Bearer <token>

{
    "source_type": "local_fs",          // optional, default from config
    "file_hints": ["notes/idea.md"],    // optional, specific files to re-scan
    "force": false                      // optional, re-ingest even if hash unchanged
}

Response 200:
{
    "ingested": 3,
    "skipped": 12,
    "errors": [],
    "duration_ms": 450
}
```

### Rationale

- `file_hints` supports manual triggers from iOS Shortcuts or Telegram bots.
- `force` enables re-ingestion after external edits that don't change the hash (unlikely but possible).
- Response includes counts for observability.
- Auth-required (follows existing auth middleware pattern).
