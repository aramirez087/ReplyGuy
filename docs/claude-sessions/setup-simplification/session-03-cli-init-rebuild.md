# Session 03: Rebuild `tuitbot init` for Speed

Paste this into a new Claude Code session:

```md
Continue from Session 02 artifacts.

Mission:
Rebuild `tuitbot init` so first-time users are not forced through heavyweight setup.

Required UX:
- Default path: Quickstart.
- Explicit path: Advanced.
- Clear command ergonomics.

Target files:
- `crates/tuitbot-cli/src/commands/mod.rs`
- `crates/tuitbot-cli/src/main.rs`
- `crates/tuitbot-cli/src/commands/init/mod.rs`
- `crates/tuitbot-cli/src/commands/init/steps.rs`
- `crates/tuitbot-cli/src/commands/init/prompts.rs`
- `crates/tuitbot-cli/src/commands/init/render.rs`
- `crates/tuitbot-cli/src/commands/init/tests.rs`

Tasks:
1. Add explicit setup mode selection (flag and/or prompt) with Quickstart as default.
2. In Quickstart, ask only critical inputs and apply safe defaults.
3. Keep advanced setup available but out of the default critical path.
4. Improve prompt text to be decisional and short.
5. Ensure resulting config is immediately usable.

Deliverables:
1. Working implementation.
2. Updated tests for mode behavior and rendered config correctness.
3. `docs/roadmap/init-simplification/session-03-handoff.md` with:
   - New CLI examples
   - Prompt count comparison (before vs after)
   - Remaining gaps for first-run success flow

Quality gates:
- `cargo fmt --all && cargo fmt --all --check`
- `RUSTFLAGS="-D warnings" cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`

Exit criteria:
- Quickstart is the default flow.
- Prompt count is materially reduced.
- Tests and lint pass.
```
