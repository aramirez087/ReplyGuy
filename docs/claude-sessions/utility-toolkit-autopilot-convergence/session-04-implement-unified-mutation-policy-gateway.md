# Session 04: Implement Unified Mutation Policy Gateway

Paste this into a new Claude Code session:

```md
Continue from Session 03 artifacts.
Mission: Build a single mutation governance gateway used by both utility and workflow/autopilot mutation paths.

Repository anchors:
- `docs/roadmap/utility-toolkit-autopilot-convergence/session-03-handoff.md`
- `crates/tuitbot-core/src/mcp_policy/mod.rs`
- `crates/tuitbot-core/src/mcp_policy/evaluator.rs`
- `crates/tuitbot-core/src/storage/rate_limits.rs`
- `crates/tuitbot-core/src/storage/mutation_audit.rs`
- `crates/tuitbot-mcp/src/tools/workflow/policy_gate.rs`
- `crates/tuitbot-mcp/src/tools/idempotency.rs`
- `crates/tuitbot-mcp/src/tools/workflow/x_actions/write.rs`
- `crates/tuitbot-mcp/src/tools/workflow/x_actions/engage.rs`

Tasks:
1. Define a single policy gateway interface in core covering block rules, rate limits, idempotency, and audit recording.
2. Route all mutation-capable toolkit and workflow tools through this gateway before execution.
3. Remove duplicate mutation checks from scattered call sites and keep one authoritative path.
4. Add tests for allowed, blocked, and rate-limited mutation scenarios with expected error codes.
5. Document the policy flow with sequence steps and failure semantics.

Deliverables:
- `docs/roadmap/utility-toolkit-autopilot-convergence/session-04-policy-gateway.md`
- `docs/roadmap/utility-toolkit-autopilot-convergence/session-04-handoff.md`

Quality gates:
- `cargo fmt --all && cargo fmt --all --check`
- `RUSTFLAGS="-D warnings" cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`

Exit criteria:
- Every mutation path uses the same gateway.
- Duplicate policy code paths are removed.
- Session 05 inputs are explicit in the handoff.
```
