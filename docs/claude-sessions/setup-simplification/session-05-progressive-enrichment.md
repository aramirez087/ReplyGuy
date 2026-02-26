# Session 05: Progressive Enrichment (Without Setup Bloat)

Paste this into a new Claude Code session:

```md
Continue from Session 04 artifacts.

Mission:
Implement progressive profiling so advanced fields (brand voice, persona, targeting depth) are collected only when they become useful.

Intent:
- Keep first-run friction low.
- Preserve product depth for users who want stronger quality and control.

Target areas:
- `crates/tuitbot-cli/src/commands/settings/`
- `crates/tuitbot-cli/src/commands/init/`
- `crates/tuitbot-core/src/config/mod.rs`
- Any UX text where advanced setup is introduced.

Tasks:
1. Add a clear "enrich profile" workflow after initial setup.
2. Introduce staged recommendations based on current config completeness.
3. Ensure advanced additions are reversible/editable via CLI.
4. Keep messaging concise and non-blocking.

Deliverables:
1. Progressive enrichment flow implemented.
2. Tests for staged behavior and config updates.
3. `docs/roadmap/init-simplification/session-05-handoff.md` with:
   - Enrichment stages
   - Trigger rules
   - UX copy inventory for follow-up polish

Quality gates:
- `cargo fmt --all && cargo fmt --all --check`
- `RUSTFLAGS="-D warnings" cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`

Exit criteria:
- Advanced profile data is opt-in and staged.
- Initial setup remains lean.
- Tests pass.
```
