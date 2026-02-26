# Session 06: Build Workflow Composers On Top Of Toolkit

Paste this into a new Claude Code session:

```md
Continue from Session 05 artifacts.
Mission: Rebuild workflow/composite tools as explicit compositions of toolkit primitives to offer both remote-control utilities and high-level autopilot building blocks.

Repository anchors:
- `docs/roadmap/utility-toolkit-autopilot-convergence/session-05-handoff.md`
- `crates/tuitbot-mcp/src/tools/workflow/composite/mod.rs`
- `crates/tuitbot-mcp/src/tools/workflow/composite/find_opportunities.rs`
- `crates/tuitbot-mcp/src/tools/workflow/composite/draft_replies.rs`
- `crates/tuitbot-mcp/src/tools/workflow/composite/propose_queue.rs`
- `crates/tuitbot-mcp/src/tools/workflow/composite/thread_plan.rs`
- `crates/tuitbot-mcp/src/tools/workflow/composite/tests.rs`
- `crates/tuitbot-mcp/src/tools/workflow/discovery.rs`
- `crates/tuitbot-mcp/src/tools/workflow/content.rs`

Tasks:
1. Define atomic workflow steps (discover, draft, queue, publish, thread-plan) with explicit typed IO contracts.
2. Refactor composite tools to call only those atomic steps and toolkit interfaces; remove hidden side effects.
3. Add one deterministic orchestrator entrypoint for autopilot cycles that composes the same atomic steps.
4. Expand integration tests to validate step composition, error propagation, and response envelopes.
5. Document when to use utility tools vs workflow composites.

Deliverables:
- `docs/roadmap/utility-toolkit-autopilot-convergence/session-06-workflow-composers.md`
- `docs/roadmap/utility-toolkit-autopilot-convergence/session-06-handoff.md`

Quality gates:
- `cargo fmt --all && cargo fmt --all --check`
- `RUSTFLAGS="-D warnings" cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`

Exit criteria:
- Composite tools are transparent compositions, not hidden monoliths.
- Utility and workflow layers are clearly separated and test-verified.
- Session 07 inputs are explicit in the handoff.
```
