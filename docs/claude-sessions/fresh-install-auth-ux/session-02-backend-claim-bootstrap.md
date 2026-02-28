# Session 02: Backend Claim Bootstrap

Paste this into a new Claude Code session:

```md
Continue from Session 01 artifacts.

Continuity
- Read docs/roadmap/fresh-install-auth-ux/charter.md and docs/roadmap/fresh-install-auth-ux/session-01-handoff.md before editing.

Mission
Implement the backend claim bootstrap flow so a fresh install can establish its passphrase and receive a valid web session during initial setup.

Repository anchors
- crates/tuitbot-server/src/routes/settings.rs
- crates/tuitbot-server/src/auth/middleware.rs
- crates/tuitbot-core/src/auth/passphrase.rs
- crates/tuitbot-core/src/auth/session.rs
- crates/tuitbot-server/src/state.rs
- docs/lan-mode.md

Tasks
1. Add a safe first-claim path to settings initialization that creates a passphrase hash only once, never persists plaintext, and avoids weakening unauthenticated endpoints.
2. Return the data needed to establish cookie auth at setup completion while keeping the API compatible for existing callers wherever practical.
3. Add or update tests for fresh-install init, auth edge cases, and passphrase bootstrap behavior.
4. Document the backend contract and any operator-facing behavior changes.

Deliverables
- crates/tuitbot-server/src/routes/settings.rs
- crates/tuitbot-core/src/auth/passphrase.rs
- crates/tuitbot-server/tests/fresh_install_auth.rs
- docs/lan-mode.md
- docs/roadmap/fresh-install-auth-ux/backend-contract.md
- docs/roadmap/fresh-install-auth-ux/session-02-handoff.md

Quality gates
- Run:
    cargo fmt --all && cargo fmt --all --check
    RUSTFLAGS="-D warnings" cargo test --workspace
    cargo clippy --workspace -- -D warnings

Exit criteria
- A new install can complete initial setup without pre-reading service logs, the security model remains intact, and Session 03 has a stable backend contract to consume.
```
