# Session 01 Handoff -- Factory Reset Danger Zone

## Completed Work

1. **Full audit of repository anchors:**
   - Bearer auth (Tauri/desktop) and cookie auth (web/LAN) boot sequences
     in `+layout.svelte`.
   - Auth-exempt routes: health, settings/status, settings/init, ws,
     auth/login, auth/status.
   - Onboarding flow: multi-step wizard -> `POST /api/settings/init` with
     optional `claim`.
   - Settings page structure: 10 sections with sticky nav, section components.
   - Storage layer: **30 user tables** (not 27 as initially estimated),
     WAL mode, pool of 4, 20 embedded migrations.
   - **7 foreign key constraints** verified from migration SQL (see charter).
   - File artifacts: `config.toml`, `passphrase_hash`, `api_token`, `media/`,
     `backups/`.
   - In-memory state: `passphrase_hash` RwLock, `runtimes` map,
     `content_generators`, `login_attempts`, `watchtower_cancel`,
     `circuit_breaker`.

2. **Charter document** (`charter.md`):
   - Destructive scope: what is cleared, preserved, and explicitly excluded.
   - Endpoint contract: `POST /api/settings/factory-reset` with
     `{ "confirmation": "RESET TUITBOT" }`.
   - Execution order: stop runtimes -> clear DB -> delete files -> clear
     memory -> VACUUM -> respond.
   - Architecture placement: core `storage/reset.rs` + server handler in
     `routes/settings.rs`.
   - UX design: Danger Zone section with typed confirmation phrase.
   - Safety rules: auth required, no content source deletion, transaction
     safety, runtime stop-first.
   - File plan: exact files to create/modify in Sessions 2 and 3.
   - Testing strategy: unit + integration + frontend checks.

## Key Decisions

| Decision | Rationale |
|----------|-----------|
| Confirmation phrase `RESET TUITBOT` (not random, not timer) | Simple, memorable, meets "explicit typed phrase" requirement. Case-sensitive for extra friction. |
| Handler in `routes/settings.rs` (not a new file) | Settings already owns init/patch/status; factory-reset is the inverse of init. Keeps related endpoints together. |
| Core logic in `storage/reset.rs` (not workflow) | This is a storage-layer operation (DELETE FROM tables). No X API, no LLM involvement. |
| DELETE rows, not DROP tables | Schema must survive for the pool and migrations to work. Empty tables are the designed init state. |
| Preserve `api_token` | Tauri reads this file on launch. If deleted, Tauri cannot auth until server restart -- breaks "live reset". |
| Single transaction for DB clears | Prevents partial state if a DELETE fails. All-or-nothing for DB portion. |
| Stop runtimes before clearing DB | Automation loops write to tables; clearing while loops run causes races. |
| Frontend redirect via existing boot logic | `+layout.svelte` already redirects to `/onboarding` when `configured=false`. No special redirect needed. |
| 30 tables (corrected from plan's 27) | Verified by reading all 20 migration files. The `strategy_reports` table and three others were missed in the initial estimate. |

## Open Issues

1. **VACUUM timing** -- VACUUM can be slow on large databases and holds an
   exclusive lock.  Consider making it optional or running it after the
   response is sent (fire-and-forget `tokio::spawn`).  Decision deferred to
   Session 2.

2. **Partial failure reporting** -- If DB transaction succeeds but file
   deletion fails (permissions, etc.), the reset is in a mixed state: DB clean
   but config still exists.  The response should report per-step success.
   Decision: log warnings and continue; response includes what succeeded.

3. **WebSocket notification** -- Should a `WsEvent` be broadcast before reset
   so connected clients redirect proactively?  The session cookie is cleared,
   so the next API call will 401 and the frontend will redirect anyway.
   Nice-to-have but not required.

4. **Multi-account runtimes** -- The `runtimes` map is keyed by `account_id`.
   Factory reset must stop ALL accounts' runtimes, not just one.  The handler
   should `drain()` the entire map.

## Exact Inputs for Session 2

### Specifications

- **Endpoint:** `POST /api/settings/factory-reset`
- **Confirmation phrase:** `"RESET TUITBOT"` (case-sensitive, exact match)
- **Core function:** `tuitbot_core::storage::reset::factory_reset(pool: &DbPool) -> Result<ResetStats, StorageError>`
- **Route placement:** In `lib.rs`, within the settings group (after
  `/settings/test-llm`, before `/settings`)

### Files to Create

1. `crates/tuitbot-core/src/storage/reset.rs` -- core DB clearing logic +
   inline `#[cfg(test)]` module
2. `crates/tuitbot-server/tests/factory_reset.rs` -- integration tests

### Files to Modify

3. `crates/tuitbot-core/src/storage/mod.rs` -- add `pub mod reset;`
4. `crates/tuitbot-server/src/routes/settings.rs` -- add `factory_reset`
   handler + `FactoryResetRequest` / response types
5. `crates/tuitbot-server/src/lib.rs` -- add
   `.route("/settings/factory-reset", post(routes::settings::factory_reset))`

### FK-Safe Deletion Order (verified from migrations)

```
 1. draft_seeds              (FK -> content_nodes.id)
 2. original_tweets          (FK -> content_nodes.id via source_node_id)
 3. content_nodes            (FK -> source_contexts.id)
 4. thread_tweets            (FK -> threads.id, ON DELETE CASCADE)
 5. account_roles            (FK -> accounts.id, ON DELETE CASCADE)
 6. target_tweets            (FK -> target_accounts.account_id)
 7. approval_edit_history    (FK -> approval_queue.id)
--- no FK constraints below this line ---
 8. reply_performance
 9. tweet_performance
10. replies_sent
11. discovered_tweets
12. threads
13. approval_queue
14. scheduled_content
15. target_accounts
16. follower_snapshots
17. content_scores
18. strategy_reports
19. rate_limits
20. action_log
21. cursors
22. author_interactions
23. media_uploads
24. llm_usage
25. x_api_usage
26. mcp_telemetry
27. mutation_audit
28. source_contexts
29. sessions
30. accounts
```

### In-Memory State to Clear (in handler)

```rust
// 1. Stop runtimes
let mut runtimes = state.runtimes.lock().await;
for (_, mut rt) in runtimes.drain() {
    rt.shutdown().await;
}
drop(runtimes);

// 2. Cancel watchtower
if let Some(ref token) = state.watchtower_cancel {
    token.cancel();
}

// 3. Clear passphrase
*state.passphrase_hash.write().await = None;

// 4. Clear generators
state.content_generators.lock().await.clear();

// 5. Clear login attempts
state.login_attempts.lock().await.clear();
```

### CI Checklist

```bash
cargo fmt --all && cargo fmt --all --check
RUSTFLAGS="-D warnings" cargo test --workspace
cargo clippy --workspace -- -D warnings
```

## Exact Inputs for Session 3

### Files to Create

1. `dashboard/src/routes/(app)/settings/DangerZoneSection.svelte`

### Files to Modify

2. `dashboard/src/lib/api.ts` -- add `factoryReset` method to `api.settings`
3. `dashboard/src/routes/(app)/settings/+page.svelte` -- import section, add
   nav entry (`AlertTriangle` icon, id `danger`), render after LAN section

### API Client Method

```typescript
factoryReset: (confirmation: string) =>
    request<{ status: string; cleared: Record<string, unknown> }>(
        '/api/settings/factory-reset',
        {
            method: 'POST',
            body: JSON.stringify({ confirmation })
        }
    )
```

### Frontend Checks

```bash
cd dashboard && npm run check
cd dashboard && npm run build
```
