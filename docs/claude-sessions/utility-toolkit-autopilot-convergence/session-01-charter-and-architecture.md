# Session 01: Charter And Architecture

Paste this into a new Claude Code session:

```md
Mission: Produce a final architecture charter that turns TuitBot into a utility-first toolkit while preserving autopilot as an optional orchestration layer.

Repository anchors:
- `README.md`
- `docs/architecture.md`
- `docs/mcp-reference.md`
- `crates/tuitbot-core/src/lib.rs`
- `crates/tuitbot-core/src/automation/mod.rs`
- `crates/tuitbot-mcp/src/lib.rs`
- `crates/tuitbot-mcp/src/server/mod.rs`
- `crates/tuitbot-mcp/src/tools/manifest.rs`

Tasks:
1. Audit the current architecture and MCP surface, then document the exact gap between “utility toolkit” behavior and current behavior.
2. Define the target architecture with three layers (Toolkit, Workflow, Autopilot) and assign concrete module ownership in this repository.
3. Define the target MCP profile model for the new system, including which tool families appear in each profile.
4. Produce a locked execution plan for Sessions 02–08 with scope boundaries, dependencies, and risk controls.
5. Record all non-negotiable design decisions with rationale; do not leave unresolved decisions.

Deliverables:
- `docs/roadmap/utility-toolkit-autopilot-convergence/charter.md`
- `docs/roadmap/utility-toolkit-autopilot-convergence/architecture-decisions.md`
- `docs/roadmap/utility-toolkit-autopilot-convergence/execution-plan.md`
- `docs/roadmap/utility-toolkit-autopilot-convergence/session-01-handoff.md`

Exit criteria:
- All deliverables exist and contain no TBD/TODO placeholders.
- Session 02 inputs are explicitly listed as file paths and commands.
- Architecture decisions are concrete enough to implement without reinterpretation.
```
