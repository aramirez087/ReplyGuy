# Session 03: Frontend First-Run UX

Paste this into a new Claude Code session:

```md
Continue from Session 02 artifacts.

Continuity
- Read docs/roadmap/fresh-install-auth-ux/charter.md, docs/roadmap/fresh-install-auth-ux/backend-contract.md, and docs/roadmap/fresh-install-auth-ux/session-02-handoff.md before editing.

Mission
Implement the frontend claim, onboarding, login, and LAN UX so fresh installs avoid the passphrase dead end while returning users keep secure access.

Repository anchors
- dashboard/src/routes/+layout.svelte
- dashboard/src/routes/onboarding/+page.svelte
- dashboard/src/routes/login/+page.svelte
- dashboard/src/routes/(app)/settings/LanAccessSection.svelte
- dashboard/src/lib/api.ts
- dashboard/src/lib/stores/auth.ts
- docs/lan-mode.md

Tasks
1. Reorder the route gate so unconfigured instances reach onboarding before login, while configured web users without a session still land on login.
2. Consume the Session 02 init contract so setup completion establishes cookie auth and reveals claim-time passphrase guidance exactly once.
3. Improve login and LAN screens for clarity and accessibility, including actionable copy, live error messaging, reset confirmation, icon button labels, and visible focus states.
4. Keep Tauri bearer mode unchanged and update user-facing copy to match the new first-run flow.

Deliverables
- dashboard/src/routes/+layout.svelte
- dashboard/src/routes/onboarding/+page.svelte
- dashboard/src/routes/login/+page.svelte
- dashboard/src/routes/(app)/settings/LanAccessSection.svelte
- dashboard/src/lib/api.ts
- dashboard/src/lib/stores/auth.ts
- docs/lan-mode.md
- docs/roadmap/fresh-install-auth-ux/session-03-handoff.md

Quality gates
- Run:
    cargo fmt --all && cargo fmt --all --check
    RUSTFLAGS="-D warnings" cargo test --workspace
    cargo clippy --workspace -- -D warnings

Exit criteria
- Fresh installs can claim and enter the app without a log hunt, returning web users still authenticate normally, and the updated UI communicates passphrase actions clearly.
```
