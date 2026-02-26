# Session 05: Refactor Autopilot To Use Toolkit Layer

Paste this into a new Claude Code session:

```md
Continue from Session 04 artifacts.
Mission: Refactor autopilot loops to consume toolkit interfaces so autonomous behavior is preserved without bypassing utility-layer boundaries.

Repository anchors:
- `docs/roadmap/utility-toolkit-autopilot-convergence/session-04-handoff.md`
- `crates/tuitbot-core/src/automation/mod.rs`
- `crates/tuitbot-core/src/automation/discovery_loop.rs`
- `crates/tuitbot-core/src/automation/mentions_loop.rs`
- `crates/tuitbot-core/src/automation/content_loop.rs`
- `crates/tuitbot-core/src/automation/thread_loop.rs`
- `crates/tuitbot-core/src/automation/approval_poster.rs`
- `crates/tuitbot-core/src/automation/adapters.rs`
- `crates/tuitbot-core/src/startup.rs`

Tasks:
1. Inject toolkit trait dependencies into automation loops and adapters using clear constructor boundaries.
2. Replace direct X client calls in loops with toolkit calls while preserving existing scheduling and scoring behavior.
3. Keep composer and autopilot mode semantics intact; verify mode checks still gate loop execution correctly.
4. Add/upgrade loop tests using mock toolkit implementations for deterministic assertions.
5. Document the new autopilot-to-toolkit call graph.

Deliverables:
- `docs/roadmap/utility-toolkit-autopilot-convergence/session-05-autopilot-refactor.md`
- `docs/roadmap/utility-toolkit-autopilot-convergence/session-05-handoff.md`

Quality gates:
- `cargo fmt --all && cargo fmt --all --check`
- `RUSTFLAGS="-D warnings" cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`

Exit criteria:
- Automation modules no longer call raw X clients directly.
- Existing autopilot behavior is preserved through tests.
- Session 06 inputs are explicit in the handoff.
```
