# Google Drive Integration — Architecture Decision Record

## Context

Sessions 01–05 built the Watchtower ingest pipeline, storage layer, Winning DNA seed extraction, and dashboard UX for local filesystem content sources. Session 06 extends the system to support Google Drive as a remote content source, feeding the same ingest pipeline and Winning DNA workflow.

## Decision: Provider Trait Abstraction

### `ContentSourceProvider` trait (`source/mod.rs`)

A new `ContentSourceProvider` async trait abstracts content reading across backends:

```rust
#[async_trait]
pub trait ContentSourceProvider: Send + Sync {
    fn source_type(&self) -> &str;
    async fn scan_for_changes(
        &self,
        since_cursor: Option<&str>,
        patterns: &[String],
    ) -> Result<Vec<SourceFile>, SourceError>;
    async fn read_content(&self, file_id: &str) -> Result<String, SourceError>;
}
```

**Why a trait, not an enum?** The trait allows future providers (Notion, Dropbox) to be added without modifying existing code. The trait covers scan + read only — watching vs polling is handled by the `WatchtowerLoop` orchestrator.

### Implementations

- **`LocalFsProvider`** (`source/local_fs.rs`): Wraps existing directory walking + file reading. Uses `spawn_blocking` for filesystem I/O. Matches the existing `WatchtowerLoop::walk_directory` behavior.

- **`GoogleDriveProvider`** (`source/google_drive.rs`): Polls Google Drive API v3 for `.md`/`.txt` files in a configured folder. Uses service-account JWT auth with self-contained RSA signing (no external JWT crate needed — uses the `sha2` and `base64` crates already in the dependency tree).

## Decision: Flat Config with Optional Fields

Rather than a discriminated union, `ContentSourceEntry` uses flat optional fields:

```rust
pub struct ContentSourceEntry {
    pub source_type: String,        // "local_fs" | "google_drive"
    pub path: Option<String>,       // local_fs only
    pub folder_id: Option<String>,  // google_drive only
    pub service_account_key: Option<String>,  // google_drive only
    pub poll_interval_seconds: Option<u64>,   // remote sources
    pub watch: bool,
    pub file_patterns: Vec<String>,
    pub loop_back_enabled: bool,
}
```

**Why flat?** The config serializes to TOML and patches arrive as JSON via the settings API. Flat optional fields preserve backward compatibility with existing configs and the generic merge logic in `settings.rs`.

## Decision: Provider-Agnostic Ingest via `ingest_content()`

A new `ingest_content()` function accepts raw text content and a provider ID, bypassing filesystem reads. The existing `ingest_file()` becomes a thin wrapper that reads from disk then delegates.

**Provider ID format:** Remote files use `gdrive://<file_id>/<filename>` as their `relative_path` in `content_nodes`. This provides stable deduplication by `(source_id, relative_path)`.

## Decision: Self-Contained RSA Signing

The Google Drive auth flow requires RS256 JWT signing. Rather than adding an external JWT or RSA crate, the provider implements minimal PKCS#8/PKCS#1 DER parsing and RSA signing using only `sha2` and `base64` (already present). This keeps the dependency tree lean.

## Decision: Remote Polling in WatchtowerLoop

The `WatchtowerLoop::run()` method now:
1. Splits sources into local (notify watcher) and remote (polling) paths
2. Runs local watcher + fallback timer for filesystem sources (unchanged)
3. Adds a remote poll timer that calls `poll_remote_sources()` for Google Drive sources
4. Both paths feed into `ingest_content()` for identical state transitions

Remote-only configurations (no local sources) use a dedicated `remote_only_loop()`.

## Limitations (v1)

- **Service account auth only**: No interactive OAuth for end-user Google accounts
- **`.md` and `.txt` only**: No Google Docs export conversion
- **Read-only**: No loop-back to Drive (would require write scope)
- **No retry/backoff**: Simple error logging + source status update
- **Polling only**: No Drive push notifications (requires public callback URL)
- **Single source in UI**: Dashboard manages `sources[0]`; multiple sources via config.toml

## Future Work

- Google Docs → Markdown export for native Google Docs files
- Interactive OAuth flow for personal Drive accounts
- Exponential backoff with jitter for API failures
- Multiple sources in the dashboard UI
- Webhook/push notification support for real-time Drive changes
