# Session 03: Expose Flat MCP Utility Profiles

Paste this into a new Claude Code session:

```md
Continue from Session 02 artifacts.
Mission: Expose a flat utility-first MCP surface with explicit profiles that separate raw toolkit actions from higher-level workflow tools.

Repository anchors:
- `docs/roadmap/utility-toolkit-autopilot-convergence/session-02-handoff.md`
- `crates/tuitbot-mcp/src/state.rs`
- `crates/tuitbot-mcp/src/lib.rs`
- `crates/tuitbot-mcp/src/server/mod.rs`
- `crates/tuitbot-mcp/src/server/write.rs`
- `crates/tuitbot-mcp/src/server/readonly.rs`
- `crates/tuitbot-mcp/src/spec/endpoints.rs`
- `crates/tuitbot-mcp/src/spec/generator.rs`
- `crates/tuitbot-mcp/src/tools/manifest.rs`
- `crates/tuitbot-cli/src/commands/mcp.rs`

Tasks:
1. Implement new profile routing for a utility-readonly profile and a utility-write profile backed by toolkit tools.
2. Ensure each spec endpoint maps to exactly one utility tool registration with deterministic naming.
3. Restrict workflow and autopilot tools to non-utility profiles; enforce this structurally in server wiring.
4. Extend manifest generation to emit profile-level summaries and verify counts in tests.
5. Regenerate committed manifests and update profile docs references.

Deliverables:
- `docs/generated/mcp-manifest-utility-readonly.json`
- `docs/generated/mcp-manifest-utility-write.json`
- `docs/roadmap/utility-toolkit-autopilot-convergence/session-03-profiles.md`
- `docs/roadmap/utility-toolkit-autopilot-convergence/session-03-handoff.md`

Quality gates:
- `cargo fmt --all && cargo fmt --all --check`
- `RUSTFLAGS="-D warnings" cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`

Exit criteria:
- Utility profiles expose only toolkit-oriented tools.
- Boundary tests enforce profile isolation and zero forbidden tool leakage.
- Session 04 inputs are explicit in the handoff.
```
