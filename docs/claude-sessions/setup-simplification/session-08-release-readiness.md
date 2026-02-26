# Session 08: Release Readiness and Go/No-Go

Paste this into a new Claude Code session:

```md
Continue from Session 07 artifacts.

Mission:
Finalize the setup simplification initiative with a strict go/no-go decision package.

Tasks:
1. Run full validation suite and record outcomes.
2. Perform UX sanity check on Quickstart and Advanced flows.
3. Verify docs + commands + tests are consistent.
4. Produce a launch checklist and rollback strategy for implementation defects.

Deliverables:
1. `docs/roadmap/init-simplification/release-checklist.md`
2. `docs/roadmap/init-simplification/go-no-go-report.md` containing:
   - Completed criteria
   - Known risks
   - Severity-ranked issue list
   - Final recommendation: GO or NO-GO

Mandatory command runbook:
- `cargo fmt --all && cargo fmt --all --check`
- `RUSTFLAGS="-D warnings" cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`

Exit criteria:
- Go/no-go report is complete, factual, and decisive.
- All blocking checks pass or are explicitly documented as blockers.
- The initiative is ready for controlled release.
```
