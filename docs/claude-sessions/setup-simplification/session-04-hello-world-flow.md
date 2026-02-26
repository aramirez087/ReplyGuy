# Session 04: True "Hello World" Flow

Paste this into a new Claude Code session:

```md
Continue from Session 03 artifacts.

Mission:
Create a first-success experience where users can produce a meaningful output immediately after setup, without deep profile work.

Goal:
A user should be able to go from install to first successful bot action with a tiny number of commands and minimal cognitive load.

Likely touch points:
- `crates/tuitbot-cli/src/commands/init/mod.rs`
- `crates/tuitbot-cli/src/commands/test.rs`
- `crates/tuitbot-cli/src/commands/tick.rs`
- `crates/tuitbot-cli/src/commands/run.rs`
- Possibly add a dedicated command for first-run validation/demo.

Tasks:
1. Define and implement a "first success" command path (example: quick setup + dry-run action).
2. Ensure users can verify correctness before any risky mutation.
3. Return clear, human-readable guidance for next steps.
4. Make failures actionable with precise remediations.

Deliverables:
1. CLI implementation for first-success flow.
2. Tests covering happy path and common failure paths.
3. `docs/roadmap/init-simplification/session-04-handoff.md` with:
   - New command sequence
   - Expected outputs
   - Failure matrix and fixes

Quality gates:
- `cargo fmt --all && cargo fmt --all --check`
- `RUSTFLAGS="-D warnings" cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`

Exit criteria:
- A new user can complete a real first-success path quickly.
- No advanced profile fields are required to prove value.
- Test coverage includes the new flow.
```
