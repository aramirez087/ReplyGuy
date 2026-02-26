# Session 02: Build Utility Toolkit Core Layer

Paste this into a new Claude Code session:

```md
Continue from Session 01 artifacts.
Mission: Implement the new stateless Toolkit layer in `tuitbot-core` and make it the only path for low-level X operations.

Repository anchors:
- `docs/roadmap/utility-toolkit-autopilot-convergence/charter.md`
- `docs/roadmap/utility-toolkit-autopilot-convergence/architecture-decisions.md`
- `crates/tuitbot-core/src/lib.rs`
- `crates/tuitbot-core/src/x_api/client.rs`
- `crates/tuitbot-core/src/x_api/types.rs`
- `crates/tuitbot-mcp/src/tools/workflow/x_actions/read.rs`
- `crates/tuitbot-mcp/src/tools/workflow/x_actions/write.rs`
- `crates/tuitbot-mcp/src/tools/workflow/x_actions/engage.rs`
- `crates/tuitbot-mcp/src/tools/workflow/x_actions/media.rs`

Tasks:
1. Create `crates/tuitbot-core/src/toolkit/` with typed request/response models and traits for read, write, engage, and media operations.
2. Implement toolkit adapters backed by existing X API clients; keep toolkit logic stateless and free of workflow policy.
3. Refactor MCP `x_actions/*` modules to call toolkit interfaces instead of raw client calls.
4. Add focused unit tests for toolkit adapters and error mapping.
5. Update core module exports so toolkit interfaces are first-class and discoverable.

Deliverables:
- `crates/tuitbot-core/src/toolkit/mod.rs`
- `crates/tuitbot-core/src/toolkit/read.rs`
- `crates/tuitbot-core/src/toolkit/write.rs`
- `crates/tuitbot-core/src/toolkit/engage.rs`
- `crates/tuitbot-core/src/toolkit/media.rs`
- `docs/roadmap/utility-toolkit-autopilot-convergence/session-02-toolkit-core.md`
- `docs/roadmap/utility-toolkit-autopilot-convergence/session-02-handoff.md`

Quality gates:
- `cargo fmt --all && cargo fmt --all --check`
- `RUSTFLAGS="-D warnings" cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`

Exit criteria:
- All low-level X actions flow through toolkit traits.
- Toolkit tests pass and verify deterministic error translation.
- Session 03 inputs are explicit in the handoff.
```
