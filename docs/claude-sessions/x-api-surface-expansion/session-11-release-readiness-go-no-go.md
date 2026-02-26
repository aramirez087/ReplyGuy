# Session 11: Release Readiness and Go/No-Go

Paste this into a new Claude Code session:

```md
Continue from Session 10 artifacts.

Mission:
Finalize the X API surface expansion with a strict, evidence-based go/no-go package.

Tasks:
1. Run full quality and conformance validation.
2. Verify profile safety boundaries across `readonly`, `api-readonly`, `write`, `admin`.
3. Verify universal and generated tool paths for:
   - reliability
   - deterministic outputs
   - error envelope consistency
4. Confirm docs/manifests/versioning are internally consistent.
5. Produce final recommendation: GO or NO-GO with blocking issues if any.

Deliverables:
1. `docs/roadmap/x-api-surface-expansion/release-checklist.md`
2. `docs/roadmap/x-api-surface-expansion/go-no-go-report.md` including:
   - completed criteria
   - outstanding gaps
   - severity-ranked issue list
   - recommendation with rationale
3. Updated machine-readable manifests and coverage artifacts.

Mandatory command runbook:
- `cargo fmt --all && cargo fmt --all --check`
- `RUSTFLAGS="-D warnings" cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`
- conformance harness command from Session 09

Exit criteria:
- All blocking requirements are met or explicitly listed as blockers.
- Final recommendation is decisive and evidence-backed.
- Initiative is ready for controlled rollout or scoped remediation.
```

