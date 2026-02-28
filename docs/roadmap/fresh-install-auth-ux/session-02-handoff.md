# Session 02 Handoff — Backend Claim Bootstrap

**Date:** 2026-02-28
**Branch:** `feat/init_improvements`

---

## What Changed

This session implemented the backend claim bootstrap flow. A fresh install can now establish its passphrase and receive a valid web session during `POST /api/settings/init`.

### Files Modified

| File | Change |
|------|--------|
| `crates/tuitbot-core/src/auth/error.rs` | Added `AlreadyClaimed` variant to `AuthError` |
| `crates/tuitbot-core/src/auth/passphrase.rs` | Added `create_passphrase_hash()`, `is_claimed()`, and 5 unit tests |
| `crates/tuitbot-server/src/routes/settings.rs` | Extended `init_settings` with optional `claim` handling (passphrase hash + session cookie); added `claimed` field to `config_status` |
| `crates/tuitbot-server/src/main.rs` | Made `ensure_passphrase()` conditional on `cli.host == "0.0.0.0"` |
| `docs/lan-mode.md` | Added "First-Time Setup (Browser)" section |

### Files Created

| File | Purpose |
|------|---------|
| `crates/tuitbot-server/tests/fresh_install_auth.rs` | 8 integration tests for the claim bootstrap flow |
| `docs/roadmap/fresh-install-auth-ux/backend-contract.md` | Full API contract for the modified endpoints |
| `docs/roadmap/fresh-install-auth-ux/session-02-handoff.md` | This file |

---

## Design Decisions Made

### D1: `claim` field extracted via `obj.remove()` — not a typed wrapper struct

The `init_settings` handler continues to accept `Json<Value>`. The `claim` field is extracted with `obj.remove("claim")` before JSON-to-TOML conversion. This avoids coupling the server to the core's `Config` layout and maintains backward compatibility.

### D2: Passphrase validation — minimum 8 characters, no format requirement

The frontend will suggest 4-word EFF passphrases, but the backend accepts any string >= 8 chars. This is the simplest useful validation.

### D3: `is_claimed()` checks the filesystem, not in-memory state

The `config_status` endpoint uses `is_claimed()` which reads `data_dir/passphrase_hash`. This is always correct regardless of when the passphrase was created relative to server startup.

### D4: Conditional passphrase generation uses `cli.host`, not `bind_host`

`cli.host` reflects explicit user intent ("I started with `--host 0.0.0.0`"). For first-time installs (no config), `cli.host == bind_host`. For returning users, the passphrase already exists. Using `cli.host` is simpler and more predictable.

### D5: Return type of `init_settings` changed to `impl IntoResponse`

The claim path sets a `Set-Cookie` header. The previous `Result<Json<Value>, ApiError>` return type can't include headers. The new `Result<impl IntoResponse, ApiError>` handles both claim and non-claim paths using Axum's `.into_response()` pattern.

### D6: Atomicity is check-then-write, not `O_EXCL`

Consistent with the existing `ensure_passphrase` pattern. The race window is negligible because `init_settings` already returns 409 if config exists, so a second concurrent request fails at the config level before reaching claim.

---

## CI Results

All checks pass:

```
cargo fmt --all --check         ✅
RUSTFLAGS="-D warnings" cargo test --workspace  ✅ (all tests pass, including 8 new)
cargo clippy --workspace -- -D warnings          ✅
```

### New Tests (all passing)

| Test | Validates |
|------|-----------|
| `claim_creates_passphrase_and_session` | Full claim flow: hash created, cookie set, csrf_token returned |
| `claim_rejects_already_claimed` | 409 when passphrase_hash already exists |
| `claim_rejects_short_passphrase` | 400 for passphrases < 8 chars |
| `init_without_claim_works_as_before` | Backward compatibility: no claim = no cookie/csrf |
| `double_init_returns_409` | Existing behavior: config already exists |
| `config_status_includes_claimed_false` | `claimed: false` on fresh instance |
| `config_status_includes_claimed_true` | `claimed: true` after passphrase creation |
| `init_with_claim_produces_valid_session` | Session from claim is usable for authenticated requests |

---

## Open Issues

### Resolved in this session

| Issue | Resolution |
|-------|-----------|
| `create_passphrase_hash` atomicity | Check-then-write, consistent with `ensure_passphrase` |
| `claim.passphrase` validation rules | Minimum 8 characters, no maximum, no format requirement |
| Error shape for claim failures | 409 for already-claimed, 400 for invalid passphrase |
| Impact on `--reset-passphrase` | No change needed — reset overwrites the hash file; a subsequent claim is correctly rejected because the file exists |

### Deferred to Session 03

| Issue | Notes |
|-------|-------|
| EFF wordlist delivery mechanism | Static JSON file? Inline TypeScript array? Lazy-loaded module? Decide based on bundle impact. The wordlist is ~13KB gzipped. |
| Claim step position in onboarding wizard | Before Review (step 7) or integrated into Review? Decide based on UX flow. |
| `beforeunload` guard implementation | Prevent navigation away without saving passphrase. Need to ensure it doesn't fire after successful submit. |
| Frontend gate reordering in `+layout.svelte` | Check `configured`/`claimed` before auth for non-bearer users. The `claimed` field from `GET /api/settings/status` is now available. |
| CSRF token storage after claim | The frontend must store `csrf_token` from the init response the same way it does after login. |

---

## Exact Inputs for Session 03

### Read First
- `docs/roadmap/fresh-install-auth-ux/charter.md` — full charter
- `docs/roadmap/fresh-install-auth-ux/backend-contract.md` — API contract (created this session)
- `docs/roadmap/fresh-install-auth-ux/session-02-handoff.md` — this file

### Backend Contract Summary

The frontend needs to consume these changes:

1. **`GET /api/settings/status`** now returns `claimed: bool`. Use this to decide whether to show login or onboarding.

2. **`POST /api/settings/init`** accepts optional `claim: { passphrase: string }`:
   - If included and successful: response has `csrf_token` field + `Set-Cookie` header
   - If included and already claimed: 409
   - If omitted: existing behavior, no session

3. **Startup passphrase generation** is now conditional:
   - `--host 0.0.0.0`: auto-generates (unchanged)
   - `--host 127.0.0.1` (default): defers to browser claim flow

### Frontend Files to Modify (suggested)

1. **`dashboard/src/routes/+layout.svelte`** — Reorder gate: check config/claimed before auth for non-bearer users
2. **`dashboard/src/routes/onboarding/+page.svelte`** — Add claim step with passphrase generation
3. **`dashboard/src/routes/login/+page.svelte`** — Update copy for fresh-install context
4. **`dashboard/src/lib/api.ts`** — Ensure `settings.init()` passes `claim` and handles `Set-Cookie`
5. **`dashboard/src/lib/stores/auth.ts`** — Handle claim-time session bootstrap (store CSRF token)

### Frontend Files to Create (suggested)

1. **`dashboard/src/lib/components/onboarding/ClaimStep.svelte`** — Passphrase creation UI

### Test Expectations

- Fresh install in web mode: browser → onboarding → claim → dashboard (no login screen)
- Fresh install in Tauri mode: unchanged (bearer → onboarding → dashboard)
- Returning web user (session expired): → login → dashboard
- Configured web user (valid session): → dashboard directly
