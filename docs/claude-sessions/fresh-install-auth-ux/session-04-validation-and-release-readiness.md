# Session 04: Validation And Release Readiness

Paste this into a new Claude Code session:

```md
Continue from Session 03 artifacts.

Continuity
- Read docs/roadmap/fresh-install-auth-ux/charter.md and all prior handoffs under docs/roadmap/fresh-install-auth-ux/ before making final changes.

Mission
Validate the end-to-end fresh-install auth UX, close small gaps, and publish a go or no-go release-readiness report.

Repository anchors
- docs/roadmap/fresh-install-auth-ux/charter.md
- docs/roadmap/fresh-install-auth-ux/backend-contract.md
- dashboard/src/routes/+layout.svelte
- dashboard/src/routes/onboarding/+page.svelte
- dashboard/src/routes/login/+page.svelte
- dashboard/src/routes/(app)/settings/LanAccessSection.svelte
- crates/tuitbot-server/src/routes/settings.rs
- docs/lan-mode.md

Tasks
1. Verify the implemented behavior against the charter for fresh install, returning login, and LAN passphrase reset flows.
2. Run the full quality gates and fix only small validation findings that fit in one session; otherwise document blockers clearly.
3. Reconcile any mismatched docs, UX copy, or edge-case handling discovered during validation.
4. Publish a release-readiness report with go or no-go status, residual risks, and recommended follow-up work.

Deliverables
- docs/roadmap/fresh-install-auth-ux/release-readiness.md
- docs/roadmap/fresh-install-auth-ux/session-04-handoff.md

Quality gates
- Run:
    cargo fmt --all && cargo fmt --all --check
    RUSTFLAGS="-D warnings" cargo test --workspace
    cargo clippy --workspace -- -D warnings

Exit criteria
- The checks pass or blockers are explicitly documented, the final report states ship readiness, and the handoff closes the epic with any remaining risks called out.
```
