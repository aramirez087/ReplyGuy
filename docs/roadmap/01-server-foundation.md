# 01 — Server Foundation (`tuitbot-server`)

> **Goal:** Create the `tuitbot-server` crate — an axum HTTP server that exposes
> tuitbot-core's storage layer as a read-only REST API. This is the bedrock that
> every subsequent task builds on.

## Context

Tuitbot is a Rust workspace with three existing crates:

- `tuitbot-core` — all business logic, storage (SQLite/SQLx), automation, scoring, etc.
- `tuitbot-cli` — thin CLI binary
- `tuitbot-mcp` — MCP server for AI agent integration

We are adding `tuitbot-server`, a fourth crate that serves as the HTTP API backend
for a desktop dashboard. The server imports `tuitbot-core` and exposes its data
over HTTP. It owns zero business logic.

## What to build

### 1. Scaffold the crate

Create `crates/tuitbot-server/` with:

- `Cargo.toml` — depends on `tuitbot-core` (path + version), `axum`, `tokio`,
  `serde`, `serde_json`, `tower-http` (cors, trace), `tracing`, `anyhow`
- Add it to the workspace `members` in the root `Cargo.toml`
- Include full crates.io metadata (description, license, repository, homepage,
  documentation, keywords) to match existing crates

### 2. AppState

Create `src/state.rs`:

```rust
pub struct AppState {
    pub db: tuitbot_core::storage::DbPool,
    pub config_path: PathBuf,
}
```

The state is shared via `axum::Extension<Arc<AppState>>` or axum's `State` extractor.
Initialize the DB pool using `tuitbot_core::storage::init_db()` with the path from
config (default `~/.tuitbot/tuitbot.db`).

### 3. Route modules (read-only for now)

Create `src/routes/` with these modules. Each returns JSON. Use proper HTTP status
codes and a consistent error envelope.

#### `analytics.rs`
- `GET /api/analytics/followers` — follower snapshots (query: `days=7`)
- `GET /api/analytics/performance` — reply + tweet performance summaries
- `GET /api/analytics/topics` — topic scores from `content_scores` table

Map these to existing functions in `tuitbot_core::storage::analytics`.

#### `approval.rs`
- `GET /api/approval` — list pending approval items (query: `status=pending`)

Map to `tuitbot_core::storage::approval_queue::get_pending_items()`.

#### `activity.rs`
- `GET /api/activity` — recent actions from `action_log` table (query: `limit=50`)

Map to `tuitbot_core::storage::action_log`.

#### `replies.rs`
- `GET /api/replies` — recent replies sent (query: `limit=50`, `offset=0`)

Map to `tuitbot_core::storage::replies`.

#### `content.rs`
- `GET /api/content/tweets` — recent original tweets posted
- `GET /api/content/threads` — recent threads posted

Map to `tuitbot_core::storage::tweets` and `tuitbot_core::storage::threads`.

#### `targets.rs`
- `GET /api/targets` — list target accounts and their state

Map to `tuitbot_core::storage::target_accounts`.

#### `health.rs`
- `GET /api/health` — returns `{"status": "ok", "version": "..."}`, confirms DB connectivity

### 4. Router assembly

Create `src/lib.rs` that builds the axum `Router`:

```rust
pub fn build_router(state: Arc<AppState>) -> Router {
    Router::new()
        .nest("/api", api_routes())
        .layer(CorsLayer::permissive())  // localhost-only in production
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
```

### 5. Binary entry point

Create `src/main.rs`:

- Parse CLI args (optional: `--port`, `--config`)
- Load config path (default `~/.tuitbot/config.toml`)
- Initialize DB via `tuitbot_core::storage::init_db()`
- Build router, bind to `127.0.0.1:3001`, serve

### 6. Error handling

Create `src/error.rs` with an `ApiError` type that implements `IntoResponse`:

- Maps `tuitbot_core::StorageError` → 500
- Maps not-found scenarios → 404
- Returns JSON: `{"error": "message"}`

## What NOT to build yet

- Authentication/auth middleware (task 02)
- WebSocket support (task 02)
- Write/mutation endpoints (task 02)
- Frontend (task 03)
- Any new storage queries — only use what tuitbot-core already exposes. If a query
  doesn't exist, add it to tuitbot-core (not to the server crate)

## Acceptance criteria

- [ ] `cargo build -p tuitbot-server` compiles
- [ ] `cargo test -p tuitbot-server` passes (at minimum: health endpoint test)
- [ ] `cargo run -p tuitbot-server` starts and responds to `curl http://localhost:3001/api/health`
- [ ] All 7 route groups return valid JSON (test with curl against a real DB or write integration tests with `init_test_db()`)
- [ ] CI checklist passes: `cargo fmt --all --check && cargo clippy --workspace -- -D warnings && RUSTFLAGS="-D warnings" cargo test --workspace`
- [ ] Crate has full metadata and `cargo package -p tuitbot-server --allow-dirty` succeeds

## Reference files

- `crates/tuitbot-core/src/storage/mod.rs` — DB init, pool type
- `crates/tuitbot-core/src/storage/analytics.rs` — analytics queries
- `crates/tuitbot-core/src/storage/approval_queue.rs` — approval queue queries
- `crates/tuitbot-core/src/storage/action_log.rs` — action log queries
- `crates/tuitbot-core/src/storage/replies.rs` — reply queries
- `crates/tuitbot-core/src/storage/tweets.rs` — tweet queries
- `crates/tuitbot-core/src/storage/threads.rs` — thread queries
- `crates/tuitbot-core/src/storage/target_accounts.rs` — target account queries
- `crates/tuitbot-mcp/Cargo.toml` — reference for crate metadata format
