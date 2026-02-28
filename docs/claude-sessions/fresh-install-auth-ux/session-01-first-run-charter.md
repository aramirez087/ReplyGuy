# Session 01: First-Run Charter

Paste this into a new Claude Code session:

```md
Continuity
- Start from the current repository state only.

Mission
Define the implementation charter for the fresh-install auth UX epic without making substantive product-code changes.

Repository anchors
- dashboard/src/routes/+layout.svelte
- dashboard/src/routes/login/+page.svelte
- crates/tuitbot-server/src/main.rs
- crates/tuitbot-server/src/routes/settings.rs
- crates/tuitbot-server/src/routes/lan.rs
- crates/tuitbot-server/src/auth/middleware.rs
- docs/lan-mode.md

Tasks
1. Audit the current first-run, login, and LAN passphrase behavior across the anchors.
2. Write a charter that fixes the auth-before-onboarding inversion, defines instance claiming, and preserves the current security model.
3. Split the implementation into backend and frontend sessions with explicit risks, acceptance criteria, and file-level touch points.
4. Keep code changes limited to documentation updates needed to support the charter; do not implement the product changes yet.

Deliverables
- docs/roadmap/fresh-install-auth-ux/charter.md
- docs/roadmap/fresh-install-auth-ux/session-01-handoff.md

Quality gates
- If you touch code, run:
    cargo fmt --all && cargo fmt --all --check
    RUSTFLAGS="-D warnings" cargo test --workspace
    cargo clippy --workspace -- -D warnings

Exit criteria
- The charter defines the target UX, backend contract changes, frontend routing changes, key risks, and exact inputs for Session 02.
```
