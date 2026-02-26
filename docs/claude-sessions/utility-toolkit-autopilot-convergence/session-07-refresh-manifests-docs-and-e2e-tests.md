# Session 07: Refresh Manifests Docs And E2E Tests

Paste this into a new Claude Code session:

```md
Continue from Session 06 artifacts.
Mission: Align docs, manifests, and end-to-end verification with the new utility-first architecture and autopilot overlay.

Repository anchors:
- `docs/roadmap/utility-toolkit-autopilot-convergence/session-06-handoff.md`
- `README.md`
- `docs/mcp-reference.md`
- `docs/architecture.md`
- `docs/operations.md`
- `scripts/generate-mcp-manifests.sh`
- `scripts/check-mcp-manifests.sh`
- `crates/tuitbot-mcp/src/tools/boundary_tests.rs`
- `crates/tuitbot-mcp/src/tools/conformance_tests/mod.rs`

Tasks:
1. Rewrite product and architecture docs to present Toolkit, Workflow, and Autopilot as distinct layers with profile guidance.
2. Regenerate all MCP manifests and ensure committed JSON matches runtime generation.
3. Add/expand end-to-end tests covering utility-only flows and autopilot-composed flows.
4. Add explicit operational runbooks for profile selection and safe mutation usage.
5. Verify examples and command snippets are executable and consistent.

Deliverables:
- `docs/roadmap/utility-toolkit-autopilot-convergence/session-07-docs-and-e2e.md`
- `docs/roadmap/utility-toolkit-autopilot-convergence/session-07-handoff.md`

Quality gates:
- `cargo fmt --all && cargo fmt --all --check`
- `RUSTFLAGS="-D warnings" cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`

Exit criteria:
- Docs and manifests reflect the implemented architecture with no contradictions.
- E2E coverage includes both utility and autopilot paths.
- Session 08 inputs are explicit in the handoff.
```
