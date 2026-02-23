# 02 — API Mutations, Auth & WebSocket

> **Goal:** Complete the REST API with write endpoints, add local auth, and add
> WebSocket support for real-time event streaming. After this task, the server is
> fully functional and ready for a frontend to consume.

## Prerequisites

- Task 01 completed: `tuitbot-server` crate exists with read-only routes, compiles, tests pass.

## Context

Task 01 built read-only GET endpoints. Now we need:

1. **Mutation endpoints** — approve/reject items, update settings, manage targets,
   compose manual tweets
2. **Local auth** — a bearer token generated on first run, stored in the config dir,
   required for all API requests
3. **WebSocket** — a `/api/ws` endpoint that streams real-time events to the dashboard

## What to build

### 1. Auth middleware

Create `src/auth.rs`:

- On first server start, if `~/.tuitbot/api_token` doesn't exist, generate a random
  256-bit token (hex-encoded), write it to `~/.tuitbot/api_token` with `0600` permissions
- Create an axum middleware layer that checks `Authorization: Bearer <token>` on all
  `/api/*` routes except `/api/health`
- Return 401 with `{"error": "unauthorized"}` on missing/invalid token
- The dashboard (Tauri) reads this file at startup to authenticate

### 2. Mutation routes

#### `approval.rs` (extend from task 01)
- `POST /api/approval/:id/approve` — approve a queued item → calls core's approval queue approve + posts to X
- `POST /api/approval/:id/reject` — reject a queued item
- `POST /api/approval/approve-all` — batch approve all pending items

Map to `tuitbot_core::storage::approval_queue::approve_item()`, `reject_item()`, etc.
For actual posting on approve, you'll need access to the X API client — add it to `AppState`.

#### `targets.rs` (extend from task 01)
- `POST /api/targets` — add a new target account (body: `{"username": "..."}`)
- `DELETE /api/targets/:username` — remove a target account

Map to `tuitbot_core::storage::target_accounts`.

#### `settings.rs` (new)
- `GET /api/settings` — return the current config as JSON (read and parse the TOML file)
- `PATCH /api/settings` — update specific config fields (merge into existing TOML, write back)

For PATCH, accept a partial JSON object matching the config structure. Read the current
TOML, merge the provided fields, validate, write back. Use `toml` crate for serialization.

#### `content.rs` (extend from task 01)
- `POST /api/content/tweets` — compose and queue a manual tweet
  Body: `{"text": "...", "scheduled_for": "ISO8601 optional"}`
- `POST /api/content/threads` — compose and queue a manual thread
  Body: `{"tweets": ["...", "..."], "scheduled_for": "ISO8601 optional"}`

If approval mode is on, these go to the approval queue. Otherwise, post directly.

#### `runtime.rs` (new)
- `GET /api/runtime/status` — is the automation runtime running? which loops are active?
- `POST /api/runtime/start` — start the automation runtime (if not already running)
- `POST /api/runtime/stop` — gracefully stop the runtime (cancel token)

Add an `Option<Runtime>` or runtime handle to `AppState` (behind a `Mutex` or `RwLock`).

### 3. WebSocket hub

Create `src/ws.rs`:

- `GET /api/ws` → WebSocket upgrade (auth required via query param `?token=...`)
- Use `tokio::sync::broadcast` channel in `AppState` for event fan-out
- Define event types as a tagged enum:

```rust
#[derive(Serialize)]
#[serde(tag = "type")]
pub enum WsEvent {
    ActionPerformed { action_type: String, target: String, content: String, timestamp: String },
    ApprovalQueued { id: i64, action_type: String, content: String },
    FollowerUpdate { count: i64, change: i64 },
    RuntimeStatus { running: bool, active_loops: Vec<String> },
    Error { message: String },
}
```

- The broadcast channel is created in `AppState` with capacity 256
- Mutation endpoints and the automation runtime publish events to this channel
- Each WebSocket connection subscribes and forwards events as JSON

### 4. Update AppState

Extend `AppState` from task 01:

```rust
pub struct AppState {
    pub db: DbPool,
    pub config_path: PathBuf,
    pub event_tx: broadcast::Sender<WsEvent>,
    pub api_token: String,
    // X API client and runtime handle added as needed
}
```

### 5. Event publishing from core

The automation loops in `tuitbot-core` need a way to publish events without depending
on the server crate. Options:

- **Option A (preferred):** Add an optional `tokio::sync::broadcast::Sender<serde_json::Value>`
  to the core `Runtime` struct. When set, loops publish events. When `None` (CLI mode), no-op.
- **Option B:** Use a trait callback that the server implements.

Go with Option A — it's simpler and broadcast channels are already in the tokio ecosystem.

## What NOT to build yet

- Frontend (task 03)
- Any UI-specific data transformations — keep responses as close to the raw storage types as possible

## Acceptance criteria

- [ ] Auth middleware works: requests without token get 401, requests with token succeed
- [ ] All mutation endpoints work (test with curl or integration tests)
- [ ] WebSocket connects at `/api/ws?token=...` and receives events
- [ ] Publishing a test event via broadcast channel arrives at connected WebSocket clients
- [ ] Runtime start/stop endpoints work (start automation, stop it gracefully)
- [ ] Settings GET/PATCH round-trips correctly (read config → modify → read again)
- [ ] CI checklist passes

## Reference files

- `crates/tuitbot-server/` — everything from task 01
- `crates/tuitbot-core/src/automation/mod.rs` — `Runtime` struct, `CancellationToken`
- `crates/tuitbot-core/src/automation/posting_queue.rs` — posting queue, approval routing
- `crates/tuitbot-core/src/storage/approval_queue.rs` — approve/reject functions
- `crates/tuitbot-core/src/config/mod.rs` — config structure, loading, defaults
