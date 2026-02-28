# Session 01 Handoff — First-Run Charter

**Date:** 2026-02-28
**Branch:** `feat/init_improvements`

---

## What Changed

This session produced documentation only — no source code was modified.

### Files Created

| File | Purpose |
|------|---------|
| `docs/roadmap/fresh-install-auth-ux/charter.md` | Full implementation charter: problem statement, audit findings, target UX, 6 design decisions, security model, 4-session breakdown with per-session file lists, acceptance criteria, and risks |
| `docs/roadmap/fresh-install-auth-ux/session-01-handoff.md` | This file |

### Key Findings from Audit

1. **Auth-before-onboarding inversion confirmed.** `+layout.svelte` lines 34–44 redirect to `/login` before the config check at lines 49–57. Web users on a fresh install hit a dead end.

2. **`/settings/init` is already auth-exempt.** The middleware exemption list in `middleware.rs:36–49` includes both `/settings/init` and `/api/settings/init`. This is the natural extension point for the claim flow — no new unauthenticated surface area needed.

3. **`AppState.passphrase_hash` is `RwLock<Option<String>>`** (`state.rs`). It already supports runtime updates, so the claim flow can set the hash without a server restart.

4. **`ensure_passphrase()` runs unconditionally** (`main.rs:84`). It generates a passphrase even on localhost-only starts where no web user will ever see the terminal output. This should be made conditional.

5. **The onboarding wizard has 8 steps** (Welcome → X API → Business → LLM → Language → Sources → Validate → Review). The claim step will be inserted before or at the Review step.

6. **`login/+page.svelte` has no recovery path.** The hint text only mentions `--host 0.0.0.0` for LAN access — nothing about what to do if the passphrase is lost.

---

## Design Decisions Made

Six decisions documented in the charter:

1. **Instance claiming via `POST /api/settings/init`** — extend with optional `claim` object containing passphrase.
2. **Frontend gate reordering** — check config/claimed status before auth for non-bearer users.
3. **Session bootstrap at claim time** — return `Set-Cookie` + CSRF from the init endpoint.
4. **Client-side passphrase generation** — EFF wordlist bundled as static asset.
5. **Deferred passphrase for Tauri** — skip claim step when in bearer mode.
6. **Conditional startup passphrase generation** — only auto-generate when `--host 0.0.0.0`.

---

## Open Issues

### Resolved in this session

| Issue | Resolution |
|-------|-----------|
| Where does the claim live in the API? | Inside `POST /api/settings/init` as an optional field — no new endpoint |
| Does claiming need its own endpoint? | No — atomic config+claim in one POST prevents partial states |
| What about the `claim` field in the TOML conversion? | The `claim` field must be stripped before JSON→TOML conversion (it's not a config field). Session 02 will handle this by extracting `claim` from the body before passing to `json_to_toml()` |
| Race condition on concurrent claims | First writer wins; `create_passphrase_hash` checks file existence before writing |

### Deferred to Session 02

| Issue | Notes |
|-------|-------|
| Exact `create_passphrase_hash` implementation | Should it use `O_EXCL` for atomicity, or is check-then-write sufficient given the one-shot nature of `/settings/init`? Session 02 should decide based on the existing `ensure_passphrase` pattern |
| `claim.passphrase` validation rules | Minimum length? Must be multi-word? Session 02 should define; suggested: minimum 8 characters, no maximum |
| Error shape for claim-specific failures | Should a claim failure return 409 (already claimed) vs 400 (invalid passphrase)? Suggested: 409 for already-claimed, 400 for invalid passphrase format |
| Impact on `--reset-passphrase` CLI flag | No change needed — it already overwrites the hash file. But need to verify the flow: reset → hash file exists → claim rejected (correct, user should use login) |

### Deferred to Session 03

| Issue | Notes |
|-------|-------|
| EFF wordlist delivery mechanism | Static JSON file? Inline TypeScript array? Lazy-loaded module? Session 03 should decide based on bundle impact |
| Claim step position in onboarding wizard | Before Review (step 7) or as part of Review? Session 03 should decide based on UX flow |
| `beforeunload` guard implementation | Need to ensure it doesn't fire after successful claim submit |

---

## Exact Inputs for Session 02

### Read First
- `docs/roadmap/fresh-install-auth-ux/charter.md` — the full charter (created this session)
- `docs/roadmap/fresh-install-auth-ux/session-01-handoff.md` — this file

### Source Files to Modify
1. **`crates/tuitbot-core/src/auth/passphrase.rs`** — Add `create_passphrase_hash(data_dir, plaintext) -> Result<(), AuthError>` that:
   - Checks if hash file exists → returns error if yes
   - Hashes plaintext with bcrypt (cost 12, matching existing `hash_passphrase`)
   - Writes hash to `data_dir/passphrase_hash` with 0600 permissions
   - Never logs the plaintext

2. **`crates/tuitbot-server/src/routes/settings.rs`** — Extend `init_settings`:
   - Add `#[serde(default)] claim: Option<ClaimRequest>` to a new request struct (currently accepts raw `Value`)
   - Extract `claim` before JSON→TOML conversion
   - If claim present + unclaimed: create hash, create session, add `Set-Cookie` + `csrf_token` to response
   - If claim present + already claimed: return 409
   - If claim absent: current behavior exactly

3. **`crates/tuitbot-server/src/routes/settings.rs`** — Extend `config_status`:
   - Add `claimed: bool` to response (check if `data_dir/passphrase_hash` exists)

4. **`crates/tuitbot-server/src/main.rs`** — Make `ensure_passphrase` conditional:
   - If `cli.host == "0.0.0.0"`: run `ensure_passphrase` as today (print to terminal)
   - If `cli.host == "127.0.0.1"`: load hash if exists, skip generation if not
   - This keeps backward compatibility for explicit LAN mode users

### Test Expectations
- `POST /api/settings/init` with `claim.passphrase` on unclaimed instance → 200 + `Set-Cookie` + hash file created
- `POST /api/settings/init` with `claim.passphrase` on already-claimed instance → 409
- `POST /api/settings/init` without `claim` → current behavior (200, no cookie)
- `GET /api/settings/status` → includes `claimed: bool`

### CI Must Pass
```bash
cargo fmt --all && cargo fmt --all --check
RUSTFLAGS="-D warnings" cargo test --workspace
cargo clippy --workspace -- -D warnings
```
