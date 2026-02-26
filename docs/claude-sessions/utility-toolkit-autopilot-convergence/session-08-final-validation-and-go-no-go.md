# Session 08: Final Validation And Go No-Go

Paste this into a new Claude Code session:

```md
Continue from Session 07 artifacts.
Mission: Execute full-system validation and publish a release-quality go/no-go decision for Utility Toolkit + Autopilot Convergence.

Repository anchors:
- `docs/roadmap/utility-toolkit-autopilot-convergence/session-07-handoff.md`
- `docs/roadmap/utility-toolkit-autopilot-convergence/charter.md`
- `docs/roadmap/utility-toolkit-autopilot-convergence/architecture-decisions.md`
- `docs/roadmap/utility-toolkit-autopilot-convergence/execution-plan.md`
- `docs/roadmap/utility-toolkit-autopilot-convergence/session-07-docs-and-e2e.md`
- `scripts/check-mcp-manifests.sh`

Tasks:
1. Run full quality gates and record exact command outputs with timestamps.
2. Run manifest drift checks and targeted MCP boundary/conformance suites.
3. Validate at least one utility profile flow and one autopilot flow end-to-end.
4. Produce a go/no-go report with hard blockers, non-blocking risks, and release recommendation.
5. Produce a concise post-epic retrospective with follow-up actions outside this epic.

Deliverables:
- `docs/roadmap/utility-toolkit-autopilot-convergence/final-validation-report.md`
- `docs/roadmap/utility-toolkit-autopilot-convergence/go-no-go.md`
- `docs/roadmap/utility-toolkit-autopilot-convergence/post-epic-retrospective.md`
- `docs/roadmap/utility-toolkit-autopilot-convergence/session-08-handoff.md`

Quality gates:
- `cargo fmt --all && cargo fmt --all --check`
- `RUSTFLAGS="-D warnings" cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`

Exit criteria:
- All quality gates pass with no ignored failures.
- Go/no-go decision is explicit and justified with evidence.
- Final artifacts allow independent execution without prior session memory.
```
